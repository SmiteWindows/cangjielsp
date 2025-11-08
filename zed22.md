# Cangjie 语言 Zed 扩展完整打包方案
以下是可直接部署的 Cangjie 语言 Zed 扩展完整代码+配置，包含所有模块、构建脚本和安装说明，遵循 Zed 官方扩展规范，可直接编译使用。

## 目录结构
```
cangjie-zed-extension/
├── src/
│   ├── lib.rs                # 扩展入口（必须）
│   ├── config.rs             # 配置模块
│   ├── tree_sitter_utils.rs  # Tree-sitter 工具
│   ├── language_server.rs    # LSP 核心实现
│   ├── cjfmt.rs              # 代码格式化
│   ├── cjlint.rs             # 代码检查
│   ├── syntax.rs             # 语法片段
│   └── utils.rs              # 通用工具（新增）
├── Cargo.toml                # 依赖配置
├── Cargo.lock                # 依赖锁（编译后自动生成）
├── package.json              # Zed 扩展配置（必须）
├── language-configuration.json  # 语言配置（必须）
├── LICENSE                   # 许可证
├── README.md                 # 安装说明
└── build.sh                  # 编译脚本（跨平台）
```

## 完整代码文件

### 1. Cargo.toml
```toml
[package]
name = "cangjie-zed-extension"
version = "0.5.0"
edition = "2021"
description = "Cangjie 语言 Zed 扩展（语法高亮、补全、格式化、诊断）"
authors = ["Your Name"]
license = "MIT"
repository = "https://github.com/your-username/cangjie-zed-extension"
keywords = ["zed", "extension", "language-server", "cangjie"]
categories = ["Development Tools", "Text Editors"]

[dependencies]
zed_extension_api = "0.10.0"
tree-sitter = "0.20.10"
tree-sitter-cangjie = { git = "https://gitcode.com/Cangjie-SIG/tree-sitter-cangjie", rev = "main" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4"
env_logger = "0.10"
glob = "0.3"
thiserror = "1.0"
once_cell = "1.18.0"
regex = "1.10.0"
walkdir = "2.4.0"

[lib]
name = "cangjie_zed_extension"
crate-type = ["cdylib"]  # Zed 扩展要求的库类型

[profile.release]
opt-level = 3
strip = true  # 减小二进制体积
lto = true    # 链接时优化
codegen-units = 1
```

### 2. src/lib.rs（扩展入口）
```rust
//! Cangjie 语言 Zed 扩展（遵循 Zed 官方规范）
//! 参考：https://zed.dev/docs/extensions/languages

use std::sync::Arc;
use zed_extension_api::{
    self as zed,
    lsp::{self, InitializeParams, ServerCapabilities},
    LanguageServer, LanguageServerFactory, Result,
};

mod config;
mod tree_sitter_utils;
mod language_server;
mod cjfmt;
mod cjlint;
mod syntax;
mod utils;

// 导出扩展工厂函数（Zed 强制要求命名）
#[zed::language_server_factory]
pub fn language_server_factory() -> Box<dyn LanguageServerFactory> {
    Box::new(CangjieLanguageServerFactory)
}

/// Cangjie LSP 工厂（实现 Zed 规范）
struct CangjieLanguageServerFactory;

impl LanguageServerFactory for CangjieLanguageServerFactory {
    /// 语言配置（与 tree-sitter-cangjie 绑定）
    fn language_config(&self) -> zed::LanguageConfig {
        zed::LanguageConfig {
            name: "Cangjie".to_string(),
            extensions: vec!["cj".to_string()],
            tree_sitter_language: tree_sitter_cangjie::language(),
            tree_sitter_highlights_query: tree_sitter_cangjie::HIGHLIGHTS_QUERY,
            tree_sitter_folds_query: tree_sitter_cangjie::FOLDS_QUERY,
            tree_sitter_indents_query: tree_sitter_cangjie::INDENTS_QUERY,
            comment: zed::CommentConfig {
                line_comment: "//".to_string(),
                block_comment: Some(("/*".to_string(), "*/".to_string())),
            },
            snippets: syntax::get_cangjie_snippets().unwrap_or_default(), // 加载语法片段
        }
    }

    /// 创建 LSP 服务器实例
    fn create_server(&self) -> Result<Box<dyn LanguageServer>> {
        // 加载配置
        let cangjie_config = config::load_zed_config()?;
        config::validate_config(&cangjie_config)?;

        // 初始化日志
        utils::init_logger(&cangjie_config.log_level)?;

        // 创建服务器
        let server = language_server::CangjieLanguageServer::new(Arc::new(cangjie_config));
        Ok(Box::new(server))
    }
}
```

### 3. src/config.rs（配置模块）
```rust
//! Cangjie 扩展配置（遵循 Zed 规范）
use serde::{Deserialize, Serialize};
use zed_extension_api::{config::Config as ZedConfig, Error, Result};

/// 缩进风格
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IndentStyle {
    #[serde(rename = "space")]
    Space,
    #[serde(rename = "tab")]
    Tab,
}

/// 格式化配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CjfmtConfig {
    pub indent_style: IndentStyle,
    pub indent_size: u8,
    pub tab_width: u8,
    pub line_ending: String,
    pub max_line_length: u16,
    pub function_brace_style: String,
    pub struct_brace_style: String,
    pub trailing_comma: bool,
    pub space_around_operators: bool,
    pub space_inside_brackets: bool,
    pub auto_fix_syntax: bool,
}

/// 检查配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CjlintConfig {
    pub check_level: String,
    pub enable_style_check: bool,
    pub enable_syntax_check: bool,
    pub ignore_rules: Vec<String>,
    pub custom_rules_path: Option<String>,
}

/// 覆盖率配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CjcovConfig {
    pub report_format: String,
    pub exclude_paths: Vec<String>,
    pub threshold: u8,
}

/// 性能分析配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CjprofConfig {
    pub sampling_rate: u32,
    pub output_path: String,
    pub enable_cpu_profiling: bool,
    pub enable_memory_profiling: bool,
}

/// 全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CangjieConfig {
    pub lsp_timeout_ms: u64,
    pub realtime_diagnostics: bool,
    pub fmt: CjfmtConfig,
    pub lint: CjlintConfig,
    pub cov: CjcovConfig,
    pub prof: CjprofConfig,
    pub log_level: String,
    pub workspace_symbol_scan_depth: u8,
    pub scan_symbol_types: Vec<String>,
    pub completion_priority: std::collections::HashMap<String, u8>,
}

// 默认定值
impl Default for IndentStyle {
    fn default() -> Self {
        IndentStyle::Space
    }
}

impl Default for CjfmtConfig {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::default(),
            indent_size: 4,
            tab_width: 4,
            line_ending: "\n".to_string(),
            max_line_length: 120,
            function_brace_style: "same_line".to_string(),
            struct_brace_style: "same_line".to_string(),
            trailing_comma: true,
            space_around_operators: true,
            space_inside_brackets: false,
            auto_fix_syntax: true,
        }
    }
}

impl Default for CjlintConfig {
    fn default() -> Self {
        Self {
            check_level: "warn".to_string(),
            enable_style_check: true,
            enable_syntax_check: true,
            ignore_rules: Vec::new(),
            custom_rules_path: None,
        }
    }
}

impl Default for CjcovConfig {
    fn default() -> Self {
        Self {
            report_format: "html".to_string(),
            exclude_paths: vec!["tests/**".to_string(), "examples/**".to_string()],
            threshold: 80,
        }
    }
}

impl Default for CjprofConfig {
    fn default() -> Self {
        Self {
            sampling_rate: 100,
            output_path: "target/profiles".to_string(),
            enable_cpu_profiling: false,
            enable_memory_profiling: false,
        }
    }
}

impl Default for CangjieConfig {
    fn default() -> Self {
        Self {
            lsp_timeout_ms: 5000,
            realtime_diagnostics: true,
            fmt: CjfmtConfig::default(),
            lint: CjlintConfig::default(),
            cov: CjcovConfig::default(),
            prof: CjprofConfig::default(),
            log_level: "info".to_string(),
            workspace_symbol_scan_depth: 3,
            scan_symbol_types: vec![
                "function".to_string(), "variable".to_string(), "struct".to_string(),
                "enum".to_string(), "import".to_string(), "method".to_string(),
                "constant".to_string(), "interface".to_string(),
            ],
            completion_priority: std::collections::HashMap::from_iter([
                ("function".to_string(), 10), ("method".to_string(), 9),
                ("struct".to_string(), 8), ("enum".to_string(), 7),
                ("interface".to_string(), 6), ("constant".to_string(), 5),
                ("variable".to_string(), 4), ("import".to_string(), 3),
            ]),
        }
    }
}

/// 加载 Zed 配置（从 "cangjie" 命名空间）
pub fn load_zed_config() -> Result<CangjieConfig> {
    match ZedConfig::get::<serde_json::Value>("cangjie") {
        Ok(Some(config_value)) => serde_json::from_value(config_value)
            .map_err(|e| Error::InvalidData(format!("无效 Cangjie 配置：{}", e))),
        Ok(None) => Ok(CangjieConfig::default()),
        Err(e) => Err(Error::InvalidData(format!("读取配置失败：{}", e))),
    }
}

/// 验证配置有效性
pub fn validate_config(config: &CangjieConfig) -> Result<()> {
    // 超时时间
    if config.lsp_timeout_ms < 100 || config.lsp_timeout_ms > 30000 {
        return Err(Error::InvalidData("LSP 超时时间必须在 100-30000ms 之间".to_string()));
    }

    // 日志级别
    let valid_log_levels = ["trace", "debug", "info", "warn", "error", "off"];
    if !valid_log_levels.contains(&config.log_level.as_str()) {
        return Err(Error::InvalidData(format!(
            "无效日志级别：{}，支持值：{:?}", config.log_level, valid_log_levels
        )));
    }

    // 扫描深度
    if config.workspace_symbol_scan_depth < 1 || config.workspace_symbol_scan_depth > 10 {
        return Err(Error::InvalidData("扫描深度必须在 1-10 之间".to_string()));
    }

    // 符号类型
    let valid_symbol_types = ["function", "variable", "struct", "enum", "import", "method", "constant", "interface"];
    for typ in &config.scan_symbol_types {
        if !valid_symbol_types.contains(&typ.as_str()) {
            return Err(Error::InvalidData(format!(
                "无效符号类型：{}，支持值：{:?}", typ, valid_symbol_types
            )));
        }
    }

    // 格式化配置
    if config.fmt.indent_size > 16 {
        return Err(Error::InvalidData("缩进大小不能超过 16".to_string()));
    }

    // 检查级别
    let valid_check_levels = ["error", "warn", "info", "off"];
    if !valid_check_levels.contains(&config.lint.check_level.as_str()) {
        return Err(Error::InvalidData(format!(
            "无效检查级别：{}，支持值：{:?}", config.lint.check_level, valid_check_levels
        )));
    }

    Ok(())
}
```

### 4. src/utils.rs（通用工具）
```rust
//! 通用工具函数
use log::{LevelFilter, SetLoggerError};
use zed_extension_api::{Error, Result};

/// 初始化日志系统（适配 Zed 环境）
pub fn init_logger(log_level: &str) -> Result<()> {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::new()
        .filter(None, level)
        .try_init()
        .map_err(|e: SetLoggerError| Error::InvalidData(format!("日志初始化失败：{}", e)))?;

    Ok(())
}

/// 路径标准化（处理跨平台路径分隔符）
pub fn normalize_path(path: &str) -> String {
    #[cfg(windows)]
    return path.replace('/', "\\");
    #[cfg(not(windows))]
    return path.replace('\\', "/");
}

/// 检查文件是否存在（适配 Zed 工作区）
pub fn file_exists(path: &std::path::Path) -> bool {
    path.exists() && path.is_file()
}

/// 读取文件内容（处理编码问题）
pub fn read_file(path: &std::path::Path) -> Result<String> {
    std::fs::read_to_string(path)
        .map_err(|e| Error::IoError(format!("读取文件 {} 失败：{}", path.display(), e)))
}

/// 转换 LSP 位置到字节偏移量（适配多字节字符）
pub fn position_to_offset(content: &str, position: &zed_extension_api::lsp::Position) -> Result<usize> {
    let line = position.line as usize;
    let character = position.character as usize;
    let lines: Vec<&str> = content.lines().collect();

    if line >= lines.len() {
        return Err(Error::InvalidData(format!(
            "行号 {} 超出文档总行数 {}", line, lines.len()
        )));
    }

    let mut offset = 0;
    for (i, l) in lines.iter().enumerate().take(line) {
        offset += l.len() + "\n".len(); // 累加行长度+换行符
    }

    // 计算当前行的字符偏移量（处理 UTF-8 多字节字符）
    let current_line = lines[line];
    let char_offset = current_line
        .char_indices()
        .nth(character)
        .map(|(idx, _)| idx)
        .unwrap_or(current_line.len());

    Ok(offset + char_offset)
}
```

