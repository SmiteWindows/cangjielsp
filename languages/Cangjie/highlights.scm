; 基础字面量
(stringLiteral) @string
(multilineString) @string
; (multiline_raw_string) @string.raw
; 若需单独高亮分隔符（如 r、#、"），可拆分：
(multiline_raw_string
  "#" @keyword.operator ; 原始标记 r 高亮为运算符
  (token) @string.delimiter ; 分隔符（"、#）高亮为字符串分隔符
  (string_content) @string.content ; 内部内容高亮为字符串内容
)
(templateString) @string
[
    (integerLiteral)
    (floatLiteral)
] @number
(booleanLiteral) @constant.builtin

; 变量与标识符
(varBindingPattern) @variable
(varDefinition (varName) @variable)
(letDeclaration (varName) @variable)
(thisSuperExpression) @variable.builtin
"_" @variable.other  ; 占位符

; 函数与属性
(funcName) @function
(propertyName) @property
(macroCall (macroName) @function.macro)

; 类型相关
(className) @type
(structName) @type
(interfaceName) @type
(returnType) @type
(typeAlias (typeAliasName) @type.alias)
(genericType (identifier) @type.parameter)
[
    "Int8"
    "Int16"
    "Int32"
    "Int64"
    "IntNative"
    "UInt8"
    "UInt16"
    "UInt32"
    "UInt64"
    "UIntNative"
    "Float16"
    "Float32"
    "Float64"
    "Bool"
    "Rune"
    "Unit"
    "Nothing"
    "CPointer"
    "CString"
    "Byte"
    "Int"
    "UInt"
    "This"
    "VArry"
    "Any"
    "Object"
    "Array"
    "None"
    "Some"
    "Option"
    "String"
    "Exception"
    "Error"
] @type.builtin

; 关键字与修饰符
(modifiers) @keyword
[
    "as"
    "abstract"
    "break"
    "case"
    "catch"
    "class"
    "const"
    "continue"
    "do"
    "else"
    "enum"
    "extend"
    "for"
    "func"
    "false"
    "finally"
    "foreign"
    "get"
    "handle"
    "if"
    "in"
    "is"
    "init"
    "import"
    "interface"
    "inout"
    "internal"
    "let"
    "mut"
    "main"
    "macro"
    "match"
    "open"
    "operator"
    "override"
    "package"
    "perform"
    "prop"
    "private"
    "protected"
    "public"
    "quote"
    "redef"
    "resume"
    "return"
    "sealed"
    "set"
    "spawn"
    "super"
    "static"
    "struct"
    "synchronized"
    "try"
    "this"
    "true"
    "type"
    "throw"
    "throwing"
    "unsafe"
    "var"
    "where"
    "while"
] @keyword

; 运算符
[
    "@"; 右结合
    ".";
    "["
    "]"
    "("
    ")"
    "++"
    "--"
    "?"
    "!"; 右结合
    "-"; 右结合
    "**"; 右结合
    "*"
    "/"
    "%"
    "+"
    ;"-"; 左结合
    "<<"
    ">>"
    "..="
    ".."
    "<"
    "<="
    ">"
    ">="
    "is"
    "as"
    "!="
    "=="
    "&"
    "^"
    "|"
    "&&"
    "||"
    "??"; coalescing 操作符  右结合
    "|>"; pipeline 操作符
    "~>"; composition 操作符
    "="
    "**="
    "*="
    "/="
    "%="
    "+="
    "-="
    "<<="
    ">>="
    "&="
    "^="
    "|="
    "&&="
    "||="
    "->"
    "<-"; 模式匹配操作符
    "<:"
    "=>"
    "{"
    "}"
    ";"
    ":"
    "@!"
    "#"
    "$"
    "!in"
    ; "..."
    ; ","
] @operator

; 注释
[
    (blockComment)
    (lineComment)
] @comment

; 常量与枚举
(constDefinition (constName) @constant)
(enumBody (identifier) @constant.enum)
(errorSetBody (identifier) @constant.error)

; 类继承：Father 是类型引用
(classDefinition
  "<:" @operator
  (typeIdentifier) @local.reference
  (#set! reference.kind "type") ; 标记为类型引用（父类/接口）
)

; 泛型约束：Comparable 是类型引用
(genericType
  "<:" @operator
  (typeIdentifier) @local.reference
  (#set! reference.kind "type")
)
; @Deprecated @Attribute @Frozen @FastNative @When
; @sourcePackage() @sourceFile() @sourceLine()
; macro package
