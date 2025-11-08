//! 语法定义与高亮配置管理（整合所有仓颉相关文件类型）
use std::collections::HashMap;
use zed_extension_api as zed;

/// 语法高亮配置映射（文件类型 -> 高亮规则）
pub fn get_syntax_highlights() -> HashMap<String, zed::SyntaxHighlightConfig> {
    let mut highlights = HashMap::new();

    // 主语言（.cj 文件）高亮
    highlights.insert(
        "Cangjie".to_string(),
        zed::SyntaxHighlightConfig {
            scope: "source.cangjie".to_string(),
            patterns: vec![
                // 关键字
                zed::HighlightPattern {
                    match_type: zed::MatchType::Regex,
                    match_text: r"\b(if|else|for|while|fn|struct|enum|module|import|export|return|let|const|mut|async|await)\b".to_string(),
                    scope: "keyword.control.cangjie".to_string(),
                },
                // 类型
                zed::HighlightPattern {
                    match_type: zed::MatchType::Regex,
                    match_text: r"\b(Int|Float|Bool|String|Array|Map|Void)\b".to_string(),
                    scope: "support.type.cangjie".to_string(),
                },
                // 常量
                zed::HighlightPattern {
                    match_type: zed::MatchType::Regex,
                    match_text: r"\b(true|false|null)\b".to_string(),
                    scope: "constant.language.cangjie".to_string(),
                },
                // 函数调用
                zed::HighlightPattern {
                    match_type: zed::MatchType::Regex,
                    match_text: r"\b[a-zA-Z_][a-zA-Z0-9_]*\s*\(".to_string(),
                    scope: "entity.name.function.cangjie".to_string(),
                },
                // 注释
                zed::HighlightPattern {
                    match_type: zed::MatchType::Regex,
                    match_text: r"//.*$".to_string(),
                    scope: "comment.line.double-slash.cangjie".to_string(),
                },
                zed::HighlightPattern {
                    match_type: zed::MatchType::Regex,
                    match_text: r"/\*[\s\S]*?\*/".to_string(),
                    scope: "comment.block.cangjie".to_string(),
                },
            ],
            injections: vec![],
        },
    );

    // 其他配置文件高亮（继承 Toml 基础高亮）
    let config_file_patterns = vec![
        zed::HighlightPattern {
            match_type: zed::MatchType::Regex,
            match_text: r"\b(sample|analyze|report|ignore|threshold|advanced|indent|line_width|naming|collect)\b".to_string(),
            scope: "keyword.section.cangjie-config".to_string(),
        },
        zed::HighlightPattern {
            match_type: zed::MatchType::Regex,
            match_text: r"\b(Cpu|Memory|Coroutine|Lock|Io|Gc|Text|Html|Json|Sarif|Flamegraph)\b".to_string(),
            scope: "type.enum.cangjie-config".to_string(),
        },
    ];

    highlights.insert(
        "CangjieCjpm".to_string(),
        zed::SyntaxHighlightConfig {
            scope: "source.cangjie.cjpm".to_string(),
            patterns: config_file_patterns.clone(),
            injections: vec![],
        },
    );

    highlights.insert(
        "CangjieCjdb".to_string(),
        zed::SyntaxHighlightConfig {
            scope: "source.cangjie.cjdb".to_string(),
            patterns: config_file_patterns.clone(),
            injections: vec![],
        },
    );

    highlights.insert(
        "CangjieCjlint".to_string(),
        zed::SyntaxHighlightConfig {
            scope: "source.cangjie.cjlint".to_string(),
            patterns: config_file_patterns.clone(),
            injections: vec![],
        },
    );

    highlights.insert(
        "CangjieCjfmt".to_string(),
        zed::SyntaxHighlightConfig {
            scope: "source.cangjie.cjfmt".to_string(),
            patterns: config_file_patterns.clone(),
            injections: vec![],
        },
    );

    highlights.insert(
        "CangjieCjcov".to_string(),
        zed::SyntaxHighlightConfig {
            scope: "source.cangjie.cjcov".to_string(),
            patterns: config_file_patterns.clone(),
            injections: vec![],
        },
    );

    highlights.insert(
        "CangjieCjprof".to_string(),
        zed::SyntaxHighlightConfig {
            scope: "source.cangjie.cjprof".to_string(),
            patterns: config_file_patterns,
            injections: vec![],
        },
    );

    highlights
}

/// 获取语法片段（代码补全模板）
pub fn get_snippets() -> HashMap<String, Vec<zed::Snippet>> {
    let mut snippets = HashMap::new();

    // 主语言代码片段
    snippets.insert(
        "Cangjie".to_string(),
        vec![
            zed::Snippet {
                name: "函数定义".to_string(),
                trigger: "fn".to_string(),
                body: "fn ${1:function_name}(${2:params}) -> ${3:Void} {\n  ${4:// 函数体}\n  return ${5:null};\n}".to_string(),
                description: "创建一个仓颉函数".to_string(),
            },
            zed::Snippet {
                name: "结构体定义".to_string(),
                trigger: "struct".to_string(),
                body: "struct ${1:StructName} {\n  ${2:field_name}: ${3:Type},\n}".to_string(),
                description: "创建一个结构体".to_string(),
            },
            zed::Snippet {
                name: "异步函数".to_string(),
                trigger: "asyncfn".to_string(),
                body: "async fn ${1:function_name}(${2:params}) -> ${3:Void} {\n  let result = await ${4:async_operation};\n  return ${5:result};\n}".to_string(),
                description: "创建异步函数".to_string(),
            },
        ],
    );

    // 配置文件片段（以 cjprof 为例，其他类似）
    snippets.insert(
        "CangjieCjprof".to_string(),
        vec![
            zed::Snippet {
                name: "基础采样配置".to_string(),
                trigger: "sample".to_string(),
                body: "[sample]\ntypes = [Cpu, Coroutine, Memory]\ninterval = 10\nduration = 30\ndir = \"target/cjprof/samples\"\nincremental = false".to_string(),
                description: "cjprof 采样配置模板".to_string(),
            },
            zed::Snippet {
                name: "火焰图配置".to_string(),
                trigger: "flamegraph".to_string(),
                body: "[report.flamegraph]\nenable = true\nwidth = 1200\nheight = 600\ntheme = Color".to_string(),
                description: "火焰图可视化配置".to_string(),
            },
        ],
    );

    snippets
}
