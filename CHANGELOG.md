# Changelog

All notable changes, per release tag. Untagged work sits under
`Unreleased`.

## Unreleased

- Corrected the format-on-save claim with a measurement: VLS runs
  `v fmt -inprocess -w` itself, and when that fails it answers with no
  edits and logs nothing outside debug builds. The install checklist now
  sends you to `v fmt -w` in a terminal first, and the README flags the
  two tasks that need a compiling, formatting V.
- New troubleshooting entry for `GC_MALLOC_UNCOLLECTABLE` /
  `GC_FREE` / `v_set_unhandled_exception_filter` implicit-declaration
  errors: V's `vlib/builtin/c` is missing from the install, so `v fmt`,
  `v build` and `v run` fail while VLS keeps working.
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
  the nested or the flat payload shape.
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
