; 仓颉语法高亮规则（基于TokenKind和AST节点适配）

; 字符串字面量
(LitConstExpr
  (literal
    kind: STRING_LITERAL @string
    kind: SINGLE_QUOTED_STRING_LITERAL @string
    kind: JSTRING_LITERAL @string
    kind: MULTILINE_STRING @string
    kind: MULTILINE_RAW_STRING @string
    kind: RUNE_LITERAL @string
    kind: RUNE_BYTE_LITERAL @string
  )
)

; 多行原始字符串细化高亮
(LitConstExpr
  (literal
    kind: MULTILINE_RAW_STRING
    value: (string_content) @content
    (#match? @content "^(#+)\".*")
    (match-group 1 @keyword.operator) ; 原始标记#高亮
    (match-group 2 @string.delimiter) ; 起始引号"
  )
  (literal
    kind: MULTILINE_RAW_STRING
    value: (string_content) @content
    (#match? @content ".*\"(#+)")
    (match-group 1 @string.delimiter) ; 结束引号"
    (match-group 2 @keyword.operator) ; 结束标记#
  )
  (string_content) @string.content ; 字符串内容
)

; 数字字面量
(LitConstExpr
  (literal
    kind: INTEGER_LITERAL @number
    kind: FLOAT_LITERAL @number
  )
)

; 布尔与单元字面量
(LitConstExpr
  (literal
    kind: BOOL_LITERAL @constant.builtin
    kind: UNIT_LITERAL @constant.builtin
  )
)

; 变量与标识符
(var_binding_pattern) @variable
(var_definition (identifier) @variable)
(let_declaration (identifier) @variable)
(ThisExpr) @variable.builtin
(SuperExpr) @variable.builtin
"_" @variable.other ; 占位符
(identifier
  kind: DOLLAR_IDENTIFIER @variable.special ; $x特殊变量
)

; 函数与属性
(func_declaration (identifier) @function)
(macro_call (identifier) @function.macro)
(property_declaration (identifier) @property)

; 类型相关
(class_declaration (identifier) @type)
(struct_declaration (identifier) @type)
(interface_declaration (identifier) @type)
(enum_declaration (identifier) @type)
(union_declaration (identifier) @type)
(return_type) @type
(type_alias_declaration (identifier) @type.alias)
(generic_type (type_identifier) @type.parameter)
(error_set_declaration (identifier) @type)

; 内置类型
(identifier
  (#match? @value "^(Int8|Int16|Int32|Int64|IntNative|UInt8|UInt16|UInt32|UInt64|UIntNative|Float16|Float32|Float64|Bool|Rune|Unit|Nothing|CPointer|CString|Byte|Int|UInt|This|VArray|Any|Object|Array|None|Some|Option|String|Exception|Error)$")
  @type.builtin
)

; 关键字与修饰符
(Token
  kind: [
    IMPORT CLASS INTERFACE FUNC MACRO QUOTE LET VAR CONST TYPE INIT THIS SUPER
    IF ELSE CASE TRY CATCH FINALLY FOR DO WHILE THROW RETURN CONTINUE BREAK
    IN NOT_IN MATCH WHERE EXTEND WITH PROP STATIC PUBLIC PRIVATE INTERNAL
    PROTECTED OVERRIDE REDEF ABSTRACT SEALED OPEN FOREIGN INOUT MUT UNSAFE
    OPERATOR SPAWN SYNCHRONIZED MAIN TRUE FALSE HANDLE PERFORM RESUME THROWING
  ]
  @keyword
)

; 运算符
(Token
  kind: [
    DOT COMMA LPAREN RPAREN LSQUARE RSQUARE LCURL RCURL EXP MUL MOD DIV ADD SUB
    INCR DECR AND OR COALESCING EQ NE LT GT LE GE ASSIGN ADD_ASSIGN SUB_ASSIGN
    MUL_ASSIGN DIV_ASSIGN MOD_ASSIGN AND_ASSIGN OR_ASSIGN XOR_ASSIGN LSHIFT
    RSHIFT LSHIFT_ASSIGN RSHIFT_ASSIGN ARROW FAT_ARROW PIPE DOUBLE_PIPE
    DOUBLE_AMPERSAND POW_ASSIGN NOT AT AT_EXCL DOLLAR COLON SEMICOLON
    RANGE RANGE_INCLUSIVE UPPERBOUND
  ]
  @operator
)

; 注释
(Token
  kind: COMMENT @comment
)
(block_comment) @comment
(line_comment) @comment

; 常量与枚举
(const_declaration (identifier) @constant)
(enum_body (identifier) @constant.enum)
(error_set_body (identifier) @constant.error)

; 类继承与泛型约束
(class_declaration
  (Token kind: UPPERBOUND @operator)
  (type_identifier) @local.reference
  (#set! reference.kind "type")
)
(generic_type
  (Token kind: UPPERBOUND @operator)
  (type_identifier) @local.reference
  (#set! reference.kind "type")
)

; 注解与宏标记
(Token
  kind: ANNOTATION @attribute
  kind: MACRO @function.macro
)
