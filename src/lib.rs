use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use zed_extension_api as zed;
use zed_extension_api::{
    serde, serde_json, Command, CompletionItem, CompletionItemKind, CompletionParams,
    DebugAdapterBinary, DebugConfig, DebugRequest, DebugScenario, DebugTaskDefinition, Document,
    DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, LanguageServerId,
    LspRequestHandler, Position, Range, SlashCommand, SlashCommandArgumentCompletion,
    SlashCommandOutput, SymbolKind, TextEdit, Worktree, WorktreeId,
};

// -------------------------- 扩展元信息（严格遵循 Zed ExtensionMeta 规范）--------------------------
pub const EXTENSION_META: zed::ExtensionMeta = zed::ExtensionMeta {
    name: "cangjie-official",
    display_name: "Cangjie Official Support",
    description:
        "仓颉编程语言官方 Zed 扩展：基于最新官方文档规范，支持语法补全、LSP、调试、格式化、代码大纲",
    version: "1.0.4", // 适配官方文档最新迭代
    author: "Cangjie Official & Zed Community",
    repository: "https://gitcode.com/Cangjie/cangjie-zed-extension",
    license: "Apache-2.0",
    categories: &["languages", "debuggers", "build-tools", "package-managers"],
    keywords: &["cangjie", "仓颉", "official", "lsp", "debug", "zed"],
};

