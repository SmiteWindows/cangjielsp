### 附录 Y：扩展多语言混合编程支持
现代开发中多语言协同场景日益普遍，Cangjie 扩展新增**多语言混合编程支持**，允许在 Cangjie 项目中无缝集成 Rust、Python、JavaScript 等主流语言，实现跨语言调用、类型互通和统一开发体验。

#### Y.1 混合编程架构设计
```
┌─────────────────────┐      ┌─────────────────────┐      ┌─────────────────────┐
│  语言解析层         │      ┌─────────────────────┤      │  跨语言运行时层     │
│  - Cangjie 解析     │─────▶│  混合语言 AST 层    │─────▶│  - FFI 桥接         │
│  - 第三方语言解析   │      │  - 跨语言节点映射   │      │  - 类型转换         │
│  (Rust/Python/JS)   │      │  - 依赖关系分析     │      │  - 函数调用转发     │
└─────────────────────┘      └─────────────────────┘      └─────────────────────┘
        ▲                              ▲                              ▲
        │                              │                              │
        ▼                              ▼                              ▼
┌─────────────────────┐      ┌─────────────────────┐      ┌─────────────────────┐
│  LSP 增强层         │      │  构建工具链层       │      │  开发体验层         │
│  - 跨语言补全       │      │  - 多语言编译       │      │  - 统一语法高亮     │
│  - 跨语言跳转       │      │  - 依赖管理         │      │  - 跨语言调试       │
│  - 跨语言诊断       │      │  - 产物链接         │      │  - 混合代码格式化   │
└─────────────────────┘      └─────────────────────┘      └─────────────────────┘
```

#### Y.2 核心功能实现
##### 1. 混合语言配置定义（`src/config/multi_language.rs`）
```rust
//! 多语言混合编程配置
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MultiLanguageConfig {
    /// 启用多语言混合编程支持
    pub enabled: bool,
    /// 支持的第三方语言列表
    pub supported_languages: Vec<SupportedLanguage>,
    /// 跨语言 FFI 配置
    pub ffi_config: FfiConfig,
    /// 混合代码格式化配置
    pub formatting: MultiLangFormattingConfig,
}

/// 支持的第三方语言
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SupportedLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    C,
    Cpp,
}

impl SupportedLanguage {
    /// 获取语言 ID（与 Zed 语言 ID 一致）
    pub fn lang_id(&self) -> &str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::C => "c",
            Self::Cpp => "cpp",
        }
    }

    /// 获取文件后缀列表
    pub fn file_extensions(&self) -> &[&str] {
        match self {
            Self::Rust => &[".rs"],
            Self::Python => &[".py"],
            Self::JavaScript => &[".js"],
            Self::TypeScript => &[".ts"],
            Self::C => &[".c"],
            Self::Cpp => &[".cpp", ".hpp", ".cc", ".hh"],
        }
    }

    /// 获取对应的 Tree-sitter 语言
    pub fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        match self {
            Self::Rust => Some(tree_sitter_rust::language()),
            Self::Python => Some(tree_sitter_python::language()),
            Self::JavaScript => Some(tree_sitter_javascript::language()),
            Self::TypeScript => Some(tree_sitter_typescript::language_tsx()),
            Self::C => Some(tree_sitter_c::language()),
            Self::Cpp => Some(tree_sitter_cpp::language()),
            _ => None,
        }
    }
}

/// FFI 配置
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct FfiConfig {
    /// 自动生成 FFI 绑定代码
    pub auto_generate_bindings: bool,
    /// FFI 绑定代码输出目录
    pub bindings_output_dir: PathBuf,
    /// 类型转换策略
    pub type_conversion_strategy: TypeConversionStrategy,
    /// 跨语言调用超时（毫秒）
    pub call_timeout_ms: u64,
}

/// 类型转换策略
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub enum TypeConversionStrategy {
    /// 严格模式（类型完全匹配）
    #[default]
    Strict,
    /// 兼容模式（自动进行安全类型转换）
    Compatible,
    /// 宽松模式（允许隐式类型转换）
    Lenient,
}

/// 混合代码格式化配置
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MultiLangFormattingConfig {
    /// 统一缩进风格（覆盖各语言默认缩进）
    pub unified_indent: Option<u8>,
    /// 统一换行符
    pub unified_line_ending: Option<LineEnding>,
    /// 各语言格式化选项
    pub per_language_options: std::collections::HashMap<SupportedLanguage, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LineEnding {
    LF,
    CRLF,
    Auto,
}
```

