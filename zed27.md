### 附录 Q：扩展兼容性适配指南

#### Q.1 Zed 版本兼容性处理
Zed 编辑器处于快速迭代阶段，扩展需适配不同版本的 API 差异，避免因 API 变更导致功能失效。

##### 1. API 版本检测
```rust
//! src/utils/zed_compatibility.rs
use zed_extension_api::{self as zed, Result};

/// Zed API 版本信息
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ZedVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl ZedVersion {
    /// 解析版本字符串（如 "0.135.0"）
    pub fn parse(version_str: &str) -> Result<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() < 2 {
            return Err(zed::Error::user(format!("Invalid Zed version: {}", version_str)));
        }

        Ok(Self {
            major: parts[0].parse()?,
            minor: parts[1].parse()?,
            patch: if parts.len() >= 3 { parts[2].parse()? } else { 0 },
        })
    }

    /// 获取当前 Zed 版本
    pub fn current() -> Result<Self> {
        let version_str = zed::system::zed_version()?;
        Self::parse(&version_str)
    }

    /// 检查是否支持某一特性（基于版本号）
    pub fn supports_feature(&self, feature_min_version: &str) -> Result<bool> {
        let min_version = Self::parse(feature_min_version)?;
        Ok(self >= &min_version)
    }
}

/// 特性版本映射（记录各功能所需的最低 Zed 版本）
pub const FEATURE_VERSIONS: &[(&str, &str)] = &[
    ("range_formatting", "0.130.0"),
    ("custom_notifications", "0.132.0"),
    ("output_panel_api", "0.133.0"),
    ("semantic_tokens", "0.135.0"),
    ("language_client_custom_requests", "0.131.0"),
];

/// 检查是否支持指定特性
pub fn supports_feature(feature_name: &str) -> Result<bool> {
    let current_version = ZedVersion::current()?;
    let min_version = FEATURE_VERSIONS
        .iter()
        .find(|(name, _)| *name == feature_name)
        .map(|(_, version)| version)
        .ok_or_else(|| zed::Error::user(format!("Unknown feature: {}", feature_name)))?;
    
    current_version.supports_feature(min_version)
}
```

##### 2. 兼容性代码示例（以语义令牌为例）
```rust
//! src/lsp/semantic_tokens.rs
use super::super::utils::zed_compatibility::{supports_feature, ZedVersion};
use zed_extension_api::{self as zed, Result, lsp::SemanticTokensParams};

pub fn provide_semantic_tokens(params: &SemanticTokensParams) -> Result<Option<zed::lsp::SemanticTokensResult>> {
    // 检查当前 Zed 版本是否支持语义令牌功能
    let supports_semantic_tokens = supports_feature("semantic_tokens")?;
    if !supports_semantic_tokens {
        crate::utils::log::warn!(
            "Semantic tokens not supported by current Zed version. Minimum required: 0.135.0"
        );
        return Ok(None);
    }

    // 兼容处理：0.135.0 与 0.136.0+ 的 API 差异
    let current_version = ZedVersion::current()?;
    if current_version < &ZedVersion::parse("0.136.0")? {
        // 适配旧版本 API（如使用简化的令牌类型）
        provide_semantic_tokens_v0_135(params)
    } else {
        // 新版本 API（支持完整语义令牌类型）
        provide_semantic_tokens_v0_136(params)
    }
}

/// 适配 Zed 0.135.0 语义令牌 API
fn provide_semantic_tokens_v0_135(params: &SemanticTokensParams) -> Result<Option<zed::lsp::SemanticTokensResult>> {
    // 旧版本实现逻辑...
}

/// 适配 Zed 0.136.0+ 语义令牌 API
fn provide_semantic_tokens_v0_136(params: &SemanticTokensParams) -> Result<Option<zed::lsp::SemanticTokensResult>> {
    // 新版本实现逻辑...
}
```

#### Q.2 跨平台兼容性处理
Zed 支持 macOS、Linux、Windows 三大平台，扩展需处理平台差异（文件路径、外部命令、系统 API 等）。

