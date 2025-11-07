; 基础括号对（带上下文约束，避免误匹配）
(
  (:or
    (parenthesizedExpression) ; 表达式括号 (a + b)
    (callExpression)          ; 函数调用括号 f(...)
    (parameterList)           ; 参数列表括号 (x: int, y: bool)
  )
  "(" @open
  ")" @close
)

(
  (:or
    (arrayLiteral)            ; 数组 [1,2,3]
    (sliceType)               ; 索引 [i]
  )
  "[" @open
  "]" @close
)

(
  (:or
    (block)                   ; 代码块 { ... }
    (objectLiteral)           ; 对象字面量 { key: val }
    (structLiteral)           ; 结构体字面量 { x: 1 }
  )
  "{" @open
  "}" @close
)

; 字符串/字符引号对（排除转义符号）
(stringLiteral
  (stringContent
    "\"" @open
    (#not-preceded-by? "\\")
  )
  (stringContent
    "\"" @close
    (#not-preceded-by? "\\")
  )
)
; 字符类型字面量
(:or (charLiteral) (stringLiteral))
  (charContent
    "r'" @open
    (#not-preceded-by? "\\")
  )
  (charContent
    "'" @close
    (#not-preceded-by? "\\")
  )
)
; RUNE_BYTE_LITERAL
(:or (charLiteral) (stringLiteral))
  (charContent
    "b'" @open
    (#not-preceded-by? "\\")
  )
  (charContent
    "'" @close
    (#not-preceded-by? "\\")
  )
)

; JSTRING_LITERAL
(:or (charLiteral) (stringLiteral))
  (charContent
    "J\"" @open
    (#not-preceded-by? "\\")
  )
  (charContent
    "'" @close
    (#not-preceded-by? "\\")
  )
)
; 多行字符串
(multilineString
  "\"\"\"" @open
  "\"\"\"" @close)

(multilineString
  "```" @open
  "```" @close)
; MULTILINE_RAW_STRING
; 插值字符串
(templateString
  "${" @open
  "}" @close)

; 泛型参数：左 < 在 genericType 下，右 > 也在同一节点下
(genericType
  "<" @open
  (typeArguments) ; 中间的参数节点
  ">" @close)

; 块注释：左 /* 和右 */ 都属于 blockComment 节点
(blockComment
  "/*" @open
  (commentContent) ; 注释内容节点
  "*/" @close)

; Payload 自定义分隔符
(payload
  "|" @open
  (#not-following-sibling? "|")
)
(payload
  "|" @close
  (#not-preceding-sibling? "|")
)

; 区间分隔符
(rangeExpression
  ".." @separator)

(rangeExpression
  "..=" @separator)
; DOLLAR_IDENTIFIER|        /*  e.g. "$x"          */