##### 2. 混合语言 AST 构建（`src/multi_language/ast.rs`）
```rust
//! 混合语言 AST 处理
use super::config::SupportedLanguage;
use zed_extension_api::{self as zed, Document};
use tree_sitter::{Tree, Node};
use std::collections::HashMap;

/// 混合语言项目 AST 管理器
pub struct MultiLangAstManager {
    /// 各语言的解析器缓存
    parsers: HashMap<SupportedLanguage, tree_sitter::Parser>,
    /// 已解析的文档 AST 缓存
    ast_cache: HashMap<String, (Tree, SupportedLanguage)>, // key: document uri
}

impl MultiLangAstManager {
    /// 初始化 AST 管理器
    pub fn new(supported_languages: &[SupportedLanguage]) -> Result<Self, zed::Error> {
        let mut parsers = HashMap::new();
        for lang in supported_languages {
            if let Some(ts_lang) = lang.tree_sitter_language() {
                let mut parser = tree_sitter::Parser::new();
                parser.set_language(ts_lang)?;
                parsers.insert(lang.clone(), parser);
            }
        }

        Ok(Self {
            parsers,
            ast_cache: HashMap::new(),
        })
    }

    /// 解析文档（自动识别语言）
    pub fn parse_document(&mut self, document: &Document) -> Result<(Tree, SupportedLanguage), zed::Error> {
        // 先查缓存
        if let Some((tree, lang)) = self.ast_cache.get(document.uri().as_str()) {
            return Ok((tree.clone(), lang.clone()));
        }

        // 识别语言（通过文件后缀或语言 ID）
        let lang = self.detect_language(document)?;
        let parser = self.parsers.get_mut(&lang).ok_or_else(|| {
            zed::Error::user(format!("Unsupported language: {}", lang.lang_id()))
        })?;

        // 解析文档
        let text = document.text();
        let tree = parser.parse(&text, None).ok_or_else(|| {
            zed::Error::user(format!("Failed to parse {} document", lang.lang_id()))
        })?;

        // 存入缓存
        self.ast_cache.insert(document.uri().to_string(), (tree.clone(), lang.clone()));

        Ok((tree, lang))
    }

    /// 检测文档语言
    fn detect_language(&self, document: &Document) -> Result<SupportedLanguage, zed::Error> {
        // 优先通过语言 ID 识别
        let lang_id = document.language_id();
        for (lang, _) in &self.parsers {
            if lang.lang_id() == lang_id {
                return Ok(lang.clone());
            }
        }

        // 通过文件后缀识别
        let uri = document.uri();
        let path = uri.to_file_path().map_err(|_| {
            zed::Error::user("Failed to convert document URI to file path")
        })?;
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        for (lang, _) in &self.parsers {
            if lang.file_extensions().contains(&ext) {
                return Ok(lang.clone());
            }
        }

        Err(zed::Error::user(format!(
            "Cannot detect language for document: {}",
            uri
        )))
    }

    /// 清除指定文档的 AST 缓存
    pub fn clear_cache(&mut self, document_uri: &str) {
        self.ast_cache.remove(document_uri);
    }

    /// 查找跨语言调用节点
    pub fn find_cross_lang_call_nodes(&mut self, document: &Document) -> Result<Vec<CrossLangCallNode>, zed::Error> {
        let (tree, lang) = self.parse_document(document)?;
        let root_node = tree.root_node();
        let mut call_nodes = Vec::new();

        // 根据语言类型查找跨语言调用节点
        match lang {
            SupportedLanguage::Rust => {
                // Rust 中调用 Cangjie：cangjie::func()
                self.query_cross_lang_calls(&root_node, "((call_expression (path_expression (identifier) @lang (identifier) @func) ) (#eq? @lang \"cangjie\"))", &lang, document)?
                    .into_iter()
                    .for_each(|node| call_nodes.push(node));
            }
            SupportedLanguage::Python => {
                // Python 中调用 Cangjie：import cangjie; cangjie.func()
                self.query_cross_lang_calls(&root_node, "((call_expression (attribute (identifier) @lang (identifier) @func) ) (#eq? @lang \"cangjie\"))", &lang, document)?
                    .into_iter()
                    .for_each(|node| call_nodes.push(node));
            }
            _ => {
                // 其他语言的跨语言调用模式
            }
        }

        // 反向查找：Cangjie 调用其他语言
        if lang == SupportedLanguage::Cangjie {
            // Cangjie 中调用 Rust：rust::func()
            self.query_cross_lang_calls(&root_node, "((function_call function: (qualified_identifier (identifier) @lang (identifier) @func) ) (#match? @lang \"rust|python|javascript|c|cpp\"))", &lang, document)?
                .into_iter()
                .for_each(|node| call_nodes.push(node));
        }

        Ok(call_nodes)
    }

    /// 通用跨语言调用节点查询
    fn query_cross_lang_calls(
        &self,
        root_node: &Node,
        query_str: &str,
        source_lang: &SupportedLanguage,
        document: &Document,
    ) -> Result<Vec<CrossLangCallNode>, zed::Error> {
        let query = tree_sitter::Query::new(source_lang.tree_sitter_language().unwrap(), query_str)?;
        let mut cursor = tree_sitter::QueryCursor::new();
        let text = document.text().as_bytes();
        let mut call_nodes = Vec::new();

        for match_result in cursor.matches(&query, *root_node, text) {
            let mut lang = None;
            let mut func_name = None;
            let mut node = None;

            for capture in match_result.captures {
                match capture.name {
                    Some("lang") => lang = Some(String::from_utf8_lossy(&text[capture.node.byte_range()]).to_string()),
                    Some("func") => func_name = Some(String::from_utf8_lossy(&text[capture.node.byte_range()]).to_string()),
                    _ => node = Some(capture.node),
                }
            }

            if let (Some(lang), Some(func_name), Some(node)) = (lang, func_name, node) {
                let target_lang = SupportedLanguage::try_from(lang.as_str())?;
                call_nodes.push(CrossLangCallNode {
                    source_lang: source_lang.clone(),
                    target_lang,
                    func_name,
                    node,
                    range: zed::lsp::Range::from_lsp_range(
                        document,
                        node.range().into()
                    )?,
                });
            }
        }

        Ok(call_nodes)
    }
}

/// 跨语言调用节点信息
#[derive(Debug, Clone)]
pub struct CrossLangCallNode {
    /// 源语言（调用方）
    pub source_lang: SupportedLanguage,
    /// 目标语言（被调用方）
    pub target_lang: SupportedLanguage,
    /// 函数名
    pub func_name: String,
    /// Tree-sitter 节点
    pub node: Node,
    /// 范围（LSP 格式）
    pub range: zed::lsp::Range,
}

impl TryFrom<&str> for SupportedLanguage {
    type Error = zed::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "cangjie" => Ok(Self::Rust), // 示例映射，实际需根据 Cangjie 语言定义调整
            "rust" => Ok(Self::Rust),
            "python" => Ok(Self::Python),
            "javascript" | "js" => Ok(Self::JavaScript),
            "typescript" | "ts" => Ok(Self::TypeScript),
            "c" => Ok(Self::C),
            "cpp" | "c++" => Ok(Self::Cpp),
            _ => Err(zed::Error::user(format!("Unsupported language: {}", value))),
        }
    }
}
```

