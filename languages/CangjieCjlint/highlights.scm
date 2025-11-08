; cjlint.toml 高亮规则
((section (identifier) @keyword.section.cjlint)
 (match? @keyword.section.cjlint "(rules|exclude|include|check_level|fix|output_format)"))

((key (identifier) @keyword.attribute.rule)
 (match? @keyword.attribute.rule "(enabled|level|options)"))

((string) @constant.language.level
 (match? @constant.language.level "(error|warn|info|off)"))

(number) @number
(boolean) @constant.bool
(array) @container.array
