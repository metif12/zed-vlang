---
layout: default
title: V language support for Zed
---

<div class="hero" markdown="1">

# V language support for Zed

Syntax highlighting, [VLS](https://github.com/vlang/vls) language server
integration, snippets and task presets for the
[V programming language](https://vlang.io) in the
[Zed editor](https://zed.dev).

<div class="btnrow" markdown="1">
[Install guide](./install.md){: .btn } [GitHub](https://github.com/metif12/zed-vlang){: .btn .ghost }
</div>

</div>

<div class="cards" markdown="1">

<div class="card" markdown="1">

### Highlighting

Tree-sitter grammar with brackets, outline symbols and auto-indent.

</div>

<div class="card" markdown="1">

### VLS server

Completion, diagnostics, hover, signature help, inlay hints, rename,
symbols, call hierarchy and code actions — read off the live server.

</div>

<div class="card" markdown="1">

### Snippets

71 prefixes with working tab stops, synced with upstream.

</div>

<div class="card" markdown="1">

### Run buttons

Inline Run on `fn main` and `fn test_*`, wired to tagged tasks.

</div>

<div class="card" markdown="1">

### Tasks

Copy-paste run/build/test/check/vet/fmt/prod tasks for your project.

</div>

</div>

Ported from the feature set of
[`vscode-vlang`](https://github.com/vlang/vscode-vlang): the same
Tree-sitter-backed highlighting ideas, the same VLS-first approach to
completion/diagnostics/code actions, and the same (superset) snippet set —
our 71 snippets cover all 54 upstream ones plus the 12 `veb` snippets from
upstream PR #608, with fixed descriptions and richer tab stops.

## Contents

- [Install](./install.md) — requirements, dev-extension install, verify checklist
- [Troubleshooting](./troubleshooting.md) — every known install failure, verbatim
- [Snippet reference](./snippets.md) — all 71 snippets with prefixes
- [Working with tests](./tests.md) — test/source toggle, skeletons, coverage, c2v
- [Why the Rust shim stays](./why-rust-shim.md) — why a full-V port is not possible today
- [Why managed-first](./why-path-only.md) — rationale, and what icons would cost
- [Changelog](https://github.com/metif12/zed-vlang/blob/main/CHANGELOG.md) — what each release changed

## Features

- **Editing** for `.v`, `.vsh`, `.vh`, `.vv` and `v.mod` via the
  maintained [`tree-sitter-v`](https://github.com/undivisible/tree-sitter-v)
  grammar: syntax highlighting, bracket matching, outline symbols,
  auto-indent, and inline Run buttons on `fn main` and `fn test_*`.
- **Language server** from VLS (started automatically for V files).
  Every capability below is advertised by the live VLS 0.0.3 server
  itself (read off its LSP `initialize` response), and the starred ones
  were additionally watched working in Zed: completion\*, diagnostics\*,
  hover\*, signature help, inlay hints, go-to-definition (plus
  declaration, implementation and type-definition), references\*,
  rename, document and workspace symbols, call hierarchy, code actions
  (including the Fill struct literal), code lens (`vls.runFile`,
  `vls.runTests`), document formatting, folding ranges\* and semantic
  tokens. Go-to-definition and rename additionally need V's V1
  compatibility compiler (`make` on `PATH`). The extension uses its
  pinned managed build first, with a `PATH` `vls` as fallback.
- **Snippets**: 71 prefixes (`fn`, `for`, `stru`, `vapp`, …) with working tab stops.
- **Tasks**: copy-paste `.zed/tasks.json` for run/build/test/fmt
  ([README](https://github.com/metif12/zed-vlang#what-you-get)).

## Project links

- Extension source: <https://github.com/metif12/zed-vlang>
- Upstream VS Code extension: <https://github.com/vlang/vscode-vlang>
- Grammar: <https://github.com/undivisible/tree-sitter-v> (ISC, referenced — never vendored)
- Language server: <https://github.com/vlang/vls>

## License

MIT. The grammar stays a referenced dependency (ISC upstream), never vendored.
