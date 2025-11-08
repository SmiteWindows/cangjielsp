; cjcov.toml 高亮规则
((section (identifier) @keyword.section.cjcov)
 (match? @keyword.section.cjcov "(collect|report|filter|threshold|advanced)"))

((string) @constant.language.mode
 (match? @constant.language.mode "(full|partial|fast)"))

((string) @constant.language.format
 (match? @constant.language.format "(text|html|json|xml|sarif)"))

(number) @number
(boolean) @constant.bool
(array) @container.array