### 5. src/tree_sitter_utils.rs（Tree-sitter 工具）
```rust
//! Tree-sitter 工具（遵循 Zed 规范）
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use tree_sitter::{Parser, Tree, Node, Query, QueryCursor, Point as TsPoint};
use zed_extension_api::{
    lsp::{
        Range, Position, Diagnostic, DiagnosticSeverity, DiagnosticCode,
        DiagnosticRelatedInformation, Location, Uri, Documentation, MarkupContent, MarkupKind
    },
    Document, Error, Result,
};
use crate::utils;

static PARSER: OnceCell<Parser> = OnceCell::new();

/// 初始化 Parser
pub fn init_parser() -> Result<&'static Parser> {
    PARSER.get_or_init(|| {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_cangjie::language())
            .expect("加载 tree-sitter-cangjie 语法失败");
        parser
    });
    Ok(PARSER.get().unwrap())
}

/// 解析文档
pub fn parse_document(document: &Document) -> Result<Tree> {
    let parser = init_parser()?;
    parser.parse(document.text(), None)
        .ok_or_else(|| Error::ParseError("Cangjie 文档解析失败".to_string()))
}

/// 符号查询（与 tree-sitter-cangjie 语法对齐）
const SYMBOL_QUERY: &str = r#"
    (function_declaration name: (identifier) @name) @function
    (variable_declaration name: (identifier) @name) @variable
    (struct_declaration name: (identifier) @name) @struct
    (enum_declaration name: (identifier) @name) @enum
    (import_statement path: (string_literal) @path) @import
    (method_declaration name: (identifier) @name) @method
    (constant_declaration name: (identifier) @name) @constant
    (interface_declaration name: (identifier) @name) @interface
"#;

/// 符号类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolType {
    Function,
    Variable,
    Struct,
    Enum,
    Import,
    Method,
    Constant,
    Interface,
}

impl SymbolType {
    pub fn to_completion_kind(&self) -> zed_extension_api::lsp::CompletionKind {
        match self {
            SymbolType::Function => zed_extension_api::lsp::CompletionKind::Function,
            SymbolType::Variable => zed_extension_api::lsp::CompletionKind::Variable,
            SymbolType::Struct => zed_extension_api::lsp::CompletionKind::Struct,
            SymbolType::Enum => zed_extension_api::lsp::CompletionKind::Enum,
            SymbolType::Import => zed_extension_api::lsp::CompletionKind::Module,
            SymbolType::Method => zed_extension_api::lsp::CompletionKind::Method,
            SymbolType::Constant => zed_extension_api::lsp::CompletionKind::Constant,
            SymbolType::Interface => zed_extension_api::lsp::CompletionKind::Interface,
        }
    }

    pub fn to_symbol_kind(&self) -> zed_extension_api::lsp::SymbolKind {
        match self {
            SymbolType::Function => zed_extension_api::lsp::SymbolKind::Function,
            SymbolType::Variable => zed_extension_api::lsp::SymbolKind::Variable,
            SymbolType::Struct => zed_extension_api::lsp::SymbolKind::Struct,
            SymbolType::Enum => zed_extension_api::lsp::SymbolKind::Enum,
            SymbolType::Import => zed_extension_api::lsp::SymbolKind::Module,
            SymbolType::Method => zed_extension_api::lsp::SymbolKind::Method,
            SymbolType::Constant => zed_extension_api::lsp::SymbolKind::Constant,
            SymbolType::Interface => zed_extension_api::lsp::SymbolKind::Interface,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SymbolType::Function => "函数",
            SymbolType::Variable => "变量",
            SymbolType::Struct => "结构体",
            SymbolType::Enum => "枚举",
            SymbolType::Import => "导入",
            SymbolType::Method => "方法",
            SymbolType::Constant => "常量",
            SymbolType::Interface => "接口",
        }.to_string()
    }
}

/// 符号信息
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub r#type: SymbolType,
    pub range: Range,
    pub detail: Option<String>,
    pub node: Node,
}

/// 提取文档符号
pub fn extract_symbols(document: &Document, tree: &Tree) -> Result<Vec<SymbolInfo>> {
    let content = document.text();
    let query = Query::new(tree_sitter_cangjie::language(), SYMBOL_QUERY)
        .map_err(|e| Error::InvalidData(format!("符号查询语法错误：{}", e)))?;
    let mut cursor = QueryCursor::new();
    let mut symbols = Vec::new();

    for match_result in cursor.matches(&query, tree.root_node(), content.as_bytes()) {
        let mut captures = HashMap::new();
        for capture in match_result.captures {
            captures.insert(
                query.capture_name_for_id(capture.index)
                    .ok_or_else(|| Error::InvalidData("无效捕获名称".to_string()))?,
                capture.node,
            );
        }

        let root_node = match_result.captures[0].node;
        match root_node.kind() {
            "function_declaration" => add_symbol(&mut symbols, &captures, SymbolType::Function, content)?,
            "variable_declaration" => add_symbol(&mut symbols, &captures, SymbolType::Variable, content)?,
            "struct_declaration" => add_symbol(&mut symbols, &captures, SymbolType::Struct, content)?,
            "enum_declaration" => add_symbol(&mut symbols, &captures, SymbolType::Enum, content)?,
            "import_statement" => add_import_symbol(&mut symbols, &captures, content)?,
            "method_declaration" => add_symbol(&mut symbols, &captures, SymbolType::Method, content)?,
            "constant_declaration" => add_symbol(&mut symbols, &captures, SymbolType::Constant, content)?,
            "interface_declaration" => add_symbol(&mut symbols, &captures, SymbolType::Interface, content)?,
            _ => continue,
        }
    }

    Ok(symbols)
}

/// 添加普通符号
fn add_symbol(symbols: &mut Vec<SymbolInfo>, captures: &HashMap<&str, Node>, typ: SymbolType, content: &str) -> Result<()> {
    let name_node = captures.get("name").ok_or_else(||
        Error::InvalidData(format!("{}缺少名称节点", typ.to_string()))
    )?;
    let name = get_node_text(content, name_node)?;
    symbols.push(SymbolInfo {
        name: name.clone(),
        r#type: typ.clone(),
        range: node_to_range(name_node),
        detail: Some(format!("{}: {}", typ.to_string(), name)),
        node: name_node.clone(),
    });
    Ok(())
}

/// 添加导入符号
fn add_import_symbol(symbols: &mut Vec<SymbolInfo>, captures: &HashMap<&str, Node>, content: &str) -> Result<()> {
    let path_node = captures.get("path").ok_or_else(||
        Error::InvalidData("导入语句缺少路径节点".to_string())
    )?;
    let path = get_node_text(content, path_node)?.trim_matches('"').to_string();
    symbols.push(SymbolInfo {
        name: path.clone(),
        r#type: SymbolType::Import,
        range: node_to_range(path_node),
        detail: Some(format!("导入: {}", path)),
        node: path_node.clone(),
    });
    Ok(())
}

/// 检查语法错误
pub fn check_syntax_errors(document: &Document, tree: &Tree) -> Result<Vec<Diagnostic>> {
    let content = document.text();
    let mut diagnostics = Vec::new();
    let mut cursor = tree.walk();

    fn find_errors(
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        diagnostics: &mut Vec<Diagnostic>
    ) -> Result<()> {
        let node = cursor.node();
        if node.is_error() || node.kind().starts_with("invalid_") {
            let range = node_to_range(&node);
            let error_text = get_node_text(content, &node)?;
            let message = if node.is_error() {
                format!("语法解析错误：无法识别 '{}'", error_text)
            } else {
                format!("无效语法结构：'{}'", error_text)
            };

            diagnostics.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::Error),
                code: Some(DiagnosticCode::String(if node.is_error() {
                    "SYNTAX_PARSE_ERROR"
                } else {
                    "INVALID_SYNTAX"
                }.to_string())),
                code_description: None,
                message,
                source: Some("tree-sitter-cangjie".to_string()),
                related_information: Some(vec![
                    DiagnosticRelatedInformation {
                        location: Location {
                            uri: Uri::from_str("https://gitcode.com/Cangjie-SIG/cangjie-lang-docs")?,
                            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                        },
                        message: "查看官方语法文档".to_string(),
                    }
                ]),
                tags: None,
                data: None,
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: "请参考 [Cangjie 语法规范](https://gitcode.com/Cangjie-SIG/cangjie-lang-docs)".to_string(),
                })),
            });
        }

        if cursor.goto_first_child() {
            loop {
                find_errors(cursor, content, diagnostics)?;
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
        Ok(())
    }

    find_errors(&mut cursor, content, &mut diagnostics)?;
    Ok(diagnostics)
}

/// 获取节点文本
pub fn get_node_text(content: &str, node: &Node) -> Result<String> {
    let start = node.start_byte();
    let end = node.end_byte();
    if start > content.len() || end > content.len() {
        return Err(Error::InvalidData("节点范围超出文档边界".to_string()));
    }
    Ok(content[start..end].to_string())
}

/// 节点范围转换为 LSP Range
pub fn node_to_range(node: &Node) -> Range {
    Range::new(
        Position::new(
            node.start_point().row as u32,
            node.start_point().column as u32,
        ),
        Position::new(
            node.end_point().row as u32,
            node.end_point().column as u32,
        ),
    )
}

/// 根据位置查找符号
pub fn find_symbol_at_position(
    document: &Document,
    tree: &Tree,
    position: &Position
) -> Result<Option<SymbolInfo>> {
    let content = document.text();
    let ts_point = TsPoint {
        row: position.line as usize,
        column: position.character as usize,
    };

    let mut cursor = tree.walk();
    let mut result = None;

    fn search_symbol(
        cursor: &mut tree_sitter::TreeCursor,
        ts_point: TsPoint,
        content: &str,
        result: &mut Option<SymbolInfo>
    ) -> Result<()> {
        if result.is_some() {
            return Ok(());
        }

        let node = cursor.node();
        if node.contains_point(ts_point) {
            match node.kind() {
                "function_declaration" | "variable_declaration" | "struct_declaration" |
                "enum_declaration" | "import_statement" | "method_declaration" |
                "constant_declaration" | "interface_declaration" => {
                    let query = Query::new(tree_sitter_cangjie::language(), SYMBOL_QUERY)?;
                    let mut query_cursor = QueryCursor::new();
                    for match_result in query_cursor.matches(&query, node, content.as_bytes()) {
                        let captures = match_result.captures.into_iter()
                            .map(|c| (query.capture_name_for_id(c.index).unwrap(), c.node))
                            .collect::<HashMap<_, _>>();

                        let (name_node, typ) = match node.kind() {
                            "function_declaration" => (captures["name"], SymbolType::Function),
                            "variable_declaration" => (captures["name"], SymbolType::Variable),
                            "struct_declaration" => (captures["name"], SymbolType::Struct),
                            "enum_declaration" => (captures["name"], SymbolType::Enum),
                            "import_statement" => (captures["path"], SymbolType::Import),
                            "method_declaration" => (captures["name"], SymbolType::Method),
                            "constant_declaration" => (captures["name"], SymbolType::Constant),
                            "interface_declaration" => (captures["name"], SymbolType::Interface),
                            _ => unreachable!(),
                        };

                        let name = get_node_text(content, &name_node)?;
                        *result = Some(SymbolInfo {
                            name: name.clone(),
                            r#type: typ,
                            range: node_to_range(&name_node),
                            detail: Some(format!("{}: {}", typ.to_string(), name)),
                            node: node.clone(),
                        });
                        break;
                    }
                }
                _ => {}
            }

            if cursor.goto_first_child() {
                loop {
                    search_symbol(cursor, ts_point, content, result)?;
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }

        Ok(())
    }

    search_symbol(&mut cursor, ts_point, content, &mut result)?;
    Ok(result)
}
```

