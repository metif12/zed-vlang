# V language support for Zed

Syntax highlighting, VLS language server integration, snippets and task
presets for the [V programming language](https://vlang.io) in the
[Zed editor](https://zed.dev).

Ported from the feature set of
[`vscode-vlang`](https://github.com/vlang/vscode-vlang): the same
Tree-sitter-backed highlighting ideas, the same VLS-first approach to
completion/diagnostics/code actions, and the same snippet set.

## Requirements

- The V toolchain on `PATH` ([install](https://vlang.io)): `v --version`
  should print `V 0.5.x`.
- VLS on `PATH`: `vls --version` should print `VLS 0.0.3` or newer.
  VLS publishes no standalone binaries, so install it once with the
  V toolchain and keep it on `PATH`:

  ```sh
  v ls --install
  ```

  (Older guides mention `v build-vls`; current V provides it as
  `v ls --install`, which prints `VLS was updated to version: VLS 0.0.3`.)

## Install

There is no marketplace release; install the checkout as a dev extension:

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

- **Highlighting** for `.v`, `.vsh`, `.vh`, `.vv` and `v.mod` via the
  maintained [`tree-sitter-v`](https://github.com/undivisible/tree-sitter-v)
  grammar, plus bracket matching.
- **Language server**: completion, diagnostics, hover, go-to-definition,
  references, rename, symbols and code actions (including Fill struct
  literal) from VLS, started automatically for V files.
- **Snippets**: 71 snippets in one set — the 59-snippet V set (functions,
  structs, matches, error handling, tests, numeric types) plus 12 `veb`
  web-framework snippets (`vapp`, `vget`, `vpst`, …), synced with upstream
  (kept in a single `v.json` because Zed matches snippet files by language
  name).
- **Tasks**: extensions cannot ship tasks in Zed, so add these to your
  project's `.zed/tasks.json` (run with `task: spawn`). This mirrors the
  `V: Build` / `V: Run` / `V: Test` tasks of `vscode-vlang`:

```json
[
  {
    "label": "V: run current file",
    "command": "v",
    "args": ["run", "$ZED_FILE"],
    "cwd": "$ZED_WORKTREE_ROOT"
  },
  {
    "label": "V: build workspace",
    "command": "v",
    "args": ["build", "."],
    "cwd": "$ZED_WORKTREE_ROOT"
  },
  {
    "label": "V: test workspace",
    "command": "v",
    "args": ["test", "."],
    "cwd": "$ZED_WORKTREE_ROOT"
  },
  {
    "label": "V: format current file",
    "command": "v",
    "args": ["fmt", "-w", "$ZED_FILE"],
    "cwd": "$ZED_WORKTREE_ROOT"
  }
]
```

The file must contain a JSON array (a bare object is rejected by Zed:
`Failed to parse tasks file content as a JSON array`).

## License

MIT. The grammar is a separate dependency
([ISC](https://github.com/undivisible/tree-sitter-v/blob/main/LICENSE)).
