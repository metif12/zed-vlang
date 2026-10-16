---
layout: default
title: Why managed-first
---

# Why managed-first (PATH as fallback)

The extension prefers your tools over downloading: `src/lib.rs` looks up
`vls` with `worktree.which("vls")` first, and only then falls back to a
pinned managed build. This page records why PATH comes first, and what
the fallback cost.

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
- **Tiny code.** The whole integration is ~30 lines; every failure mode
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

## What managed downloads took (done 2026-10-16)

The blocker above is gone: our fork ([metif12/vls](https://github.com/metif12/vls))
merges the open upstream PRs and publishes per-platform zips
(`vls-windows-x86_64.zip`, `vls-linux-x86_64.zip`, `vls-macos-aarch64.zip`)
from a pinned tag. The extension is now a hybrid:

- `worktree.which("vls")` still wins — a user install stays paired with
  its own V toolchain (which VLS shells out to). Nothing about existing
  setups changed.
- Otherwise the extension downloads the pinned tag once (cached per
  version under its working directory), reports install status, and
  needs the one-time `download_file` capability grant Zed prompts for.
  Bumping the pinned tag is a deliberate, re-verified code change —
  never floating.

Remaining costs, accepted: whoever cuts the next `vls-N` tag owns the
three-platform build; offline machines must bring their own `vls`.

## Verdict (updated: managed-first)

The extension uses the pinned managed build whenever obtainable, and a
`PATH` install only as fallback (offline, unsupported platform, declined
download). Newer managed builds arrive via deliberate, re-verified pin
bumps — never floating. Stay that way; revisit only if upstream VLS
publishes its own standalone binaries (then the fork can retire).
Publishing to the Zed extension registry is out of scope by decision.
