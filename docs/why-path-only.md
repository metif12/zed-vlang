---
layout: default
title: Why PATH-only
---

# Why PATH-only (no managed build)

The extension uses whatever `vls` resolves to on `PATH`. There is no
managed download any more.

## Why PATH-only

- **There is nothing to download.** VLS publishes no standalone
  binaries — its GitHub releases ship only a `.vsix` (checked 0.1 and
  0.3). A download path would first require someone to build, host and
  sign per-platform binaries. Nobody does, so PATH-only is not a
  shortcut but the only honest design.
- **The toolchain stays the user's.** toolchain, VLS and extension stay
  version-matched the way the user installed them (`v ls --install`
  builds VLS against their own V). Managed copies would drift from the
  compiler the user actually builds with.
- **No hosting or trust liability.** No binaries means no supply-chain
  surface, no per-platform build matrix (Windows/macOS/Linux ×
  x64/arm64), no update channel to maintain.
- **Tiny code.** The whole integration is ~15 lines; every failure mode
  was measured, not guessed (see
  [Troubleshooting](./troubleshooting.md)).

Costs, stated plainly: setup friction (`v ls --install` plus a working
`RUSTUP_HOME`), no auto-update, and version drift is the user's job.
The install guide and troubleshooting exist to pay the first down.

## What icons would take

File icons in Zed come from icon **themes**, not language extensions —
there is no `icon` contribution point for a language. Options:

- (a) Ship a second, theme-type extension associating an icon with V
  files.
- (b) Contribute `v.svg` to existing popular themes (upstream already
  ships one at `icons/v.svg`; check its license first — new upstream
  code is GPL-2.0-only).

Pros: visual identity in the file tree. Cons: a second extension to
maintain and publish for zero editor function, plus theme conflicts.
Verdict: not worth it; revisit only if Zed gains language icons.

Publishing to the Zed extension registry is out of scope by decision.