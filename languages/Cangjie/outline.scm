; 仓颉语法声明语法符号提取规则（基于AST节点）

; 1. 测试声明
(test_declaration
  "test" @context
  [
    (string)
    (identifier)
  ] @name
) @item

; 2. 函数声明（带实现、返回类型）
(func_declaration
  "public"? @context
  [
    "extern"
    "export"
    "inline"
    "static"
    "protected"
    "internal"
    "private"
    "mut"
  ]? @context
  "func" @context
  name: (identifier) @name
  (return_type) @context
  (block) @context  ; 仅显示带实现的函数
) @item

; 3. main函数声明
(main_decl
  "main" @context
  name: (identifier) @name
  (return_type) @context
  (block) @context  ; 仅显示带实现的函数
) @item

; 4. 全局变量声明（排除局部变量）
(program
  (var_decl
    "public"? @context
    (identifier) @name
    ("=" (literal) @context)?  ; 仅保留字面量初始值
    (#not-has-parent? func_declaration block)
  ) @item)

; 5. 结构体（父条目）+ 字段（子条目）
(struct_declaration
  "public"? @context
  "struct" @context
  (identifier) @name
) @item

(struct_declaration
  (body
    (var_decl
      "public"? @context
      (identifier) @name
      ("=" (literal) @context)?) @item
    (container_field
      . (identifier) @name) @item
  )
)

; 6. 枚举（父条目）+ 成员（子条目）
(enum_declaration
  "public"? @context
  "enum" @context
  (identifier) @name
) @item

(enum_declaration
  (body
    (var_decl
      "public"? @context
      (identifier) @name
      ("=" (literal) @context)?) @item
  )
)

; 7. 类声明
(class_declaration
  "public"? @context
  "abstract"? @context
  "class" @context
  (identifier) @name
  ("<:" (type_identifier) @context)?
) @item

; 8. 接口声明
(interface_declaration
  "public"? @context
  "interface" @context
  (identifier) @name
) @item

; 9. 类型别名
(type_alias
  "public"? @context
  "type" @context
  (identifier) @name
  "=" (type_identifier) @context
) @item

; 10. 宏声明
(macro_decl
  "public"? @context
  "macro" @context
  (identifier) @name
) @item

; 11. 主构造函数声明
(primary_ctor_decl
  [
    "public"
    "internal"
    "private"
  ]? @context
  (identifier) @name  ; 构造函数名（通常与类名一致）
  (func_params) @context
  (block)? @context
) @item

; 12. 属性声明
(prop_decl
  "public"? @context
  "prop" @context
  (identifier) @name
  ":" (type_identifier) @context
  (block) @context  ; 属性体（包含getter/setter）
) @item
