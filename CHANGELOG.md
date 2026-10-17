# Changelog

All notable changes, per release tag. Untagged work sits under
`Unreleased`.

## Unreleased

- `language_servers.vls.settings` is now forwarded to VLS verbatim via
  `language_server_workspace_configuration`, so VLS's two settings actually
  take effect. Measured: driving the pinned server over LSP,
  `{"settings":{"vls":{"inlayHints":false}}}` takes the inlay-hint count from
  1 to 0 and `{"inlayHints":true}` brings it back. Zed pushes this payload on
  every settings change, which is how a push-configuration server like VLS is
  reached.
- **Corrected: there is no `coverageDirectory` setting.** The install guide
  listed it, and it came from a guess — VLS silently ignores it. The server
  has exactly two settings, both booleans: `inlayHints` and `diagnostics`.
- **Corrected: the key is camelCase `inlayHints`.** The snake_case
  `inlay_hints` used for the field in V's own source is ignored, because the
  field carries an `@[json: 'inlayHints']` attribute.
- **Corrected: the nested and flat shapes both do work.** The install guide
  said Zed 1.24.2 changed nothing. That was measured against stock VLS 0.0.3
  from `v ls --install`, which has none of the compatibility shapes; the
  pinned build accepts four shapes.
- The VLS settings still have no settings UI: Zed's
  `language_server_workspace_configuration_schema` hook is in the 0.8.0 WIT
  but absent from the published 0.7.0 Rust trait. Tracked as blocked on 0.8.0.
- README capability list completed. `documentHighlightProvider` (highlight all
  occurrences) and `selectionRangeProvider` (expand selection) are advertised
  by the server and were missing from the list, which claimed to be complete.
- Semantic tokens verified as needing no `semantic_token_rules.json`: VLS's
  legend is entirely standard LSP token types and no custom type appears in
  the tokens it actually returns.
- Completion entries are now tinted by kind, so VLS results separate into
  types, functions, properties, constants and keywords at a glance. Uses
  `label_for_completion`, available in every published extension API (Zed wraps
  it in `labels_for_completions`), so this is not gated on the unpublished
  0.8.0. Highlight names are limited to captures Zed's themes define. A
  signature from VLS is appended dimmed only when it adds information —
  VLS often restates the label inside `detail`, and that noise is now
  suppressed. Completion filtering still matches the identifier alone.
- CI gained a `cargo test` step. The workflow previously had no test job, so
  the four new unit tests covering completion labels would not have run.
- V script shebangs now select the V language: `first_line_pattern` matches
  `#!/usr/bin/env v`, `#!/usr/bin/v`, `#!/usr/bin/env v run` and
  `#!/usr/bin/env -S v run`, while still rejecting `#!/usr/bin/env python3`
  and `#!/bin/sh`. The validator now asserts both directions.
- Added a `brackets` autoclose table. This also fixes a real 0.6.0 defect:
  `overrides.scm` shipped defining `@comment`/`@string` scopes that nothing
  consumed, so it configured nothing. Those scopes are now consumed by
  `not_in`, and the validator fails if they ever go unconsumed again.
  V-specific forms only: `/* */` block comments are supported (measured with
  `v -check`), while backtick and `"""` string pairs are deliberately absent
  because V 0.5.2 rejects both.
- Added `block_comment` so `editor::ToggleBlockComment` works on `/* */`.
- `(match_expression) @indent` added to `indents.scm`, matching `v fmt`'s
  two-level match-arm indentation (arms at +2 relative to the block body).
- New `redactions.scm`: string, rune and numeric literals render redacted
  while screen sharing or recording. It was one of the ten query assets Zed
  loads from a language extension, and the only cheap one still missing.
- `scripts/check-extension.py` reports how many of Zed's ten loadable query
  assets the extension ships, so the remaining two (`injections`, `debugger`)
  stay visible rather than forgotten.
- ROADMAP rewritten against a measured baseline: Zed's own query-asset loader
  and `config.toml` schema, `zed-extensions/zig`, and `lv37/zed-v` — which is
  **already published in the Zed registry as `extensions/v`**. Two of the
  old "not portable" claims were wrong and are corrected: debugging and
  slash commands are both in the extension API since 0.7.0, with debugging
  blocked only by V shipping no debug adapter.
