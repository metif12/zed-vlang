; Vim text objects (`af`/`if`, `ac`/`ic`) — mirrors Go/Python in Zed core.
; Node and field names attested in the pinned parser's node-types.json.
; Methods are plain function_declarations with a receiver, so they are
; covered by the function rule too.

(function_declaration
  body: (block
    "{"
    (_)* @function.inside
    "}")) @function.around

(struct_declaration
  (struct_field_declaration_list
    ("{"
      (_)* @class.inside
      "}")?)) @class.around

(enum_declaration
  (enum_member_declaration_list
    ("{"
      (_)* @class.inside
      "}")?)) @class.around

(interface_declaration
  (interface_spec_list
    ("{"
      (_)* @class.inside
      "}")?)) @class.around

(comment)+ @comment.around
