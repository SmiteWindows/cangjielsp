[
  (block)
  (switch_expression)
  (initializer_list)
  (asm_expression)
  (if_statement)
  (while_statement)
  (for_statement)
  (if_expression)
  (else_clause)
  (for_expression)
  (while_expression)
  (function_signature)
  (struct_declaration)
  (enum_declaration)
  (union_declaration)
  (opaque_declaration)
  (error_set_declaration)
] @fold

; 细化参数列表折叠（仅多参数时）
(parameters
  (#has-more-than? 2)
) @fold

; 细化函数调用折叠（仅多参数/多行时）
(call_expression
  (#or
    (#has-more-than? arguments 3)
    (#is-multiline? arguments)
  )
) @fold

; 补充折叠场景
(function_declaration
  (block) @fold)

(class_declaration) @fold
(interface_declaration) @fold

(block_comment) @fold

(array_literal
  (#has-more-than? 3)
) @fold

(generic_type
  (#has-more-than? type_arguments 2)
) @fold

; 类型条件表达式（仅含 else 时折叠）
(if_type_expression
  (#has-child? else_clause)
) @fold
