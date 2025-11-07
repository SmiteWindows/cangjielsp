; ==============================================
; 标签1：仓颉单元测试（tag: cangjie-test）
; 适配场景：@Test标记的测试类、测试函数及参数化测试
; 支持标签过滤@Tag、排除带"// 跳过测试"注释的用例
; ==============================================

; 测试类中的测试用例（带@TestCase）
(
  (class_declaration
    (attribute (identifier) @_test_attr (#eq? @_test_attr "Test"))
    (attribute (identifier) @_tag_attr (#eq? @_tag_attr "Tag"))*
    name: (identifier) @class_name
    body: (class_body
      (function_declaration
        (attribute (identifier) @_case_attr (#eq? @_case_attr "TestCase"))
        (attribute (identifier) @_case_tag_attr (#eq? @_case_tag_attr "Tag"))*
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
  (#set! test.tags @_tag_attr @_case_tag_attr)
)

; 独立测试函数（无类包裹的@Test函数）
(
  (function_declaration
    (attribute (identifier) @_test_attr (#eq? @_test_attr "Test"))
    (attribute (identifier) @_tag_attr (#eq? @_tag_attr "Tag"))*
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
  (#set! test.tags @_tag_attr)
)

; 参数化测试用例（带参数列表的@TestCase）
(
  (class_declaration
    (attribute (identifier) @_test_attr (#eq? @_test_attr "Test"))
    name: (identifier) @class_name
    body: (class_body
      (function_declaration
        (attribute
          (identifier) @_case_attr (#eq? @_case_attr "TestCase")
          (argument_list (expression)) ; 匹配参数化测试的参数列表
        )
        name: (identifier) @test_name
        parameters: (parameter_list) @params
        (#not-has-ancestor?
          (line_comment)
          (#match? @line_comment "跳过测试|skip test")
        )
      ) @test_case
    )
  )
  (#set! tag cangjie-test)
  (#set! test.type "parameterized")
  (#set! test.class @class_name)
  (#set! test.name @test_name)
  (#set! test.parameters @params)
  (#set! test.file @file)
)

; 动态测试套（@TestBuilder函数）
(
  (function_declaration
    (attribute (identifier) @_builder_attr (#eq? @_builder_attr "TestBuilder"))
    name: (identifier) @builder_name
    return_type: (identifier) @_return_type (#eq? @_return_type "TestSuite")
  ) @dynamic_test
  (#set! tag cangjie-test)
  (#set! test.type "dynamic")
  (#set! test.name @builder_name)
  (#set! test.file @file)
)

; ==============================================
; 标签2：仓颉程序入口 main 函数（tag: cangjie-run）
; 适配仓颉入口规范：支持带/不带参数、返回值的main函数
; ==============================================
(
  (function_declaration
    "public"?
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
      (#eq? @_return_type "Unit")
      (#eq? @_return_type "Int64") ; 支持返回Int64类型的main函数
      (#not-exists? @_return_type)
    )
  ) @entry
  (#set! tag cangjie-run)
  (#set! program.entry "main")
  (#set! program.file @file)
)