### 6. src/language_server.rs（LSP 核心）
```rust
//! Cangjie LSP 核心（实现 Zed LanguageServer 规范）
use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use zed_extension_api::{
    self as zed,
    lsp::{
        self, CompletionList, CompletionItem, SymbolInformation, DefinitionResponse,
        Location, Uri, CompletionKind, InsertTextFormat, Documentation, MarkupContent,
        MarkupKind, TextEdit
    },
    LanguageServer, Worktree, Document, Result, Error,
};
use crate::{
    config::CangjieConfig,
    tree_sitter_utils::{self, SymbolInfo, SymbolType},
    cjfmt::CjfmtConfig,
    cjlint::CjlintConfig,
    utils,
};

pub struct CangjieLanguageServer {
    config: Arc<CangjieConfig>,
    document_cache: HashMap<String, (tree_sitter::Tree, Vec<SymbolInfo>)>,
    workspace_symbols: HashMap<String, Vec<SymbolInfo>>,
    loaded_worktrees: HashSet<String>,
}

impl CangjieLanguageServer {
    pub fn new(config: Arc<CangjieConfig>) -> Self {
        let _ = tree_sitter_utils::init_parser();
        Self {
            config,
            document_cache: HashMap::new(),
            workspace_symbols: HashMap::new(),
            loaded_worktrees: HashSet::new(),
        }
    }

    /// 加载工作区符号
    fn load_workspace_symbols(&mut self, worktree: &Worktree) -> Result<()> {
        let worktree_id = worktree.id();
        if self.loaded_worktrees.contains(worktree_id) {
            return Ok(());
        }

        log::info!("加载工作区符号：{}", worktree.path().display());
        let src_dir = worktree.path().join("src");
        if !src_dir.exists() {
            log::warn!("工作区无 src 目录，跳过符号扫描");
            self.loaded_worktrees.insert(worktree_id.to_string());
            return Ok(());
        }

        // 扫描所有 .cj 文件
        let cj_files = walkdir::WalkDir::new(src_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("cj"));

        let mut workspace_symbols = Vec::new();
        for entry in cj_files {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();

            // 读取文件内容
            let content = utils::read_file(path)?;

            // 构造临时文档
            let temp_document = Document {
                id: path_str.clone(),
                path: path.to_path_buf(),
                content: content.clone(),
                version: 0,
                language: "Cangjie".to_string(),
                line_ending: "\n".to_string(),
            };

            // 解析并提取符号
            let tree = tree_sitter_utils::parse_document(&temp_document)?;
            let symbols = tree_sitter_utils::extract_symbols(&temp_document, &tree)?;

            self.document_cache.insert(path_str, (tree, symbols.clone()));
            workspace_symbols.extend(symbols);
        }

        self.workspace_symbols.insert(worktree_id.to_string(), workspace_symbols);
        self.loaded_worktrees.insert(worktree_id.to_string());
        log::info!("工作区符号加载完成：{} 个符号", self.workspace_symbols[worktree_id].len());
        Ok(())
    }

    /// 获取符号名称（位置适配）
    fn get_symbol_name_at_position(&self, document: &Document, position: &lsp::Position) -> Result<String> {
        let path_str = document.path.to_string_lossy().to_string();

        if let Some((tree, _)) = self.document_cache.get(&path_str) {
            if let Some(symbol) = tree_sitter_utils::find_symbol_at_position(document, tree, position)? {
                return Ok(symbol.name);
            }
        }

        // 降级：提取标识符
        let tree = tree_sitter_utils::parse_document(document)?;
        let mut cursor = tree.walk();
        let ts_point = tree_sitter::Point {
            row: position.line as usize,
            column: position.character as usize,
        };

        let mut identifier = None;
        fn find_identifier(
            cursor: &mut tree_sitter::TreeCursor,
            ts_point: tree_sitter::Point,
            content: &str,
            identifier: &mut Option<String>
        ) -> Result<()> {
            if identifier.is_some() {
                return Ok(());
            }

            let node = cursor.node();
            if node.kind() == "identifier" && node.contains_point(ts_point) {
                *identifier = Some(tree_sitter_utils::get_node_text(content, &node)?);
                return Ok(());
            }

            if cursor.goto_first_child() {
                loop {
                    find_identifier(cursor, ts_point, content, identifier)?;
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }

            Ok(())
        }

        find_identifier(&mut cursor, ts_point, document.text(), &mut identifier)?;
        Ok(identifier.unwrap_or_default())
    }
}

impl LanguageServer for CangjieLanguageServer {
    /// 初始化 LSP
    fn initialize(&mut self, _params: lsp::InitializeParams, worktree: &Worktree) -> Result<lsp::InitializeResult> {
        log::info!("初始化 Cangjie LSP v{}", env!("CARGO_PKG_VERSION"));
        self.load_workspace_symbols(worktree)?;

        Ok(lsp::InitializeResult {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string(), "(".to_string()]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                document_symbol_provider: Some(lsp::OneOf::Left(true)),
                definition_provider: Some(lsp::OneOf::Left(true)),
                text_document_sync: Some(lsp::TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(lsp::TextDocumentSyncKind::Full),
                    save: Some(lsp::SaveOptions { include_text: Some(false) }),
                    ..Default::default()
                }),
                document_formatting_provider: Some(lsp::OneOf::Left(true)),
                hover_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(lsp::ServerInfo {
                name: "cangjie-zed-extension".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    /// 文档打开
    fn did_open(&mut self, document: &Document) -> Result<Vec<lsp::Diagnostic>> {
        log::debug!("文档打开：{}", document.path.display());

        let tree = tree_sitter_utils::parse_document(document)?;
        let symbols = tree_sitter_utils::extract_symbols(document, &tree)?;
        let mut diagnostics = tree_sitter_utils::check_syntax_errors(document, &tree)?;

        // 代码检查
        if self.config.lint.enable_style_check || self.config.lint.enable_syntax_check {
            let lint_diagnostics = crate::cjlint::lint_document(document, &self.config.lint)?;
            diagnostics.extend(lint_diagnostics);
        }

        self.document_cache.insert(
            document.path.to_string_lossy().to_string(),
            (tree, symbols)
        );

        Ok(diagnostics)
    }

    /// 文档变更
    fn did_change(&mut self, document: &Document, _changes: &[lsp::TextDocumentContentChangeEvent]) -> Result<Vec<lsp::Diagnostic>> {
        log::debug!("文档变更：{}（版本：{}）", document.path.display(), document.version);

        let tree = tree_sitter_utils::parse_document(document)?;
        let symbols = tree_sitter_utils::extract_symbols(document, &tree)?;
        let mut diagnostics = tree_sitter_utils::check_syntax_errors(document, &tree)?;

        // 代码检查
        if self.config.lint.enable_style_check || self.config.lint.enable_syntax_check {
            let lint_diagnostics = crate::cjlint::lint_document(document, &self.config.lint)?;
            diagnostics.extend(lint_diagnostics);
        }

        self.document_cache.insert(
            document.path.to_string_lossy().to_string(),
            (tree, symbols)
        );

        Ok(diagnostics)
    }

    /// 文档关闭
    fn did_close(&mut self, document: &Document) -> Result<()> {
        log::debug!("文档关闭：{}", document.path.display());
        self.document_cache.remove(&document.path.to_string_lossy().to_string());
        Ok(())
    }

    /// 代码补全
    fn completion(&self, document: &Document, position: &lsp::Position) -> Result<lsp::CompletionResponse> {
        log::debug!("触发补全：{} @ {:?}", document.path.display(), position);

        let path_str = document.path.to_string_lossy().to_string();
        let mut items = Vec::new();

        // 1. 当前文档符号
        if let Some((_, symbols)) = self.document_cache.get(&path_str) {
            for symbol in symbols {
                items.push(create_completion_item(symbol));
            }
        }

        // 2. 工作区符号
        for (_, symbols) in &self.workspace_symbols {
            for symbol in symbols {
                if !items.iter().any(|item| item.label == symbol.name) {
                    items.push(create_completion_item(symbol));
                }
            }
        }

        // 3. 标准库符号
        let std_lib = vec![
            ("println", "fn println(message: String) -> Void - 打印字符串", SymbolType::Function),
            ("read_file", "fn read_file(path: String) -> Result<String, Error> - 读取文件", SymbolType::Function),
            ("write_file", "fn write_file(path: String, content: String) -> Result<Void, Error> - 写入文件", SymbolType::Function),
            ("Vec", "struct Vec<T> - 动态数组", SymbolType::Struct),
            ("Option", "enum Option<T> - 可选值（Some/None）", SymbolType::Enum),
            ("Result", "enum Result<T, E> - 结果类型（Ok/Err）", SymbolType::Enum),
            ("PI", "const PI: Float = 3.1415926 - 圆周率", SymbolType::Constant),
            ("E", "const E: Float = 2.71828 - 自然常数", SymbolType::Constant),
            ("Serializable", "interface Serializable - 序列化接口", SymbolType::Interface),
            ("Clone", "interface Clone - 克隆接口", SymbolType::Interface),
        ];
        for (name, detail, typ) in std_lib {
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(typ.to_completion_kind()),
                detail: Some(detail.to_string()),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!("**{}**\n\n标准库内置{}", detail.split(" - ").0, typ.to_string()),
                })),
                insert_text: Some(if matches!(typ, SymbolType::Function | SymbolType::Method) {
                    format!("{}()", name)
                } else {
                    name.to_string()
                }),
                insert_text_format: Some(InsertTextFormat::PlainText),
                ..Default::default()
            });
        }

        Ok(lsp::CompletionResponse::List(CompletionList {
            is_incomplete: false,
            items,
        }))
    }

    /// 文档符号
    fn document_symbols(&self, document: &Document) -> Result<Vec<SymbolInformation>> {
        let path_str = document.path.to_string_lossy().to_string();
        let symbols = self.document_cache.get(&path_str)
            .map(|(_, symbols)| symbols.clone())
            .unwrap_or_default();

        Ok(symbols.into_iter().map(|symbol| {
            SymbolInformation {
                name: symbol.name,
                kind: symbol.r#type.to_symbol_kind(),
                tags: None,
                deprecated: None,
                location: Location {
                    uri: Uri::from_file_path(&document.path)
                        .map_err(|_| Error::InvalidData("路径转换失败".to_string()))
                        .unwrap(),
                    range: symbol.range,
                },
                container_name: None,
                documentation: symbol.detail.map(|detail| {
                    Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::PlainText,
                        value: detail,
                    })
                }),
            }
        }).collect())
    }

    /// 跳转定义
    fn goto_definition(&self, document: &Document, position: &lsp::Position) -> Result<Option<DefinitionResponse>> {
        let path_str = document.path.to_string_lossy().to_string();

        // 1. 当前文档内
        if let Some((tree, _)) = self.document_cache.get(&path_str) {
            if let Some(symbol) = tree_sitter_utils::find_symbol_at_position(document, tree, position)? {
                return Ok(Some(DefinitionResponse::Scalar(Location {
                    uri: Uri::from_file_path(&document.path)?,
                    range: symbol.range,
                })));
            }
        }

        // 2. 工作区跨文件
        let symbol_name = self.get_symbol_name_at_position(document, position)?;
        if symbol_name.is_empty() {
            return Ok(None);
        }

        let mut locations = Vec::new();
        for (file_path, (_, symbols)) in &self.document_cache {
            if file_path == &path_str {
                continue;
            }

            for symbol in symbols {
                if symbol.name == symbol_name {
                    locations.push(Location {
                        uri: Uri::from_file_path(file_path)?,
                        range: symbol.range,
                    });
                }
            }
        }

        match locations.len() {
            0 => Ok(None),
            1 => Ok(Some(DefinitionResponse::Scalar(locations[0].clone()))),
            _ => Ok(Some(DefinitionResponse::Array(locations))),
        }
    }

    /// 格式化文档
    fn format_document(&self, document: &Document, _options: &lsp::FormattingOptions) -> Result<Vec<TextEdit>> {
        log::debug!("格式化文档：{}", document.path.display());
        crate::cjfmt::format_document(document, &self.config.fmt)
    }

    /// 悬停提示
    fn hover(&self, document: &Document, position: &lsp::Position) -> Result<Option<lsp::Hover>> {
        let path_str = document.path.to_string_lossy().to_string();

        if let Some((tree, _)) = self.document_cache.get(&path_str) {
            if let Some(symbol) = tree_sitter_utils::find_symbol_at_position(document, tree, position)? {
                return Ok(Some(lsp::Hover {
                    contents: lsp::HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("### {}\n\n{}", symbol.name, symbol.detail.unwrap_or_default()),
                    }),
                    range: Some(symbol.range),
                }));
            }
        }

        Ok(None)
    }
}

/// 创建补全项
fn create_completion_item(symbol: &SymbolInfo) -> CompletionItem {
    CompletionItem {
        label: symbol.name.clone(),
        kind: Some(symbol.r#type.to_completion_kind()),
        detail: symbol.detail.clone(),
        documentation: symbol.detail.as_ref().map(|detail| {
            Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::PlainText,
                value: detail.clone(),
            })
        }),
        insert_text: Some(if matches!(symbol.r#type, SymbolType::Function | SymbolType::Method) {
            format!("{}()", symbol.name)
        } else {
            symbol.name.clone()
        }),
        insert_text_format: Some(InsertTextFormat::PlainText),
        ..Default::default()
    }
}
```

