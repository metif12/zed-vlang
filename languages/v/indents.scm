(block) @indent
; `match p.y {` sits at the block level and its arms indent one further level,
; matching `v fmt` output (measured on V 0.5.2: arms at +2 relative to fn body).
(match_expression) @indent
(struct_field_declaration_list) @indent
(enum_member_declaration_list) @indent
(interface_spec_list) @indent
