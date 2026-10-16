---
layout: default
title: Why the Rust shim stays
---

# Why the Rust shim stays

**Question:** can this extension be written entirely in V instead of Rust?

**Answer (2026-10-16):** no — not with today's tooling. This page records
why, so the question does not need re-investigation.

A Zed extension is not "any wasm file". It must be a WASM **Component**
speaking the host's WIT API (`zed:extension/...`), and Zed's own build log
says what it accepts:

```text
INFO [extension::extension_builder] compiling Rust crate for extension ...
```

Three facts block a V port:

1. **Only a Rust SDK exists.** The `zed_extension_api` crate
   ("APIs for creating Zed extensions in Rust") is the sole binding
   provider. There is no official SDK for any other language.
2. **V's wasm backend emits plain core modules, not components.**
   `v build` offers `-b wasm`, but V has no WIT / Component Model
   support — porting would mean hand-rolling the canonical ABI and the
   full WIT binding surface in V, hundreds of lines to replace ~30.
3. **There is nothing worth porting.** The whole Rust side is 30 lines
   (`src/lib.rs`: resolve `vls` on `PATH`, return a `Command`) and is
   verified working. Everything else — queries, snippets, tasks, docs —
   is already language-agnostic.

**What would change this:** V gaining Component Model output plus WIT
bindings, or Zed shipping a non-Rust extension SDK. Re-evaluate then;
until that day, see [Install](./install.md).