##### 1. 平台检测工具
```rust
//! src/utils/platform.rs
use std::env;

/// 操作系统类型
#[derive(Debug, Clone, PartialEq)]
pub enum OS {
    MacOS,
    Linux,
    Windows,
    Unknown,
}

impl OS {
    /// 获取当前操作系统
    pub fn current() -> Self {
        #[cfg(target_os = "macos")]
        return Self::MacOS;
        #[cfg(target_os = "linux")]
        return Self::Linux;
        #[cfg(target_os = "windows")]
        return Self::Windows;
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        return Self::Unknown;
    }

    /// 获取系统默认的换行符
    pub fn line_ending(&self) -> &'static str {
        match self {
            Self::Windows => "\r\n",
            _ => "\n",
        }
    }

    /// 获取系统默认的工具路径分隔符
    pub fn path_separator(&self) -> &'static str {
        match self {
            Self::Windows => ";",
            _ => ":",
        }
    }

    /// 检查是否为类 Unix 系统（macOS/Linux）
    pub fn is_unix(&self) -> bool {
        matches!(self, Self::MacOS | Self::Linux)
    }

    /// 检查是否为 Windows 系统
    pub fn is_windows(&self) -> bool {
        matches!(self, Self::Windows)
    }
}

/// 获取系统特定的工具可执行文件名（如 "cangjiec" -> "cangjiec.exe" on Windows）
pub fn get_executable_name(name: &str) -> String {
    if OS::current().is_windows() {
        format!("{}.exe", name)
    } else {
        name.to_string()
    }
}

/// 规范化文件路径（处理 Windows 反斜杠）
pub fn normalize_path(path: &str) -> String {
    if OS::current().is_windows() {
        path.replace('\\', "/")
    } else {
        path.to_string()
    }
}
```

##### 2. 跨平台外部命令调用示例
```rust
//! src/utils/tool_exec.rs
use super::platform::{OS, get_executable_name};
use std::process::{Command, Stdio};
use zed_extension_api::{self as zed, Result};

pub fn execute_tool(tool_name: &str, args: &[&str]) -> Result<(String, String)> {
    let os = OS::current();
    let tool_path = find_tool_path(tool_name)?;
    
    // 跨平台命令执行配置
    let mut command = Command::new(tool_path);
    command.args(args);

    // Windows 平台特殊配置（隐藏命令行窗口）
    if os.is_windows() {
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    // 捕获输出
    let output = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok((stdout, stderr))
}

/// 跨平台查找工具路径
fn find_tool_path(tool_name: &str) -> Result<String> {
    let executable_name = get_executable_name(tool_name);
    let os = OS::current();

    // 1. 检查配置中的路径
    let config = crate::config::load_config()?;
    if let Some(path) = &config.tools.compiler_path {
        return Ok(path.clone());
    }

    // 2. 检查系统 PATH
    if let Ok(path) = which::which(&executable_name) {
        return Ok(path.to_string_lossy().into_owned());
    }

    // 3. 检查平台特定的默认路径
    let default_paths = match os {
        OS::MacOS => vec![
            "/usr/bin/",
            "/usr/local/bin/",
            "~/Library/Application Support/Cangjie/bin/",
        ],
        OS::Linux => vec![
            "/usr/bin/",
            "/usr/local/bin/",
            "~/.cargo/bin/",
            "~/bin/",
        ],
        OS::Windows => vec![
            "C:\\Program Files\\Cangjie\\bin\\",
            "C:\\Program Files (x86)\\Cangjie\\bin\\",
            "%APPDATA%\\Cangjie\\bin\\",
        ],
        OS::Unknown => vec![],
    };

    for path in default_paths {
        let expanded_path = shellexpand::tilde(path).into_owned();
        let full_path = std::path::Path::new(&expanded_path).join(&executable_name);
        if full_path.exists() {
            return Ok(full_path.to_string_lossy().into_owned());
        }
    }

    Err(zed::Error::user(format!(
        "{} not found. Please install it or specify the path in config.",
        tool_name
    )))
}
```

### 附录 R：扩展监控与诊断工具

#### R.1 扩展运行监控
为便于排查线上问题，扩展可集成运行监控功能，记录关键操作日志、性能指标和错误信息。

