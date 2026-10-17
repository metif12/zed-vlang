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

- VLS: comes with the extension. A `PATH` install is still respected
  as fallback (verified end-to-end on `VLS 0.0.3`); otherwise the pinned
  build downloads once — approve Zed's one-time download prompt when it
  appears. To install VLS yourself
  (takes a few minutes to compile from source):

  ```sh
  v ls --install
  vls --version
  # VLS 0.0.3
  ```

  (Older guides mention `v build-vls`; current V provides it as
  `v ls --install`.) Launch Zed from a shell where `vls` resolves —
  the extension inherits Zed's `PATH`.

### Version compatibility

`v ls` has shipped since V 0.3.1 (PR `vlang/v#15009`, commit `235ef23`,
2022-07-11); `v build-vls` never existed as a subcommand. Verified
end-to-end on V 0.5.2 + VLS 0.0.3. The newer GitHub VLS tag `0.3` is not
a drop-in upgrade with current V (see
[Troubleshooting](./troubleshooting.md#vls-03-is-not-a-drop-in-upgrade)).
The extension itself is version-agnostic: it resolves `vls` on `PATH`
and speaks LSP stdio.

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

## Configure VLS (limited)

VLS understands exactly **two** settings, both booleans, read from a pushed
LSP `workspace/didChangeConfiguration` under a `vls` key:

- `inlayHints` — show inlay type hints for inferred types
- `diagnostics` — run live compile-time diagnostics while editing

Set them under `language_servers`, in your `settings.json`:

```json
{
  "language_servers": {
    "vls": { "settings": { "vls": { "inlayHints": false, "diagnostics": true } } }
  }
}
```

Three corrections to what this page used to say, each measured against the
running server rather than inferred:

- **There is no `coverageDirectory` setting, and no coverage support at
  all.** It was listed here from a guess. The pinned VLS contains zero
  references to coverage anywhere in its source, so the setting does not
  exist, the feature does not exist, and there is nothing to point it at.
  (Upstream `vscode-vlang` does expose `v.vls.coverage.enabled`, which makes
  *its own* extension run and parse `v cover` — it is not a VLS capability
  this extension can borrow.) Coverage therefore stays a `v cover` recipe in
  `docs/tests.md`.
- **The key is camelCase `inlayHints`.** The snake_case `inlay_hints` used for
  the field in V's own source is *not* accepted — that is what the `@[json:
  'inlayHints']` attribute on the field is for.
- **Both the nested and the flat shapes do work.** An earlier version of this
  page reported that Zed 1.24.2 changed nothing. That measurement was taken
  against stock VLS 0.0.3 from `v ls --install`, which has none of the
  compatibility shapes; the pinned build does. Driving the live server,
  `{"settings":{"vls":{"inlayHints":false}}}` takes the inlay-hint count from
  1 to 0, and `{"settings":{"vls":{"inlayHints":true}}}` brings it back.

The extension now forwards `language_servers.vls.settings` to VLS verbatim
via `language_server_workspace_configuration`. It cannot yet supply a JSON
Schema for them, so there is no settings UI for these two keys: Zed only added
`language_server_workspace_configuration_schema` in the 0.8.0 API, which is
not published. Until 0.8.0 ships, the JSON above is the way to set them.

If you use a `PATH` VLS from stock V 0.5.x instead of the pinned build, check
its source before trusting any of this — the settings surface above is the
pinned fork's, and older VLS does not have these compat shapes.

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
