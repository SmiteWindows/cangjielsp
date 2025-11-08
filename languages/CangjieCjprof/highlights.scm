; cjprof.toml 高亮规则
((section (identifier) @keyword.section.cjprof)
 (match? @keyword.section.cjprof "(sample|analyze|report|ignore|threshold|advanced)"))

((key (identifier) @keyword.attribute.sample)
 (match? @keyword.attribute.sample "(types|interval|duration|dir|incremental|enable_debug_info)"))

((key (identifier) @keyword.attribute.analyze)
 (match? @keyword.attribute.analyze "(hotspot_threshold|call_stack_depth|merge_same_stacks)"))

((key (identifier) @keyword.attribute.report)
 (match? @keyword.attribute.report "(formats|flamegraph|show_optimization_hints)"))

((string) @constant.language.sample_type
 (match? @constant.language.sample_type "(Cpu|Memory|Coroutine|Lock|Io|Gc)"))

((string) @constant.language.report_format
 (match? @constant.language.report_format "(Text|Html|Json|Sarif|Flamegraph)"))

((string) @constant.language.theme
 (match? @constant.language.theme "(Color|Mono|Heatmap)"))

(number) @number
(boolean) @constant.bool
(array) @container.array
