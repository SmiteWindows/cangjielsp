; 仓颉语法缩进规则（基于AST节点和Token类型）

; 缩进起始点（@indent.begin）
(init_declaration) @indent.begin  ; 构造函数初始化

; 类/结构体/接口定义体
(class_declaration
  (Body) @indent.begin)            ; 类体 { ... }
(struct_declaration
  (Body) @indent.begin)            ; 结构体体 { ... }
(interface_declaration
  (Body) @indent.begin)            ; 接口体 { ... }

; 函数与方法定义
(func_declaration
  (Block) @indent.begin)           ; 函数体 { ... }
(operator_declaration
  (Block) @indent.begin)           ; 运算符函数体 { ... }
(lambda_expr
  (Block) @indent.begin)           ; 匿名函数体 { ... }

; 多行函数调用参数
(call_expression
  (argument_list) @indent.begin)   ; 多行参数列表 ( ... )

; 集合与字面量
(array_literal) @indent.begin      ; 数组 [ ... ]
(struct_literal) @indent.begin     ; 结构体字面量 { ... }
(tuple_literal) @indent.begin      ; 元组 ( ... )

; 条件与分支语句
(if_expr) @indent.begin            ; if表达式
(match_expr) @indent.begin         ; match模式匹配
(match_case) @indent.branch        ; case分支（仅对齐不缩进）
(if_type_expr) @indent.begin       ; 类型条件表达式

; 循环语句
(for_in_expr) @indent.begin        ; for-in循环
(while_expr) @indent.begin         ; while循环

; 异常处理
(try_expr) @indent.begin           ; try表达式
(catch_clause) @indent.begin       ; catch子句
(finally_clause) @indent.begin     ; finally子句

; 缩进结束点（@indent.end）与分支对齐
(array_literal
  (Token kind: RSQUARE) @indent.end @indent.branch)  ; 数组结束 ]
(tuple_literal
  (Token kind: RPAREN) @indent.end @indent.branch)   ; 元组结束 )
(struct_literal
  (Token kind: RCURL) @indent.end @indent.branch)    ; 结构体结束 }
(Body
  (Token kind: RCURL) @indent.end @indent.branch)    ; 类/结构体/接口体结束 }
(Block
  (Token kind: RCURL) @indent.end @indent.branch)    ; 代码块结束 }
(call_expression
  (Token kind: RPAREN) @indent.end @indent.branch)   ; 函数调用结束 )

; 自动适配缩进（@indent.auto）
[
  (line_comment)                   ; 单行注释
  (block_comment)                  ; 块注释
] @indent.auto

; 忽略缩进（@indent.ignore）
[
  (multiline_string)               ; 多行字符串
  (template_string)                ; 模板字符串
  (multiline_raw_string)           ; 多行原始字符串
] @indent.ignore

; 单行代码块忽略缩进（无Block节点时）
(if_expr
  (expr) @indent.ignore
  (#not-has-child? (Block)))       ; 单行if（无块）
(for_in_expr
  (expr) @indent.ignore
  (#not-has-child? (Block)))       ; 单行for-in（无块）
(while_expr
  (expr) @indent.ignore
  (#not-has-child? (Block)))       ; 单行while（无块）