- Recorded three assumptions that measurement refuted, so they are not
  re-derived: extensions genuinely cannot ship tasks (the Zig extension's
  `tasks.json` is dead weight, absent from Zed's asset loader); `$` is not a
  V word character (V rejects `bar$`); V has no backtick or `"""` strings.
- Also documented that `v fmt -` cannot be exercised through a PowerShell
  pipeline (it injects a BOM; use `cmd /c "v fmt - < file"`). Zed spawns the
  formatter directly and is unaffected.

- Formatting, measured end to end on Zed 1.24.2: VLS advertises
  `documentFormattingProvider` but answers with no edits for every input
  (six cases driven directly over LSP, with a working `v` on `PATH`; a
  proxy `v.exe` recorded the `v fmt -inprocess -w <temp copy>` call it
  makes before giving up, logged only in debug builds). The docs now
  ship the external formatter that does work — `v fmt -` on stdin, under
  `"languages": {"V": {...}}`, verified with `Ctrl-Alt-L` and with
  `Ctrl+S` alone — plus the trap that the legacy per-language `"V"` block
  is silently ignored on 1.24.2.
- New troubleshooting entry for `GC_MALLOC_UNCOLLECTABLE` / `GC_FREE` /
  `v_set_unhandled_exception_filter` implicit-declaration errors: a V
  whose generated `src.c` lacks those declarations fails every C build
  (`v fmt`, `v build`, `v run`) under tcc and gcc alike, while VLS keeps
  working. Verified that a V 0.5.2 release build formats the same file
  on the same machine.
- New task `V: install c2v (slow, one-time)`: fails fast without
  `clang`, clones the module when it is absent, then builds it — the
  portable half of upstream #617, whose `Install c2v` error action has
  no Zed equivalent (no command API). POSIX variant documented.
- Fixed a stale ROADMAP line that still claimed the extension never
  downloads anything (it has shipped pinned VLS since 0.5.0).
- CI cancels superseded runs per ref and both workflows cap job time.

## v0.6.0

- Vim text objects (`textobjects.scm`): `af`/`if` over functions,
  `ac`/`ic` over structs, enums, interfaces and comments.
- Scope overrides (`overrides.scm`): inclusive comment scope and string
  scopes, mirroring the Go and Python extensions.
- `code_fence_block_name = "v"`: fenced V blocks in Markdown highlight
  as V.
- Capability list now matches GUI proof: stars kept only where a
  capability was watched working in Zed (go-to-definition and rename
  lost theirs — they need V's V1 compatibility compiler, i.e. `make` on
  `PATH`; references and folding gained theirs).
- Troubleshooting gains that missing-compiler symptom (quoted verbatim
  from the server log) and the harmless
  `Error during workspace diagnostics pull` noise.
- New `Configure VLS (limited)` install section: the three settings VLS
  understands (`inlayHints`, `diagnostics`, `coverageDirectory`), plus
  the measurement that Zed 1.24.2 changes no VLS behaviour for either
  the nested or the flat payload shape. (Both claims later proved wrong;
  see Unreleased — `coverageDirectory` does not exist, and the payloads do
  work against the pinned VLS.)
- `CHANGELOG.md` added, curated per tag from git history.
- Install checklist notes format-on-save (via VLS formatting).

## v0.5.0

- Managed-first VLS: the extension downloads its pinned fork build
  (`metif12/vls` tag `vls-2`) with install-status reporting, keeping a
  `PATH` `vls` as fallback. Includes the vls-1 lesson (bare-boolean
  `documentLink` rejected by strict clients → struct form) and the pin
  bump to vls-2.

## v0.4.0

- Version bump only (ships v0.3.0 content as 0.4.0).

## v0.3.0

- Inline Run buttons (`runnables.scm` on `fn main` / `fn test_*`) with
  tagged tasks; `docs/tests.md` (toggle/skeleton/coverage recipes).
- Full task parity: save-before-run, 11 tasks, `V: show V version`,
  guarded c2v task (both branches executed).
- Precise feature docs (every LS capability backed by the live server
  response), PATH-only design record.
- ROADMAP, dependabot (actions + cargo), committed `Cargo.lock`;
  dependabot merges (`actions/checkout` 7, `zed_extension_api` 0.7.0).
- Install-dialog gotchas from the release-zip path.

## v0.2.0

- Release-zip install path that works: flat zips, folder-level and
  `RUSTUP_HOME`/wasm-target failure docs.
- Readable hero CTA (specificity fix + AA contrast), version
  compatibility matrix (`v ls` floor 0.3.1, VLS 0.3 verdict).
- `v ls --install` corrections everywhere, full-V-port ADR.

## v0.1.0

- Initial extension, verified end to end in Zed: Tree-sitter
  highlighting (with the `builtin_type` and full-SHA grammar fixes),
  outline/indents queries, VLS over `PATH`, 71 snippets (59 + 12 `veb`),
  8-task array, docs site with verbatim troubleshooting, CI test +
  release workflows.