##### 1. 监控数据结构
```rust
//! src/monitoring/mod.rs
use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize};
use zed_extension_api::{self as zed, Result};

/// 监控事件类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MonitorEventType {
    /// 功能调用（如补全、格式化）
    FunctionCall { name: String, duration_ms: u64 },
    /// 错误发生
    Error { message: String, stack_trace: Option<String>, context: String },
    /// 配置变更
    ConfigChange { key: String, old_value: String, new_value: String },
    /// 扩展启动/关闭
    Lifecycle { event: LifecycleEvent },
}

/// 生命周期事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LifecycleEvent {
    Startup,
    Shutdown,
    Activate,
    Deactivate,
}

/// 监控记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonitorRecord {
    timestamp: u64, // 毫秒级时间戳
    event_type: MonitorEventType,
    zed_version: String,
    extension_version: String,
    os: String,
    session_id: String,
}

/// 监控管理器
pub struct MonitorManager {
    session_id: String,
    zed_version: String,
    extension_version: String,
    os: String,
    enabled: bool,
    log_file: Option<std::fs::File>,
}

impl MonitorManager {
    /// 初始化监控管理器
    pub fn init() -> Result<Self> {
        let config = crate::config::load_config()?;
        let enabled = config.monitoring.enabled;

        // 生成唯一会话 ID
        let session_id = uuid::Uuid::new_v4().to_string();
        let zed_version = zed::system::zed_version()?;
        let extension_version = env!("CARGO_PKG_VERSION").to_string();
        let os = match zed::system::os()? {
            zed::system::OS::MacOS => "macos",
            zed::system::OS::Linux => "linux",
            zed::system::OS::Windows => "windows",
            zed::system::OS::Unknown => "unknown",
        }.to_string();

        // 打开日志文件（如启用监控）
        let log_file = if enabled {
            let log_dir = zed::system::config_dir()?.join("zed/extensions/cangjie/monitoring");
            std::fs::create_dir_all(&log_dir)?;
            let log_path = log_dir.join(format!("session-{}.log", session_id));
            Some(std::fs::File::create(log_path)?)
        } else {
            None
        };

        let mut manager = Self {
            session_id,
            zed_version,
            extension_version,
            os,
            enabled,
            log_file,
        };

        // 记录启动事件
        manager.record_event(MonitorEventType::Lifecycle {
            event: LifecycleEvent::Startup,
        })?;

        Ok(manager)
    }

    /// 记录监控事件
    pub fn record_event(&mut self, event_type: MonitorEventType) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let record = MonitorRecord {
            timestamp: Instant::now().duration_since(Instant::unix_epoch()).as_millis() as u64,
            event_type,
            zed_version: self.zed_version.clone(),
            extension_version: self.extension_version.clone(),
            os: self.os.clone(),
            session_id: self.session_id.clone(),
        };

        // 写入日志文件（JSON 格式，每行一条）
        let log_line = serde_json::to_string(&record)? + "\n";
        if let Some(file) = &mut self.log_file {
            use std::io::Write;
            file.write_all(log_line.as_bytes())?;
            file.flush()?;
        }

        Ok(())
    }

    /// 记录功能调用耗时
    pub fn record_function_call<T, F: FnOnce() -> Result<T>>(
        &mut self,
        function_name: &str,
        func: F,
    ) -> Result<T> {
        let start = Instant::now();
        let result = func();
        let duration = start.elapsed().as_millis() as u64;

        self.record_event(MonitorEventType::FunctionCall {
            name: function_name.to_string(),
            duration_ms: duration,
        })?;

        result
    }

    /// 记录错误
    pub fn record_error(&mut self, error: &dyn std::error::Error, context: &str) -> Result<()> {
        self.record_event(MonitorEventType::Error {
            message: error.to_string(),
            stack_trace: std::backtrace::Backtrace::capture().to_string().into(),
            context: context.to_string(),
        })
    }
}

/// 全局监控管理器实例
static MONITOR_MANAGER: std::sync::Mutex<Option<MonitorManager>> = std::sync::Mutex::new(None);

/// 初始化监控
pub fn init_monitoring() -> Result<()> {
    let manager = MonitorManager::init()?;
    *MONITOR_MANAGER.lock()? = Some(manager);
    Ok(())
}

/// 记录功能调用耗时（简化接口）
pub fn monitor_function<T, F: FnOnce() -> Result<T>>(function_name: &str, func: F) -> Result<T> {
    if let Some(manager) = &mut *MONITOR_MANAGER.lock()? {
        manager.record_function_call(function_name, func)
    } else {
        func()
    }
}

/// 记录错误（简化接口）
pub fn monitor_error(error: &dyn std::error::Error, context: &str) -> Result<()> {
    if let Some(manager) = &mut *MONITOR_MANAGER.lock()? {
        manager.record_error(error, context)
    } else {
        Ok(())
    }
}
```

