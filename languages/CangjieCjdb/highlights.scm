; cjdb.toml 高亮规则
((section (identifier) @keyword.section.cjdb)
 (match? @keyword.section.cjdb "(session|breakpoints|logging|performance)"))

((key (identifier) @keyword.attribute.session)
 (match? @keyword.attribute.session "(port|timeout|attach_child_processes|enable_coroutine_debug)"))

((key (identifier) @keyword.attribute.performance)
 (match? @keyword.attribute.performance "(enable_profiling|profiling_types|profiling_interval)"))

(string) @string
(number) @number
(boolean) @constant.bool
(array) @container.array
