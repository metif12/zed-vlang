# Roadmap

Known gaps and what would close them, cheapest first. Mirrors the
practice of upstream [`vscode-vlang`'s ROADMAP](https://github.com/vlang/vscode-vlang/blob/master/ROADMAP.md):
read this before opening a pull request so we do not duplicate each other.

## Synced with upstream (done)

- Snippet set incl. the #602 prefix fixes and the #608 `veb` file.
- Task model: build/run/test/prod/check/vet/fmt, run-current-file,
  test-current-file, emit-C, c2v, show-version, save-before-run.
- Inline Run buttons (`runnables.scm`) as the port of Run Main / Run File
  / Run Test actions.
- Test-file toggle mapping, skeleton recipe, coverage recipe
  (`docs/tests.md`).

## Blocked (needs something else first)

- `v.generateTests` as a one-click task: needs a V shipping
  `v cover --write-tests` (absent from V 0.5.2's help). Recipe lives in
  `docs/tests.md` until then.
- `v.translateWithC2v` execution: needs `c2v` built
  (`v install --git https://github.com/vlang/c2v`, `v -g ~/.vmodules/c2v/`)
  plus `clang` on `PATH`. Task shape is documented.
- Playground sharing: endpoint is `POST <playgroundRoot>/share`
  (multipart `code` field, JSON `{hash}` back), but the round-trip is
  unmeasured — no task until one live run confirms the shape.
- VLS 0.3: builds only with era-pinned V and needs `VLS_V_COMMAND`;
  stay on 0.0.3 with current V (see `docs/troubleshooting.md`).
- Publish to the Zed extension registry (PR to
  `zed-industries/extensions`): needs the publishing prerequisites
  reviewed against this repo (grammar reuse rules, snippet scoping).

## Not portable (by design)

- File icons (`#598`): icon themes own file icons in Zed, not language
  extensions.
- Managed VLS/V toolchain install, update checks, executable picker
  (`v.install`, `v.tools.checkForUpdates`, `v.selectExecutable`, #597's
  server-identity gate, #596): this extension is PATH-only by design —
  it resolves `vls` and never downloads.
- Coverage gutters, test explorer, playground webview, VPM UI, skills UI,
  status bar, debugger launch, CodeLens popups, quick-pick dialogs
  (`#565` UI shell, `#601` command shell, `#607` lightbulb,
  `#598` pickers, decoders/convertEncoding as commands): Zed exposes no
  extension API for custom editor commands or views. Pure-text cores are
  documented as recipes in `docs/tests.md` where they exist.
- Grammar edge cases (`#599` i32-as-numeric, `#603`–`#605` lexing):
  TextMate-only fixes. The tree-sitter grammar defines equivalent rules
  (`builtin_type` keywords, exponent/separator/ASI tokens), but the
  pinned build predates some of them; any residual mis-highlight is
  upstream-grammar territory (a different repo), not this extension.
- `v.mod` as a separate language: works as-is via `path_suffixes`;
  splitting it would share the same grammar and fragment everything.