##### 2. 监控功能使用示例
```rust
//! src/lsp/formatting.rs
use super::super::monitoring::{monitor_function, monitor_error};

pub fn format_document(
    document: &zed::Document,
    tree: &tree_sitter::Tree,
    options: &zed::lsp::FormattingOptions,
) -> Result<Vec<zed::lsp::TextEdit>> {
    // 使用监控包装功能调用，自动记录耗时
    monitor_function("format_document", move || {
        let content = document.text();
        let config = crate::config::load_config()?;

        // 格式化逻辑
        let formatted_content = match crate::formatter::format(
            &content,
            &tree,
            &config.formatting,
            options,
        ) {
            Ok(content) => content,
            Err(e) => {
                // 记录错误信息
                monitor_error(&e, &format!("Formatting document: {}", document.uri()))?;
                return Err(e);
            }
        };

        // 生成文本编辑
        let edit = zed::lsp::TextEdit {
            range: zed::lsp::Range {
                start: zed::lsp::Position::default(),
                end: zed::lsp::Position {
                    line: content.lines().count() as u32,
                    character: 0,
                },
            },
            new_text: formatted_content,
        };

        Ok(vec![edit])
    })
}
```

#### R.2 诊断工具集成
扩展可集成自诊断工具，帮助用户快速排查常见问题。

