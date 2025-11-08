; cjfmt.toml 高亮规则
((section (identifier) @keyword.section.cjfmt)
 (match? @keyword.section.cjfmt "(indent|line_width|newline|space|naming|ignore|advanced)"))

((key (identifier) @keyword.attribute.indent)
 (match? @keyword.attribute.indent "(style|size)"))

((string) @constant.language.naming
 (match? @constant.language.naming "(snake_case|pascal_case|camel_case|upper_snake_case|kebab-case|preserve)"))

(number) @number
(boolean) @constant.bool
(array) @container.array
