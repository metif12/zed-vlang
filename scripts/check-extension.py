"""Extension self-checks: manifests parse, snippets are well-formed with
unique prefixes (one prefix, one snippet), README/docs code fences parse,
docs links resolve, and every Tree-sitter node referenced by
languages/v/*.scm exists in the pinned grammar's generated parser (Zed
builds from the grammar's checked-in src/parser.c and fails the whole
language on one bad query).

Usage: python3 scripts/check-extension.py  (exit nonzero on violation)
"""
import json
import re
import sys
import tomllib
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FAILURES: list[str] = []


def fail(msg: str) -> None:
    FAILURES.append(msg)
    print(f"FAIL: {msg}")


def main() -> int:
    ext = tomllib.loads((ROOT / "extension.toml").read_text(encoding="utf-8"))
    for key in ("id", "name", "version", "schema_version"):
        if key not in ext:
            fail(f"extension.toml: missing key {key!r}")
    gram = ext.get("grammars", {}).get("v", {})
    rev = gram.get("rev", "")
    if not re.fullmatch(r"[0-9a-f]{40}", rev):
        fail(f"extension.toml: grammars.v.rev must be a full 40-char SHA, got {rev!r}")
    for rel in ext.get("snippets", []):
        if not (ROOT / rel).is_file():
            fail(f"extension.toml: snippet file missing: {rel}")

    cfg = tomllib.loads((ROOT / "languages" / "v" / "config.toml").read_text(encoding="utf-8"))
    if cfg.get("grammar") != "v":
        fail("languages/v/config.toml: grammar must be 'v'")

    # Every regex-valued key must compile, and `first_line_pattern` must both
    # match a real V shebang and reject a non-V one. A pattern that matches
    # everything is worse than none: it would claim every extensionless script.
    for key in ("first_line_pattern", "increase_indent_pattern",
                "decrease_indent_pattern", "collapsed_placeholder"):
        if key in cfg and key != "collapsed_placeholder":
            try:
                re.compile(cfg[key])
            except re.error as e:
                fail(f"languages/v/config.toml: {key} is not a valid regex: {e}")
    shebang = cfg.get("first_line_pattern")
    if shebang:
        must_match = ["#!/usr/bin/env v", "#!/usr/bin/v", "#!/usr/bin/env v run",
                      "#!/usr/bin/env -S v run"]
        must_not = ["#!/usr/bin/env python3", "#!/bin/sh"]
        for s in must_match:
            if not re.match(shebang, s):
                fail(f"languages/v/config.toml: first_line_pattern misses V shebang {s!r}")
        for s in must_not:
            if re.match(shebang, s):
                fail(f"languages/v/config.toml: first_line_pattern wrongly matches {s!r}")
        print(f"languages/v/config.toml: first_line_pattern ok ({len(must_match)} V shebangs, {len(must_not)} rejections)")

    # Zed loads exactly ten Tree-sitter query assets per language. Report
    # coverage so a newly-shippable asset cannot be forgotten silently.
    supported_queries = ["highlights", "brackets", "outline", "indents",
                         "injections", "overrides", "redactions", "runnables",
                         "debugger", "textobjects"]
    lang_dir = ROOT / "languages" / "v"
    present = [q for q in supported_queries if (lang_dir / f"{q}.scm").is_file()]
    missing = [q for q in supported_queries if q not in present]
    print(f"languages/v: {len(present)}/{len(supported_queries)} Zed query assets present")
    if missing:
        print(f"  not shipped: {', '.join(missing)}")

    # overrides.scm only configures anything when something consumes its
    # scopes: `brackets[].not_in` or a `[overrides.<scope>]` table. Without a
    # consumer the file is inert (it shipped that way in 0.6.0).
    overrides_scm = lang_dir / "overrides.scm"
    if overrides_scm.is_file():
        scopes = set()
        for line in overrides_scm.read_text(encoding="utf-8").splitlines():
            line = line.split(";")[0].strip()
            m = re.search(r"@([a-z_]+)(?:\.inclusive)?\s*$", line)
            if m:
                scopes.add(m.group(1))
        consumed = set(cfg.get("overrides", {}))
        for br in cfg.get("brackets", []) or []:
            consumed.update(br.get("not_in", []) or [])
        for br in cfg.get("disabled_scopes_by_bracket_ix", []) or []:
            consumed.update(br)
        if scopes and not (scopes & consumed):
            fail(f"languages/v/overrides.scm defines scope(s) {sorted(scopes)} that "
                 f"nothing consumes: add them to a brackets[].not_in list or an "
                 f"[overrides.<scope>] table, or the file is inert")

    total = 0
    for path in sorted((ROOT / "snippets").glob("*.json")):
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as e:
            fail(f"{path.name}: invalid JSON: {e}")
            continue
        for name, snip in data.items():
            total += 1
            if not isinstance(snip.get("prefix"), str):
                fail(f"{path.name}:{name}: prefix must be a string")
            if not isinstance(snip.get("body"), list) or not snip["body"]:
                fail(f"{path.name}:{name}: body must be a non-empty list")
        prefixes = [snip["prefix"] for snip in data.values()
                    if isinstance(snip.get("prefix"), str)]
        dupes = sorted({p for p in set(prefixes) if prefixes.count(p) > 1})
        if dupes:
            fail(f"{path.name}: prefixes offered twice (one prefix, one "
                 f"snippet — upstream PR #602): {dupes}")
    print(f"snippets: {total} entries across snippets/*.json")
    if total == 0:
        fail("no snippets found")

    if re.fullmatch(r"[0-9a-f]{40}", rev):
        url = (
            f"https://raw.githubusercontent.com/undivisible/tree-sitter-v/"
            f"{rev}/src/node-types.json"
        )
        try:
            with urllib.request.urlopen(url, timeout=60) as resp:
                node_types = {n["type"] for n in json.loads(resp.read().decode("utf-8"))}
        except Exception as e:  # noqa: BLE001 - report, don't crash CI
            fail(f"could not fetch node-types.json at {rev[:12]}: {e}")
            node_types = set()
        for scm_path in sorted((ROOT / "languages" / "v").glob("*.scm")):
            text = scm_path.read_text(encoding="utf-8")
            # Drop full-line `;` comments first (prose like "(task tags …)"
            # is not a node), then #predicates ((#eq? ...), (#match? ...),
            # (#set! tag ...)): they reference captures, never nodes.
            text = "\n".join(
                line for line in text.splitlines()
                if not line.lstrip().startswith(";")
            )
            text = re.sub(r"\(#[^()\n]*\)", "", text)
            used = sorted(set(re.findall(r"\(([a-z_][a-z0-9_]*)(?=[\s)])", text)))
            bad = [u for u in used if u not in node_types]
            if bad and node_types:
                fail(f"{scm_path.name}: unknown node types for pinned parser: {bad}")
            else:
                print(f"{scm_path.name}: {len(used)} named nodes, all known")

    md_files = [ROOT / "README.md"] + sorted((ROOT / "docs").glob("*.md"))
    fence = re.compile(r"```(\w+)\n(.*?)```", re.S)
    for md_path in md_files:
        if not md_path.exists():
            continue
        for lang, body in fence.findall(md_path.read_text(encoding="utf-8")):
            try:
                if lang == "json":
                    json.loads(body)
                elif lang == "toml":
                    tomllib.loads(body)
                else:
                    continue
            except Exception as e:  # noqa: BLE001 - report, don't crash CI
                fail(f"{md_path.name}: unparsable ```{lang} block: {e}")
    print(f"code fences parse in {len([p for p in md_files if p.exists()])} markdown files")

    # The README tasks block is the copy-paste deliverable: Zed wants a JSON
    # array, each entry needs a unique label and a command.
    for lang, body in fence.findall((ROOT / "README.md").read_text(encoding="utf-8")):
        if lang != "json" or '"label"' not in body:
            continue
        try:
            tasks = json.loads(body)
        except json.JSONDecodeError:
            continue  # already reported by the fence loop above
        if not isinstance(tasks, list):
            fail("README.md: tasks block must be a JSON array (Zed rejects a bare object)")
            continue
        labels = [t.get("label") if isinstance(t, dict) else None for t in tasks]
        if any(not isinstance(l, str) or not l for l in labels):
            fail("README.md: every task needs a non-empty string label")
        no_cmd = [l for t, l in zip(tasks, labels)
                  if isinstance(t, dict) and not isinstance(t.get("command"), str)]
        if no_cmd:
            fail(f"README.md: tasks without a command: {no_cmd}")
        dupes = sorted({l for l in labels if l and labels.count(l) > 1})
        if dupes:
            fail(f"README.md: duplicate task labels: {dupes}")
        print(f"README.md: {len(tasks)} tasks, labels unique")

        # Zed runs a task in the user's login shell, which is PowerShell on
        # Windows and bash/zsh elsewhere. A `command` holding shell syntax
        # therefore cannot be portable, and the README presents this array as
        # the copy-paste deliverable. Keep every command a bare executable and
        # push preconditions into args/docs.
        shellish = re.compile(r"[;&|]|\b(if|then|else|fi|for|do|done)\b"
                              r"|\$env:|Get-Command|Test-Path|Write-(Error|Output)")
        offenders = [(t.get("label"), t.get("command")) for t in tasks
                     if isinstance(t, dict) and isinstance(t.get("command"), str)
                     and shellish.search(t["command"])]
        for label, command in offenders:
            fail(f"README.md: task {label!r} puts shell syntax in `command` "
                 f"({command!r}); use a bare executable plus args so the task "
                 f"works in PowerShell and POSIX shells alike")
        if not offenders:
            print(f"README.md: all {len(tasks)} task commands are shell-free")

    link = re.compile(r"\]\((\./[^)#\s]+)(?:#[^)\s]*)?\)")
    for md_path in sorted((ROOT / "docs").glob("*.md")):
        for target in link.findall(md_path.read_text(encoding="utf-8")):
            if not (ROOT / "docs" / target).exists():
                fail(f"{md_path.name}: links to missing {target}")
    print("docs relative links resolve")

    if FAILURES:
        print(f"{len(FAILURES)} failure(s)")
        return 1
    print("all extension checks passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
