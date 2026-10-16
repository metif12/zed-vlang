---
layout: default
title: Troubleshooting
---

# Troubleshooting

All errors below were observed verbatim during verified installs
(`zed: open log`). Each names its own fix.

## `failed to fetch revision <sha>`

```text
ERROR [crates/extension_host/src/extension_host.rs:1182] compiling grammar 'v':
  failed to fetch revision ed235d6c43c9 in directory "…\grammars\v"
ERROR [extensions_ui] Failed to install dev extension: compiling grammar 'v'
```

Zed fetches the grammar `rev` by exact hash; a 12-character prefix does
not resolve. Fix: pin the full 40-character commit SHA in
`extension.toml`:

```toml
[grammars.v]
repository = "https://github.com/undivisible/tree-sitter-v"
rev = "ed235d6c43c9e29ebcabe8c237cc6823599201c5"
```

## `Invalid node type "builtin_type"`

```text
ERROR [language::language_registry] failed to load language V:
Error loading highlights query
Caused by:
    Query error at 24:3. Invalid node type "builtin_type"
```

The pinned grammar's `grammar.js` defines `builtin_type`, but its
checked-in generated parser (`src/parser.c`, `src/node-types.json`) predates
the rule — and Zed builds from the checked-in parser. Fix: do not reference
the node in `languages/v/highlights.scm` (builtin types still highlight as
`@type` via `type_identifier`).

## `Impossible pattern` in an outline query

```text
ERROR [language::language_registry] failed to load language V:
Error loading outline query
Caused by:
    Query error at 13:3. Impossible pattern:
      name: (generic_type) @name) @item
```

One impossible pattern fails the **whole** language load (highlighting,
VLS and snippets all go dark). Fix: only use field/type combinations the
pinned parser's `src/node-types.json` actually allows — here the interface
`name:` field allows only `type_identifier`.

## Tasks file rejected

```text
ERROR [project_settings] Failed to set local tasks in ".zed\tasks.json":
"Failed to parse tasks file content as a JSON array:
 invalid type: map, expected a sequence at line 1 column 0"
```

`.zed/tasks.json` must contain a JSON **array** of task objects, not a bare
object. (A file saved with a UTF-8 BOM fails differently —
`expected value at line 1 column 1` — so save it BOM-free.)

## `vls not found on PATH`

The extension resolves VLS via `worktree.which("vls")` and reports a clear
error otherwise. Fix: run `v ls --install`, confirm `vls --version`, and
(re)launch Zed from a shell where `vls` resolves.

## Benign: `working directory 'grammars\v' not indexed`

```text
ERROR [crates/worktree/src/worktree.rs:3600] working directory `grammars\v` not indexed
```

Appears during every successful build; the build still finishes
(`finished compiling extension … in ~4s`). Safe to ignore.

## Snippets listed twice after repeated reinstalls

After several `install dev extension` rounds without restarting Zed, a
completion menu may show each snippet row twice. Fully quit Zed and reopen:
with a clean restart every snippet is listed exactly once (verified for
`fn` and `vapp`). This is Zed session state, not extension data — the
registered set is correct.

## `No extension manifest found for extension X`

The folder dialog was pointed at the wrong level: Zed needs the folder
that directly contains `extension.toml`. With a release zip, that is the
folder you extracted into (not an outer wrapper, not `src/`).

## `failed to retrieve the wasm32-wasip2 target libdir`

```text
ERROR [extensions_ui] Failed to install dev extension: compiling Rust extension
    failed to retrieve the `wasm32-wasip2` target libdir: error: rustup
    could not choose a version of rustc to run, because one wasn't
    specified explicitly, and no default is configured.
```

Zed found a `rustup` whose home holds no toolchain and no default (e.g. a
package manager's rustup exporting an empty `RUSTUP_HOME`). Fix: point
`RUSTUP_HOME` at the home that holds your toolchain (or run
`rustup default stable` there), add both wasm targets
(`rustup target add wasm32-wasip1 wasm32-wasip2`), and confirm
`rustup target list --installed` lists them before retrying.

## `v run` fails with tcc/GC errors

` tcc: error: unresolved reference to 'GC_MALLOC_UNCOLLECTABLE'` (plus a gcc
fallback failing for missing `make`) is the V toolchain on that machine,
not this extension — `v -gc none run` works around it.

## VLS 0.3 is not a drop-in upgrade

The GitHub VLS tag `0.3` (newer than the `VLS 0.0.3` that `v ls --install`
provides) was measured side-by-side and does not work with current V:

- its source uses a removed `pref` API, so it does not compile with
  V 0.5.2 — it needs its era-pinned compiler;
- hover/definition shell out to `v`, so without `VLS_V_COMMAND` pointing
  at that pinned compiler, hover answers `null`;
- it has no `--version` flag (it reports version `0.0.2` in the LSP
  `initialize` response instead).

Stay on `0.0.3` with current V.
