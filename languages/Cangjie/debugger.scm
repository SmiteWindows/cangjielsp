; ------------------------------
; 可调试变量（@debug-variable）
; ------------------------------
; 变量声明（排除占位符 _ 和类型名）
(variable_declaration
  (identifier) @debug-variable
  (#not-eq? @debug-variable "_")
  (#not-has-type? @debug-variable type_identifier)
)

; 赋值表达式（左侧目标变量 + 右侧源变量）
(assignment_expression
  left: (identifier) @debug-variable
  (#not-eq? @debug-variable "_")
)
(assignment_expression
  right: (identifier) @debug-variable
  (#not-eq? @debug-variable "_")
)

; 初始化列表/数组/对象中的变量
(initializer_list (identifier) @debug-variable)
(array_literal (identifier) @debug-variable)
(object_literal (identifier) @debug-variable)

; 循环语句中的变量
(for_statement (identifier) @debug-variable)
(while_statement (identifier) @debug-variable)
(for_expression (identifier) @debug-variable)
(while_expression (identifier) @debug-variable)

; 条件语句/表达式中的变量
(if_statement condition: (identifier) @debug-variable)
(if_expression condition: (identifier) @debug-variable)
(match_expression (identifier) @debug-variable)
(match_case
  "=>"
  (expression (identifier) @debug-variable)
)

; 二元/一元/索引/范围表达式中的变量
(binary_expression (identifier) @debug-variable)
(unary_expression argument: (identifier) @debug-variable)
(index_expression index: (identifier) @debug-variable)
(range_expression (identifier) @debug-variable)

; 函数相关变量（参数 + 调用参数）
(parameter name: (identifier) @debug-variable)
(call_expression
  (arguments (identifier) @debug-variable)
)
(builtin_function
  (call_expression
    (arguments (identifier) @debug-variable)
  )
)

; 异常处理变量
(catch_expression (identifier) @debug-variable)
(try_expression (identifier) @debug-variable)

; 返回/跳转语句中的变量
(return_expression (identifier) @debug-variable)
(break_expression (identifier) @debug-variable)
(continue_expression (identifier) @debug-variable)

; 模板字符串插值中的变量
(template_string
  (interpolation (identifier) @debug-variable)
)

; 其他场景变量
(payload (identifier) @debug-variable)
(parenthesized_expression (identifier) @debug-variable)

; ------------------------------
; 变量类型标记（可选，帮助调试器分类）
; ------------------------------
; 函数参数
(parameter name: (identifier) @debug-variable)
(#set! variable.kind "parameter")

; 局部变量（函数内）
(function_declaration
  (variable_declaration (identifier) @debug-variable)
  (#set! variable.kind "local")
)

; 类/结构体字段
(class_declaration
  (container_field (identifier) @debug-variable)
  (#set! variable.kind "field")
)
(struct_declaration
  (container_field (identifier) @debug-variable)
  (#set! variable.kind "field")
)

; 全局变量（源文件根级）
(source_file
  (variable_declaration (identifier) @debug-variable)
  (#set! variable.kind "global")
)

; 常量（只读）
(const_declaration (identifier) @debug-variable)
(#set! variable.readonly true)

; ------------------------------
; 调试作用域（@debug-scope）
; ------------------------------
; 块级作用域
(block) @debug-scope
(#set! scope.kind "block")

; 函数作用域
(function_declaration) @debug-scope
(#set! scope.kind "function")

; 类/结构体/接口作用域
(class_declaration) @debug-scope
(#set! scope.kind "class")
(struct_declaration) @debug-scope
(#set! scope.kind "struct")
(interface_declaration) @debug-scope
(#set! scope.kind "interface")

; 循环/条件作用域
(for_statement) @debug-scope
(#set! scope.kind "loop")
(while_statement) @debug-scope
(#set! scope.kind "loop")
(if_statement) @debug-scope
(#set! scope.kind "conditional")
(switch_statement) @debug-scope
(#set! scope.kind "conditional")

; 异常处理作用域
(try_statement) @debug-scope
(#set! scope.kind "try")
(catch_expression) @debug-scope
(#set! scope.kind "catch")
