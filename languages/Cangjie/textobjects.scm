; 仓颉语言文本对象规则（基于AST节点）

; 类与类似结构
[
  (ClassDecl (body) @class.inside)
  (StructDecl (body) @class.inside)
  (InterfaceDecl (body) @class.inside)
  (EnumDecl (body) @class.inside)
  (ExtendDecl (body) @class.inside)
] @class.around

; 函数与方法
[
  (FuncDecl (block) @function.inside)
  (OperatorDecl (block) @function.inside)
  (MainDecl (block) @function.inside)
  (MacroDecl (block) @function.inside)
  (PropDecl) @function.inside
] @function.around

; 类/结构体内部方法
(ClassDecl
  (Body
    (FuncDecl) @method.around
    (FuncDecl (block) @method.inside)
  )
)

(StructDecl
  (Body
    (FuncDecl) @method.around
    (FuncDecl (block) @method.inside)
  )
)

; 主构造函数
(PrimaryCtorDecl
  () @method.around
  (block) @method.inside
)

; 注释
[
  (lineComment)
  (blockComment)
] @comment.inside

[
  (lineComment)+
  (blockComment)
] @comment.around

; 参数
[
  (FuncParam) @parameter.around
  (namedParameter) @parameter.around
]

[
  (FuncParam (identifier) @parameter.inside) ; 参数名
  (FuncParam (type) @parameter.inside) ; 参数类型
  (namedParameter (identifier) @parameter.inside)
  (namedParameter (type) @parameter.inside)
  (GenericParam (identifier) @parameter.inside)
  (LambdaExpr (lambdaParameters (identifier) @parameter.inside))
  (parameterList (FuncParam*) @parameter.inside)
  (PrimaryCtorDecl (parameterList (FuncParam*) @parameter.inside))
] @parameter.around

; 条件/循环语句
[
  (IfExpr) @conditional.around
  (MatchExpr) @conditional.around
  (WhileExpr) @loop.around
  (DoWhileExpr) @loop.around
  (ForInExpr) @loop.around
]

(IfExpr (ifBlock) @conditional.inside)
(MatchExpr (MatchCase*) @conditional.inside)
(WhileExpr (block) @loop.inside)
(DoWhileExpr (block) @loop.inside)
(ForInExpr (block) @loop.inside)

; 容器（数组/元组/结构体字面量）
[
  (ArrayLiteral) @container.around
  (TupleLiteral) @container.around
  (StructLiteral) @container.around
]

(ArrayLiteral (arrayElements) @container.inside)
(TupleLiteral (tupleElements) @container.inside)
(StructLiteral (structFields) @container.inside)

; 字符串
[
  (stringLiteral) @string.around
  (multilineString) @string.around
]

(stringLiteral (stringContent) @string.inside)
(multilineString (stringContent) @string.inside)

; 类继承结构（选中 class Son <: Father）
(ClassDecl
  (identifier) @class.inside
  "<:" @operator
  (typeIdentifier) @class.inside
) @class.around
