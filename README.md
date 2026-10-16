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
  (via `runnables.scm`, wired to the tagged tasks below).
- **Language server** from VLS (started automatically for V files).
  Every capability below is advertised by the live VLS 0.0.3 server
  itself (read off its LSP `initialize` response), and the starred ones
  were additionally watched working in Zed: completion\*, diagnostics\*,
  hover\*, signature help, inlay hints, go-to-definition\* (plus
  declaration, implementation and type-definition), references, rename,
  document and workspace symbols, call hierarchy, code actions
  (including the Fill struct literal), code lens (`vls.runFile`,
  `vls.runTests`), document formatting, folding ranges and semantic
  tokens.
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
  `v.executablePath` setting).

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
    "label": "V: translate C to V (needs c2v)",
    "command": "if (Get-Command c2v -ErrorAction SilentlyContinue) { c2v \"$ZED_FILE\" } else { Write-Error \"c2v is not installed. Run: v install --git https://github.com/vlang/c2v ; then: v -g ~/.vmodules/c2v/ ; c2v also needs clang on PATH.\"; exit 1 }",
    "args": [],
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