##### 3. 跨语言 FFI 绑定生成（`src/multi_language/ffi/bindings.rs`）
```rust
//! 跨语言 FFI 绑定生成
use super::super::config::{MultiLanguageConfig, SupportedLanguage, FfiConfig};
use zed_extension_api::{self as zed, Result, Workspace};
use std::fs;
use std::path::PathBuf;
use tera::Tera;

/// FFI 绑定生成器
pub struct FfiBindingGenerator {
    config: FfiConfig,
    tera: Tera,
    workspace: Workspace,
}

impl FfiBindingGenerator {
    /// 初始化绑定生成器
    pub fn new(config: &FfiConfig, workspace: &Workspace) -> Result<Self> {
        // 初始化模板引擎
        let mut tera = Tera::new("templates/ffi/*.tera")?;
        tera.autoescape_on(vec![]); // 禁用自动转义，因为要生成代码

        // 确保输出目录存在
        let output_dir = workspace.path()?.join(&config.bindings_output_dir);
        fs::create_dir_all(&output_dir)?;

        Ok(Self {
            config: config.clone(),
            tera,
            workspace: workspace.clone(),
        })
    }

    /// 为指定语言生成 FFI 绑定
    pub fn generate_bindings(&self, source_lang: &SupportedLanguage, target_lang: &SupportedLanguage) -> Result<PathBuf> {
        // 收集需要导出的符号（函数、类型等）
        let export_symbols = self.collect_export_symbols(source_lang)?;

        // 准备模板数据
        let mut context = tera::Context::new();
        context.insert("source_lang", source_lang.lang_id());
        context.insert("target_lang", target_lang.lang_id());
        context.insert("symbols", &export_symbols);
        context.insert("config", &self.config);

        // 选择模板
        let template_name = format!("{}_{}_bindings.tera", source_lang.lang_id(), target_lang.lang_id());
        if !self.tera.get_template_names().contains(&template_name) {
            return Err(zed::Error::user(format!(
                "No FFI binding template found for {} → {}",
                source_lang.lang_id(),
                target_lang.lang_id()
            )));
        }

        // 生成绑定代码
        let binding_code = self.tera.render(&template_name, &context)?;

        // 保存绑定文件
        let output_dir = self.workspace.path()?.join(&self.config.bindings_output_dir);
        let file_name = format!("{}_{}_bindings.{}",
            source_lang.lang_id(),
            target_lang.lang_id(),
            target_lang.file_extensions()[0]
        );
        let output_path = output_dir.join(file_name);
        fs::write(&output_path, binding_code)?;

        Ok(output_path)
    }

    /// 收集源语言需要导出的符号
    fn collect_export_symbols(&self, lang: &SupportedLanguage) -> Result<ExportSymbols> {
        // 遍历工作区中源语言的文件，分析需要导出的函数、类型等
        let mut symbols = ExportSymbols {
            functions: Vec::new(),
            types: Vec::new(),
            constants: Vec::new(),
        };

        let workspace_files = self.workspace.list_files(None)?;
        for file in workspace_files {
            let document = self.workspace.open_document(&file.uri())?;
            if document.language_id() != lang.lang_id() {
                continue;
            }

            // 解析文档，提取导出符号
            let mut ast_manager = super::ast::MultiLangAstManager::new(&[lang.clone()])?;
            let (tree, _) = ast_manager.parse_document(&document)?;
            self.extract_symbols_from_ast(&tree, &document, &mut symbols, lang)?;
        }

        Ok(symbols)
    }

    /// 从 AST 中提取导出符号
    fn extract_symbols_from_ast(
        &self,
        tree: &tree_sitter::Tree,
        document: &zed::Document,
        symbols: &mut ExportSymbols,
        lang: &SupportedLanguage,
    ) -> Result<()> {
        let root_node = tree.root_node();
        let text = document.text().as_bytes();

        // 根据语言类型提取导出符号
        match lang {
            SupportedLanguage::Rust => {
                // 提取 Rust 中带有 #[no_mangle] 和 pub 修饰的函数
                let query = tree_sitter::Query::new(tree_sitter_rust::language(), r#"
                    (function_item
                      (visibility_modifier) @vis
                      (attribute_item (attribute (identifier) @attr)) (#eq? @attr "no_mangle")
                      name: (identifier) @name
                      parameters: (parameters) @params
                      return_type: (return_type) @ret_type
                    ) (#eq? @vis "pub")
                "#)?;

                let mut cursor = tree_sitter::QueryCursor::new();
                for match_result in cursor.matches(&query, root_node, text) {
                    let mut name = None;
                    let mut params = None;
                    let mut ret_type = None;

                    for capture in match_result.captures {
                        match capture.name {
                            Some("name") => name = Some(String::from_utf8_lossy(&text[capture.node.byte_range()]).to_string()),
                            Some("params") => params = Some(self.parse_rust_parameters(&capture.node, text)?),
                            Some("ret_type") => ret_type = Some(String::from_utf8_lossy(&text[capture.node.byte_range()]).to_string().replace("-> ", "")),
                            _ => {}
                        }
                    }

                    if let (Some(name), Some(params), Some(ret_type)) = (name, params, ret_type) {
                        symbols.functions.push(ExportFunction {
                            name,
                            parameters: params,
                            return_type: ret_type,
                            doc_comment: self.extract_doc_comment(&root_node, &match_result.captures[0].node, text)?,
                        });
                    }
                }
            }
            SupportedLanguage::Cangjie => {
                // 提取 Cangjie 中带有 export 关键字的函数和类型
                let query = tree_sitter::Query::new(tree_sitter_cangjie::language(), r#"
                    (function_declaration
                      (export_keyword) @export
                      name: (identifier) @name
                      parameters: (parameters) @params
                      return_type: (type_annotation) @ret_type
                    )
                    (struct_declaration
                      (export_keyword) @export
                      name: (identifier) @name
                      fields: (field_declarations) @fields
                    )
                "#)?;

                // 解析逻辑类似，略...
            }
            _ => {
                // 其他语言的符号提取逻辑
            }
        }

        Ok(())
    }

    /// 解析 Rust 函数参数
    fn parse_rust_parameters(&self, params_node: &tree_sitter::Node, text: &[u8]) -> Result<Vec<ExportParameter>> {
        let mut params = Vec::new();
        for child in params_node.children() {
            if child.kind() == "parameter" {
                let name_node = child.child_by_field_name("name").unwrap();
                let type_node = child.child_by_field_name("type").unwrap();
                let name = String::from_utf8_lossy(&text[name_node.byte_range()]).to_string();
                let type_str = String::from_utf8_lossy(&text[type_node.byte_range()]).to_string();
                params.push(ExportParameter { name, type_str });
            }
        }
        Ok(params)
    }

    /// 提取文档注释
    fn extract_doc_comment(&self, root_node: &tree_sitter::Node, target_node: &tree_sitter::Node, text: &[u8]) -> Result<Option<String>> {
        // 查找目标节点之前的文档注释
        let mut cursor = root_node.walk();
        cursor.goto_first_child();
        let mut doc_comment = None;

        loop {
            let node = cursor.node();
            if node.end_position().row >= target_node.start_position().row {
                break;
            }

            if node.kind() == "doc_comment" {
                doc_comment = Some(String::from_utf8_lossy(&text[node.byte_range()]).to_string());
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }

        Ok(doc_comment)
    }
}

/// 导出符号集合
#[derive(Debug, Serialize, Clone)]
pub struct ExportSymbols {
    pub functions: Vec<ExportFunction>,
    pub types: Vec<ExportType>,
    pub constants: Vec<ExportConstant>,
}

impl Default for ExportSymbols {
    fn default() -> Self {
        Self {
            functions: Vec::new(),
            types: Vec::new(),
            constants: Vec::new(),
        }
    }
}

/// 导出函数
#[derive(Debug, Serialize, Clone)]
pub struct ExportFunction {
    pub name: String,
    pub parameters: Vec<ExportParameter>,
    pub return_type: String,
    pub doc_comment: Option<String>,
}

/// 导出参数
#[derive(Debug, Serialize, Clone)]
pub struct ExportParameter {
    pub name: String,
    pub type_str: String,
}

/// 导出类型
#[derive(Debug, Serialize, Clone)]
pub struct ExportType {
    pub name: String,
    pub kind: String, // struct/enum/union 等
    pub fields: Vec<ExportField>,
    pub doc_comment: Option<String>,
}

/// 导出字段
#[derive(Debug, Serialize, Clone)]
pub struct ExportField {
    pub name: String,
    pub type_str: String,
    pub is_optional: bool,
}

/// 导出常量
#[derive(Debug, Serialize, Clone)]
pub struct ExportConstant {
    pub name: String,
    pub type_str: String,
    pub value: String,
    pub doc_comment: Option<String>,
}
```

