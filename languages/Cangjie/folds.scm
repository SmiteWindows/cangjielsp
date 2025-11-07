; 仓颉语法折叠规则（基于AST节点类型适配）
[
  (Block)                          ; 代码块 { ... }
  (MatchExpr)                      ; 模式匹配表达式 match { ... }
  (ArrayLiteral)                   ; 数组字面量 [ ... ]
  (IfExpr)                         ; 条件表达式 if (...) { ... }
  (WhileExpr)                      ; while循环 while (...) { ... }
  (ForExpr)                        ; for循环 for (...) { ... }
  (FuncDecl)                       ; 函数声明 func ... { ... }
  (ClassDecl)                      ; 类声明 class ... { ... }
  (StructDecl)                     ; 结构体声明 struct ... { ... }
  (EnumDecl)                       ; 枚举声明 enum ... { ... }
  (InterfaceDecl)                  ; 接口声明 interface ... { ... }
  (UnionDecl)                      ; 联合声明 union ... { ... }
  (LambdaExpr)                     ; 匿名函数 { ... }
  (ErrorSetDecl)                   ; 错误集声明 error ... { ... }
] @fold

; 细化参数列表折叠（仅多参数时）
(parameter_list
  (#has-more-than? 2)              ; 参数数量超过2个时折叠
) @fold

; 细化函数调用折叠（多参数或多行时）
(CallExpr
  (#or
    (#has-more-than? arguments 3)  ; 参数数量超过3个
    (#is-multiline? arguments)     ; 参数跨多行
  )
) @fold

; 函数体折叠（无论参数数量）
(FuncDecl
  (Block) @fold                    ; 函数体块始终折叠
)

; 泛型类型参数折叠（多类型参数时）
(generic_type
  (#has-more-than? type_arguments 2)  ; 泛型参数超过2个
) @fold

; 数组字面量折叠（多元素时）
(ArrayLiteral
  (#has-more-than? elements 3)     ; 数组元素超过3个
) @fold

; 块注释折叠
(block_comment) @fold              ; /* ... */ 块注释

; 条件表达式折叠（含else分支时）
(IfExpr
  (#has-child? else_clause)        ; 包含else分支的if表达式
) @fold

; 宏展开节点折叠（宏内容通常较长）
(MacroExpandExpr) @fold            ; 宏调用表达式 @M(...)
(MacroExpandDecl) @fold            ; 宏调用声明 @M class ...
