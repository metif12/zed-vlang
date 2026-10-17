; Screen-share / recording redactions: Zed renders these nodes in a redacted
; form so literals (often tokens, keys, URLs) do not leak while presenting or
; recording. One of the ten query assets Zed loads from a language extension
; (crates/language_core/src/queries.rs); this one was missing.
[
  (interpreted_string_literal)
  (raw_string_literal)
  (rune_literal)
  (c_string_literal)
  (int_literal)
  (float_literal)
  (imaginary_literal)
] @redact
