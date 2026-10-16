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
`.v` file itself. Like upstream, the task checks for the binary first
and fails with setup instructions when it is missing. Prerequisites,
verbatim from upstream:

```text
c2v is not installed. Run `v install --git https://github.com/vlang/c2v`
to clone it, then `v -g ~/.vmodules/c2v/` to build it, and try again.
c2v also needs a `clang` binary on your PATH.
```

Save the `.c` file before spawning the task (upstream saves it for you;
Zed tasks run on disk state — ours declares `"save": "current"`).

The README task speaks Windows PowerShell (what Zed tasks use there).
The POSIX equivalent, same behavior:

```sh
command -v c2v >/dev/null || { echo "c2v is not installed. Run: v install --git https://github.com/vlang/c2v ; then: v -g ~/.vmodules/c2v/ ; c2v also needs clang on PATH."; exit 1; }
c2v "$ZED_FILE"
```
