---
layout: default
title: Troubleshooting
---

# Troubleshooting

All errors below were observed verbatim during verified installs
(`zed: open log`). Each names its own fix.

## The palette or dialog does not respond

Three environment gotchas, all measured: a fresh folder first triggers
Zed's Unrecognized Project / Restricted Mode modal — accept Trust before
opening the palette. The folder dialog's Select Folder button must be
clicked — Enter only navigates. And the palette shortcut follows your
`base_keymap` (JetBrains maps it to Ctrl+Shift+A, not Ctrl+Shift+P);
typing with a non-Latin keyboard layout lands in the editor instead of
the palette, so switch to English first. Tasks also need a folder
worktree: opening a lone file roots the worktree at that file and
`task: spawn` shows `No matches` — open the folder instead.

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

## Go-to-definition / rename do nothing

```text
vls: the configured V compiler cannot serve completion, hover, signature
help, or go to definition, because its V1 compatibility compiler is
missing… Diagnostics and formatting are unaffected.
```

Those features need V's V1 compatibility compiler, which needs `make` on
`PATH`. Without it the server says so explicitly; diagnostics and hover
keep working regardless. Rename and code actions were likewise never
observed working without it.

## `Error during workspace diagnostics pull`

```text
ERROR … failed to deserialize response … WorkspaceDiagnosticReportResult
```

Log noise: VLS answers workspace pull diagnostics with `[]`, which Zed
rejects. Push diagnostics (squiggles, panel, hover popups) are unaffected
— safe to ignore.

## Format Document / format on save changes nothing

Zed asks VLS (`documentFormattingProvider` is advertised — measured on
the pinned build), and VLS runs the real formatter itself:
`v fmt -inprocess -w` on a temporary copy of the buffer. So a silent
no-op means **`v fmt` failed**, not that Zed skipped the request. The
server logs that failure only in debug builds, so nothing reaches the
Zed log; the LSP response is simply `[]`.

Check it directly first:

```console
$ v fmt -w somefile.v   # exit 0 and the file changes = V can format
```

## Every V build fails with a GC error

```text
warning: implicit tcc could not be used for this build (tcc: error:
unresolved reference to 'GC_MALLOC_UNCOLLECTABLE'), regenerating it
with gcc.exe
C compilation error (from gcc.exe):
src.c:1374:19: error: implicit declaration of function
'GC_MALLOC_UNCOLLECTABLE' [-Wimplicit-function-declaration]
```

This is V, not Zed: the generated `src.c` is missing the declarations
that live in V's C runtime directory. On a checkout where
`vlib/builtin/c` is absent, `v fmt`, `v build` and `v run` all fail this
way while VLS keeps working (its diagnostics come from its own parser,
not from a C build). Reinstall or upgrade V so `vlib/builtin/c` is
present — on a Windows install that is `C:\Users\<you>\v\vlib\builtin\c`.

## Managed VLS download fails or never starts

The extension uses its pinned build first (`metif12/vls` tag `vls-2`),
so without a download there is no server. Two things can stop it: Zed
prompts once for the `download_file` capability — declining leaves no
server (approve it, or put a `vls` on `PATH`, which is still respected
as fallback). And offline machines get the download error verbatim.