// -------------------------- 扩展配置（与 Zed Config 深度集成）--------------------------
#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct CangjieConfig {
    /// 仓颉 SDK 路径（覆盖 CANGJIE_HOME 环境变量）
    #[serde(default)]
    pub sdk_path: Option<PathBuf>,

    /// LSP 相关配置（参考官方文档 LSP 章节）
    #[serde(default)]
    pub lsp: LspConfig,

    /// 代码格式化配置（参考官方文档格式化规范）
    #[serde(default)]
    pub formatting: FormattingConfig,

    /// 调试配置（参考官方文档调试章节）
    #[serde(default)]
    pub debug: DebugConfig,
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct LspConfig {
    #[serde(default = "default_lsp_log_level")]
    pub log_level: String,

    #[serde(default = "default_lsp_auto_import")]
    pub auto_import: bool,
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct FormattingConfig {
    #[serde(default = "default_indent_size")]
    pub indent_size: u32,

    #[serde(default = "default_line_width")]
    pub line_width: u32,

    #[serde(default = "default_use_tabs")]
    pub use_tabs: bool,

    #[serde(default = "default_newline_style")]
    pub newline_style: NewlineStyle,
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NewlineStyle {
    Lf,
    Crlf,
    Cr,
}

impl Default for NewlineStyle {
    fn default() -> Self {
        Self::Lf
    }
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct DebugConfig {
    #[serde(default = "default_track_goroutines")]
    pub track_goroutines: bool,

    #[serde(default = "default_show_async_stack")]
    pub show_async_stack: bool,

    #[serde(default = "default_debug_info_level")]
    pub debug_info_level: DebugInfoLevel,
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DebugInfoLevel {
    Full,
    Limited,
    None,
}

impl Default for DebugInfoLevel {
    fn default() -> Self {
        Self::Full
    }
}

// 配置默认值（基于官方文档推荐配置）
fn default_lsp_log_level() -> String {
    "info".into()
}
fn default_lsp_auto_import() -> bool {
    true
}
fn default_indent_size() -> u32 {
    4
}
fn default_line_width() -> u32 {
    120
}
fn default_use_tabs() -> bool {
    false
}
fn default_track_goroutines() -> bool {
    true
}
fn default_show_async_stack() -> bool {
    true
}

// -------------------------- 仓颉语言核心语法（完全基于官方文档）--------------------------
mod cangjie_syntax {
    use super::*;

    /// 关键字（参考：cangjie_docs/dev/grammar/keywords.md）
    pub const KEYWORDS: &[&str] = &[
        "module",
        "import",
        "export",
        "func",
        "struct",
        "enum",
        "class",
        "interface",
        "concurrent",
        "async",
        "await",
        "defer",
        "panic",
        "recover",
        "return",
        "if",
        "else",
        "for",
        "while",
        "match",
        "break",
        "continue",
        "const",
        "let",
        "var",
        "static",
        "pub",
        "priv",
        "unsafe",
        "inline",
        "noinline",
        "true",
        "false",
        "nil",
        "as",
        "is",
        "in",
        "from",
        "to",
        "try",
        "catch",
        "finally",
        "throw",
    ];

    /// 基础类型（参考：cangjie_docs/dev/grammar/types/primitive_types.md）
    pub const PRIMITIVE_TYPES: &[&str] = &[
        "bool", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
        "char", "string", "bytes", "any", "void", "usize", "isize",
    ];

    /// 复合类型（参考：cangjie_docs/dev/grammar/types/compound_types.md）
    pub const COMPOUND_TYPES: &[&str] = &[
        "Array", "Slice", "Map", "Tuple", "Option", "Result", "Future", "Stream",
    ];

    /// 标准库模块（参考：cangjie_docs/dev/std_lib/README.md）
    pub const STDLIB_MODULES: &[&str] = &[
        "std::io",
        "std::fs",
        "std::net",
        "std::json",
        "std::log",
        "std::crypto",
        "std::time",
        "std::collections",
        "std::sync",
        "std::concurrent",
        "std::math",
        "std::errors",
        "std::strings",
        "std::fmt",
        "std::reflect",
        "std::testing",
        "std::encoding",
        "std::datetime",
        "std::os",
        "std::path",
    ];

    /// 官方装饰器（参考：cangjie_docs/dev/grammar/decorators.md）
    pub const DECORATORS: &[&str] = &[
        "@test",
        "@benchmark",
        "@deprecated",
        "@inline",
        "@noinline",
        "@unsafe",
        "@optimize",
        "@profile",
        "@serializable",
        "@deserializable",
        "@warn",
        "@ignore",
        "@cfg",
        "@feature",
    ];

    /// 常用函数（参考：cangjie_docs/dev/std_lib/common_functions.md）
    pub const COMMON_FUNCTIONS: &[&str] = &[
        "print",
        "println",
        "eprint",
        "eprintln",
        "panic",
        "recover",
        "len",
        "cap",
        "push",
        "pop",
        "get",
        "set",
        "delete",
        "contains",
        "range",
        "map",
        "filter",
        "reduce",
        "any",
        "all",
        "find",
        "find_index",
    ];
}

// -------------------------- 工作区状态管理（支持 Zed 多工作区）--------------------------
#[derive(Default)]
struct WorktreeState {
    /// 包配置（cj.toml，参考：cangjie_docs/dev/package_management/cj_toml.md）
    package_config: Option<CangjiePackageConfig>,

    /// 依赖缓存
    dependencies: HashSet<String>,

    /// 模块缓存
    modules: HashSet<String>,
}

/// 仓颉包配置（严格对齐官方 cj.toml 规范：cangjie_docs/dev/package_management/cj_toml.md）
#[derive(Debug, serde::Deserialize, Default, Clone)]
struct CangjiePackageConfig {
    package: Option<PackageMeta>,
    dependencies: Option<HashMap<String, String>>,
    dev_dependencies: Option<HashMap<String, String>>,
    build: Option<BuildConfig>,
    module: Option<ModuleConfig>,
    features: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
struct PackageMeta {
    name: String,
    version: String,
    authors: Option<Vec<String>>,
    description: Option<String>,
    edition: String, // 必需字段（参考官方文档）
    repository: Option<String>,
    license: Option<String>,
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
struct BuildConfig {
    target: Option<String>,
    opt_level: Option<String>,
    debug: Option<bool>,
    lto: Option<bool>,
    strip: Option<bool>,
    features: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
struct ModuleConfig {
    name: Option<String>,
    export: Option<Vec<String>>,
    path: Option<String>,
    public: Option<bool>,
}

// -------------------------- 扩展核心结构体（实现 Zed Extension 接口）--------------------------
#[derive(Default)]
pub struct CangjieExtension {
    /// 工作区状态（按 WorktreeId 隔离）
    worktree_states: HashMap<WorktreeId, WorktreeState>,

    /// 全局配置（支持热更新）
    config: Arc<CangjieConfig>,

    /// LSP 请求处理器
    lsp_handler: Option<Arc<dyn LspRequestHandler>>,
}

// -------------------------- Zed Extension 接口实现（严格遵循 API 规范）--------------------------
impl zed::Extension for CangjieExtension {
    /// 返回扩展元信息（Zed 必需接口）
    fn meta(&self) -> &zed::ExtensionMeta {
        &EXTENSION_META
    }

    /// 初始化扩展（Zed 启动时调用）
    fn initialize(
        &mut self,
        _host: Arc<dyn zed::ExtensionHost>,
        config: zed::Config,
    ) -> zed::Result<()> {
        // 从 Zed 配置加载仓颉专属配置
        self.config = Arc::new(config.get("cangjie").unwrap_or_default());
        zed::log::info!("仓颉扩展初始化完成，配置: {:?}", self.config);
        Ok(())
    }

    /// 更新配置（Zed 配置修改时热更新）
    fn update_config(&mut self, config: zed::Config) -> zed::Result<()> {
        self.config = Arc::new(config.get("cangjie").unwrap_or_default());
        zed::log::info!("仓颉扩展配置已更新");
        Ok(())
    }

    /// 销毁扩展（Zed 退出时清理资源）
    fn destroy(&mut self) -> zed::Result<()> {
        self.worktree_states.clear();
        self.lsp_handler.take();
        zed::log::info!("仓颉扩展已销毁");
        Ok(())
    }

    /// 注册 LSP 请求处理器（Zed LSP 集成必需）
    fn register_lsp_request_handler(
        &mut self,
        handler: Arc<dyn LspRequestHandler>,
    ) -> zed::Result<()> {
        self.lsp_handler = Some(handler);
        Ok(())
    }

    // -------------------------- LSP 集成（基于官方 LSP 规范）--------------------------
    fn language_server_command(
        &mut self,
        ls_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Command> {
        if ls_id.as_ref() != "cangjie-language-server" {
            return Err(zed::Error::InvalidRequest(format!(
                "不支持的 LSP ID: {}",
                ls_id
            )));
        }

        // 加载工作区状态
        let state = self.get_or_create_worktree_state(worktree.id());
        self.load_package_config(worktree, state)?;

        // 查找 LSP 二进制文件（优先级：配置 > CANGJIE_HOME > PATH）
        let lsp_path = self.find_official_binary("cj-lsp")?;
        let lsp_path_str = lsp_path
            .to_str()
            .ok_or_else(|| zed::Error::InvalidPath("LSP 路径包含非 UTF-8 字符".into()))?;

        // 构建 LSP 启动参数（参考官方 LSP 文档）
        let mut args = vec![
            "--stdio".into(),
            "--log-level".into(),
            self.config.lsp.log_level.clone(),
            "--project-root".into(),
            worktree
                .path()
                .to_str()
                .ok_or_else(|| zed::Error::InvalidPath("项目路径包含非 UTF-8 字符".into()))?
                .into(),
            "--lang-version".into(),
            state
                .package_config
                .as_ref()
                .and_then(|pkg| pkg.package.as_ref())
                .map(|meta| meta.edition.clone())
                .unwrap_or_else(|| "1.0.4".into()),
        ];

        // 启用自动导入（基于配置）
        if self.config.lsp.auto_import {
            args.push("--enable-auto-import".into());
        }

        Ok(Command {
            command: lsp_path_str.into(),
            args,
            env: HashMap::new(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        ls_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        if ls_id.as_ref() != "cangjie-language-server" {
            return Ok(None);
        }

        let state = self.get_or_create_worktree_state(worktree.id());
        let package_config = self.load_package_config(worktree, state)?;

        // 格式化配置映射（与官方 fmt 工具对齐）
        let newline_style = match self.config.formatting.newline_style {
            NewlineStyle::Lf => "lf",
            NewlineStyle::Crlf => "crlf",
            NewlineStyle::Cr => "cr",
        };

        Ok(Some(serde_json::json!({
            "languageVersion": package_config.package.as_ref().map(|meta| meta.edition.clone()).unwrap_or("1.0.4".into()),
            "packageConfig": package_config,
            "formatting": {
                "indentSize": self.config.formatting.indent_size,
                "lineWidth": self.config.formatting.line_width,
                "useTabs": self.config.formatting.use_tabs,
                "newlineStyle": newline_style,
            },
            "features": {
                "autoImport": self.config.lsp.auto_import,
                "asyncAwait": true,
                "concurrent": true,
                "decorators": true,
                "errorDiagnostics": true,
            }
        })))
    }

    // -------------------------- 调试集成（基于官方 DAP 规范）--------------------------
    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        _task: DebugTaskDefinition,
        user_provided_path: Option<String>,
        _worktree: &Worktree,
    ) -> zed::Result<DebugAdapterBinary> {
        if adapter_name != "cangjie-dap" {
            return Err(zed::Error::InvalidRequest(format!(
                "不支持的调试适配器: {}",
                adapter_name
            )));
        }

        // 查找调试适配器路径
        let dap_path = match user_provided_path {
            Some(path) => PathBuf::from(path),
            None => self.find_official_binary("cj-debug")?,
        };

        let dap_path_str = dap_path
            .to_str()
            .ok_or_else(|| zed::Error::InvalidPath("调试适配器路径包含非 UTF-8 字符".into()))?;

        // 调试适配器参数（参考官方调试文档）
        let debug_info_level = match self.config.debug.debug_info_level {
            DebugInfoLevel::Full => "full",
            DebugInfoLevel::Limited => "limited",
            DebugInfoLevel::None => "none",
        };

        let args = vec![
            "--log-level".into(),
            "info".into(),
            "--lang-version".into(),
            "1.0.4".into(),
            "--track-goroutines".into(),
            self.config.debug.track_goroutines.to_string(),
            "--show-async-stack".into(),
            self.config.debug.show_async_stack.to_string(),
            "--debug-info-level".into(),
            debug_info_level.into(),
        ];

        Ok(DebugAdapterBinary {
            path: dap_path_str.into(),
            args,
            env: HashMap::new(),
        })
    }

    fn dap_config_to_scenario(&mut self, config: DebugConfig) -> zed::Result<DebugScenario> {
        // 必选字段校验（参考官方调试配置规范）
        let program = config
            .get("program")
            .and_then(|v| v.as_str())
            .ok_or_else(|| zed::Error::InvalidConfig("调试配置缺少必需字段 `program`".into()))?;

        // 可选字段处理
        let args = config
            .get("args")
            .unwrap_or(&serde_json::Value::Array(vec![]))
            .clone();
        let cwd = config
            .get("cwd")
            .and_then(|v| v.as_str())
            .unwrap_or("${workspaceFolder}");
        let stop_on_entry = config
            .get("stopOnEntry")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let target = config
            .get("target")
            .and_then(|v| v.as_str())
            .unwrap_or("native");

        Ok(DebugScenario {
            adapter_name: "cangjie-dap".into(),
            request: DebugRequest::Launch(serde_json::json!({
                "program": program,
                "args": args,
                "cwd": cwd,
                "stopOnEntry": stop_on_entry,
                "target": target,
                "trackGoroutines": self.config.debug.track_goroutines,
                "showAsyncStack": self.config.debug.show_async_stack,
                "debugInfoLevel": match self.config.debug.debug_info_level {
                    DebugInfoLevel::Full => "full",
                    DebugInfoLevel::Limited => "limited",
                    DebugInfoLevel::None => "none",
                },
            })),
            source_file_map: HashMap::new(),
        })
    }

    // -------------------------- 代码补全（基于官方语法文档）--------------------------
    fn completions(
        &self,
        params: &CompletionParams,
        document: &Document,
        _worktree: &Worktree,
    ) -> zed::Result<Vec<CompletionItem>> {
        // 仅对仓颉文件提供补全
        if !self.is_cangjie_file(document.path()) {
            return Ok(vec![]);
        }

        let cursor = params.position;
        let line = document.line(cursor.line as usize)?;
        let prefix = self.extract_completion_prefix(line, cursor.character as usize);
        let mut completions = Vec::new();

        // 1. 关键字补全
        for kw in cangjie_syntax::KEYWORDS {
            if kw.starts_with(&prefix) {
                completions.push(self.create_completion_item(
                    kw,
                    CompletionItemKind::Keyword,
                    "仓颉关键字",
                    format!("官方语法关键字：{}", kw),
                    &prefix,
                    cursor,
                ));
            }
        }

        // 2. 基础类型补全
        for ty in cangjie_syntax::PRIMITIVE_TYPES {
            if ty.starts_with(&prefix) {
                completions.push(self.create_completion_item(
                    ty,
                    CompletionItemKind::Type,
                    "仓颉基础类型",
                    format!("官方基础类型：{}", ty),
                    &prefix,
                    cursor,
                ));
            }
        }

        // 3. 复合类型补全
        for ty in cangjie_syntax::COMPOUND_TYPES {
            if ty.starts_with(&prefix) {
                completions.push(self.create_completion_item(
                    ty,
                    CompletionItemKind::Type,
                    "仓颉复合类型",
                    format!("官方复合类型：{}", ty),
                    &prefix,
                    cursor,
                ));
            }
        }

        // 4. 标准库模块补全
        for module in cangjie_syntax::STDLIB_MODULES {
            if module.starts_with(&prefix) {
                completions.push(self.create_completion_item(
                    module,
                    CompletionItemKind::Module,
                    "仓颉标准库模块",
                    format!("官方标准库模块：{}", module),
                    &prefix,
                    cursor,
                ));
            }
        }

        // 5. 装饰器补全
        for decorator in cangjie_syntax::DECORATORS {
            if decorator.starts_with(&prefix) {
                completions.push(self.create_completion_item(
                    decorator,
                    CompletionItemKind::Decorator,
                    "仓颉装饰器",
                    format!("官方装饰器：{}", decorator),
                    &prefix,
                    cursor,
                ));
            }
        }

        // 6. 常用函数补全
        for func in cangjie_syntax::COMMON_FUNCTIONS {
            if func.starts_with(&prefix) {
                completions.push(self.create_completion_item(
                    func,
                    CompletionItemKind::Function,
                    "仓颉常用函数",
                    format!("官方常用函数：{}", func),
                    &prefix,
                    cursor,
                ));
            }
        }

        Ok(completions)
    }

    // -------------------------- 文档符号（代码大纲，基于官方语法）--------------------------
    fn document_symbols(
        &self,
        _params: &DocumentSymbolParams,
        document: &Document,
        _worktree: &Worktree,
    ) -> zed::Result<Option<DocumentSymbolResponse>> {
        if !self.is_cangjie_file(document.path()) {
            return Ok(None);
        }

        let mut symbols = Vec::new();
        let text = document.text();

        // 1. 提取模块定义（module xxx;）
        let module_pattern = regex::Regex::new(r#"^\s*module\s+([\w::]+)\s*;"#)?;
        for (line_idx, line) in text.lines().enumerate() {
            if let Some(capture) = module_pattern.captures(line) {
                let name = capture[1].to_string();
                symbols.push(DocumentSymbol {
                    name: name.clone(),
                    kind: SymbolKind::Module,
                    range: self.line_range(document, line_idx)?,
                    selection_range: self.symbol_selection_range(line, &name)?,
                    children: None,
                    detail: Some("仓颉模块".to_string()),
                    deprecated: false,
                });
            }
        }

        // 2. 提取函数定义（func/async/concurrent func）
        let func_pattern = regex::Regex::new(r#"^\s*(concurrent|async)?\s*func\s+(\w+)\s*\(""#)?;
        for (line_idx, line) in text.lines().enumerate() {
            if let Some(capture) = func_pattern.captures(line) {
                let func_type = capture[1].unwrap_or("普通");
                let name = capture[2].to_string();
                symbols.push(DocumentSymbol {
                    name: name.clone(),
                    kind: SymbolKind::Function,
                    range: self.line_range(document, line_idx)?,
                    selection_range: self.symbol_selection_range(line, &name)?,
                    children: None,
                    detail: Some(format!("仓颉{}函数", func_type)),
                    deprecated: false,
                });
            }
        }

        // 3. 提取结构体定义（struct xxx { ... }）
        let struct_pattern = regex::Regex::new(r#"^\s*struct\s+(\w+)\s*\{"#)?;
        for (line_idx, line) in text.lines().enumerate() {
            if let Some(capture) = struct_pattern.captures(line) {
                let name = capture[1].to_string();
                symbols.push(DocumentSymbol {
                    name: name.clone(),
                    kind: SymbolKind::Struct,
                    range: self.line_range(document, line_idx)?,
                    selection_range: self.symbol_selection_range(line, &name)?,
                    children: None,
                    detail: Some("仓颉结构体".to_string()),
                    deprecated: false,
                });
            }
        }

        // 4. 提取接口定义（interface xxx { ... }）
        let interface_pattern = regex::Regex::new(r#"^\s*interface\s+(\w+)\s*\{"#)?;
        for (line_idx, line) in text.lines().enumerate() {
            if let Some(capture) = interface_pattern.captures(line) {
                let name = capture[1].to_string();
                symbols.push(DocumentSymbol {
                    name: name.clone(),
                    kind: SymbolKind::Interface,
                    range: self.line_range(document, line_idx)?,
                    selection_range: self.symbol_selection_range(line, &name)?,
                    children: None,
                    detail: Some("仓颉接口".to_string()),
                    deprecated: false,
                });
            }
        }

        // 5. 提取枚举定义（enum xxx { ... }）
        let enum_pattern = regex::Regex::new(r#"^\s*enum\s+(\w+)\s*\{"#)?;
        for (line_idx, line) in text.lines().enumerate() {
            if let Some(capture) = enum_pattern.captures(line) {
                let name = capture[1].to_string();
                symbols.push(DocumentSymbol {
                    name: name.clone(),
                    kind: SymbolKind::Enum,
                    range: self.line_range(document, line_idx)?,
                    selection_range: self.symbol_selection_range(line, &name)?,
                    children: None,
                    detail: Some("仓颉枚举".to_string()),
                    deprecated: false,
                });
            }
        }

        Ok(Some(DocumentSymbolResponse::Symbols(symbols)))
    }

    // -------------------------- 代码格式化（基于官方 fmt 工具）--------------------------
    fn format_document(
        &self,
        document: &Document,
        _worktree: &Worktree,
    ) -> zed::Result<Option<Vec<TextEdit>>> {
        if !self.is_cangjie_file(document.path()) {
            return Ok(None);
        }

        // 查找官方格式化工具 cj
        let cj_path = self.find_official_binary("cj")?;
        let mut cmd = std::process::Command::new(cj_path);

        // 构建格式化参数（参考官方 fmt 文档）
        let mut args = vec![
            "fmt",
            "--stdin",
            "--indent",
            &self.config.formatting.indent_size.to_string(),
            "--line-width",
            &self.config.formatting.line_width.to_string(),
            "--lang-version",
            "1.0.4",
        ];

        if self.config.formatting.use_tabs {
            args.push("--tabs");
        }

        match self.config.formatting.newline_style {
            NewlineStyle::Crlf => args.push("--newline-style=crlf"),
            NewlineStyle::Cr => args.push("--newline-style=cr"),
            _ => {}
        }

        // 执行格式化
        let mut child = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| zed::Error::ExecutionFailed(format!("启动格式化工具失败: {}", e)))?;

        child
            .stdin
            .as_mut()
            .ok_or_else(|| zed::Error::ExecutionFailed("无法获取格式化工具标准输入".into()))?
            .write_all(document.text().as_bytes())?;

        let output = child.wait_with_output()?;
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(zed::Error::ExecutionFailed(format!(
                "格式化失败: {}",
                err.trim()
            )));
        }

        let formatted_text = String::from_utf8(output.stdout)?;
        if formatted_text == document.text() {
            return Ok(None);
        }

        // 生成全文替换的 TextEdit
        let full_range = Range::new(
            Position::new(0, 0),
            Position::new(
                document.line_count() as u32,
                document.line(document.line_count() - 1)?.len() as u32,
            ),
        );

        Ok(Some(vec![TextEdit {
            range: full_range,
            new_text: formatted_text,
        }]))
    }

    // -------------------------- Slash 命令（基于官方工具链文档）--------------------------
    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: &Worktree,
    ) -> zed::Result<SlashCommandOutput> {
        let cj_path = self.find_official_binary("cj")?;
        let cj_str = cj_path
            .to_str()
            .ok_or_else(|| zed::Error::InvalidPath("cj 工具路径包含非 UTF-8 字符".into()))?;

        // 映射 Slash 命令到官方 cj 命令（参考：cangjie_docs/dev/toolchain/cj.md）
        let (cmd_args, desc) = match command.as_str() {
            "cangjie: build" => (vec!["build"], "构建项目"),
            "cangjie: run" => (vec!["run"], "运行项目"),
            "cangjie: test" => (vec!["test"], "运行测试"),
            "cangjie: clean" => (vec!["clean"], "清理构建产物"),
            "cangjie: pkg: add" => (vec!["pkg", "add"], "添加依赖"),
            "cangjie: pkg: remove" => (vec!["pkg", "remove"], "移除依赖"),
            "cangjie: pkg: update" => (vec!["pkg", "update"], "更新依赖"),
            "cangjie: pkg: list" => (vec!["pkg", "list"], "列出依赖"),
            "cangjie: mod: create" => (vec!["mod", "create"], "创建模块"),
            "cangjie: mod: export" => (vec!["mod", "export"], "导出模块"),
            "cangjie: check" => (vec!["check"], "静态检查代码"),
            "cangjie: fmt" => (vec!["fmt"], "格式化项目代码"),
            "cangjie: doc" => (vec!["doc"], "生成项目文档"),
            "cangjie: new" => (vec!["new"], "创建新项目"),
            "cangjie: check-env" => return self.check_env(),
            _ => {
                return Err(zed::Error::InvalidRequest(format!(
                    "不支持的命令: {}",
                    command
                )));
            }
        };

        // 构建完整命令
        let mut full_cmd = vec![cj_str];
        full_cmd.extend(cmd_args);
        full_cmd.extend(args.iter().map(|s| s.as_str()));

        // 执行命令
        let output = std::process::Command::new(full_cmd[0])
            .args(&full_cmd[1..])
            .current_dir(worktree.path())
            .output()
            .map_err(|e| zed::Error::ExecutionFailed(format!("执行 {} 命令失败: {}", desc, e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(SlashCommandOutput::Message(format!(
                "✅ 仓颉 {} 成功:\n{}",
                desc,
                stdout.trim()
            )))
        } else {
            Err(zed::Error::ExecutionFailed(format!(
                "❌ 仓颉 {} 失败（退出码: {}）:\nstdout: {}\nstderr: {}",
                desc,
                output.status.code().unwrap_or(-1),
                stdout.trim(),
                stderr.trim()
            )))
        }
    }

    /// Slash 命令参数补全
    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: &Worktree,
    ) -> zed::Result<Vec<SlashCommandArgumentCompletion>> {
        let arg_index = args.len();
        match command.as_str() {
            "cangjie: build" => self.complete_build_args(arg_index),
            "cangjie: test" => self.complete_test_args(arg_index),
            "cangjie: pkg: add" => self.complete_pkg_add_args(arg_index),
            "cangjie: mod: create" => self.complete_mod_create_args(arg_index),
            "cangjie: fmt" => self.complete_fmt_args(arg_index),
            "cangjie: new" => self.complete_new_args(arg_index),
            _ => Ok(vec![]),
        }
    }
}

// -------------------------- 辅助方法（封装通用逻辑）--------------------------
impl CangjieExtension {
    /// 判断是否为仓颉文件（基于官方支持的后缀）
    fn is_cangjie_file(&self, path: &Path) -> bool {
        let exts = &["cj", "cj.h", "cjmod", "cjtest", "cjh"];
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| exts.contains(&ext))
            .unwrap_or(false)
    }

    /// 获取或创建工作区状态
    fn get_or_create_worktree_state(&mut self, worktree_id: WorktreeId) -> &mut WorktreeState {
        self.worktree_states.entry(worktree_id).or_default()
    }

    /// 加载工作区包配置（cj.toml）
    fn load_package_config(
        &mut self,
        worktree: &Worktree,
        state: &mut WorktreeState,
    ) -> zed::Result<&CangjiePackageConfig> {
        if state.package_config.is_some() {
            return Ok(state.package_config.as_ref().unwrap());
        }

        let cj_toml_path = worktree.path().join("cj.toml");
        if !cj_toml_path.exists() {
            return Err(zed::Error::NotFound(format!(
                "未找到仓颉包配置文件: {}（参考：https://gitcode.com/Cangjie/cangjie_docs/blob/dev/package_management/cj_toml.md）",
                cj_toml_path.display()
            )));
        }

        let content = std::fs::read_to_string(&cj_toml_path)
            .map_err(|e| zed::Error::IoError(format!("读取 cj.toml 失败: {}", e)))?;

        let config: CangjiePackageConfig = toml::from_str(&content)
            .map_err(|e| zed::Error::InvalidConfig(format!("cj.toml 格式错误: {}", e)))?;

        // 验证必需字段（edition）
        if config
            .package
            .as_ref()
            .map(|meta| meta.edition.is_empty())
            .unwrap_or(true)
        {
            return Err(zed::Error::InvalidConfig(
                "cj.toml 的 package.edition 字段为必需项（参考官方文档）".into(),
            ));
        }

        // 缓存依赖和模块
        state.dependencies.clear();
        if let Some(deps) = &config.dependencies {
            state.dependencies.extend(deps.keys().cloned());
        }
        if let Some(dev_deps) = &config.dev_dependencies {
            state.dependencies.extend(dev_deps.keys().cloned());
        }

        state.modules.clear();
        if let Some(module) = &config.module {
            if let Some(imports) = &module.export {
                state.modules.extend(imports.iter().cloned());
            }
        }

        state.package_config = Some(config);
        Ok(state.package_config.as_ref().unwrap())
    }

    /// 查找官方工具路径（cj/cj-lsp/cj-debug）
    fn find_official_binary(&self, binary_name: &str) -> zed::Result<PathBuf> {
        let binary_name = if cfg!(windows) {
            format!("{}.exe", binary_name)
        } else {
            binary_name.to_string()
        };

        // 1. 从配置的 sdk_path 查找
        if let Some(sdk_path) = &self.config.sdk_path {
            let binary_path = sdk_path.join("bin").join(&binary_name);
            if binary_path.exists() {
                return Ok(binary_path);
            }
        }

        // 2. 从 CANGJIE_HOME 查找
        if let Ok(cangjie_home) = std::env::var("CANGJIE_HOME") {
            let binary_path = PathBuf::from(cangjie_home).join("bin").join(&binary_name);
            if binary_path.exists() {
                return Ok(binary_path);
            }
        }

        // 3. 从 PATH 查找
        which::which(&binary_name)
            .map_err(|e| zed::Error::NotFound(format!(
                "未找到 {} 工具（请安装仓颉 SDK 并配置环境变量，参考：https://gitcode.com/Cangjie/cangjie_docs/blob/dev/installation.md）: {}",
                binary_name, e
            )))
    }

    /// 提取补全前缀
    fn extract_completion_prefix(&self, line: &str, cursor_col: usize) -> String {
        line[0..cursor_col]
            .chars()
            .rev()
            .take_while(|c| c.is_alphanumeric() || *c == ':' || *c == '@' || *c == '_')
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    /// 创建补全项
    fn create_completion_item(
        &self,
        label: &str,
        kind: CompletionItemKind,
        detail: &str,
        documentation: String,
        prefix: &str,
        cursor: Position,
    ) -> CompletionItem {
        CompletionItem {
            label: label.to_string(),
            kind,
            detail: Some(detail.to_string()),
            documentation: Some(zed::Documentation::String(documentation)),
            insert_text: Some(label.to_string()),
            range: Range::new(
                Position::new(
                    cursor.line,
                    (cursor.character as usize - prefix.len()) as u32,
                ),
                Position::new(cursor.line, cursor.character),
            ),
            sort_text: Some(label.to_string()),
            filter_text: Some(label.to_string()),
            ..Default::default()
        }
    }

    /// 计算行范围（用于文档符号）
    fn line_range(&self, document: &Document, line_idx: usize) -> zed::Result<Range> {
        let line = document.line(line_idx)?;
        Ok(Range::new(
            Position::new(line_idx as u32, 0),
            Position::new(line_idx as u32, line.len() as u32),
        ))
    }

    /// 计算符号选择范围（仅选中符号名称）
    fn symbol_selection_range(&self, line: &str, symbol_name: &str) -> zed::Result<Range> {
        let start_col = line
            .find(symbol_name)
            .ok_or_else(|| zed::Error::NotFound(format!("符号 {} 未找到", symbol_name)))?
            as u32;
        let end_col = start_col + symbol_name.len() as u32;
        Ok(Range::new(
            Position::new(0, start_col),
            Position::new(0, end_col),
        ))
    }

    /// 环境检查
    fn check_env(&self) -> zed::Result<SlashCommandOutput> {
        let mut checks = Vec::new();

        // 检查 cj 工具
        match self.find_official_binary("cj") {
            Ok(path) => checks.push(format!("✅ cj 工具: {}", path.display())),
            Err(e) => checks.push(format!("❌ cj 工具: {}", e)),
        }

        // 检查 LSP
        match self.find_official_binary("cj-lsp") {
            Ok(path) => checks.push(format!("✅ LSP 服务器: {}", path.display())),
            Err(e) => checks.push(format!("❌ LSP 服务器: {}", e)),
        }

        // 检查调试适配器
        match self.find_official_binary("cj-debug") {
            Ok(path) => checks.push(format!("✅ 调试适配器: {}", path.display())),
            Err(e) => checks.push(format!("❌ 调试适配器: {}", e)),
        }

        // 检查 CANGJIE_HOME
        if let Ok(home) = std::env::var("CANGJIE_HOME") {
            checks.push(format!("✅ CANGJIE_HOME: {}", home));
        } else {
            checks.push("⚠️  未配置 CANGJIE_HOME 环境变量（参考官方安装文档）".to_string());
        }

        Ok(SlashCommandOutput::Message(format!(
            "仓颉环境检查结果:\n{}",
            checks.join("\n")
        )))
    }

    // -------------------------- 命令补全辅助方法 --------------------------
    fn complete_build_args(
        &self,
        arg_index: usize,
    ) -> zed::Result<Vec<SlashCommandArgumentCompletion>> {
        match arg_index {
            1 => Ok(vec![
                self.create_slash_completion("--target", "指定构建目标架构"),
                self.create_slash_completion("--opt-level", "指定优化级别（0/1/2/3/s/z）"),
                self.create_slash_completion("--debug", "生成调试信息"),
                self.create_slash_completion("--release", "发布模式构建"),
                self.create_slash_completion("--lto", "启用 LTO 优化"),
                self.create_slash_completion("--strip", "剥离符号信息"),
                self.create_slash_completion("--features", "启用指定特性"),
            ]),
            2 => {
                let last_arg = self.get_last_arg(&[], arg_index);
                if last_arg == Some("--target".to_string()) {
                    Ok(vec![
                        self.create_slash_completion("native", "本地架构（默认）"),
                        self.create_slash_completion("ohos-aarch64", "鸿蒙 ARM64"),
                        self.create_slash_completion("windows-x86_64", "Windows X86_64"),
                        self.create_slash_completion("linux-x86_64", "Linux X86_64"),
                        self.create_slash_completion("macos-aarch64", "macOS ARM64"),
                    ])
                } else {
                    Ok(vec![])
                }
            }
            _ => Ok(vec![]),
        }
    }

    fn complete_test_args(
        &self,
        arg_index: usize,
    ) -> zed::Result<Vec<SlashCommandArgumentCompletion>> {
        match arg_index {
            1 => Ok(vec![
                self.create_slash_completion("--filter", "过滤测试用例"),
                self.create_slash_completion("--bench", "运行基准测试"),
                self.create_slash_completion("--verbose", "详细输出模式"),
                self.create_slash_completion("--no-capture", "不捕获输出"),
                self.create_slash_completion("--jobs", "并行测试任务数"),
            ]),
            _ => Ok(vec![]),
        }
    }

    fn complete_pkg_add_args(
        &self,
        arg_index: usize,
    ) -> zed::Result<Vec<SlashCommandArgumentCompletion>> {
        if arg_index == 1 {
            Ok(cangjie_syntax::STDLIB_MODULES
                .iter()
                .map(|m| self.create_slash_completion(m, format!("标准库模块: {}", m)))
                .collect())
        } else {
            Ok(vec![self.create_slash_completion("--dev", "添加为开发依赖")])
        }
    }

    fn complete_mod_create_args(
        &self,
        arg_index: usize,
    ) -> zed::Result<Vec<SlashCommandArgumentCompletion>> {
        if arg_index == 1 {
            Ok(vec![
                self.create_slash_completion("utils", "工具模块"),
                self.create_slash_completion("network", "网络模块"),
                self.create_slash_completion("storage", "存储模块"),
                self.create_slash_completion("concurrent", "并发模块"),
                self.create_slash_completion("async", "异步模块"),
                self.create_slash_completion("api", "API 模块"),
            ])
        } else {
            Ok(vec![
                self.create_slash_completion("--path", "指定模块路径"),
                self.create_slash_completion("--public", "设置为公共模块"),
            ])
        }
    }

    fn complete_fmt_args(
        &self,
        arg_index: usize,
    ) -> zed::Result<Vec<SlashCommandArgumentCompletion>> {
        match arg_index {
            1 => Ok(vec![
                self.create_slash_completion("--check", "仅检查格式，不修改文件"),
                self.create_slash_completion("--fix", "自动修复格式问题（默认）"),
                self.create_slash_completion("--indent", "指定缩进长度"),
                self.create_slash_completion("--line-width", "指定行宽限制"),
                self.create_slash_completion("--tabs", "使用制表符缩进"),
                self.create_slash_completion("--newline-style", "指定换行符风格"),
            ]),
            2 => {
                let last_arg = self.get_last_arg(&[], arg_index);
                match last_arg.as_deref() {
                    Some("--newline-style") => Ok(vec![
                        self.create_slash_completion("lf", "LF（默认，Unix 风格）"),
                        self.create_slash_completion("crlf", "CRLF（Windows 风格）"),
                        self.create_slash_completion("cr", "CR（老式 Mac 风格）"),
                    ]),
                    _ => Ok(vec![]),
                }
            }
            _ => Ok(vec![]),
        }
    }

    fn complete_new_args(
        &self,
        arg_index: usize,
    ) -> zed::Result<Vec<SlashCommandArgumentCompletion>> {
        if arg_index == 1 {
            Ok(vec![
                self.create_slash_completion("console", "控制台应用（默认）"),
                self.create_slash_completion("web", "Web 应用"),
                self.create_slash_completion("api", "API 服务"),
                self.create_slash_completion("library", "库项目"),
                self.create_slash_completion("cli", "命令行工具"),
            ])
        } else {
            Ok(vec![
                self.create_slash_completion("--name", "指定项目名称"),
                self.create_slash_completion("--edition", "指定语言版本（默认 1.0.4）"),
                self.create_slash_completion("--path", "指定项目路径"),
                self.create_slash_completion("--git", "初始化 Git 仓库"),
            ])
        }
    }

    /// 创建 Slash 命令补全项
    fn create_slash_completion(&self, label: &str, detail: &str) -> SlashCommandArgumentCompletion {
        SlashCommandArgumentCompletion {
            label: label.to_string(),
            detail: Some(detail.to_string()),
            insert_text: Some(label.to_string()),
        }
    }

    /// 获取上一个参数
    fn get_last_arg(&self, args: &[String], arg_index: usize) -> Option<String> {
        arg_index.checked_sub(1).and_then(|i| args.get(i).cloned())
    }
}

// -------------------------- 注册扩展（符合 Zed 规范）--------------------------
zed::register_extension!(CangjieExtension);
