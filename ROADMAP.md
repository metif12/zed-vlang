# Roadmap

Known gaps and what would close them, cheapest first. Mirrors the
practice of upstream [`vscode-vlang`'s ROADMAP](https://github.com/vlang/vscode-vlang/blob/master/ROADMAP.md):
read this before opening a pull request so we do not duplicate each other.

## How to read this file

Every item is tagged with how its status was established:

- **measured** — checked on this machine, with the command named. Safe to
  implement from.
- **in Zed** — verified by reading Zed's own source, cited by file. Holds
  without running the GUI.
- **needs a GUI check** — the API exists and the node names are verified, but
  the visible result in Zed was not watched. Do the GUI check before claiming
  it works.
- **blocked** — needs something outside this repository to land first.

Do not promote an item out of "needs a GUI check" on the strength of a
compiling build. That is the mistake this file exists to prevent.

## Comparison baseline (measured 2026-10-17)

Checked against, rather than assumed from memory:

- **Zed core** `crates/language_core/src/queries.rs` — the authoritative list
  of loadable query assets. Exactly ten: `highlights`, `brackets`, `outline`,
  `indents`, `injections`, `overrides`, `redactions`, `runnables`, `debugger`,
  `textobjects`. `semantic_token_rules.json` loads separately
  (`docs/src/extensions/languages.md`).
- **Zed core** `crates/language_core/src/language_config.rs` — the full
  `config.toml` schema (43 keys).
- **`zed-industries/extensions`** (1931 stars) — language extensions are
  submodules; official ones live under `zed-extensions/`.
- **`zed-extensions/zig`** v0.4.2 — the closest systems-language analogue.
- **`lv37/zed-v`** v0.5.0 — **already in the Zed registry as `extensions/v`**
  (submodule → `github.com/lv37/zed-v`, 41 stars, last push 2026-04-22).
  Read this file against that: it is the incumbent, not a hypothetical rival.
- **`zed_extension_api`** — 0.7.0 published; 0.8.0 is in Zed's tree but
  `publish = false`, so 0.7.0 is the ceiling for now.

Where this repository is already ahead of both references: 71 snippets,
`outline`/`indents`/`overrides`/`textobjects`/`redactions` queries, inline
Run buttons, and a docs site. `lv37/zed-v` ships none of those.

## Shipped since the last roadmap (measured)

- `first_line_pattern` for V shebangs (`#!/usr/bin/env v`, `#!/usr/bin/v`,
  `#!/usr/bin/env -S v run`), so extensionless V scripts open as V. Verified
  against 4 real shebangs and 2 non-V ones it must reject.
- A `brackets` autoclose table, which also makes `overrides.scm` (shipped
  inert in 0.6.0) actually configure something: the `@comment`/`@string`
  scopes are now consumed by `not_in`. The validator now fails if the scopes
  ever go unconsumed again.
- `block_comment` for `/* */`. **measured**: V 0.5.2 accepts block comments
  (`v -check` on a file containing one exits 0; a deliberately invalid
  control file exits 1, so the check is real).
- `(match_expression) @indent`, matching `v fmt`'s two-level match-arm
  indentation (arms sit at +2 relative to the enclosing block body).
- `redactions.scm`, one of the ten loadable query assets, previously absent.

## Next, cheapest first

1. **`hard_tabs` decision — needs a decision, not a measurement.**
   **measured**: `v fmt` on V 0.5.2 emits **32 tab characters and 0 spaces**
   (`cmd /c "v fmt - < file"`, then count leading whitespace). This
   repository ships `hard_tabs = false`, so pressing Tab in Zed inserts a
   space and the formatter later rewrites it to a tab. Zed's own Go config
   sets `hard_tabs = true`, and `gofmt` has the same property. Recommend
   flipping to `true`, but it changes an editor default users feel, so it
   should be a conscious choice with a CHANGELOG note, not a drive-by.
2. **`injections.scm`** — the last cheap query asset. Needs a concrete V
   embedding to be worth it; `veb`'s `.html` templates are the candidate
   (V ships `$tmpl()` and `.html` files with V expressions). Node names must
   be checked against the pinned grammar first.
3. **`semantic_token_rules.json`** — **needs a GUI check.** The extension
   advertises VLS semantic tokens but ships no rules for VLS's custom token
   types. Capture VLS's real token types over LSP, then map them to theme
   styles. Do not guess the token names.
4. ~~**Completion labels via `labels_for_completions`**~~ — **shipped.**
   `label_for_completion` tints entries by kind and appends VLS's signature
   only when it is not already in the label. Available since 0.7.0, not
   0.8.0. Covered by four unit tests. **Still needs a GUI check** for the
   visual result; the unit tests prove the mapping, not the rendering.
5. **VLS settings schema** — **in Zed.** 0.7.0 exposes
   `language_server_initialization_options` plus its JSON Schema, and
   `language_server_workspace_configuration` plus its schema. That is how an
   extension contributes settings that Zed renders in its own settings UI.
   `docs/index.md` currently tells users to hand-write three VLS keys;
   a schema would replace that prose with real UI. **needs a GUI check**,
   and VLS's accepted shape must be measured first — the 0.6.0 notes record
   that Zed 1.24.2 ignored both nested and flat payloads.
6. **`debugger.scm` + DAP** — **in Zed, blocked on V.** Zed's extension API
   has supported DAP since 0.6.0 (`get-dap-binary`, `dap-locator-create-scenario`,
   `run-dap-locator`), and `config.toml` accepts a `debuggers` array. So the
   old claim "debugger launch is not portable" is **out of date**. The real
   blocker is upstream: **measured** — V 0.5.2's `v help` has no debug
   subcommand and no DAP flag, so there is no adapter to launch. Revisit when
   V ships one.
7. **Slash commands** — **in Zed.** `run_slash_command` /
   `complete_slash_command_argument` (since 0.7.0) mean a `/v-…` command is
   now possible, which the old roadmap called non-portable. Low value until a
   V action is worth wrapping; revisit with the `v.generateTests` item below.

## Blocked (needs something else first)

- `v.generateTests` as a one-click task: needs a V shipping
  `v cover --write-tests` (absent from V 0.5.2's help). Recipe lives in
  `docs/tests.md` until then. Upstream #607 narrows the same action to
  the functions a coverage run never entered, which needs their coverage
  state plumbed into the action — there is no equivalent input here. A
  slash command would not fix this either; the V command still does not exist.
- `v.translateWithC2v` execution: needs `c2v` built plus `clang` on
  `PATH`. The README now ships the install half as a task
  (`V: install c2v (slow, one-time)`, upstream #617 does the same two
  steps from an error action); the translation itself stays guarded with
  setup instructions.
- Playground sharing: endpoint is `POST <playgroundRoot>/share`
  (multipart `code` field, JSON `{hash}` back), but the round-trip is
  unmeasured — no task until one live run confirms the shape.
- VLS 0.3: builds only with era-pinned V and needs `VLS_V_COMMAND`;
  stay on 0.0.3 with current V (see `docs/troubleshooting.md`).

## Not portable (by design)

- File icons (`#598`): icon themes own file icons in Zed, not language
  extensions.
- V toolchain install, update checks, executable picker
  (`v.install`, `v.tools.checkForUpdates`, `v.selectExecutable`, #597's
  server-identity gate, #596): Zed exposes no API to offer a toolchain
  picker or update check. Note the distinction from the managed VLS download,
  which *is* an API (`download_file`) and has shipped since 0.5.0.
- Custom editor views — coverage gutters, test explorer, playground
  webview, VPM UI, skills UI, status bar, CodeLens popups, quick-pick
  dialogs (`#565` UI shell, `#598` pickers, decoders/convertEncoding as
  commands): still no extension API for these. Slash commands
  (`run_slash_command`) cover the "run something for me" half but not a
  docked panel. Pure-text cores are documented as recipes in
  `docs/tests.md` where they exist.
- Grammar edge cases (`#599` i32-as-numeric, `#603`–`#605` lexing):
  TextMate-only fixes. The tree-sitter grammar defines equivalent rules
  (`builtin_type` keywords, exponent/separator/ASI tokens), but the
  pinned build predates some of them; any residual mis-highlight is
  upstream-grammar territory (a different repo), not this extension.
- `v.mod` as a separate language: works as-is via `path_suffixes`;
  splitting it would share the same grammar and fragment everything.

## Corrections to keep from re-measuring

Facts that were assumed, checked, and found false. Each cost real time, so
they are recorded to stop the next person paying it twice.

- **Extensions cannot ship tasks. Confirmed, and it is not a
  documentation gap.** `zed-extensions/zig` contains
  `languages/zig/tasks.json`, which invites the conclusion that they can.
  They cannot: that filename appears nowhere in Zed's extension
  language-asset loader (`crates/language_core/src/queries.rs`), only in
  settings/UI code. The Zig file is dead weight. Tasks stay a documented
  copy-paste into `.zed/tasks.json`, as this README already says.
- **`$` is not a V word character.** The registry's V extension sets
  `word_characters = ["$", "#"]`. **measured** — V 0.5.2 rejects `bar$`
  outright ("unexpected `$`") and allows `$` only on a fixed comptime
  whitelist (`$tmpl()`, `$env()`, `$embed_file()`, `$pkgconfig()`,
  `$veb.html()`, `$compile_error()`, `$compile_warn()`, `$d()`, `$res()`,
  `$zero()`, `$new()`). Do not copy that setting.
- **V has no backtick raw strings and no `"""` long strings.**
  **measured** — both are rejected by `v -check` on 0.5.2 (a backtick
  string parses as an invalid character literal). Any `brackets` table
  copied from Go/Python must drop the backtick pair; V is not Go.
- **`v fmt -` breaks under a PowerShell pipeline.** Piping a file into
  `v fmt -` from PowerShell injects a BOM and V fails with
  `invalid character`. Use `cmd /c "v fmt - < file"` when testing. Zed
  spawns the process directly and is unaffected; this only matters for
  humans reproducing the formatter in a terminal.
