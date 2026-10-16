# Changelog

All notable changes, per release tag. Only the `Unreleased` section moves
without a tag.

## Unreleased

- Vim text objects (`textobjects.scm`): `af`/`if` over functions,
  `ac`/`ic` over structs, enums, interfaces and comments.
- Scope overrides (`overrides.scm`): inclusive comments, string scopes.
- `code_fence_block_name = "v"`: fenced V blocks highlight as V.
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
