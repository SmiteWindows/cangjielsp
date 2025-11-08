; 仓颉语法高亮规则
(comment) @comment
(string) @string
(number) @number
(boolean) @constant.bool
(null) @constant.language

(keyword) @keyword
(keyword.control) @keyword.control
(keyword.operator) @keyword.operator
(keyword.type) @keyword.type

(identifier) @variable
(function_declaration name: (identifier)) @function
(struct_declaration name: (identifier)) @type.struct
(enum_declaration name: (identifier)) @type.enum
(module_declaration name: (identifier)) @namespace

(parameter name: (identifier)) @variable.parameter
(field_declaration name: (identifier)) @variable.other.member

(call_expression function: (identifier)) @function.call
(member_expression property: (identifier)) @property