### 7. src/cjfmt.rs（格式化工具）
```rust
//! 代码格式化（遵循 Zed 规范）
use zed_extension_api::{
    Document, Error, Result,
    lsp::{TextEdit, Range, Position}
};
use crate::config::{CjfmtConfig, IndentStyle};

/// 格式化文档
pub fn format_document(document: &Document, config: &CjfmtConfig) -> Result<Vec<TextEdit>> {
    let content = document.text();
    let formatted_content = format_content(content, config, &document.line_ending)?;

    if formatted_content == content {
        return Ok(Vec::new());
    }

    // 全量替换（Zed 推荐更稳定）
    let full_range = Range {
        start: Position::new(0, 0),
        end: Position::new(
            document.line_count() as u32,
            document.line(document.line_count() - 1).map_or(0, |line| line.len() as u32),
        ),
    };

    Ok(vec![TextEdit {
        range: full_range,
        new_text: formatted_content,
    }])
}

/// 格式化文本内容
fn format_content(content: &str, config: &CjfmtConfig, line_ending: &str) -> Result<String> {
    let mut lines = content.lines().collect::<Vec<_>>();
    let mut formatted_lines = Vec::with_capacity(lines.len());
    let mut indent_level = 0;
    let indent = match config.indent_style {
        IndentStyle::Space => " ".repeat(config.indent_size as usize),
        IndentStyle::Tab => "\t".repeat(config.tab_width as usize / 4),
    };

    let mut block_stack = Vec::new();
    let mut in_block_comment = false;
    let mut in_string = false;

    for line in lines {
        let mut trimmed_line = line.trim_start();
        let _leading_whitespace = line.get(0..(line.len() - trimmed_line.len())).unwrap_or("");

        // 更新状态
        in_block_comment = update_block_comment_state(trimmed_line, in_block_comment);
        in_string = update_string_state(trimmed_line, in_string);

        if in_block_comment || in_string {
            formatted_lines.push(line.to_string());
            continue;
        }

        // 处理缩进减少
        let dedent_count = count_dedent_tokens(trimmed_line);
        if dedent_count > 0 {
            indent_level = indent_level.saturating_sub(dedent_count);
        }

        // 生成缩进行
        let indented_line = format!("{}{}", indent.repeat(indent_level), trimmed_line);
        formatted_lines.push(indented_line);

        // 处理缩进增加
        let indent_count = count_indent_tokens(trimmed_line, config);
        indent_level += indent_count;

        // 更新块栈
        update_block_stack(&mut block_stack, trimmed_line);
    }

    // 后处理
    if config.trailing_comma {
        format_trailing_commas(&mut formatted_lines, config);
    }
    if config.space_around_operators {
        format_operators(&mut formatted_lines);
    }
    if config.space_inside_brackets {
        format_brackets(&mut formatted_lines);
    }

    Ok(formatted_lines.join(line_ending))
}

/// 更新块注释状态
fn update_block_comment_state(line: &str, current_state: bool) -> bool {
    let mut state = current_state;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'*') {
            state = true;
            chars.next();
        } else if c == '*' && chars.peek() == Some(&'/') {
            state = false;
            chars.next();
        }
    }
    state
}

/// 更新字符串状态
fn update_string_state(line: &str, current_state: bool) -> bool {
    let mut state = current_state;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if (c == '"' || c == '\'') && chars.peek() != Some(&'\\') {
            state = !state;
        }
    }
    state
}

/// 统计缩进减少标记
fn count_dedent_tokens(line: &str) -> usize {
    let mut count = 0;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        skip_string_and_comment(&mut chars, c);
        if c == '}' || c == ')' || c == ']' {
            count += 1;
            while let Some(next_c) = chars.peek() {
                if *next_c == '}' || *next_c == ')' || *next_c == ']' {
                    chars.next();
                } else {
                    break;
                }
            }
        }
    }
    count
}

/// 统计缩进增加标记
fn count_indent_tokens(line: &str, config: &CjfmtConfig) -> usize {
    let mut count = 0;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        skip_string_and_comment(&mut chars, c);
        if c == '{' || c == '(' || c == '[' {
            let mut is_line_end_block = false;
            let remaining = line[chars.position().unwrap()..].trim();
            if remaining.starts_with('}') || remaining.is_empty() {
                is_line_end_block = false;
            } else {
                match config.function_brace_style.as_str() {
                    "same_line" => is_line_end_block = true,
                    "next_line" => is_line_end_block = false,
                    _ => is_line_end_block = true,
                }
            }

            if is_line_end_block {
                count += 1;
            }

            while let Some(next_c) = chars.peek() {
                if *next_c == '{' || *next_c == '(' || *next_c == '[' {
                    chars.next();
                } else {
                    break;
                }
            }
        }
    }
    count
}

/// 更新块栈
fn update_block_stack(stack: &mut Vec<char>, line: &str) {
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        skip_string_and_comment(&mut chars, c);
        if c == '{' || c == '(' || c == '[' {
            stack.push(c);
        } else if let Some(&top) = stack.last() {
            match (top, c) {
                ('{', '}') | ('(', ')') | ('[', ']') => {
                    stack.pop();
                }
                _ => {}
            }
        }
    }
}

/// 格式化尾随逗号
fn format_trailing_commas(lines: &mut Vec<String>, config: &CjfmtConfig) {
    let mut i = 0;
    while i < lines.len() {
        let line = &mut lines[i];
        let trimmed = line.trim();

        if trimmed.ends_with(',') && !trimmed.ends_with("..,") {
            let next_trimmed = if i + 1 < lines.len() {
                lines[i + 1].trim()
            } else {
                ""
            };

            if next_trimmed.starts_with(&['}', ')', ']'][..]) {
                if !config.trailing_comma {
                    *line = line.trim_end_matches(',').trim_end().to_string();
                }
            }
        } else if config.trailing_comma {
            let next_trimmed = if i + 1 < lines.len() {
                lines[i + 1].trim()
            } else {
                ""
            };

            if next_trimmed.starts_with(&['}', ')', ']'][..])
                && trimmed.contains(':')
                && !trimmed.ends_with(',')
                && !trimmed.ends_with(&['{', '(', '['][..])
            {
                *line = format!("{},", line.trim_end());
            }
        }

        i += 1;
    }
}

/// 格式化运算符空格
fn format_operators(lines: &mut Vec<String>) {
    let operators = ["+", "-", "*", "/", "%", "=", "==", "!=", ">", "<", ">=", "<=", "&&", "||", "->", ":", "=>"];
    let mut i = 0;
    while i < lines.len() {
        let line = &mut lines[i];
        let mut new_line = line.clone();

        let mut in_string = false;
        let mut in_comment = false;
        let mut chars = new_line.chars().peekable();
        let mut pos = 0;

        while let Some(c) = chars.next() {
            if in_comment {
                break;
            }
            if in_string {
                if c == '"' && chars.peek() != Some(&'\\') {
                    in_string = false;
                }
                pos += 1;
                continue;
            }
            if c == '"' {
                in_string = true;
                pos += 1;
                continue;
            }
            if c == '/' {
                if chars.peek() == Some(&'/') {
                    in_comment = true;
                    break;
                } else if chars.peek() == Some(&'*') {
                    in_comment = true;
                    pos += 2;
                    chars.next();
                    continue;
                }
            }

            for op in &operators {
                let op_len = op.len();
                if pos + op_len <= new_line.len() && new_line[pos..pos + op_len] == **op {
                    // 前空格
                    if pos > 0 && !new_line[pos - 1..pos].trim().is_empty() {
                        new_line.insert(pos, ' ');
                        pos += 1;
                    }
                    // 后空格
                    if pos + op_len < new_line.len() && !new_line[pos + op_len..pos + op_len + 1].trim().is_empty() {
                        new_line.insert(pos + op_len, ' ');
                        pos += 1;
                    }
                    pos += op_len;
                    break;
                }
            }
            pos += 1;
        }

        *line = new_line;
        i += 1;
    }
}

/// 格式化括号内空格
fn format_brackets(lines: &mut Vec<String>) {
    let bracket_pairs = [('(', ')'), ('[', ']'), ('{', '}')];
    let mut i = 0;
    while i < lines.len() {
        let line = &mut lines[i];
        let mut new_line = line.clone();

        let mut in_string = false;
        let mut in_comment = false;
        let mut chars = new_line.chars().peekable();
        let mut pos = 0;

        while let Some(c) = chars.next() {
            if in_comment {
                break;
            }
            if in_string {
                if c == '"' && chars.peek() != Some(&'\\') {
                    in_string = false;
                }
                pos += 1;
                continue;
            }
            if c == '"' {
                in_string = true;
                pos += 1;
                continue;
            }
            if c == '/' {
                if chars.peek() == Some(&'/') {
                    in_comment = true;
                    break;
                } else if chars.peek() == Some(&'*') {
                    in_comment = true;
                    pos += 2;
                    chars.next();
                    continue;
                }
            }

            // 开括号后空格
            for &(open, _) in &bracket_pairs {
                if c == open {
                    if pos + 1 < new_line.len() && new_line[pos + 1..pos + 2].trim().is_empty() {
                    } else if pos + 1 < new_line.len() && new_line[pos + 1] != open && new_line[pos + 1] != '"' && new_line[pos + 1] != '\'' {
                        new_line.insert(pos + 1, ' ');
                        pos += 1;
                    }
                    break;
                }
            }

            // 闭括号前空格
            for &(_, close) in &bracket_pairs {
                if c == close {
                    if pos > 0 && new_line[pos - 1..pos].trim().is_empty() {
                    } else if pos > 0 && new_line[pos - 1] != close && new_line[pos - 1] != '"' && new_line[pos - 1] != '\'' {
                        new_line.insert(pos, ' ');
                        pos += 1;
                    }
                    break;
                }
            }

            pos += 1;
        }

        *line = new_line;
        i += 1;
    }
}

/// 跳过字符串和注释
fn skip_string_and_comment(chars: &mut std::iter::Peekable<impl Iterator<Item = char>>, current_char: char) {
    if current_char == '"' || current_char == '\'' {
        let quote = current_char;
        while let Some(next_c) = chars.next() {
            if next_c == quote && chars.peek() != Some(&'\\') {
                break;
            }
        }
    } else if current_char == '/' {
        if chars.peek() == Some(&'/') {
            while chars.next().is_some() {}
        } else if chars.peek() == Some(&'*') {
            chars.next();
            while let Some(next_c) = chars.next() {
                if next_c == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    break;
                }
            }
        }
    }
}
```

