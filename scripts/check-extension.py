"""Extension self-checks: manifests parse, snippets are well-formed, and every
Tree-sitter node referenced by languages/v/*.scm exists in the pinned
grammar's generated parser (Zed builds from the grammar's checked-in
src/parser.c and fails the whole language on one bad query).

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
            used = sorted(set(re.findall(r"\(([a-z_][a-z0-9_]*)(?=[\s)])", text)))
            bad = [u for u in used if u not in node_types]
            if bad and node_types:
                fail(f"{scm_path.name}: unknown node types for pinned parser: {bad}")
            else:
                print(f"{scm_path.name}: {len(used)} named nodes, all known")

    if FAILURES:
        print(f"{len(FAILURES)} failure(s)")
        return 1
    print("all extension checks passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
