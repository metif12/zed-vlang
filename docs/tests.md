---
layout: default
title: Working with tests
---

# Working with V tests

Zed has no extension API for custom editor commands, so the test-file
features of `vscode-vlang` land here as documented workflows backed by
the [task list](../README.md#what-you-get). All shapes below mirror the
upstream implementation.

## Jump between source and test file

Upstream's `v.toggleTestFile` maps names in the same directory only:
`foo.v` becomes `foo_test.v` (insert `_test` before the extension) and
`foo_test.v` back to `foo.v` (strip the suffix). In Zed, Ctrl+P the
counterpart name — same rule, same directory.

## Generate a test file

Upstream's `v.generateTestFile` writes `foo_test.v` beside `foo.v` with
the module line plus one `fn test_<name>()` per **public** function, and
refuses to overwrite an existing file. Recipe: create `foo_test.v`,
paste the `module` line, and expand the `test` snippet once per public
function. Never overwrite by hand what the command would refuse to.

## Generate tests for uncovered functions

Upstream's `v.generateTests` shells out to:

```text
v cover <coverage-dir> --write-tests --filter <absolute-source-path> -P false
```

run in the project root after a coverage test run, with filtering by file
(not by function) — test bodies are composed by `v` itself. The QuickFix
lightbulb around it is VSCode-only and has no Zed equivalent. Note: `v
cover` has no help topic on V 0.5.2, so this recipe needs a V that ships
`v cover --write-tests`; the task shape above is otherwise directly
runnable as a Zed task.

## Translate C to V

The README task `V: translate C to V (needs c2v)` runs the file through
the [c2v](https://github.com/vlang/c2v) binary, which writes the sibling
`.v` file itself. Save the `.c` file before spawning the task (upstream
saves it for you; Zed tasks run on disk state — ours declares
`"save": "current"`).

## c2v setup

Two README tasks do the setup, in order:

1. `V: clone c2v (one-time)` — runs
   `v install --git https://github.com/vlang/c2v`.
2. `V: build c2v (needs clang)` — runs `v -g ~/.vmodules/c2v/`.

`clang` must be on `PATH` before step 2, and `c2v` must be on `PATH` before
translating. Upstream instead hangs an `Install c2v` action off the failure
message; Zed cannot express that, because the extension API has no commands.

These tasks carry **no shell guard**, deliberately. Zed runs a task in your
login shell — PowerShell on Windows, bash/zsh elsewhere — so a single
`command` string cannot contain a precondition that works in both. A failed
`c2v` invocation saying `command not found` is the guard.

`v -g ~/.vmodules/c2v/` relies on the shell expanding `~`, which bash and
zsh do and PowerShell does not. On Windows, either run
`v -g $env:USERPROFILE\.vmodules\c2v\` directly, or point the task's `args`
at your own path.
