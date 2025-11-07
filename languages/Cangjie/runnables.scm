; ==============================================
; 标签1：仓颉单元测试（tag: cangjie-test）
; 适配场景：@Test标记的测试类和测试函数
; 支持排除带"// 跳过测试"注释的用例
; ==============================================
(
  (class_declaration
    (attribute (identifier) @_test_attr (#eq? @_test_attr "Test"))
    name: (identifier) @class_name
    body: (class_body
      (function_declaration
        (attribute (identifier) @_case_attr (#eq? @_case_attr "TestCase"))?
        name: (identifier) @test_name
        (#not-has-ancestor?
          (line_comment)
          (#match? @line_comment "跳过测试|skip test")
        )
      ) @test_case
    )
  )
  (#set! tag cangjie-test)
  (#set! test.type "unit")
  (#set! test.class @class_name)
  (#set! test.name @test_name)
  (#set! test.file @file)
)

; 独立测试函数（无类包裹的@Test函数）
(
  (function_declaration
    (attribute (identifier) @_test_attr (#eq? @_test_attr "Test"))
    name: (identifier) @test_name
    (#not-has-ancestor?
      (line_comment)
      (#match? @line_comment "跳过测试|skip test")
    )
  ) @test_case
  (#set! tag cangjie-test)
  (#set! test.type "unit")
  (#set! test.name @test_name)
  (#set! test.file @file)
)

; ==============================================
; 标签2：仓颉程序入口 main 函数（tag: cangjie-run）
; 适配仓颉入口规范：支持带/不带参数、返回值的main函数
; ==============================================
(
  (function_declaration
    "pub"?
    "func"
    name: (identifier) @_name (#eq? @_name "main")
    parameters: (parameter_list
      (parameter
        name: (identifier) @_param_name (#eq? @_param_name "args")
        type: (qualified_type
          (identifier) @_type1 (#eq? @_type1 "Array")
          (type_arguments (identifier) @_type2 (#eq? @_type2 "String"))
        )
      )?
    )
    return_type: (identifier)? @_return_type
    (#or?
      (#eq? @_return_type "Void")
      (#not-exists? @_return_type)
    )
  ) @entry
  (#set! tag cangjie-run)
  (#set! program.entry "main")
  (#set! program.file @file)
)