##### 1. 自诊断命令实现
```rust
//! src/extension.rs
pub fn activate() -> Result<(), zed::Error> {
    // ... 其他初始化 ...

    // 注册自诊断命令
    zed::commands::register_command(
        "cangjie.runDiagnostics",
        "Cangjie: Run Self-Diagnostic",
        |_context: CommandContext| async move {
            let mut results = Vec::new();

            // 1. 检查扩展版本
            results.push(diagnose_extension_version()?);

            // 2. 检查 Zed 版本兼容性
            results.push(diagnose_zed_compatibility()?);

            // 3. 检查外部工具
            results.push(diagnose_external_tools()?);

            // 4. 检查语法解析
            results.push(diagnose_syntax_parsing()?);

            // 5. 检查 LSP 连接
            results.push(diagnose_lsp_connection()?);

            // 生成诊断报告
            let report = generate_diagnostic_report(&results);

            // 显示报告（输出面板 + 弹窗）
            zed::workspace::current().show_output_panel("Cangjie Diagnostic Report", &report)?;
            zed::workspace::current().show_info_message("Diagnostic completed. See output panel for details.")?;

            Ok(())
        },
    )?;

    Ok(())
}

/// 诊断结果类型
#[derive(Debug, Clone)]
enum DiagnosticResult {
    Pass(String),
    Warn(String),
    Fail(String),
}

/// 检查扩展版本
fn diagnose_extension_version() -> Result<DiagnosticResult> {
    let version = env!("CARGO_PKG_VERSION");
    Ok(DiagnosticResult::Pass(format!(
        "Extension version: {}",
        version
    )))
}

/// 检查 Zed 版本兼容性
fn diagnose_zed_compatibility() -> Result<DiagnosticResult> {
    let zed_version = zed::system::zed_version()?;
    let min_version = "0.130.0";
    let is_compatible = ZedVersion::parse(&zed_version)? >= ZedVersion::parse(min_version)?;

    if is_compatible {
        Ok(DiagnosticResult::Pass(format!(
            "Zed version compatible ({} >= {})",
            zed_version, min_version
        )))
    } else {
        Ok(DiagnosticResult::Fail(format!(
            "Zed version incompatible: {} < {} (minimum required)",
            zed_version, min_version
        )))
    }
}

/// 检查外部工具
fn diagnose_external_tools() -> Result<DiagnosticResult> {
    let config = crate::config::load_config()?;
    let compiler_name = get_executable_name("cangjiec");

    match find_tool_path("cangjiec") {
        Ok(path) => Ok(DiagnosticResult::Pass(format!(
            "Cangjie compiler found: {}",
            path
        ))),
        Err(e) => {
            if config.tools.auto_detect {
                Ok(DiagnosticResult::Warn(format!(
                    "Cangjie compiler not found (auto-detect enabled): {}",
                    e
                )))
            } else {
                Ok(DiagnosticResult::Fail(format!(
                    "Cangjie compiler not found (auto-detect disabled): {}",
                    e
                )))
            }
        }
    }
}

/// 检查语法解析
fn diagnose_syntax_parsing() -> Result<DiagnosticResult> {
    let test_code = "fn test() -> bool { true }";
    match tree_sitter_cangjie::parse(test_code, None) {
        Ok(tree) => {
            if tree.root_node().has_error() {
                Ok(DiagnosticResult::Fail(
                    "Syntax parsing failed for test code".to_string()
                ))
            } else {
                Ok(DiagnosticResult::Pass(
                    "Syntax parsing works correctly".to_string()
                ))
            }
        }
        Err(e) => Ok(DiagnosticResult::Fail(format!(
            "Syntax parsing error: {}",
            e
        ))),
    }
}

/// 检查 LSP 连接
fn diagnose_lsp_connection() -> Result<DiagnosticResult> {
    let workspace = zed::workspace::current();
    let document = workspace.create_document("diagnostic-test.cang", "".to_string(), false)?;

    match zed::language_client::get(&document) {
        Ok(_client) => Ok(DiagnosticResult::Pass(
            "LSP connection established successfully".to_string()
        )),
        Err(e) => Ok(DiagnosticResult::Fail(format!(
            "LSP connection failed: {}",
            e
        ))),
    }
}

/// 生成诊断报告
fn generate_diagnostic_report(results: &[DiagnosticResult]) -> String {
    let mut report = format!(
        "Cangjie Extension Diagnostic Report\n{}",
        "-".repeat(50)
    );

    let mut pass_count = 0;
    let mut warn_count = 0;
    let mut fail_count = 0;

    for (i, result) in results.iter().enumerate() {
        match result {
            DiagnosticResult::Pass(msg) => {
                pass_count += 1;
                report.push_str(&format!("\n{}: ✅ {}", i + 1, msg));
            }
            DiagnosticResult::Warn(msg) => {
                warn_count += 1;
                report.push_str(&format!("\n{}: ⚠️  {}", i + 1, msg));
            }
            DiagnosticResult::Fail(msg) => {
                fail_count += 1;
                report.push_str(&format!("\n{}: ❌ {}", i + 1, msg));
            }
        }
    }

    report.push_str(&format!(
        "\n{}",
        "-".repeat(50)
    ));
    report.push_str(&format!(
        "\nSummary: {} Passed, {} Warnings, {} Failed",
        pass_count, warn_count, fail_count
    ));

    if fail_count > 0 {
        report.push_str(&format!(
            "\n\nRecommendations:\n1. Check the extension documentation for troubleshooting\n2. Update Zed to the latest version\n3. Verify external tool installation\n4. Submit an issue with this report if the problem persists"
        ));
    }

    report
}
```

### 附录 S：扩展生态集成指南

#### S.1 与 Cangjie 语言生态集成
Cangjie 扩展可与 Cangjie 语言的其他工具（如包管理器、文档生成器）集成，提供更完整的开发体验。