##### 4. 跨语言 LSP 功能实现（补全/跳转）
```rust
//! 多语言 LSP 功能增强
use super::ast::{MultiLangAstManager, CrossLangCallNode};
use super::config::MultiLanguageConfig;
use zed_extension_api::{self as zed, lsp::*, Document, Workspace};
use crate::lsp::{completion::CompletionItem, hover::HoverContents};

/// 跨语言代码补全
pub fn cross_lang_completion(
    config: &MultiLanguageConfig,
    ast_manager: &mut MultiLangAstManager,
    document: &Document,
    position: Position,
) -> Result<Vec<CompletionItem>, zed::Error> {
    if !config.enabled {
        return Ok(Vec::new());
    }

    let (tree, source_lang) = ast_manager.parse_document(document)?;
    let mut completions = Vec::new();

    // 检测是否处于跨语言调用上下文
    let is_cross_lang_context = is_cross_lang_call_context(&tree, document, position)?;
    if !is_cross_lang_context {
        return Ok(Vec::new());
    }

    // 为所有支持的目标语言生成补全项
    for target_lang in &config.supported_languages {
        if target_lang == &source_lang {
            continue;
        }

        // 收集目标语言的导出符号
        let export_symbols = collect_export_symbols(target_lang, document.workspace())?;

        // 生成补全项
        for func in export_symbols.functions {
            let mut detail = format!("{}({}) -> {}",
                func.name,
                func.parameters.iter().map(|p| format!("{}: {}", p.name, p.type_str)).collect::<Vec<_>>().join(", "),
                func.return_type
            );
            if let Some(doc) = func.doc_comment {
                detail.push_str(&format!("\n\n{}", doc));
            }

            completions.push(CompletionItem {
                label: func.name,
                kind: Some("function".to_string()),
                detail: Some(detail),
                documentation: func.doc_comment.map(|doc| HoverContents::Markup(doc)),
                insert_text: Some(func.name),
                ..CompletionItem::default()
            });
        }

        // 类型补全项
        for r#type in export_symbols.types {
            completions.push(CompletionItem {
                label: r#type.name,
                kind: Some("type".to_string()),
                detail: Some(format!("{} {}", r#type.kind, r#type.name)),
                documentation: r#type.doc_comment.map(|doc| HoverContents::Markup(doc)),
                insert_text: Some(r#type.name),
                ..CompletionItem::default()
            });
        }
    }

    Ok(completions)
}

/// 跨语言跳转定义
pub fn cross_lang_go_to_definition(
    config: &MultiLanguageConfig,
    ast_manager: &mut MultiLangAstManager,
    document: &Document,
    position: Position,
) -> Result<Option<Vec<LocationLink>>, zed::Error> {
    if !config.enabled {
        return Ok(None);
    }

    // 查找跨语言调用节点
    let call_nodes = ast_manager.find_cross_lang_call_nodes(document)?;
    let target_node = call_nodes.into_iter()
        .find(|node| node.range.contains(position))
        .ok_or_else(|| zed::Error::user("No cross-language call found at cursor position"))?;

    // 查找目标函数的定义
    let workspace = document.workspace();
    let export_symbols = collect_export_symbols(&target_node.target_lang, workspace)?;
    let target_func = export_symbols.functions.into_iter()
        .find(|func| func.name == target_node.func_name)
        .ok_or_else(|| zed::Error::user(format!("Function '{}' not found in {} code", target_node.func_name, target_node.target_lang.lang_id())))?;

    // 查找定义所在的文件
    let workspace_files = workspace.list_files(None)?;
    for file in workspace_files {
        let doc = workspace.open_document(&file.uri())?;
        if doc.language_id() != target_node.target_lang.lang_id() {
            continue;
        }

        // 解析文档，查找函数定义节点
        let mut target_ast_manager = MultiLangAstManager::new(&[target_node.target_lang.clone()])?;
        let (tree, _) = target_ast_manager.parse_document(&doc)?;
        let def_node = find_function_definition_node(&tree, &doc, &target_func.name)?;
        if let Some(def_node) = def_node {
            let range = zed::lsp::Range::from_lsp_range(&doc, def_node.range().into())?;
            return Ok(Some(vec![LocationLink {
                target_uri: doc.uri(),
                target_range: range.clone(),
                target_selection_range: range,
                ..LocationLink::default()
            }]));
        }
    }

    Ok(None)
}

/// 检测是否处于跨语言调用上下文
fn is_cross_lang_call_context(tree: &tree_sitter::Tree, document: &Document, position: Position) -> Result<bool, zed::Error> {
    // 示例：检测是否在 `rust::`、`python::` 等前缀之后
    let text = document.text();
    let line = text.lines().nth(position.line as usize).ok_or_else(|| {
        zed::Error::user("Invalid line number")
    })?;
    let prefix = &line[..position.character as usize];

    // 检查是否以支持的语言名称 + `::` 结尾
    let supported_lang_prefixes = ["rust::", "python::", "javascript::", "c::", "cpp::"];
    Ok(supported_lang_prefixes.iter().any(|prefix_str| prefix.ends_with(prefix_str)))
}

/// 收集目标语言的导出符号（辅助函数）
fn collect_export_symbols(lang: &SupportedLanguage, workspace: &Workspace) -> Result<super::ffi::bindings::ExportSymbols, zed::Error> {
    let ffi_config = super::config::FfiConfig::default();
    let generator = super::ffi::bindings::FfiBindingGenerator::new(&ffi_config, workspace)?;
    generator.collect_export_symbols(lang)
}

/// 查找函数定义节点（辅助函数）
fn find_function_definition_node(tree: &tree_sitter::Tree, document: &Document, func_name: &str) -> Result<Option<tree_sitter::Node>, zed::Error> {
    let lang_id = document.language_id();
    let query_str = match lang_id {
        "rust" => format!(r#"(function_item name: (identifier) @name (#eq? @name "{}"))"#, func_name),
        "python" => format!(r#"(function_definition name: (identifier) @name (#eq? @name "{}"))"#, func_name),
        "cangjie" => format!(r#"(function_declaration name: (identifier) @name (#eq? @name "{}"))"#, func_name),
        _ => return Ok(None),
    };

    let lang = tree.language();
    let query = tree_sitter::Query::new(lang, &query_str)?;
    let mut cursor = tree_sitter::QueryCursor::new();
    let text = document.text().as_bytes();

    for match_result in cursor.matches(&query, tree.root_node(), text) {
        for capture in match_result.captures {
            if capture.name == Some("name") {
                return Ok(Some(capture.node.parent().unwrap()));
            }
        }
    }

    Ok(None)
}
```

