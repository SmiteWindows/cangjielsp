; 全局函数定义（排除类/结构体内部方法）
(functionDefinition
  (funcName) @definition.function
  (#not-has-parent? classBody structBody)
  (#set! priority 200)
)

; 类定义
(classDefinition
  (className) @definition.class
  (#set! priority 200)
)

; 结构体定义
(structDefinition
  (structName) @definition.struct
  (#set! priority 200)
)

; 接口定义
(interfaceDefinition
  (interfaceName) @definition.interface
  (#set! priority 200)
)

; 类型别名定义
(typeAlias
  (typeAliasName) @definition.typeAlias
  (#set! priority 200)
)

; 宏定义
(macroDefinition
  (macroName) @definition.macro
  (#set! priority 200)
)

; 枚举定义
(enumDefinition
  (enumName) @definition.enum
  (#set! priority 200)
)

; 联合体定义
(unionDefinition
  (unionName) @definition.union
  (#set! priority 200)
)

; 常量定义
(constDefinition
  (constName) @definition.constant
  (#set! priority 200)
)

; 全局变量定义（可选，根据语言是否支持全局变量调整）
(variableDefinition
  (varName) @definition.variable
  (#not-has-parent? functionBody block) ; 排除局部变量
  (#set! priority 200)
)

; 类/结构体内部方法
(classBody
  (functionDefinition
    (funcName) @definition.method
    (#set! priority 200)
  )
)

(structBody
  (functionDefinition
    (funcName) @definition.method
    (#set! priority 200)
  )
)

; 类/结构体字段
(containerField
  (fieldName) @definition.field
  (#set! priority 200)
)

; 运算符重载函数
(operatorFunctionDefinition
  (operator) @definition.function
  (#set! priority 200)
)

; 错误集定义
(errorSetDefinition
  (errorSetName) @definition.error
  (#set! priority 200)
)
