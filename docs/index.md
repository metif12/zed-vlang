# V language support for Zed

Syntax highlighting, [VLS](https://github.com/vlang/vls) language server
integration, snippets and task presets for the
[V programming language](https://vlang.io) in the
[Zed editor](https://zed.dev).

Ported from the feature set of
[`vscode-vlang`](https://github.com/vlang/vscode-vlang): the same
Tree-sitter-backed highlighting ideas, the same VLS-first approach to
completion/diagnostics/code actions, and the same (superset) snippet set —
our 59 snippets cover all 54 upstream ones, with fixed descriptions and
richer tab stops. Upstream snippets have not changed since 2022; see the
[snippet reference](./snippets.md).

## Contents

- [Install](./install.md) — requirements, dev-extension install, verify checklist
- [Troubleshooting](./troubleshooting.md) — every known install failure, verbatim
- [Snippet reference](./snippets.md) — all 59 snippets with prefixes

## Features

- **Highlighting** for `.v`, `.vsh`, `.vh`, `.vv` and `v.mod` via the
  maintained [`tree-sitter-v`](https://github.com/undivisible/tree-sitter-v)
  grammar, plus bracket matching, outline symbols and auto-indent.
- **Language server**: completion, diagnostics, hover, go-to-definition,
  references, rename, symbols and code actions from VLS, started
  automatically for V files. VLS-only by design: VLS publishes no
  standalone binaries, so the extension resolves your `PATH` install
  (`worktree.which("vls")`) instead of downloading anything.
- **Snippets**: 59 prefixes (`fn`, `for`, `stru`, …) with working tab stops.
- **Tasks**: copy-paste `.zed/tasks.json` for run/build/test/fmt
  ([README](https://github.com/metif12/zed-vlang#what-you-get)).

## Project links

- Extension source: <https://github.com/metif12/zed-vlang>
- Upstream VS Code extension: <https://github.com/vlang/vscode-vlang>
- Grammar: <https://github.com/undivisible/tree-sitter-v> (ISC, referenced — never vendored)
- Language server: <https://github.com/vlang/vls>

## License

MIT. The grammar stays a referenced dependency (ISC upstream), never vendored.