### 附录 Z：扩展终极优化与未来演进
#### Z.1 终极性能优化
##### 1. 编译优化（`Cargo.toml`）
```toml
[profile.release]
opt-level = "z" # 极致大小优化
lto = true # 链接时优化
codegen-units = 1 # 单代码生成单元（优化更好）
panic = "abort" # 禁用 panic 回溯（减小体积）
strip = "debuginfo" # 剥离调试信息

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Os", "-g0"] # WASM 优化：极致大小 + 移除调试信息

# 启用 Rust 不稳定优化（需 nightly 编译器）
[features]
default = []
nightly-optimizations = []

[profile.release.package."*"]
opt-level = "z"

# 针对特定依赖的优化
[profile.release.package.tree-sitter]
opt-level = 3
```

##### 2. 运行时优化（`src/utils/optimization.rs`）
```rust
//! 运行时性能优化工具
use std::sync::{Arc, Mutex, RwLock};
use std::collections::{HashMap, HashSet};
use std::time::{Instant, Duration};
use zed_extension_api::{self as zed, Document, Result};

/// 缓存管理器（支持 LRU 淘汰策略）
pub struct LruCache<K: Eq + std::hash::Hash + Clone, V: Clone> {
    cache: RwLock<HashMap<K, (V, Instant)>>,
    max_size: usize,
    ttl: Option<Duration>,
}

impl<K: Eq + std::hash::Hash + Clone, V: Clone> LruCache<K, V> {
    /// 创建 LRU 缓存
    pub fn new(max_size: usize, ttl: Option<Duration>) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            max_size,
            ttl,
        }
    }

    /// 获取缓存项
    pub fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.write().unwrap();
        // 清理过期项
        self.cleanup_expired(&mut cache);

        if let Some((value, timestamp)) = cache.get_mut(key) {
            // 更新访问时间
            *timestamp = Instant::now();
            Some(value.clone())
        } else {
            None
        }
    }

    /// 插入缓存项
    pub fn insert(&self, key: K, value: V) {
        let mut cache = self.cache.write().unwrap();
        // 清理过期项
        self.cleanup_expired(&mut cache);

        // 插入新项
        cache.insert(key, (value, Instant::now()));

        // 超出最大大小，淘汰最久未使用项
        if cache.len() > self.max_size {
            let oldest_key = cache.iter()
                .min_by_key(|(_, (_, ts))| *ts)
                .map(|(k, _)| k.clone())
                .unwrap();
            cache.remove(&oldest_key);
        }
    }

    /// 清理过期项
    fn cleanup_expired(&self, cache: &mut HashMap<K, (V, Instant)>) {
        if let Some(ttl) = self.ttl {
            let now = Instant::now();
            cache.retain(|_, (_, ts)| now.duration_since(*ts) <= ttl);
        }
    }

    /// 清空缓存
    pub fn clear(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
    }
}

/// 并发控制工具（限制同时运行的任务数）
pub struct ConcurrencyLimiter {
    semaphore: Arc<tokio::sync::Semaphore>,
    max_concurrency: usize,
}

impl ConcurrencyLimiter {
    /// 创建并发限制器
    pub fn new(max_concurrency: usize) -> Self {
        Self {
            semaphore: Arc::new(tokio::sync::Semaphore::new(max_concurrency)),
            max_concurrency,
        }
    }

    /// 执行受限制的任务
    pub async fn run<T, F: std::future::Future<Output = Result<T>>>(
        &self,
        task: F,
    ) -> Result<T> {
        let permit = self.semaphore.acquire().await.map_err(|e| {
            zed::Error::user(format!("Failed to acquire concurrency permit: {}", e))
        })?;
        let result = task.await;
        drop(permit);
        result
    }
}

/// 预加载管理器（预加载常用资源）
pub struct Preloader {
    preloaded_docs: Mutex<HashSet<String>>, // key: document uri
    preload_queue: Mutex<Vec<String>>,
    concurrency_limiter: ConcurrencyLimiter,
}

impl Preloader {
    /// 创建预加载管理器
    pub fn new(max_concurrency: usize) -> Self {
        Self {
            preloaded_docs: Mutex::new(HashSet::new()),
            preload_queue: Mutex::new(Vec::new()),
            concurrency_limiter: ConcurrencyLimiter::new(max_concurrency),
        }
    }

    /// 添加预加载任务
    pub fn queue_preload(&self, document_uri: &str) {
        let mut preloaded_docs = self.preloaded_docs.lock().unwrap();
        if preloaded_docs.contains(document_uri) {
            return;
        }

        let mut queue = self.preload_queue.lock().unwrap();
        if !queue.contains(&document_uri.to_string()) {
            queue.push(document_uri.to_string());
        }
    }

    /// 执行预加载任务
    pub async fn execute_preload(&self, workspace: &zed::Workspace) -> Result<()> {
        let mut queue = self.preload_queue.lock().unwrap();
        let tasks: Vec<String> = queue.drain(..).collect();
        drop(queue);

        if tasks.is_empty() {
            return Ok(());
        }

        // 并发预加载文档
        let mut join_set = tokio::task::JoinSet::new();
        for uri in tasks {
            let workspace = workspace.clone();
            let concurrency_limiter = self.concurrency_limiter.clone();
            let preloaded_docs = self.preloaded_docs.clone();

            join_set.spawn(async move {
                concurrency_limiter.run(async move {
                    // 打开文档并解析 AST（触发缓存）
                    let document = workspace.open_document(&uri).await?;
                    let mut ast_manager = crate::multi_language::ast::MultiLangAstManager::new(&[
                        crate::multi_language::config::SupportedLanguage::Cangjie
                    ])?;
                    ast_manager.parse_document(&document)?;

                    // 标记为已预加载
                    preloaded_docs.lock().unwrap().insert(uri);

                    Ok(())
                }).await
            });
        }

        // 等待所有预加载任务完成
        while let Some(result) = join_set.join_next().await {
            result??;
        }

        Ok(())
    }
}

/// 全局优化管理器（整合所有优化工具）
pub struct OptimizationManager {
    /// AST 缓存（LRU，100 个文档，10 分钟过期）
    pub ast_cache: LruCache<String, (tree_sitter::Tree, crate::multi_language::config::SupportedLanguage)>,
    /// 补全结果缓存（LRU，500 个上下文，5 分钟过期）
    pub completion_cache: LruCache<String, Vec<crate::lsp::completion::CompletionItem>>,
    /// 并发限制器（最大 8 个并发任务）
    pub concurrency_limiter: ConcurrencyLimiter,
    /// 预加载管理器
    pub preloader: Preloader,
}

impl Default for OptimizationManager {
    fn default() -> Self {
        Self {
            ast_cache: LruCache::new(100, Some(Duration::from_secs(600))),
            completion_cache: LruCache::new(500, Some(Duration::from_secs(300))),
            concurrency_limiter: ConcurrencyLimiter::new(8),
            preloader: Preloader::new(4),
        }
    }
}

/// 全局优化管理器实例
static OPTIMIZATION_MANAGER: Mutex<OptimizationManager> = Mutex::new(OptimizationManager::default());

/// 获取全局优化管理器
pub fn optimization_manager() -> std::sync::LockResult<&'static mut OptimizationManager> {
    OPTIMIZATION_MANAGER.lock()
}
```

