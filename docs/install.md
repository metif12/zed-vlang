# Install

There is no marketplace release; install the checkout as a dev extension.
Every step below was verified end-to-end (Zed 1.24.2, V 0.5.2, VLS 0.0.3).

## 1. Requirements

- The V toolchain on `PATH` ([install](https://vlang.io)):

  ```sh
  v --version
  # V 0.5.2 ...
  ```

- VLS on `PATH`. VLS publishes no standalone binaries, so install it once
  with the V toolchain (takes a few minutes to compile from source):

  ```sh
  v ls --install
  vls --version
  # VLS 0.0.3
  ```

  (Older guides mention `v build-vls`; current V provides it as
  `v ls --install`.) Launch Zed from a shell where `vls` resolves —
  the extension inherits Zed's `PATH`.

- Rust with the `wasm32-wasip1` target, so Zed can compile the extension:

  ```sh
  rustup target add wasm32-wasip1
  ```

## 2. Install the dev extension

1. In Zed, open the command palette and run `zed: install dev extension`.
2. Select the checkout directory (the one containing `extension.toml`).
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

For tasks, copy the array from the README into your project's
`.zed/tasks.json` and run it with `task: spawn` — it must be a JSON
array (a bare object is rejected), written without a UTF-8 BOM.
