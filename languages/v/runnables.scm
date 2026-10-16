; Runnable targets for V — inline Run actions in Zed, wired to the tagged
; tasks in the README (task tags match the `tag` set here).
; Node and field names attested in the pinned parser's node-types.json.

; `fn main` — runs the current file.
(((function_declaration
  name: (identifier) @run
  (#eq? @run "main"))) @_
  (#set! tag v-main))

; `fn test_*` — tests the current file.
(((function_declaration
  name: (identifier) @run
  (#match? @run "^test_"))) @_
  (#set! tag v-test))