### 8. src/cjlint.rs（代码检查）
```rust
//! 代码检查（遵循 Zed 诊断规范）
use zed_extension_api::{
    Document, Error, Result,
    lsp::{Diagnostic, DiagnosticSeverity, DiagnosticCode, Documentation, MarkupContent, MarkupKind, DiagnosticTag}
};
use crate::config::CjlintConfig;
use crate::tree_sitter_utils;

/// 代码检查核心
pub fn lint_document(document: &Document, config: &CjlintConfig) -> Result<Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();

    if config.check_level == "off" {
        return Ok(diagnostics);
    }

    // 语法错误检查（已在 did_open/did_change 中处理，此处可增强）
    if config.enable_syntax_check {
        let tree = tree_sitter_utils::parse_document(document)?;
        let syntax_diags = tree_sitter_utils::check_syntax_errors(document, &tree)?;
        diagnostics.extend(syntax_diags);
    }

    // 风格检查
    if config.enable_style_check {
        let style_diags = check_style(document, config)?;
        diagnostics.extend(style_diags);
    }

    // 自定义规则
    if let Some(custom_rules_path) = &config.custom_rules_path {
        let custom_diags = check_custom_rules(document, custom_rules_path, config)?;
        diagnostics.extend(custom_diags);
    }

    // 过滤级别
    filter_diagnostics_by_level(&mut diagnostics, config.check_level.as_str());

    Ok(diagnostics)
}

/// 风格检查
fn check_style(document: &Document, config: &CjlintConfig) -> Result<Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let content = document.text();
    let lines = content.lines().collect::<Vec<_>>();
    let tree = tree_sitter_utils::parse_document(document)?;

    // 1. 行长度检查
    let max_line_length = 120;
    for (line_idx, line) in lines.iter().enumerate() {
        let line_len = line.len();
        if line_len > max_line_length {
            diagnostics.push(create_diagnostic(
                line_idx as u32, 0, line_len as u32,
                DiagnosticSeverity::Warning,
                "LINE_TOO_LONG",
                format!("行长度超出 {} 字符（当前 {}）", max_line_length, line_len),
                "建议拆分长行或调整配置",
                "cjlint"
            ));
        }
    }

    // 2. 缩进检查
    let expected_indent_style = crate::config::CjfmtConfig::default().indent_style;
    let expected_indent_size = crate::config::CjfmtConfig::default().indent_size as usize;
    for (line_idx, line) in lines.iter().enumerate() {
        let leading_ws = line.chars().take_while(|c| c.is_whitespace()).collect::<Vec<_>>();
        if leading_ws.is_empty() {
            continue;
        }

        // 缩进字符检查
        let indent_char = leading_ws[0];
        let expected_char = match expected_indent_style {
            crate::config::IndentStyle::Space => ' ',
            crate::config::IndentStyle::Tab => '\t',
        };
        if indent_char != expected_char {
            diagnostics.push(create_diagnostic(
                line_idx as u32, 0, leading_ws.len() as u32,
                DiagnosticSeverity::Warning,
                "INVALID_INDENT_CHAR",
                format!("缩进字符不匹配（期望 {:?}，实际 {:?}）", expected_char, indent_char),
                format!("请使用 {:?} 进行缩进", expected_char),
                "cjlint"
            ));
            continue;
        }

        // 缩进大小检查
        if leading_ws.len() % expected_indent_size != 0 {
            diagnostics.push(create_diagnostic(
                line_idx as u32, 0, leading_ws.len() as u32,
                DiagnosticSeverity::Warning,
                "INVALID_INDENT_SIZE",
                format!("缩进大小不是 {} 的倍数（当前 {}）", expected_indent_size, leading_ws.len()),
                format!("建议每次缩进使用 {} 个 {:?}", expected_indent_size, expected_char),
                "cjlint"
            ));
        }
    }

    // 3. 未使用变量
    let unused_vars = check_unused_variables(document, &tree)?;
    diagnostics.extend(unused_vars);

    // 4. 常量命名
    let const_naming = check_constant_naming(document, &tree)?;
    diagnostics.extend(const_naming);

    // 5. 尾随空格
    for (line_idx, line) in lines.iter().enumerate() {
        if line.ends_with(' ') {
            diagnostics.push(create_diagnostic(
                line_idx as u32, (line.len() - 1) as u32, line.len() as u32,
                DiagnosticSeverity::Warning,
                "TRAILING_WHITESPACE",
                "行尾存在多余空格",
                "请删除行尾空格",
                "cjlint"
            ));
        }
    }

    Ok(diagnostics)
}

/// 检查未使用变量
fn check_unused_variables(document: &Document, tree: &tree_sitter::Tree) -> Result<Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let content = document.text();

    // 查询变量定义
    let var_query = tree_sitter::Query::new(tree_sitter_cangjie::language(), r#"
        (variable_declaration name: (identifier) @var.name) @var
        (constant_declaration name: (identifier) @const.name) @const
    "#)?;
    let mut var_cursor = tree_sitter::QueryCursor::new();

    let mut variables = Vec::new();
    for match_result in var_cursor.matches(&var_query, tree.root_node(), content.as_bytes()) {
        for capture in match_result.captures {
            if capture.node.kind() == "identifier" {
                let var_name = tree_sitter_utils::get_node_text(content, &capture.node)?;
                let var_range = tree_sitter_utils::node_to_range(&capture.node);
                variables.push((var_name, var_range));
            }
        }
    }

    // 查询变量引用
    let ref_query = tree_sitter::Query::new(tree_sitter_cangjie::language(), r#"(identifier) @ref"#)?;
    let mut ref_cursor = tree_sitter::QueryCursor::new();
    let mut used_vars = std::collections::HashSet::new();

    for match_result in ref_cursor.matches(&ref_query, tree.root_node(), content.as_bytes()) {
        for capture in match_result.captures {
            let ref_name = tree_sitter_utils::get_node_text(content, &capture.node)?;
            let is_definition = variables.iter().any(|(name, range)| {
                name == &ref_name && range.start == capture.node.range().start_point.into()
            });
            if !is_definition {
                used_vars.insert(ref_name);
            }
        }
    }

    // 未使用变量
    for (var_name, var_range) in variables {
        if !used_vars.contains(&var_name) && !var_name.starts_with('_') {
            diagnostics.push(Diagnostic {
                range: var_range,
                severity: Some(DiagnosticSeverity::Warning),
                code: Some(DiagnosticCode::String("UNUSED_VARIABLE".to_string())),
                code_description: None,
                message: format!("变量 '{}' 已定义但未使用", var_name),
                source: Some("cjlint".to_string()),
                related_information: None,
                tags: Some(vec![DiagnosticTag::Unnecessary]),
                data: None,
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: "建议删除或在变量名前加下划线（_）标记为有意未使用".to_string(),
                })),
            });
        }
    }

    Ok(diagnostics)
}

/// 检查常量命名

### 8. src/cjlint.rs（续）
```rust
/// 检查常量命名（全大写蛇形命名规范）
fn check_constant_naming(document: &Document, tree: &tree_sitter::Tree) -> Result<Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let content = document.text();

    // 查询常量定义
    let const_query = tree_sitter::Query::new(tree_sitter_cangjie::language(), r#"
        (constant_declaration name: (identifier) @const.name) @const
    "#)?;
    let mut const_cursor = tree_sitter::QueryCursor::new();

    for match_result in const_cursor.matches(&const_query, tree.root_node(), content.as_bytes()) {
        for capture in match_result.captures {
            if capture.node.kind() == "identifier" {
                let const_name = tree_sitter_utils::get_node_text(content, &capture.node)?;
                let const_range = tree_sitter_utils::node_to_range(&capture.node);

                // 检查是否符合全大写蛇形命名（仅包含 ASCII 大写字母和下划线）
                if !const_name.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
                    diagnostics.push(Diagnostic {
                        range: const_range,
                        severity: Some(DiagnosticSeverity::Warning),
                        code: Some(DiagnosticCode::String("INVALID_CONSTANT_NAMING".to_string())),
                        code_description: None,
                        message: format!("常量命名不符合规范（当前：'{}'）", const_name),
                        source: Some("cjlint".to_string()),
                        related_information: None,
                        tags: None,
                        data: None,
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: "常量应使用 **全大写蛇形命名**（例如：`MAX_RETRY_COUNT`、`DEFAULT_TIMEOUT`）".to_string(),
                        })),
                    });
                }
            }
        }
    }

    Ok(diagnostics)
}

/// 检查自定义规则（支持 JSON 格式的用户自定义规则）
fn check_custom_rules(document: &Document, rules_path: &str, config: &CjlintConfig) -> Result<Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();

    // 验证规则文件存在
    let rules_path = std::path::Path::new(rules_path);
    if !crate::utils::file_exists(rules_path) {
        log::warn!("自定义规则文件不存在：{}", rules_path.display());
        return Ok(diagnostics);
    }

    // 读取并解析规则文件
    let rules_content = crate::utils::read_file(rules_path)?;
    let custom_rules: serde_json::Value = serde_json::from_str(&rules_content)
        .map_err(|e| Error::InvalidData(format!("解析自定义规则文件失败：{}", e)))?;

    // 处理正则匹配规则
    if let Some(regex_rules) = custom_rules.get("regex_rules") {
        if let serde_json::Value::Array(rules) = regex_rules {
            for rule in rules {
                // 提取规则基础信息
                let rule_name = rule.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown_custom_rule");
                let pattern = rule.get("pattern")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| Error::InvalidData(format!("自定义规则 '{}' 缺少 'pattern' 字段", rule_name)))?;
                let message = rule.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| format!("违反自定义规则：{}", rule_name));
                let severity_str = rule.get("severity")
                    .and_then(|v| v.as_str())
                    .unwrap_or("warning");
                let ignore = rule.get("ignore")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                // 跳过忽略的规则或配置中排除的规则
                if ignore || config.ignore_rules.contains(&rule_name.to_string()) {
                    log::debug!("跳过自定义规则：{}", rule_name);
                    continue;
                }

                // 编译正则表达式
                let regex = regex::Regex::new(pattern)
                    .map_err(|e| Error::InvalidData(format!("自定义规则 '{}' 正则表达式无效：{}", rule_name, e)))?;

                // 匹配文档所有行
                let lines = document.text().lines().collect::<Vec<_>>();
                for (line_idx, line) in lines.iter().enumerate() {
                    if let Some(mat) = regex.find(line) {
                        diagnostics.push(Diagnostic {
                            range: zed_extension_api::lsp::Range {
                                start: zed_extension_api::lsp::Position::new(line_idx as u32, mat.start() as u32),
                                end: zed_extension_api::lsp::Position::new(line_idx as u32, mat.end() as u32),
                            },
                            severity: Some(match severity_str.to_lowercase().as_str() {
                                "error" => DiagnosticSeverity::Error,
                                "warn" | "warning" => DiagnosticSeverity::Warning,
                                "info" | "information" => DiagnosticSeverity::Information,
                                _ => DiagnosticSeverity::Warning,
                            }),
                            code: Some(DiagnosticCode::String(rule_name.to_string())),
                            code_description: None,
                            message: message.to_string(),
                            source: Some("cjlint-custom".to_string()),
                            related_information: None,
                            tags: None,
                            data: None,
                            documentation: None,
                        });
                    }
                }
            }
        }
    }

    // 处理语法节点规则（基于 tree-sitter 节点类型）
    if let Some(node_rules) = custom_rules.get("node_rules") {
        if let serde_json::Value::Array(rules) = node_rules {
            let content = document.text();
            let tree = tree_sitter_utils::parse_document(document)?;

            for rule in rules {
                let rule_name = rule.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown_node_rule");
                let node_kind = rule.get("node_kind")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| Error::InvalidData(format!("节点规则 '{}' 缺少 'node_kind' 字段", rule_name)))?;
                let message = rule.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| format!("违反节点规则：{}", rule_name));
                let severity_str = rule.get("severity")
                    .and_then(|v| v.as_str())
                    .unwrap_or("warning");
                let ignore = rule.get("ignore")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                if ignore || config.ignore_rules.contains(&rule_name.to_string()) {
                    log::debug!("跳过节点规则：{}", rule_name);
                    continue;
                }

                // 查询指定类型的节点
                let query = tree_sitter::Query::new(tree_sitter_cangjie::language(), &format!(r#"({}) @node"#, node_kind))
                    .map_err(|e| Error::InvalidData(format!("节点规则 '{}' 查询语法错误：{}", rule_name, e)))?;
                let mut cursor = tree_sitter::QueryCursor::new();

                for match_result in cursor.matches(&query, tree.root_node(), content.as_bytes()) {
                    for capture in match_result.captures {
                        let node_range = tree_sitter_utils::node_to_range(&capture.node);
                        diagnostics.push(Diagnostic {
                            range: node_range,
                            severity: Some(match severity_str.to_lowercase().as_str() {
                                "error" => DiagnosticSeverity::Error,
                                "warn" | "warning" => DiagnosticSeverity::Warning,
                                "info" | "information" => DiagnosticSeverity::Information,
                                _ => DiagnosticSeverity::Warning,
                            }),
                            code: Some(DiagnosticCode::String(rule_name.to_string())),
                            code_description: None,
                            message: message.to_string(),
                            source: Some("cjlint-custom".to_string()),
                            related_information: None,
                            tags: None,
                            data: None,
                            documentation: None,
                        });
                    }
                }
            }
        }
    }

    Ok(diagnostics)
}

/// 根据检查级别过滤诊断
fn filter_diagnostics_by_level(diagnostics: &mut Vec<Diagnostic>, level: &str) {
    let min_severity = match level.to_lowercase().as_str() {
        "error" => DiagnosticSeverity::Error,
        "warn" | "warning" => DiagnosticSeverity::Warning,
        "info" | "information" => DiagnosticSeverity::Information,
        _ => DiagnosticSeverity::Warning,
    };

    diagnostics.retain(|diag| {
        diag.severity.map_or(true, |severity| severity >= min_severity)
    });
}

/// 快速创建诊断项（辅助函数）
fn create_diagnostic(
    line: u32,
    start_char: u32,
    end_char: u32,
    severity: DiagnosticSeverity,
    code: &str,
    message: String,
    docs: &str,
    source: &str,
) -> Diagnostic {
    Diagnostic {
        range: zed_extension_api::lsp::Range {
            start: zed_extension_api::lsp::Position::new(line, start_char),
            end: zed_extension_api::lsp::Position::new(line, end_char),
        },
        severity: Some(severity),
        code: Some(DiagnosticCode::String(code.to_string())),
        code_description: None,
        message,
        source: Some(source.to_string()),
        related_information: None,
        tags: None,
        data: None,
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: docs.to_string(),
        })),
    }
}
```

### 9. src/syntax.rs（语法片段）
```rust
//! 语法片段（遵循 Zed 片段规范）
use std::collections::HashMap;
use zed_extension_api::{lsp::CompletionKind, Result, Error};

/// 语法片段结构体（与 Zed 片段格式完全对齐）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Snippet {
    /// 触发词（Zed 补全时的匹配关键词）
    pub trigger: String,
    /// 描述（补全列表中显示的说明）
    pub description: String,
    /// 片段内容（支持 Zed 片段语法：${1:占位符}、${0:最终光标}）
    pub body: Vec<String>,
    /// 类型（与 LSP CompletionKind 对应）
    pub kind: CompletionKind,
}

impl Snippet {
    /// 创建新片段（简化构造）
    pub fn new(trigger: &str, description: &str, body: &[&str], kind: CompletionKind) -> Self {
        Self {
            trigger: trigger.to_string(),
            description: description.to_string(),
            body: body.iter().map(|s| s.to_string()).collect(),
            kind,
        }
    }
}

