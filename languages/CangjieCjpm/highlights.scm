; cjpm.toml 高亮规则
((section (identifier) @keyword.section.cjpm)
 (match? @keyword.section.cjpm "(package|dependencies|dev_dependencies|build)"))

((key (identifier) @keyword.attribute.package)
 (match? @keyword.attribute.package "(name|version|authors|description|license|repository)"))

((key (identifier) @keyword.attribute.build)
 (match? @keyword.attribute.build "(target|release|features|rustc_flags)"))

(string) @string
(number) @number
(boolean) @constant.bool
