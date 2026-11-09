---
layout: default
title: Install
---

# Install

There is no marketplace release; install the checkout as a dev extension.
Every step below was verified end-to-end (Zed 1.24.2, V 0.5.2, VLS 0.0.3).

## 1. Requirements

- The V toolchain on `PATH` ([install](https://vlang.io)):

  ```sh
  v --version
  # V 0.5.2 ...
  ```

- VLS: the official language server shipped with the V toolchain. Install
  it once, then make sure `vls` is on `PATH`:

  ```sh
  v ls --install
  vls --version
  # VLS 0.0.3
  ```

  `v ls --install` places the binary under `~/.vls`. The extension
  resolves `vls` from `PATH` and speaks LSP stdio, so it always tracks
  the VLS that matches your compiler. A custom build is respected as long
  as its `vls` is first on `PATH`. Launch Zed from a shell where `vls`
  resolves — the extension inherits Zed's `PATH`.

### Version compatibility

Verified end-to-end on V 0.5.2 + VLS 0.0.3 (from `v ls --install`).
`v ls` has shipped since V 0.3.1. The extension is version-agnostic: it
resolves `vls` on `PATH` and speaks LSP stdio.

- Rust with the wasm targets, so Zed can compile the extension:

  ```sh
  rustup target add wasm32-wasip1 wasm32-wasip2
  ```

  Current Zed looks up `wasm32-wasip2`. If the log says rustup "could
  not choose a version of rustc … no default is configured", Zed found
  a rustup with an empty home (e.g. a package manager's): point
   `RUSTUP_HOME` at the home that holds your toolchain, or run
   `rustup default stable` there, and confirm
   `rustup target list --installed` shows both wasm targets.

## Configure VLS

VLS reads its settings from a pushed LSP `workspace/didChangeConfiguration`
under a `vls` key. The extension forwards `language_servers.vls.settings`
verbatim via `language_server_workspace_configuration`. Both the flat and
the nested shapes are accepted.

Top-level keys:

- `inlayHints` — show inlay type hints for inferred types
  (`variableTypes`, `parameterNames` as nested booleans)
- `diagnostics` — run live compile-time diagnostics while editing
- `defines` — compile-time defines, e.g. `["-d", "flag"]`
- `coverageDirectory` — directory a `v -coverage` run recorded into;
  VLS reads it to offer "Generate test" code actions

Set them under `language_servers`, in your `settings.json`:

```json
{
  "language_servers": {
    "vls": {
      "settings": {
        "vls": {
          "inlayHints": true,
          "diagnostics": true,
          "defines": [],
          "coverageDirectory": ""
        }
      }
    }
  }
}
```

A project may also set these in a `vls.json` next to its `v.mod`; the
editor tier wins over the project tier. The keys are camelCase
(`inlayHints`), matching VLS's JSON tags.

The extension cannot yet supply a JSON Schema for these keys: Zed only
added `language_server_workspace_configuration_schema` in the 0.8.0 API,
which is not published. Until 0.8.0 ships, the JSON above is the way to
set them.

## 2. Install the dev extension

Get the sources first — either the stable release zip or a git checkout:

- Stable: download the latest `zed-vlang-vX.Y.Z.zip` from
  [Releases](https://github.com/metif12/zed-vlang/releases) and extract
  it into an empty folder.
- Latest: `git clone https://github.com/metif12/zed-vlang`.

Then:

1. In Zed, open the command palette and run `zed: install dev extension`.
2. Select the folder that directly contains `extension.toml` — with a
   fresh extraction that is the folder you extracted into, not an outer
   wrapper and not `src/` (anything else fails with
   `No extension manifest found`).
   If the folder dialog only navigates on Enter, confirm with its
   **Select Folder** button.
3. Zed compiles the extension and the V grammar in a few seconds. Watch
   `zed: open log` for the `extension::extension_builder` lines ending in
   `finished compiling extension … in ~4s`.

## 3. Verify checklist

Open any `.v` file and confirm:

1. **Highlighting**: keywords/strings/types colored, not plain white.
2. **VLS starts**: the log shows
   `[lsp] starting language server process. binary path: "…vls(.exe)"`
   and no `vls not found` error.
3. **Hover + diagnostics**: hovering a call shows its signature
   (via `editor: hover`), and an undefined identifier is flagged, e.g.
   ``undefined ident: `undefined_identifier` (vlang undefined)``.
4. **Snippets**: typing `fn` offers the `snippet.function` row; accepting
   it expands `fn name() { … }` with `$1`/`$0` tab stops (Tab moves on).
5. **Outline/indent**: the outline panel lists functions; Enter after
   `fn main() {` indents the new line.
6. **Formatting** (optional): VLS advertises formatting but returns no
   edits (measured — see `troubleshooting.md`), so use Zed's external
   formatter. Add to `settings.json`:
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
   Then `Ctrl-Alt-L` formats the current file, and saves format too.

For tasks, copy the array from the README into your project's
`.zed/tasks.json` and run it with `task: spawn` — it must be a JSON
array (a bare object is rejected), written without a UTF-8 BOM.