##### 1. 包管理器集成（示例：Cangjie Package Manager）
```rust
//! src/lsp/package_management.rs
use zed_extension_api::{self as zed, Result, lsp::Url};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

/// 包信息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    name: String,
    version: String,
    description: String,
    dependencies: Vec<PackageDependency>,
}

/// 包依赖结构
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDependency {
    name: String,
    version: String,
    optional: bool,
}

/// 包配置文件（cangjie.toml）结构
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageConfig {
    package: PackageInfo,
    dependencies: Option<Vec<PackageDependency>>,
    dev_dependencies: Option<Vec<PackageDependency>>,
}

/// 加载包配置
pub fn load_package_config(workspace: &zed::Workspace) -> Result<Option<PackageConfig>> {
    let config_path = workspace.path()?.join("cangjie.toml");
    if !config_path.exists() {
        return Ok(None);
    }

    let config_content = std::fs::read_to_string(&config_path)?;
    let package_config: PackageConfig = toml::from_str(&config_content)?;

    Ok(Some(package_config))
}

/// 安装依赖包
pub async fn install_dependencies(workspace: &zed::Workspace) -> Result<()> {
    let config = load_package_config(workspace)?;
    let dependencies = match &config {
        Some(config) => config.package.dependencies.clone(),
        None => return Err(zed::Error::user("No cangjie.toml found in workspace root")),
    };

    if dependencies.is_empty() {
        workspace.show_info_message("No dependencies to install")?;
        return Ok(());
    }

    // 调用 Cangjie 包管理器安装依赖
    workspace.show_status_message("Installing dependencies...")?;
    let (stdout, stderr) = super::super::utils::tool_exec::execute_tool(
        "cpm", // Cangjie Package Manager 可执行文件
        &["install", "--quiet"],
    )?;

    if !stderr.is_empty() {
        return Err(zed::Error::user(format!(
            "Failed to install dependencies:\n{}",
            stderr
        )));
    }

    workspace.show_info_message(&format!(
        "Successfully installed {} dependencies",
        dependencies.len()
    ))?;
    Ok(())
}

/// 注册包管理命令
pub fn register_package_commands() -> Result<()> {
    // 安装依赖命令
    zed::commands::register_command(
        "cangjie.installDependencies",
        "Cangjie: Install Dependencies",
        |_context: zed::commands::CommandContext| async move {
            let workspace = zed::workspace::current();
            install_dependencies(&workspace).await?;
            Ok(())
        },
    )?;

    // 更新依赖命令
    zed::commands::register_command(
        "cangjie.updateDependencies",
        "Cangjie: Update Dependencies",
        |_context: zed::commands::CommandContext| async move {
            let workspace = zed::workspace::current();
            let (stdout, stderr) = super::super::utils::tool_exec::execute_tool(
                "cpm",
                &["update", "--quiet"],
            )?;

            if !stderr.is_empty() {
                return Err(zed::Error::user(format!(
                    "Failed to update dependencies:\n{}",
                    stderr
                )));
            }

            workspace.show_info_message("Dependencies updated successfully")?;
            Ok(())
        },
    )?;

    Ok(())
}
```

#### S.2 与 Zed 生态其他扩展集成
Cangjie 扩展可与 Zed 生态中的其他扩展（如调试器、Git 工具、AI 辅助编程）集成，扩展功能边界。

