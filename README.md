# V language support for Zed

[![CI](https://github.com/metif12/zed-vlang/actions/workflows/ci.yml/badge.svg)](https://github.com/metif12/zed-vlang/actions/workflows/ci.yml)

Documentation (install guide, troubleshooting, snippet reference):
<https://metif12.github.io/zed-vlang/>

Syntax highlighting, VLS language server integration, snippets and task
presets for the [V programming language](https://vlang.io) in the
[Zed editor](https://zed.dev).

Ported from the feature set of
[`vscode-vlang`](https://github.com/vlang/vscode-vlang): the same
Tree-sitter-backed highlighting ideas, the same VLS-first approach to
completion/diagnostics/code actions, and the same snippet set.

## Requirements

- The V toolchain on `PATH` ([install](https://vlang.io)): `v --version`
  should print `V 0.5.x` (verified end-to-end on `V 0.5.2`).
- VLS: comes with the extension. On first use it downloads its pinned
  build once (approve Zed's one-time download prompt) and caches it per
  version. A `vls` on `PATH` is still respected as fallback — useful
  offline or with a custom toolchain (verified end-to-end on `VLS 0.0.3`
  from `v ls --install`). To install VLS yourself:

  ```sh
  v ls --install
  ```

  (Older guides mention `v build-vls`; current V provides it as
  `v ls --install`, which prints `VLS was updated to version: VLS 0.0.3`.)

  `v ls` has shipped since V 0.3.1 (PR `vlang/v#15009`).
  `v build-vls` never existed as a subcommand — older guides used the name
  for building VLS from source. The newer GitHub VLS tag `0.3` is not a
  drop-in upgrade: it builds only with era-pinned V, needs `VLS_V_COMMAND`
  pointing at that compiler, and has no `--version` flag — stay on `0.0.3`
  with current V. The extension itself is version-agnostic (pinned
  managed build first, PATH `vls` as fallback) and speaks LSP stdio.

## Install

There is no marketplace release; install the sources as a dev extension.
Get them either from the latest release zip or from git:

- Stable: download the latest `zed-vlang-vX.Y.Z.zip` from
  [Releases](https://github.com/metif12/zed-vlang/releases) and extract
  it into an empty folder.
- Latest: `git clone https://github.com/metif12/zed-vlang`.

Then:

1. Meet the [Requirements](#requirements) above (`v --version` and
   `vls --version` must both resolve).
2. In Zed, open the command palette and run `zed: install dev extension`,
   then select the checkout directory (the one containing
   `extension.toml`). Zed compiles the extension and the V grammar in a
   few seconds; watch `zed: open log` for the
   `extension::extension_builder` lines.
3. Open any `.v`, `.vsh`, `.vh`, `.vv` or `v.mod` file. Zed starts VLS
   automatically (`[lsp] starting language server process … vls.exe`).

If the install fails, the log names the exact cause; see
`docs/troubleshooting.md` for the known ones and their fixes.

## What you get

- **Editing** for `.v`, `.vsh`, `.vh`, `.vv` and `v.mod` via the
  maintained [`tree-sitter-v`](https://github.com/undivisible/tree-sitter-v)
  grammar: syntax highlighting, bracket matching, outline symbols,
  auto-indent, and inline Run buttons on `fn main` and `fn test_*`
  (via `runnables.scm`, wired to the tagged tasks below). Extensionless
  scripts with a V shebang (`#!/usr/bin/env v`, `#!/usr/bin/v`,
  `#!/usr/bin/env -S v run`) are recognised too. `/* */` block comments work
  alongside `//`. Literals are redacted while you screen share or record.
- **Language server** from VLS (started automatically for V files).
  Every capability below is advertised by the live VLS server
  itself (read off its LSP `initialize` response), and the starred ones
  were additionally watched working in Zed: completion\*, diagnostics\*,
  hover\*, signature help, inlay hints, go-to-definition (plus
  declaration, implementation and type-definition), references\*,
  document highlight (highlight all occurrences), selection ranges (expand
  selection), rename, document and workspace symbols, call hierarchy, code
  actions
  (including the Fill struct literal), code lens (`vls.runFile`,
  `vls.runTests`), document formatting†, folding ranges\* and semantic
  tokens. VLS emits only standard LSP semantic token types, so no custom
  `semantic_token_rules.json` is needed. Go-to-definition and rename
  additionally need V's V1
  compatibility compiler (`make` on `PATH`); without it the server logs
  so explicitly while diagnostics and hover keep working.
  † VLS advertises document formatting but answers with no edits for every
  input (measured directly over LSP), so formatting goes through Zed's
  external formatter — see the settings block below, verified working.
  Two settings are configurable, `vls.inlayHints` and `vls.diagnostics`;
  `docs/install.md` has the shape. There is no `coverageDirectory` setting —
  that name came from a guess and VLS silently ignores it.
- **Snippets**: 71 snippets in one set — the 59-snippet V set (functions,
  structs, matches, error handling, tests, numeric types) plus 12 `veb`
  web-framework snippets (`vapp`, `vget`, `vpst`, …), synced with upstream
  (kept in a single `v.json` because Zed matches snippet files by language
  name).
- **Tasks**: extensions cannot ship tasks in Zed, so add these to your
  project's `.zed/tasks.json` (run with `task: spawn`). This mirrors the
  `V: Build` / `V: Run` / `V: Test` tasks of `vscode-vlang`.
  `fn main` and `fn test_*` also get inline Run buttons
  (`languages/v/runnables.scm`); they fire the tagged tasks, so keep the
  `tags` on those two entries. Replace `"v"` with your compiler's absolute
  path if it is not on `PATH` (the equivalent of upstream's
  `v.executablePath` setting). The `c2v` tasks are the portable half of
  upstream's c2v work: upstream can hang an `Install c2v` action off the
  failure, Zed has no command API, so the same two steps ship as plain
  tasks instead.

Two tasks depend on your V being able to compile and format: `V: format
current file` runs `v fmt -w`, and the external formatter below runs
`v fmt -`. Check `v fmt -w somefile.v` in a terminal first if either is a
no-op — `docs/troubleshooting.md` has the measured failure modes.

## Formatting

VLS's own formatting provider is inert on this build, so formatting goes
through Zed's external formatter. Add this to your `settings.json`:

```json
{
  "languages": {
    "V": {
      "format_on_save": "on",
      "formatter": { "external": { "command": "v", "arguments": ["fmt", "-"] } }
    }
  }
}
```

`v fmt -` reads the buffer on stdin and writes formatted V on stdout,
which is the contract Zed expects. Measured on 1.24.2: `Ctrl-Alt-L`
rewrites a mis-formatted file byte-identically to `v fmt -`, and saving
formats too. Note the legacy per-language `"V": { … }` block is silently
ignored on 1.24.2 — use `"languages"`.

To try that formatter by hand, avoid piping into it from PowerShell: the
pipeline hands `v` a byte-order mark and V aborts with `invalid character`.
Redirect the bytes instead:

```sh
cmd /c "v fmt - < main.v"      # Windows
v fmt - < main.v               # any POSIX shell
```

```json
[
  {
    "label": "V: run current file",
    "command": "v",
    "args": ["run", "$ZED_FILE"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "tags": ["v-main"],
    "save": "current"
  },
  {
    "label": "V: build workspace",
    "command": "v",
    "args": ["build", "."],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all"
  },
  {
    "label": "V: test workspace",
    "command": "v",
    "args": ["test", "."],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all"
  },
  {
    "label": "V: test current file",
    "command": "v",
    "args": ["test", "$ZED_FILE"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "tags": ["v-test"],
    "save": "current"
  },
  {
    "label": "V: format current file",
    "command": "v",
    "args": ["fmt", "-w", "$ZED_FILE"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "current"
  },
  {
    "label": "V: check workspace",
    "command": "v",
    "args": ["-check", "."],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all"
  },
  {
    "label": "V: vet workspace",
    "command": "v",
    "args": ["vet", "-W", "."],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all"
  },
  {
    "label": "V: optimized build",
    "command": "v",
    "args": ["-prod", "."],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all"
  },
  {
    "label": "V: emit C from current file",
    "command": "v",
    "args": ["-o", "out.c", "$ZED_FILE"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "current"
  },
  {
    "label": "V: clone c2v (one-time)",
    "command": "v",
    "args": ["install", "--git", "https://github.com/vlang/c2v"],
    "cwd": "$ZED_WORKTREE_ROOT"
  },
  {
    "label": "V: build c2v (needs clang)",
    "command": "v",
    "args": ["-g", "~/.vmodules/c2v/"],
    "cwd": "$ZED_WORKTREE_ROOT"
  },
  {
    "label": "V: translate C to V (needs c2v)",
    "command": "c2v",
    "args": ["$ZED_FILE"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "current"
  },
  {
    "label": "V: show V version",
    "command": "v",
    "args": ["--version"],
    "cwd": "$ZED_WORKTREE_ROOT"
  }
]
```

The file must contain a JSON array (a bare object is rejected by Zed:
`Failed to parse tasks file content as a JSON array`).

Every task above is a plain command plus arguments — no shell scripting. Zed
runs a task in your login shell (`terminal.shell`), which is PowerShell on
Windows and bash/zsh elsewhere, so a `command` containing `if`/`&&`/`;`
portable-ly cannot be written for both. Upstream `vscode-vlang` gets away
with richer tasks because it implements them in TypeScript with its own
platform-aware path handling, which an extension has no equivalent of. The
`c2v` prerequisites (a `clang` binary, and a prior clone + build) are
therefore prose in `docs/tests.md` rather than guards in the command string.

`V: build c2v` uses `~/.vmodules/c2v/`. Bash and zsh expand `~`; PowerShell
does not, so on Windows run `v -g $env:USERPROFILE\.vmodules\c2v\` by hand, or
point the task at your own path.

Extensions really cannot ship tasks. The official Zig extension contains a
`languages/zig/tasks.json`, which suggests otherwise, but that filename is
absent from Zed's extension language-asset loader
(`crates/language_core/src/queries.rs` in `zed-industries/zed`) — it only
appears in Zed's own settings and task UI code. So the copy-paste above is
the only way to get them.

File tasks save the current buffer first (`"save": "current"`), workspace
tasks save everything (`"save": "all"`) — mirroring upstream, which saves
before running.

Only `V: check workspace` runs without a C toolchain (`v -check`
exit 0 on a bare install); the rest need working tcc/gcc like any V
build. The set mirrors upstream's task enum
(`build/run/test/prod/check/vet/fmt`).

## License

MIT. The grammar is a separate dependency
([ISC](https://github.com/undivisible/tree-sitter-v/blob/main/LICENSE)).