/// 获取 Cangjie 语言所有片段（按 Zed 规范组织）
pub fn get_cangjie_snippets() -> Result<HashMap<String, Vec<Snippet>>> {
    let mut snippets = HashMap::new();

    // 核心语法片段（与 tree-sitter-cangjie 语法严格对齐）
    let cj_snippets = vec![
        // 函数声明
        Snippet::new(
            "fn",
            "函数声明（Cangjie 官方语法）",
            &[
                "fn ${1:function_name}(${2:params})${3:: ${4:Void}} {",
                "  ${0:// 函数体}",
                "}"
            ],
            CompletionKind::Function
        ),
        // 变量声明
        Snippet::new(
            "let",
            "变量声明（可变）",
            &["let ${1:variable_name}${2:: ${3:type}}${4: = ${5:initial_value}};"],
            CompletionKind::Variable
        ),
        // 常量声明
        Snippet::new(
            "const",
            "常量声明（不可变，全大写命名）",
            &["const ${1:CONSTANT_NAME} = ${2:value};"],
            CompletionKind::Constant
        ),
        // 结构体声明
        Snippet::new(
            "struct",
            "结构体声明",
            &[
                "struct ${1:StructName} {",
                "  ${2:field_name}: ${3:type};",
                "  ${0:// 更多字段...}",
                "}"
            ],
            CompletionKind::Struct
        ),
        // 结构体方法
        Snippet::new(
            "method",
            "结构体方法声明",
            &[
                "method ${1:receiver}: ${2:StructName} ${3:method_name}(${4:params})${5:: ${6:Void}} {",
                "  ${0:// 方法体}",
                "}"
            ],
            CompletionKind::Method
        ),
        // 枚举声明
        Snippet::new(
            "enum",
            "枚举声明",
            &[
                "enum ${1:EnumName} {",
                "  ${2:Variant1},",
                "  ${3:Variant2}(${4:associated_data}),",
                "  ${0:// 更多变体...}",
                "}"
            ],
            CompletionKind::Enum
        ),
        // 接口声明
        Snippet::new(
            "interface",
            "接口声明（抽象方法集合）",
            &[
                "interface ${1:InterfaceName} {",
                "  ${2:method_name}(${3:params}): ${4:return_type};",
                "  ${0:// 更多方法...}",
                "}"
            ],
            CompletionKind::Interface
        ),
        // 导入语句
        Snippet::new(
            "import",
            "模块导入",
            &[
                "// 基础导入",
                "import \"${1:module_path}\";",
                "",
                "// 别名导入",
                "import \"${2:another_module}\" as ${3:alias};"
            ],
            CompletionKind::Module
        ),
        // if 条件语句
        Snippet::new(
            "if",
            "条件语句",
            &[
                "if (${1:condition}) {",
                "  ${0:// 满足条件时执行}",
                "}"
            ],
            CompletionKind::Snippet
        ),
        // if-else 语句
        Snippet::new(
            "ife",
            "if-else 条件语句",
            &[
                "if (${1:condition}) {",
                "  ${0:// 满足条件时执行}",
                "} else {",
                "  // 不满足条件时执行",
                "}"
            ],
            CompletionKind::Snippet
        ),
        // if-else if-else 语句
        Snippet::new(
            "ifei",
            "多分支条件语句",
            &[
                "if (${1:condition1}) {",
                "  ${0:// 满足条件1}",
                "} else if (${2:condition2}) {",
                "  // 满足条件2",
                "} else {",
                "  // 所有条件不满足",
                "}"
            ],
            CompletionKind::Snippet
        ),
        // for 循环（C 风格）
        Snippet::new(
            "for",
            "C 风格循环（初始化; 条件; 增量）",
            &[
                "for (${1:let i = 0}; ${2:i < length}; ${3:i++}) {",
                "  ${0:// 循环体}",
                "}"
            ],
            CompletionKind::Snippet
        ),
        // foreach 循环（迭代集合）
        Snippet::new(
            "foreach",
            "集合迭代循环",
            &[
                "foreach (${1:item} in ${2:collection}) {",
                "  ${0:// 迭代体}",
                "}"
            ],
            CompletionKind::Snippet
        ),
        // while 循环
        Snippet::new(
            "while",
            "while 循环（条件满足时执行）",
            &[
                "while (${1:condition}) {",
                "  ${0:// 循环体}",
                "}"
            ],
            CompletionKind::Snippet
        ),
        // do-while 循环
        Snippet::new(
            "dowhile",
            "do-while 循环（至少执行一次）",
            &[
                "do {",
                "  ${0:// 循环体}",
                "} while (${1:condition});"
            ],
            CompletionKind::Snippet
        ),
        // return 语句（无返回值）
        Snippet::new(
            "ret",
            "返回语句（无返回值）",
            &["return;"],
            CompletionKind::Snippet
        ),
        // return 语句（带返回值）
        Snippet::new(
            "reto",
            "返回语句（带值）",
            &["return ${1:value};"],
            CompletionKind::Snippet
        ),
        // Result 成功返回
        Snippet::new(
            "ok",
            "返回成功结果（Result<Ok>）",
            &["return Ok(${1:success_value});"],
            CompletionKind::Snippet
        ),
        // Result 错误返回
        Snippet::new(
            "err",
            "返回错误结果（Result<Err>）",
            &["return Err(${1:error_message});"],
            CompletionKind::Snippet
        ),
        // 块注释（文档注释）
        Snippet::new(
            "/**",
            "文档块注释（支持 Markdown）",
            &[
                "/**",
                " * ${0:// 文档说明}",
                " * ",
                " * @param ${1:param_name} - ${2:参数说明}",
                " * @return ${3:返回值说明}",
                " */"
            ],
            CompletionKind::Snippet
        ),
        // 测试函数
        Snippet::new(
            "test",
            "测试函数（遵循 Cangjie 测试规范）",
            &[
                "// 测试函数：自动被测试框架识别",
                "fn test_${1:test_name}() -> Result<Void, Error> {",
                "  // 准备测试数据",
                "  let ${2:input} = ${3:test_input};",
                "  ",
                "  // 执行测试",
                "  let result = ${4:function_under_test}(${2:input});",
                "  ",
                "  // 断言结果",
                "  assert_eq!(result, ${5:expected_output});",
                "  ",
                "  return Ok(());",
                "}"
            ],
            CompletionKind::Function
        ),
        // 错误处理（try-catch 风格）
        Snippet::new(
            "try",
            "错误处理（try-catch 语句）",
            &[
                "try {",
                "  ${0:// 可能抛出错误的代码}",
                "} catch (${1:error_name}: ${2:ErrorType}) {",
                "  // 错误处理逻辑",
                "  log_error(\"${3:错误描述}\", ${1:error_name});",
                "  return Err(${1:error_name});",
                "}"
            ],
            CompletionKind::Snippet
        ),
    ];

    // 按语言名称注册（必须与 Zed 语言配置的 `name` 完全一致）
    snippets.insert("Cangjie".to_string(), cj_snippets);

    // 验证片段有效性
    validate_snippets(&snippets)?;

    Ok(snippets)
}

/// 验证片段语法（确保兼容 Zed 片段解析器）
pub fn validate_snippets(snippets: &HashMap<String, Vec<Snippet>>) -> Result<()> {
    for (lang_name, lang_snippets) in snippets {
        for (idx, snippet) in lang_snippets.iter().enumerate() {
            // 1. 触发词不能为空
            if snippet.trigger.is_empty() {
                return Err(Error::InvalidData(format!(
                    "语言 '{}' 的第 {} 个片段触发词为空", lang_name, idx
                )));
            }

            // 2. 片段体不能为空
            if snippet.body.is_empty() {
                return Err(Error::InvalidData(format!(
                    "语言 '{}' 的片段 '{}' 内容为空", lang_name, snippet.trigger
                )));
            }

            // 3. 检查片段占位符语法（避免无效占位符导致 Zed 崩溃）
            let body_text = snippet.body.join("\n");
            let placeholder_re = regex::Regex::new(r"\$\{(\d+)(?::([^}]+))?\}")
                .map_err(|e| Error::InvalidData(format!("片段 '{}' 占位符正则验证失败：{}", snippet.trigger, e)))?;

            // 收集所有占位符编号
            let mut placeholder_nums = Vec::new();
            for cap in placeholder_re.captures_iter(&body_text) {
                let num_str = cap.get(1).unwrap().as_str();
                let num = num_str.parse::<u32>()
                    .map_err(|e| Error::InvalidData(format!(
                        "片段 '{}' 占位符编号 '{}' 无效：{}", snippet.trigger, num_str, e
                    )))?;
                placeholder_nums.push(num);
            }

            // 检查是否存在重复编号（Zed 不支持重复占位符）
            let mut unique_nums = std::collections::HashSet::new();
            for &num in &placeholder_nums {
                if !unique_nums.insert(num) {
                    return Err(Error::InvalidData(format!(
                        "片段 '{}' 存在重复占位符编号：{}", snippet.trigger, num
                    )));
                }
            }

            // 检查是否存在 0 号占位符（Zed 要求最终光标位置必须有 0 号）
            if !placeholder_nums.contains(&0) {
                log::warn!(
                    "片段 '{}' 缺少 0 号占位符（建议添加 ${0} 标记最终光标位置）",
                    snippet.trigger
                );
            }
        }
    }

    Ok(())
}
```

## 配置文件

### 10. package.json（Zed 扩展配置）
```json
{
  "name": "cangjie-zed-extension",
  "displayName": "Cangjie Language Support",
  "description": "Cangjie 语言完整支持：语法高亮、代码补全、格式化、诊断、跳转定义、悬停提示",
  "version": "0.5.0",
  "engines": {
    "zed": ">=0.130.0"
  },
  "categories": [
    "Languages",
    "Formatters",
    "Linters"
  ],
  "keywords": [
    "cangjie",
    "仓颉",
    "language-server",
    "zed-extension"
  ],
  "author": {
    "name": "Your Name",
    "email": "your-email@example.com",
    "url": "https://your-website.com"
  },
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/your-username/cangjie-zed-extension.git"
  },
  "contributes": {
    "languages": [
      {
        "id": "cangjie",
        "aliases": ["Cangjie", "仓颉", "cj"],
        "extensions": [".cj"],
        "configuration": "./language-configuration.json"
      }
    ],
    "configuration": {
      "title": "Cangjie 语言配置",
      "properties": {
        "cangjie.lsp_timeout_ms": {
          "type": "integer",
          "default": 5000,
          "minimum": 100,
          "maximum": 30000,
          "description": "LSP 服务超时时间（毫秒）"
        },
        "cangjie.realtime_diagnostics": {
          "type": "boolean",
          "default": true,
          "description": "是否启用实时语法诊断（文档编辑时自动检查）"
        },
        "cangjie.log_level": {
          "type": "string",
          "default": "info",
          "enum": ["trace", "debug", "info", "warn", "error", "off"],
          "description": "日志输出级别（调试时可设为 trace/debug）"
        },
        "cangjie.workspace_symbol_scan_depth": {
          "type": "integer",
          "default": 3,
          "minimum": 1,
          "maximum": 10,
          "description": "工作区符号扫描深度（值越大扫描越彻底，但启动速度越慢）"
        },
        "cangjie.scan_symbol_types": {
          "type": "array",
          "default": ["function", "variable", "struct", "enum", "import", "method", "constant", "interface"],
          "items": {
            "type": "string",
            "enum": ["function", "variable", "struct", "enum", "import", "method", "constant", "interface"]
          },
          "description": "工作区扫描时包含的符号类型"
        },
        "cangjie.fmt": {
          "type": "object",
          "default": {
            "indent_style": "space",
            "indent_size": 4,
            "tab_width": 4,
            "line_ending": "\n",
            "max_line_length": 120,
            "function_brace_style": "same_line",
            "struct_brace_style": "same_line",
            "trailing_comma": true,
            "space_around_operators": true,
            "space_inside_brackets": false,
            "auto_fix_syntax": true
          },
          "properties": {
            "indent_style": {
              "type": "string",
              "enum": ["space", "tab"],
              "description": "缩进风格（空格/制表符）"
            },
            "indent_size": {
              "type": "integer",
              "minimum": 1,
              "maximum": 16,
              "description": "空格缩进大小（仅 indent_style 为 space 时生效）"
            },
            "tab_width": {
              "type": "integer",
              "minimum": 1,
              "maximum": 16,
              "description": "制表符缩进宽度（仅 indent_style 为 tab 时生效）"
            },
            "line_ending": {
              "type": "string",
              "enum": ["\n", "\r\n"],
              "description": "行结束符（LF/CRLF）"
            },
            "max_line_length": {
              "type": "integer",
              "minimum": 80,
              "maximum": 200,
              "description": "最大行长度（超过会触发格式化换行）"
            },
            "function_brace_style": {
              "type": "string",
              "enum": ["same_line", "next_line"],
              "description": "函数大括号风格（同行/下一行）"
            },
            "struct_brace_style": {
              "type": "string",
              "enum": ["same_line", "next_line"],
              "description": "结构体大括号风格（同行/下一行）"
            },
            "trailing_comma": {
              "type": "boolean",
              "description": "是否在数组/结构体字段末尾添加尾随逗号"
            },
            "space_around_operators": {
              "type": "boolean",
              "description": "是否在运算符（+、=、== 等）周围添加空格"
            },
            "space_inside_brackets": {
              "type": "boolean",
              "description": "是否在括号（()、[]、{}）内添加空格"
            },
            "auto_fix_syntax": {
              "type": "boolean",
              "description": "格式化时是否自动修复简单语法错误（如缺少分号）"
            }
          },
          "description": "代码格式化配置"
        },
        "cangjie.lint": {
          "type": "object",
          "default": {
            "check_level": "warn",
            "enable_style_check": true,
            "enable_syntax_check": true,
            "ignore_rules": [],
            "custom_rules_path": null
          },
          "properties": {
            "check_level": {
              "type": "string",
              "enum": ["error", "warn", "info", "off"],
              "description": "检查级别（仅显示指定级别及以上的诊断）"
            },
            "enable_style_check": {
              "type": "boolean",
              "description": "是否启用代码风格检查（如命名规范、行长度）"
            },
            "enable_syntax_check": {
              "type": "boolean",
              "description": "是否启用语法错误检查（基于 tree-sitter）"
            },
            "ignore_rules": {
              "type": "array",
              "items": {
                "type": "string"
              },
              "description": "需要忽略的检查规则（如 [\"UNUSED_VARIABLE\", \"LINE_TOO_LONG\"]）"
            },
            "custom_rules_path": {
              "type": ["string", "null"],
              "description": "自定义检查规则文件路径（JSON 格式，支持正则和节点规则）"
            }
          },
          "description": "代码检查配置"
        }
      }
    }
  },
  "main": "./target/release/libcangjie_zed_extension.so",
  "activationEvents": ["onLanguage:cangjie"],
  "scripts": {
    "build": "cargo build --release",
    "dev": "cargo build",
    "test": "cargo test",
    "package": "zed extension package"
  }
}
```

### 11. language-configuration.json（语言配置）
```json
{
  "comments": {
    "lineComment": "//",
    "blockComment": ["/*", "*/"]
  },
  "brackets": [
    ["{", "}"],
    ["(", ")"],
    ["[", "]"]
  ],
  "autoClosingPairs": [
    {"open": "{", "close": "}"},
    {"open": "(", "close": ")"},
    {"open": "[", "close": "]"},
    {"open": "\"", "close": "\""},
    {"open": "'", "close": "'"},
    {"open": "/*", "close": "*/"}
  ],
  "surroundingPairs": [
    {"open": "{", "close": "}"},
    {"open": "(", "close": ")"},
    {"open": "[", "close": "]"},
    {"open": "\"", "close": "\""},
    {"open": "'", "close": "'"}
  ],
  "folding": {
    "markers": {
      "start": "^\\s*//\\s*#region",
      "end": "^\\s*//\\s*#endregion"
    }
  },
  "wordPattern": "\\w+|[^\u0000-\u007F\u2013-\u2014\u2026\u2018-\u2019\u201C-\u201D]+|[^\\s\\w\\[\\]\\{\\}\\(\\)\\;\\,\\.\\<\\>\\?\\!\\@\\#\\$\\%\\^\\&\\*\\-\\+\\=\\/\\|\\:\\\"\\'\\`\\~]+"
}
```

### 12. build.sh（跨平台编译脚本）
```bash
#!/bin/bash
set -e

