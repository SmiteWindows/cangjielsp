; 仓颉语言符号定义规则（基于AST节点）

; 全局函数定义（排除类/结构体内部方法）
(FuncDecl
  name: (identifier) @definition.function
  (#not-has-parent? ClassDecl StructDecl)
  (#set! priority 200)
)

; main函数定义
(MainDecl
  name: (identifier) @definition.function
  (#set! priority 200)
)

; 类定义
(ClassDecl
  name: (identifier) @definition.class
  (#set! priority 200)
)

; 结构体定义
(StructDecl
  name: (identifier) @definition.struct
  (#set! priority 200)
)

; 接口定义
(InterfaceDecl
  name: (identifier) @definition.interface
  (#set! priority 200)
)

; 类型别名定义
(TypeAlias
  name: (identifier) @definition.typeAlias
  (#set! priority 200)
)

; 宏定义
(MacroDecl
  name: (identifier) @definition.macro
  (#set! priority 200)
)

; 枚举定义
(EnumDecl
  name: (identifier) @definition.enum
  (#set! priority 200)
)

; 全局变量定义（排除局部变量）
(VarDecl
  name: (identifier) @definition.variable
  (#not-has-parent? FuncDecl Block) ; 排除函数内和块内变量
  (#set! priority 200)
)

; 类内部方法
(ClassDecl
  (Body
    (FuncDecl
      name: (identifier) @definition.method
      (#set! priority 200)
    )
  )
)

; 结构体内部方法
(StructDecl
  (Body
    (FuncDecl
      name: (identifier) @definition.method
      (#set! priority 200)
    )
  )
)

; 主构造函数
(PrimaryCtorDecl
  name: (identifier) @definition.constructor
  (#set! priority 200)
)

; 类/结构体字段
(Body
  (container_field
    name: (identifier) @definition.field
    (#set! priority 200)
  )
)

; 属性定义
(PropDecl
  name: (identifier) @definition.property
  (#set! priority 200)
)

; 运算符重载函数
(OperatorDecl
  operator: (_) @definition.function
  (#set! priority 200)
)
