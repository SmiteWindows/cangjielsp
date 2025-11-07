; ------------------------------
; 可调试变量（@debug-variable）
; 适配仓颉语言 AST 节点结构
; ------------------------------

; 变量声明节点（var 声明）
(VarDecl
  (pattern
    (VarPattern identifier: (Token) @debug-variable)
    (#not-eq? @debug-variable "_")
  )
)

; 变量声明中的类型引用排除
(VarDecl
  (declType (TypeNode) @_type)
  (pattern (VarPattern identifier: (Token) @debug-variable))
  (#not-has-ancestor? @debug-variable @_type)
)

; 赋值表达式中的变量（左值和右值）
(AssignExpr
  leftExpr: (RefExpr identifier: (Token) @debug-variable)
  (#not-eq? @debug-variable "_")
)
(AssignExpr
  rightExpr: (RefExpr identifier: (Token) @debug-variable)
  (#not-eq? @debug-variable "_")
)

; 数组初始化中的变量引用
(VArrayExpr
  arguments: (ArrayList
    (Argument
      (RefExpr identifier: (Token) @debug-variable)
    )
  )
)

; 循环语句中的变量（for-in 循环）
(ForInExpr
  pattern: (VarPattern identifier: (Token) @debug-variable)
  (#not-eq? @debug-variable "_")
)

; 条件表达式中的变量
(IfExpr
  condition: (RefExpr identifier: (Token) @debug-variable)
)
(MatchExpr
  expr: (RefExpr identifier: (Token) @debug-variable)
)
(MatchCase
  resultExpr: (RefExpr identifier: (Token) @debug-variable)
)

; 二元/一元表达式中的变量
(BinaryExpr
  leftExpr: (RefExpr identifier: (Token) @debug-variable)
)
(BinaryExpr
  rightExpr: (RefExpr identifier: (Token) @debug-variable)
)
(UnaryExpr
  expr: (RefExpr identifier: (Token) @debug-variable)
)

; 函数参数变量
(FuncParam
  identifier: (Token) @debug-variable
  (#not-eq? @debug-variable "_")
)

; 函数调用中的变量参数
(CallExpr
  arguments: (ArgumentList
    (Argument
      (RefExpr identifier: (Token) @debug-variable)
    )
  )
)

; 异常处理中的变量
(TryExpr
  catchClauses: (CatchClause
    parameter: (VarPattern identifier: (Token) @debug-variable)
  )
)

; 返回表达式中的变量
(ReturnExpr
  expr: (RefExpr identifier: (Token) @debug-variable)
)

; 括号表达式中的变量
(ParenExpr
  expr: (RefExpr identifier: (Token) @debug-variable)
)

; ------------------------------
; 变量类型标记
; ------------------------------

; 函数参数
(FuncParam
  identifier: (Token) @debug-variable
  (#set! variable.kind "parameter")
)

; 局部变量（函数内声明）
(FuncDecl
  body: (Block
    (VarDecl
      (pattern (VarPattern identifier: (Token) @debug-variable))
      (#set! variable.kind "local")
    )
  )
)

; 类成员变量
(ClassDecl
  body: (ClassBody
    (VarDecl
      (pattern (VarPattern identifier: (Token) @debug-variable))
      (#set! variable.kind "field")
    )
  )
)

; 全局变量（顶级声明）
(Program
  decls: (ArrayList
    (VarDecl
      (pattern (VarPattern identifier: (Token) @debug-variable))
      (#set! variable.kind "global")
    )
  )
)

; 常量（let 声明）
(LetPatternExpr
  pattern: (VarPattern identifier: (Token) @debug-variable)
  (#set! variable.readonly true)
)

; ------------------------------
; 调试作用域（@debug-scope）
; ------------------------------

; 块级作用域
(Block) @debug-scope
(#set! scope.kind "block")

; 函数作用域
(FuncDecl) @debug-scope
(#set! scope.kind "function")

; 类作用域
(ClassDecl) @debug-scope
(#set! scope.kind "class")

; 循环作用域
(ForInExpr) @debug-scope
(#set! scope.kind "loop")
(WhileExpr) @debug-scope
(#set! scope.kind "loop")
(DoWhileExpr) @debug-scope
(#set! scope.kind "loop")

; 条件作用域
(IfExpr) @debug-scope
(#set! scope.kind "conditional")
(MatchExpr) @debug-scope
(#set! scope.kind "conditional")

; 异常处理作用域
(TryExpr) @debug-scope
(#set! scope.kind "try")
(CatchClause) @debug-scope
(#set! scope.kind "catch")