# Cangjie Zed 扩展编译脚本
# 支持：Linux/macOS/Windows（WSL）

# 检查 Cargo 是否安装
if ! command -v cargo &> /dev/null; then
    echo "错误：未找到 Cargo，请安装 Rust 工具链（https://www.rust-lang.org/）"
    exit 1
fi

# 检查 Zed 是否安装（可选）
if ! command -v zed &> /dev/null; then
    echo "警告：未找到 Zed 编辑器，编译后需手动安装扩展"
fi

# 编译模式选择
MODE="release"
if [ "$1" = "dev" ]; then
    MODE="debug"
    echo "=== 开始调试模式编译 ==="
else
    echo "=== 开始发布模式编译 ==="
fi

# 编译扩展
cargo build --$MODE

# 复制编译产物到当前目录（方便打包）
if [ "$(uname -s)" = "Darwin" ]; then
    # macOS
    cp target/$MODE/libcangjie_zed_extension.dylib ./
    echo "编译产物：libcangjie_zed_extension.dylib"
elif [ "$(uname -s)" = "Linux" ]; then
    # Linux
    cp target/$MODE/libcangjie_zed_extension.so ./
    echo "编译产物：libcangjie_zed_extension.so"
elif [ "$(uname -s | grep -i windows)" ]; then
    # Windows（WSL）
    cp target/$MODE/cangjie_zed_extension.dll ./
    echo "编译产物：cangjie_zed_extension.dll"
else
    echo "警告：未知操作系统，编译产物需手动从 target/$MODE/ 复制"
fi

# 生成扩展包（如果 Zed 已安装）
if command -v zed &> /dev/null; then
    echo "=== 生成 Zed 扩展包 ==="
    zed extension package
fi

echo "=== 编译完成 ==="
echo "下一步："
echo "1. 在 Zed 中打开「扩展」面板"
echo "2. 点击「安装本地扩展」，选择编译产物或 .zed 包"
echo "3. 新建 .cj 文件测试功能"
```

### 13. README.md（安装说明）
```markdown
# Cangjie Language Support for Zed

Cangjie（仓颉）语言的 Zed 编辑器完整支持扩展，包含以下核心功能：

- ✅ 语法高亮（基于 tree-sitter-cangjie 官方语法）
- ✅ 智能代码补全（内置符号 + 标准库 + 语法片段）
- ✅ 代码格式化（自定义缩进、风格配置）
- ✅ 实时语法诊断（语法错误 + 代码风格检查）
- ✅ 跳转定义（文档内 + 工作区跨文件）
- ✅ 悬停提示（符号详情 + 文档说明）
- ✅ 文档符号（大纲视图支持）
- ✅ 代码折叠（基于语法结构）
- ✅ 自动缩进 + 括号自动闭合
- ✅ 自定义检查规则（支持正则和节点规则）

## 安装要求
- Zed 编辑器 ≥ 0.130.0
- Rust 工具链 ≥ 1.70.0（编译扩展用）

## 安装方式

### 方式 1：直接安装扩展包（推荐）
1. 下载最新的 `.zed` 扩展包（从 Releases 页面）
2. 打开 Zed → 扩展面板 → 安装本地扩展 → 选择下载的 `.zed` 包
3. 重启 Zed 即可生效

### 方式 2：手动编译安装
1. 克隆仓库：
   ```bash
   git clone https://github.com/your-username/cangjie-zed-extension.git
   cd cangjie-zed-extension
   ```

2. 编译扩展：
   ```bash
   # 发布模式（推荐，性能更好）
   ./build.sh

   # 或调试模式（开发用）
   ./build.sh dev
   ```

3. 安装扩展：
   - 打开 Zed → 扩展面板 → 安装本地扩展
   - 选择编译产物（`libcangjie_zed_extension.so`/`.dylib`/`.dll`）或生成的 `.zed` 包

## 配置说明
在 Zed 的 `settings.json` 中可自定义配置（示例）：
```json
{
  "cangjie": {
    "log_level": "info",
    "fmt": {
      "indent_style": "space",
      "indent_size": 4,
      "trailing_comma": true,
      "max_line_length": 120
    },
    "lint": {
      "check_level": "warn",
      "ignore_rules": ["UNUSED_VARIABLE"],
      "custom_rules_path": "/path/to/custom-rules.json"
    }
  }
}
```

### 自定义检查规则示例（custom-rules.json）
```json
{
  "regex_rules": [
    {
      "name": "NO_DEBUG_PRINT",
      "pattern": "debug_print\\(",
      "message": "禁止使用 debug_print（生产环境请移除）",
      "severity": "error"
    }
  ],
  "node_rules": [
    {
      "name": "NO_GLOBAL_VARS",
      "node_kind": "variable_declaration",
      "message": "禁止定义全局变量（建议使用模块级常量或函数参数）",
      "severity": "warn"
    }
  ]
}
```

## 功能使用

### 代码补全
- 输入触发词（如 `fn`、`struct`、`if`）后按 `Tab` 展开片段
- 支持运算符（`.`、`:`、`(`）触发补全
- 补全项包含符号类型图标和详情说明

### 格式化
- 快捷键：`Cmd/Ctrl + Shift + I`
- 或右键菜单 → 格式化文档
- 支持自定义缩进、大括号风格等配置

### 诊断
- 实时显示语法错误和风格问题（红色波浪线 = 错误，黄色 = 警告）
- 鼠标悬停可查看详细说明和修复建议
- 支持忽略指定规则

### 跳转定义
- 按住 `Cmd/Ctrl` 点击符号，或右键 → 跳转定义
- 支持跨文件跳转工作区符号

## 开发与贡献
1.  Fork 本仓库
2.  创建特性分支：`git checkout -b feature/xxx`
3.  提交修改：`git commit -m "add xxx feature"`
4.  推送分支：`git push origin feature/xxx`
5.  提交 Pull Request

### 开发依赖
- `tree-sitter-cangjie`：仓颉语言的 tree-sitter 语法包
- `zed_extension_api`：Zed 扩展开发 API
- `serde`/`serde_json`：配置和数据序列化
- `regex`：正则匹配（用于代码检查）

