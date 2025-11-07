; 基础括号对（仓颉语法适配版）
(
  (:or
    (ParenExpr)              ; 括号表达式 (a + b)
    (call_expression)        ; 函数调用 f(...)
    (parameter_list)         ; 参数列表 (x: Int, y: Bool)
    (TupleLiteral)           ; 元组字面量 (1, "a")
    (MatchExpr lParen: _)    ; 模式匹配 match(...) 中的括号
    (QuoteExpr lParen: _)    ; quote表达式 quote(...) 中的括号
    (VArrayExpr lParen: _)   ; 变长数组表达式 (...) 中的括号
  )
  "(" @open
  ")" @close
)

(
  (:or
    (ArrayLiteral)           ; 数组字面量 [1, 2, 3]
  )
  "[" @open
  "]" @close
)

(
  (:or
    (Block)                  ; 代码块 { ... }
    (MatchExpr lBrace: _)    ; 模式匹配 match { ... } 中的大括号
  )
  "{" @open
  "}" @close
)

; 字符串/字符引号对（排除转义符号）
(LitConstExpr
  (literal
    kind: STRING_LITERAL
    value: (string_content) @content
    (#match? @content "^[^\\\\]\"") ; 非转义起始引号
    (#set! @open (get-char @content -1))
  )
  (literal
    kind: STRING_LITERAL
    value: (string_content) @content
    (#match? @content "\"[^\\\\]$") ; 非转义结束引号
    (#set! @close (get-char @content 0))
  )
)

; 多行字符串
(LitConstExpr
  (literal
    kind: MULTILINE_STRING
    value: "\"\"\"" @open
  )
  (literal
    kind: MULTILINE_STRING
    value: "\"\"\"" @close
  )
)

; 多行原始字符串
(LitConstExpr
  (literal
    kind: MULTILINE_RAW_STRING
    value: (string_content) @content
    (#match? @content "^#+\".*") ; 起始分隔符 e.g. "#\""
    @content @open
  )
  (literal
    kind: MULTILINE_RAW_STRING
    value: (string_content) @content
    (#match? @content ".*\"#+$") ; 结束分隔符 e.g. "\"#"
    @content @close
  )
)

; 泛型参数
(generic_type
  "<" @open
  (type_arguments)
  ">" @close
)

; 块注释
(block_comment
  "/*" @open
  (comment_content)
  "*/" @close
)

; 区间分隔符
(range_expression
  ".." @separator
)

(range_expression
  "..=" @separator
)

; 美元标识符（特殊标记）
(identifier
  value: "$" @dollar
  (#has-parent? (DOLLAR_IDENTIFIER))
)
