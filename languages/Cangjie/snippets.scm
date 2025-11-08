; 函数定义
(snippet
  (trigger "fn" "函数定义")
  (body "fn ${1:function_name}(${2:params}) -> ${3:Void} {\n  ${4:// 函数体}\n  return ${5:null};\n}")
)

; 异步函数
(snippet
  (trigger "asyncfn" "异步函数定义")
  (body "async fn ${1:function_name}(${2:params}) -> ${3:Void} {\n  let result = await ${4:async_operation};\n  return ${5:result};\n}")
)

; 结构体
(snippet
  (trigger "struct" "结构体定义")
  (body "struct ${1:StructName} {\n  ${2:field_name}: ${3:Type},\n}")
)

; 枚举
(snippet
  (trigger "enum" "枚举定义")
  (body "enum ${1:EnumName} {\n  ${2:Variant1},\n  ${3:Variant2},\n}")
)

; 导入
(snippet
  (trigger "import" "导入模块")
  (body "import ${1:module_name}${2: as alias};\n")
)

; 条件语句
(snippet
  (trigger "if" "条件语句")
  (body "if ${1:condition} {\n  ${2:// 分支逻辑}\n}${3: else {\n  ${4:// else 分支}\n}}")
)

; 循环语句
(snippet
  (trigger "for" "for 循环")
  (body "for ${1:variable} in ${2:iterable} {\n  ${3:// 循环体}\n}")
)
