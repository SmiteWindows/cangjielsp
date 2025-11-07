; Classes and similar constructs
[
  (classDefinition (classBody) @class.inside)
  (structDefinition (structBody) @class.inside)
  (interfaceDefinition (interfaceBody) @class.inside)
  (enumDefinition (enumBody) @class.inside)
  (extendDefinition (extendBody) @class.inside)
] @class.around

; Functions & Methods
[
  (functionDefinition (block) @function.inside)
  (operatorFunctionDefinition (block) @function.inside)
  (mainDefinition (block) @function.inside)
  (macroDefinition (block) @function.inside)
  (propertyDefinition) @function.inside ; 单行属性，inside = around
] @function.around

; Class methods (nested in class/struct)
(classBody
  (functionDefinition) @method.around
  (functionDefinition (block) @method.inside)
)

(structBody
  (functionDefinition) @method.around
  (functionDefinition (block) @method.inside)
)

; Comments
[
  (lineComment)
  (blockComment)
] @comment.inside

[
  (lineComment)+
  (blockComment)
] @comment.around

; Parameters
[
  (parameter) @parameter.around
  (namedParameter) @parameter.around
]

[
  (parameter (identifier) @parameter.inside) ; 参数名
  (parameter (type) @parameter.inside) ; 参数类型
  (namedParameter (identifier) @parameter.inside)
  (namedParameter (type) @parameter.inside)
  (typeParameters (identifier) @parameter.inside)
  (lambdaParameters (lambdaParameter) @parameter.inside)
  (parameterList (parameters) @parameter.inside)
  (primaryInitParamList (parameters) @parameter.inside)
] @parameter.around

; Conditional/Loop statements
[
  (if_statement) @conditional.around
  (switch_statement) @conditional.around
  (while_statement) @loop.around
  (for_statement) @loop.around
]

(if_statement (block) @conditional.inside)
(switch_statement (switch_body) @conditional.inside)
(while_statement (block) @loop.inside)
(for_statement (block) @loop.inside)

; Containers (array/object/struct literal)
[
  (arrayLiteral) @container.around
  (objectLiteral) @container.around
  (structLiteral) @container.around
]

(arrayLiteral (arrayElements) @container.inside)
(objectLiteral (objectProperties) @container.inside)
(structLiteral (structFields) @container.inside)

; Strings
[
  (stringLiteral) @string.around
  (multilineString) @string.around
]

(stringLiteral (stringContent) @string.inside)
(multilineString (stringContent) @string.inside)

; 类继承结构的文本对象（选中 class Son <: Father）
(classDefinition
  (className) @class.inside
  "<:" @operator
  (typeIdentifier) @class.inside
) @class.around