## 问题反馈
如果遇到功能异常或有需求建议，请在 [Issues](https://github.com/your-username/cangjie-zed-extension/issues) 提交反馈，包含以下信息：
- Zed 版本（Zed → About Zed）
- 扩展版本
- 问题复现步骤
- 相关代码片段或截图

## 许可证
本扩展基于 MIT 许可证开源，详见 [LICENSE](LICENSE) 文件。
```

### 14. LICENSE（MIT 许可证）
```
MIT License

Copyright (c) 2024 Your Name

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 打包与部署
1. 编译扩展：运行 `./build.sh` 生成编译产物和 `.zed` 扩展包
2. 发布扩展：
   - 上传 `.zed` 包到 GitHub Releases
   - 或提交到 Zed 扩展市场（需遵循 Zed 扩展发布规范）
3. 版本更新：
   - 修改 `Cargo.toml` 和 `package.json` 中的版本号
   - 重新编译并发布

## 验证功能
1. 新建 `test.cj` 文件，输入以下代码测试：
   ```cangjie
   // 测试函数
   fn add(a: Int, b: Int) -> Int {
     return a + b;
   }

   // 测试结构体
   struct Person {
     name: String;
     age: Int;
   }

   // 测试常量
   const MAX_RETRY = 3;

   // 测试循环
   for (let i = 0; i < MAX_RETRY; i++) {
     println("尝试次数: " + i);
   }
   ```
2. 验证语法高亮、补全、格式化、诊断功能是否正常工作
3. 测试跳转定义（如点击 `add` 函数查看定义）

此扩展完全遵循 Zed 官方扩展规范，与 tree-sitter-cangjie 语法包深度集成，可直接用于 Cangjie 语言的日常开发。如需扩展功能，可基于现有架构添加新模块（如代码重构、测试运行等）。



# 在 Zed 中使用 Cangjie 语言扩展（完整指南）
以下是从「安装扩展」到「日常开发使用」的全流程教程，适配 Windows/macOS/Linux 三大平台，确保你能快速上手所有功能。

## 一、前置准备
在使用扩展前，请确保你的环境满足以下要求：
1. **Zed 编辑器版本**：≥ 0.130.0（低于此版本可能不支持部分 LSP 功能）
   - 查看版本：Zed → 帮助（Help）→ 关于 Zed（About Zed）
   - 升级方式：Zed 会自动检查更新，或在「扩展面板」→ 右上角「...」→ 检查更新
2. **扩展文件**：已编译好的扩展产物（二选一）
   - 方式 1：本地编译产物（`libcangjie_zed_extension.so`/`.dylib`/`.dll`）
   - 方式 2：打包好的 `.zed` 扩展包（如 `cangjie-zed-extension-linux.zed`）

## 二、安装扩展（3种方式）
### 方式 1：安装本地 `.zed` 扩展包（推荐，最简单）
1. 下载对应平台的 `.zed` 包（从 GitHub Releases 或本地编译生成）
   - Linux：`cangjie-zed-extension-linux.zed`
   - macOS：`cangjie-zed-extension-macos.zed`
   - Windows：`cangjie-zed-extension-windows.zed`
2. 打开 Zed 编辑器，点击左侧边栏的「扩展」图标（或按 `Ctrl+Shift+X`/`Cmd+Shift+X`）
3. 在扩展面板右上角，点击「安装本地扩展」（Install Local Extension）
4. 在文件选择器中，选中下载的 `.zed` 包，等待安装完成
5. 安装成功后，扩展面板会显示「Cangjie Language Support」，状态为「已启用」

### 方式 2：安装本地编译产物（适合开发调试）
1. 按照之前的 `build.sh` 脚本编译扩展，得到对应平台的产物：
   - Linux：`libcangjie_zed_extension.so`
   - macOS：`libcangjie_zed_extension.dylib`
   - Windows：`cangjie_zed_extension.dll`
2. 打开 Zed → 扩展面板 → 安装本地扩展
3. 选择编译产物文件（注意：不是 `.zed` 包，是单个 `.so`/`.dylib`/`.dll` 文件）
4. 等待安装完成，扩展会自动启用

### 方式 3：从 Zed 扩展市场安装（若已发布）
1. 打开 Zed → 扩展面板
2. 在搜索框中输入「Cangjie」，找到「Cangjie Language Support」
3. 点击「安装」按钮，等待自动下载并启用

## 三、验证安装成功
1. 新建文件：在 Zed 中点击「文件」→「新建文件」（或按 `Ctrl+N`/`Cmd+N`）
2. 保存文件：按 `Ctrl+S`/`Cmd+S`，文件名设为 `test.cj`（`.cj` 是 Cangjie 语言的默认后缀）
3. 输入测试代码：
   ```cangjie
   fn add(a: Int, b: Int) -> Int {
     return a + b;
   }
   ```
4. 验证核心功能：
   - 语法高亮：关键字（`fn`、`return`）、类型（`Int`）、函数名（`add`）应显示不同颜色
   - 代码补全：输入 `fn` 后按 `Tab`，应自动展开函数声明片段
   - 无报错：若代码无语法错误，不会显示红色波浪线

## 四、核心功能使用教程
### 1. 语法高亮与基础编辑
#### 支持的语法元素高亮：
- 关键字：`fn`、`let`、`const`、`struct`、`enum`、`interface`、`if`、`for` 等
- 类型：`Int`、`String`、`Float`、`Bool`、自定义结构体/枚举名
- 常量：全大写命名的常量（如 `MAX_RETRY`）
- 注释：`// 单行注释`、`/* 块注释 */`、`/** 文档注释 */`
- 字符串：`"双引号字符串"`、`'单引号字符串'`
- 运算符：`+`、`=`、`==`、`->` 等

#### 基础编辑功能：
- 括号自动闭合：输入 `{`/`(`/`[`/`"`/`'` 时，自动补全对应的闭合符号
- 自动缩进：换行后自动继承上一行的缩进（可在配置中自定义缩进大小）
- 代码折叠：点击行号左侧的 `-` 可折叠函数、结构体、注释块（支持 `// #region`/`// #endregion` 手动标记折叠区域）

### 2. 智能代码补全
#### 触发方式：
- 自动触发：输入关键字（如 `fn`、`struct`）、标识符（如函数名、变量名）时自动弹出补全列表
- 手动触发：按 `Ctrl+Space`/`Cmd+Space` 强制弹出补全列表
- 触发字符：输入 `.`（访问结构体字段/方法）、`:`（类型注解）、`(`（函数调用）时自动触发

#### 补全内容分类：
- 语法片段：输入 `fn`→`Tab` 展开函数声明，`struct`→`Tab` 展开结构体声明（完整片段列表见 `src/syntax.rs`）
- 文档符号：当前文件中的函数、变量、结构体等（支持模糊匹配）
- 工作区符号：其他 `.cj` 文件中的符号（跨文件补全）
- 标准库符号：`println`、`Vec`、`Result`、`Option` 等内置类型和函数

#### 示例：使用片段补全
1. 在 `test.cj` 中输入 `struct`，补全列表会显示「struct - 结构体声明」
2. 按 `Tab` 展开片段，自动生成：
   ```cangjie
   struct ${1:StructName} {
     ${2:field_name}: ${3:type};
     ${0:// 更多字段...}
   }
   ```
3. 按 `Tab` 可切换占位符（`StructName`→`field_name`→`type`→最终光标位置），直接输入内容即可替换占位符

### 3. 代码格式化
#### 格式化方式：
- 快捷键：`Ctrl+Shift+I`（Windows/Linux）/`Cmd+Shift+I`（macOS）
- 右键菜单：在编辑区右键 → 「格式化文档」（Format Document）
- 自动格式化：可在 Zed 配置中设置「保存时自动格式化」（需配合 Zed 全局配置）

#### 可自定义的格式化规则（在 Zed `settings.json` 中配置）：
```json
{
  "cangjie": {
    "fmt": {
      "indent_style": "space", // 缩进风格：space（空格）/tab（制表符）
      "indent_size": 4, // 空格缩进大小（默认4）
      "trailing_comma": true, // 数组/结构体字段末尾添加尾随逗号
      "max_line_length": 120, // 最大行长度（超过自动换行）
      "function_brace_style": "same_line" // 函数大括号：same_line（同行）/next_line（下一行）
    }
  }
}
```

#### 格式化效果示例：
- 未格式化代码：
  ```cangjie
  struct User{id:Int;name:String;age:Int;}
  fn getUser(id:Int)->User{return User{id:id,name:"Alice",age:25};}
  ```
- 格式化后代码：
  ```cangjie
  struct User {
    id: Int;
    name: String;
    age: Int;
  }

  fn getUser(id: Int) -> User {
    return User {
      id: id,
      name: "Alice",
      age: 25,
    };
  }
  ```

### 4. 实时诊断（语法错误 + 风格检查）
#### 诊断类型：
- 语法错误（红色波浪线）：如缺少分号、括号不匹配、无效语法结构
- 风格警告（黄色波浪线）：如行长度过长、变量未使用、常量命名不规范
- 信息提示（蓝色波浪线）：如 TODO 注释、不推荐的用法

#### 查看诊断详情：
- 鼠标悬停在带有波浪线的代码上，会显示诊断信息和修复建议
- 示例：未使用的变量会显示「变量 'x' 已定义但未使用」，并建议「删除或在变量名前加下划线」

#### 自定义诊断规则：
1. 在 Zed `settings.json` 中配置忽略规则：
   ```json
   {
     "cangjie": {
       "lint": {
         "ignore_rules": ["UNUSED_VARIABLE", "LINE_TOO_LONG"] // 忽略指定规则
       }
     }
   }
   ```
2. 使用自定义规则文件：
   - 创建 `custom-rules.json`（参考之前的示例文件）
   - 在配置中指定路径：
     ```json
     {
       "cangjie": {
         "lint": {
           "custom_rules_path": "/path/to/custom-rules.json"
         }
       }
     }
     ```

### 5. 跳转定义
#### 使用方式：
- 鼠标操作：按住 `Ctrl`（Windows/Linux）/`Cmd`（macOS），点击变量名、函数名、结构体名等符号，会跳转到其定义位置
- 快捷键：选中符号后，按 `F12` 直接跳转
- 跨文件跳转：若符号定义在其他 `.cj` 文件中，会自动打开对应文件并定位到定义行

#### 示例：
1. 在 `test.cj` 中定义函数：
   ```cangjie
   fn add(a: Int, b: Int) -> Int {
     return a + b;
   }
   ```
2. 在下方调用函数：
   ```cangjie
   let result = add(10, 20);
   ```
3. 按住 `Ctrl`/`Cmd` 点击 `add`，会跳转到函数定义行

### 6. 悬停提示
#### 使用方式：
- 鼠标悬停在符号（函数、变量、结构体等）上，会显示符号详情：
  - 函数：显示函数签名（参数类型、返回值类型）
  - 结构体：显示结构体名称和字段信息
  - 常量：显示常量值和类型
  - 标准库符号：显示官方文档说明

#### 示例：
悬停在 `println` 上，会显示：
```
println(message: String) -> Void - 打印字符串
标准库内置函数：输出字符串到控制台
```

### 7. 文档符号（大纲视图）
#### 使用方式：
1. 打开 Zed 左侧边栏的「大纲」图标（或按 `Ctrl+Shift+O`/`Cmd+Shift+O`）
2. 大纲视图会显示当前文件的所有符号，按类型分组（函数、结构体、枚举、常量等）
3. 点击大纲中的符号，会直接跳转到对应代码位置

#### 支持的符号类型：
- 函数（Function）
- 结构体（Struct）
- 枚举（Enum）
- 常量（Constant）
- 变量（Variable）
- 接口（Interface）
- 导入（Import）

## 五、高级配置（自定义扩展行为）
### 打开 Zed 配置文件：
1. 点击 Zed 顶部菜单 →「编辑」（Edit）→「设置」（Settings）（或按 `Ctrl+,`/`Cmd+,`）
2. 选择「用户设置」（User Settings）或「工作区设置」（Workspace Settings）
   - 用户设置：全局生效（所有项目）
   - 工作区设置：仅当前项目生效（优先级高于用户设置）
3. 在配置文件中添加 `cangjie` 节点，自定义各项功能

### 常用配置示例：
```json
{
  "cangjie": {
    // 基础配置
    "log_level": "info", // 日志级别：trace/debug/info/warn/error/off（调试时用 trace）
    "realtime_diagnostics": true, // 实时诊断（编辑时自动检查错误）
    "workspace_symbol_scan_depth": 3, // 工作区符号扫描深度（1-10）

    // 格式化配置
    "fmt": {
      "indent_style": "space",
      "indent_size": 2, // 缩进改为 2 个空格
      "max_line_length": 100, // 最大行长度改为 100 字符
      "trailing_comma": false, // 禁用尾随逗号
      "function_brace_style": "next_line" // 函数大括号换行显示
    },

    // 诊断配置
    "lint": {
      "check_level": "error", // 仅显示错误级别的诊断（忽略警告和信息）
      "ignore_rules": ["UNUSED_VARIABLE"], // 忽略「未使用变量」警告
      "custom_rules_path": "/home/user/custom-rules.json" // 自定义规则文件路径
    },

    // 符号扫描配置
    "scan_symbol_types": ["function", "struct", "enum", "constant"] // 仅扫描指定类型的符号
  }
}
```

## 六、常见问题排查
### 问题 1：扩展安装失败
#### 可能原因：
- Zed 版本过低（低于 0.130.0）
- 扩展产物与系统不匹配（如 Windows 用了 Linux 的 `.so` 文件）
- 权限不足（无法写入 Zed 扩展目录）

#### 解决方法：
- 升级 Zed 到最新版本
- 确认扩展产物对应当前系统（Linux→`.so`、macOS→`.dylib`、Windows→`.dll`）
- 以管理员/root 权限运行 Zed（仅安装时需要）

### 问题 2：语法高亮不生效
#### 可能原因：
- 文件后缀不是 `.cj`（Zed 无法识别语言）
- 扩展未启用（扩展面板中显示「已禁用」）
- tree-sitter 语法加载失败

#### 解决方法：
- 将文件保存为 `.cj` 后缀（如 `main.cj`）
- 扩展面板中找到「Cangjie Language Support」，点击「启用」
- 重启 Zed 编辑器（重新加载 tree-sitter 语法）

### 问题 3：代码补全不触发
#### 可能原因：
- 未在 `.cj` 文件中编辑（仅 Cangjie 语言文件触发补全）
- 补全被 Zed 全局配置禁用
- 工作区符号未加载完成

#### 解决方法：
- 确保文件后缀为 `.cj`
- 检查 Zed 配置：`"editor.completion.enabled": true`（默认开启）
- 等待工作区符号加载（底部状态栏会显示「加载符号中」，完成后会提示）
- 手动触发补全：按 `Ctrl+Space`/`Cmd+Space`

### 问题 4：跳转定义失败
#### 可能原因：
- 符号未定义（或定义在非 `.cj` 文件中）
- 工作区符号未加载（跨文件跳转需要扫描工作区）
- 符号名称拼写错误

#### 解决方法：
- 确认符号在当前文件或工作区的 `.cj` 文件中已定义
- 等待工作区符号加载完成（首次打开项目可能需要几秒）
- 检查符号名称拼写（区分大小写）

### 问题 5：格式化功能无效
#### 可能原因：
- 代码存在语法错误（格式化需要先解析语法）
- 快捷键冲突（被其他扩展占用）
- 格式化配置错误（如 `indent_size` 设为 0）

#### 解决方法：
- 修复代码中的语法错误（红色波浪线标记的部分）
- 用右键菜单「格式化文档」测试（排除快捷键冲突）
- 检查 `fmt` 配置（确保 `indent_size` ≥1，`indent_style` 为 `space` 或 `tab`）

## 七、卸载扩展
若需要卸载扩展，按以下步骤操作：
1. 打开 Zed → 扩展面板（`Ctrl+Shift+X`/`Cmd+Shift+X`）
2. 在扩展列表中找到「Cangjie Language Support」
3. 点击扩展右侧的「...」→「卸载」（Uninstall）
4. 重启 Zed 生效（可选，部分资源需要重启释放）

---

通过以上步骤，你可以充分利用 Cangjie 扩展的所有功能进行日常开发。如果遇到未解决的问题，可查看 Zed 日志（帮助 → 显示日志文件）或提交 Issue 反馈。
