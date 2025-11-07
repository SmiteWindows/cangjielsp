; 1. 测试声明
(test_declaration
  "test" @context
  [
   (string)
   (identifier)
  ] @name
) @item

; 2. 函数声明（带实现、返回类型）
(function_declaration
  "public"? @context
  [
    "extern"
    "export"
    "inline"
    ;"noinline"
    "static"
    ;"public"
    "protected"
    "internal"
    "private"
    "mut"
  ]? @context
  "func" @context
  name: (_) @name
  (return_type) @context
  (block) @context  ; 仅显示带实现的函数
) @item
; 3. main函数声明
(function_declaration
  "main" @context
  name: (_) @name
  (return_type) @context
  (block) @context  ; 仅显示带实现的函数
) @item
; 4. 全局变量声明（排除局部变量）
(source_file
  (variable_declaration
    "public"? @context
    (identifier) @name
    ("=" (literal) @context)?  ; 仅保留字面量初始值
    (#not-has-parent? function_declaration block)
  ) @item)

; 5. 结构体（父条目）+ 字段（子条目）
(struct_declaration
  "public"? @context
  "struct" @context
  (identifier) @name
) @item

(struct_declaration
  (struct_body
    (variable_declaration
      "public"? @context
      (identifier) @name
      ("=" (literal) @context)?) @item
    (container_field
      . (identifier) @name) @item
  )
)

; 6. 联合体（父条目）+ 字段（子条目）
(union_declaration
  "public"? @context
  "union" @context
  (identifier) @name
) @item

(union_declaration
  (union_body
    (variable_declaration
      "public"? @context
      (identifier) @name
      ("=" (literal) @context)?) @item
    (container_field
      . (identifier) @name) @item
  )
)

; 7. 枚举（父条目）+ 成员（子条目）
(enum_declaration
  "public"? @context
  "enum" @context
  (identifier) @name
) @item

(enum_declaration
  (enum_body
    (variable_declaration
      "public"? @context
      (identifier) @name
      ("=" (literal) @context)?) @item
  )
)

; 8. 类声明
(class_declaration
  "public"? @context
  "abstract"? @context
  "class" @context
  (identifier) @name
  ("<:" (type_identifier) @context)?
) @item

; 9. 接口声明
(interface_declaration
  "public"? @context
  "interface" @context
  (identifier) @name
) @item

; 10. 类型别名
(type_alias
  "public"? @context
  "type" @context
  (identifier) @name
  "=" (type_identifier) @context
) @item

; 11. 宏声明
(macro_declaration
  "public"? @context
  "macro" @context
  (identifier) @name
) @item
; 12. 不透明类型（父条目）+ 内部字段
(opaque_declaration
  "public"? @context
  "opaque" @context
  (identifier) @name
) @item

(opaque_declaration
  (opaque_body
    (variable_declaration
      "public"? @context
      (identifier) @name
      ("=" (literal) @context)?) @item
  )
)