##### 1. 与 AI 辅助编程扩展集成
```rust
//! src/integration/ai_assistant.rs
use zed_extension_api::{self as zed, Result};
use serde::{Serialize, Deserialize};

/// AI 辅助编程请求结构（遵循 Zed AI 扩展通用协议）
#[derive(Debug, Serialize, Deserialize)]
pub struct AiCompletionRequest {
    pub prompt: String,
    pub language: String,
    pub context: Option<String>,
    pub max_tokens: Option<u32>,
}

/// AI 辅助编程响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct AiCompletionResponse {
    pub content: String,
    pub model: String,
    pub finish_reason: String,
}

/// 调用 AI 扩展生成 Cangjie 代码
pub async fn generate_cangjie_code(prompt: &str, context: Option<&str>) -> Result<String> {
    // 检查 AI 扩展是否已安装
    let ai_extension_id = "zed-industries.ai-assistant";
    if !zed::extensions::is_installed(ai_extension_id).await? {
        return Err(zed::Error::user(format!(
            "AI Assistant extension not found. Please install it from the extension marketplace."
        )));
    }

    // 构造请求
    let request = AiCompletionRequest {
        prompt: prompt.to_string(),
        language: "cangjie".to_string(),
        context: context.map(|s| s.to_string()),
        max_tokens: Some(500),
    };

    // 发送跨扩展消息
    let response: AiCompletionResponse = zed::extensions::send_message(
        ai_extension_id,
        "ai/generateCode",
        &request,
    ).await?;

    Ok(response.content)
}

/// 注册 AI 辅助命令
pub fn register_ai_commands() -> Result<()> {
    // AI 生成代码命令
    zed::commands::register_command(
        "cangjie.ai.generateCode",
        "Cangjie: AI Generate Code",
        |context: zed::commands::CommandContext| async move {
            let document = context.document.ok_or_else(|| {
                zed::Error::user("No active document")
            })?;

            // 显示输入框获取用户提示
            let prompt = zed::ui::show_input_box(
                "Enter code generation prompt",
                "e.g. Create a function to validate email addresses",
            ).await?
            .ok_or_else(|| zed::Error::user("Prompt cancelled"))?;

            // 获取当前文档上下文（选中的文本或光标周围内容）
            let context = match document.selection() {
                Some(selection) => Some(document.text_in_range(&selection)?),
                None => {
                    // 获取光标前后 5 行作为上下文
                    let cursor_pos = document.cursor_position()?;
                    let start_line = cursor_pos.line.saturating_sub(5);
                    let end_line = cursor_pos.line.saturating_add(5);
                    let range = zed::lsp::Range {
                        start: zed::lsp::Position { line: start_line, character: 0 },
                        end: zed::lsp::Position { line: end_line, character: 0 },
                    };
                    Some(document.text_in_range(&range)?)
                }
            };

            // 调用 AI 生成代码
            document.workspace()?.show_status_message("Generating code with AI...")?;
            let generated_code = generate_cangjie_code(&prompt, context.as_deref()).await?;

            // 将生成的代码插入到文档中
            let cursor_pos = document.cursor_position()?;
            document.insert_text(cursor_pos, &generated_code).await?;

            document.workspace()?.show_info_message("AI code generation completed")?;
            Ok(())
        },
    )?;

    // AI 解释代码命令
    zed::commands::register_command(
        "cangjie.ai.explainCode",
        "Cangjie: AI Explain Code",
        |context: zed::commands::CommandContext| async move {
            // 类似实现：获取选中代码，调用 AI 生成解释，显示在输出面板或悬停提示中
            Ok(())
        },
    )?;

    Ok(())
}
```

### 最终总结（完整版）
Cangjie 扩展作为 Zed 编辑器生态中功能完备的编程语言扩展，从**基础语法支持**到**高级生态集成**，从**开发调试**到**发布维护**，形成了完整的闭环。本指南涵盖：

1. **核心功能实现**：语法高亮、LSP 全流程（补全、格式化、跳转定义等）、代码 linting、配置系统；
2. **进阶技术方案**：性能优化、跨平台适配、国际化、安全最佳实践；
3. **工具链支持**：测试方案（单元/集成/E2E）、开发辅助脚本、监控诊断工具；
4. **生态扩展能力**：外部工具集成、跨扩展通信、语言生态联动；
5. **工程化实践**：发布流程、版本管理、社区维护、兼容性处理。

扩展的设计遵循「模块化、可扩展、高性能、用户友好」四大原则，代码结构清晰，文档详尽，便于开发者快速上手和社区贡献。

无论是 Cangjie 语言的使用者，还是 Zed 扩展的开发者，都能从本指南中获取实用的知识和代码示例。未来，扩展将持续跟进 Zed 编辑器和 Cangjie 语言的发展，不断完善功能，提升用户体验。

感谢所有贡献者的付出，感谢 Zed 团队提供的强大扩展平台，也感谢每一位用户的支持与反馈！

---

**文档版本**：v1.0.0（终极完整版）  
**最后更新**：2025-11-09  
**维护团队**：Cangjie 语言开发团队  
**官方仓库**：https://github.com/your-username/zed-cangjie-extension  
**扩展市场**：https://extensions.zed.dev/extensions/your-username/cangjie  
**支持渠道**：GitHub Issues / Discord 社区 / 邮件反馈（contact@cangjie-lang.org）