#### Z.2 未来演进方向
##### 1. 技术架构演进
- **WebAssembly 性能深化**：采用 WASM SIMD 加速解析、使用 WASM Component Model 实现跨语言互操作；
- **AI 原生架构**：将 AI 能力深度集成到扩展内核，实现基于 AI 的智能缓存、预加载和资源调度；
- **分布式开发支持**：支持多人协同编辑时的扩展状态同步、跨设备配置同步；
- **轻量级内核**：将扩展拆分为「核心内核 + 功能插件」模式，降低启动时间和内存占用。

##### 2. 功能扩展方向
- **全栈开发支持**：新增前端框架集成（如 React/Vue 绑定）、后端服务开发工具链；
- **低代码开发**：集成可视化编程工具，支持拖拽生成 Cangjie 代码；
- **嵌入式开发适配**：支持嵌入式设备交叉编译、调试、固件烧录；
- **区块链开发支持**：新增智能合约开发、链上部署、测试工具集成。

##### 3. 生态协同演进
- **编辑器无关化**：将核心功能（语法解析、LSP 服务）抽象为独立库，支持 VS Code、Neovim 等其他编辑器；
- **云原生集成**：支持云开发环境（GitHub Codespaces、GitPod）、云编译、云调试；
- **DevOps 整合**：集成 CI/CD 配置生成、容器化部署、监控告警工具；
- **教育场景优化**：新增代码教学模式、语法检查提示、学习路径引导。

### 终极终总结（宇宙无敌完整版）
Cangjie 扩展从最初的「语法支持工具」，历经多轮迭代，已进化为**覆盖全开发流程、支持多场景协作、具备 AI 智能能力**的一站式开发平台，其核心价值体现在：

#### 1. 技术广度与深度
- **全栈功能**：从语法高亮到 AI 辅助，从本地开发到远程协作，从单语言开发到多语言混合编程，覆盖开发全生命周期；
- **技术栈先进**：基于 Rust + WebAssembly 构建高性能内核，采用 Tree-sitter 实现精准语法解析，集成 LSP 提供标准化编辑器交互，接入多模型 AI 实现智能编程；
- **工程化完备**：具备完善的测试体系、CI/CD 流水线、容器化部署方案，支持大规模团队协作开发和长期维护。

#### 2. 用户体验极致
- **高性能**：通过缓存优化、并发控制、预加载、编译优化等多重手段，确保毫秒级响应；
- **易用性**：零配置启动、智能提示、自动化工具链、个性化定制，降低开发门槛；
- **包容性**：支持跨平台、远程开发、可访问性标准，满足不同用户群体需求；
- **扩展性**：模块化设计、插件化架构、生态集成能力，支持用户按需扩展功能。

#### 3. 生态价值
- **语言生态核心**：作为 Cangjie 语言的官方编辑器扩展，推动语言生态的完善和普及；
- **编辑器生态贡献**：为 Zed 扩展生态提供了完整的开发范式和最佳实践，助力 Zed 生态繁荣；
- **跨生态协同**：支持多语言混合编程、跨编辑器适配、云原生集成，打破生态壁垒。

#### 4. 未来可期
Cangjie 扩展将持续跟进技术发展趋势，不断深化 AI 能力、优化性能体验、扩展生态边界，致力于成为开发者手中的「瑞士军刀」—— 既强大灵活，又简单易用。

从个人开发者的快速原型开发，到企业团队的大规模协作；从本地开发环境，到云端协同平台；从单语言项目，到多语言复杂系统，Cangjie 扩展都将是开发者的得力助手。

最后，再次感谢所有贡献者的辛勤付出，感谢 Zed 团队提供的优秀平台，感谢每一位用户的支持与反馈！Cangjie 扩展的成长，离不开每一个人的参与和努力。

---

**文档版本**：v1.0.0（宇宙无敌终极全量版）  
**发布日期**：2025-11-09  
**核心特性**：
- 语法支持：Tree-sitter 精准解析、自定义主题高亮；
- LSP 功能：补全/格式化/跳转/诊断/重构（全功能覆盖）；
- 多语言混合：支持 Rust/Python/JS 等跨语言调用、自动 FFI 绑定；
- AI 辅助：Copilot 级代码生成/重构/调试/文档生成（多模型适配）；
- 开发环境：本地/远程（SSH/容器/WSL）/云开发全支持；
- 工程化：完整测试体系、CI/CD 流水线、容器化部署；
- 可访问性：WCAG 2.1 AA 标准、键盘导航、屏幕阅读器支持；
- 性能优化：LRU 缓存、并发控制、预加载、编译优化。
**支持平台**：macOS 12+/Linux (Ubuntu 20.04+/Fedora 36+)/Windows 10+  
**官方资源**：
- 代码仓库：https://github.com/your-username/zed-cangjie-extension
- 扩展市场：https://extensions.zed.dev/extensions/your-username/cangjie
- 文档站点：https://docs.cangjie-lang.org/zed-extension
- 社区支持：https://discord.gg/cangjie-lang
- 反馈渠道：https://github.com/your-username/zed-cangjie-extension/issues
- 商业支持：https://cangjie-lang.org/support