### 附录 K：高级扩展技巧

#### K.1 自定义 LSP 消息实现
Zed 扩展 API 支持自定义 LSP 消息，用于实现非标准 LSP 功能（如与外部工具通信、自定义代码操作等）。

##### 实现步骤：
1. **定义自定义消息结构体**（`src/lsp/custom_messages.rs`）
```rust
//! 自定义 LSP 消息
use serde::{Serialize, Deserialize};
use zed_extension_api::{lsp::Url, Result};

/// 自定义请求：运行 Cangjie 代码
#[derive(Debug, Serialize, Deserialize)]
pub struct RunCangjieCodeParams {
    /// 文档 URI
    pub text_document_uri: Url,
    /// 是否显示输出
    pub show_output: bool,
    /// 运行参数
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunCangjieCodeResponse {
    /// 退出码
    pub exit_code: i32,
    /// 标准输出
    pub stdout: String,
    /// 标准错误
    pub stderr: String,
}

/// 自定义通知：代码执行进度
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeExecutionProgressNotification {
    /// 文档 URI
    pub text_document_uri: Url,
    /// 进度百分比（0-100）
    pub progress: u8,
    /// 进度信息
    pub message: String,
}
```

2. **注册自定义消息**（`src/lsp/server.rs`）
```rust
impl zed::LanguageServer for CangjieLspServer {
    // ... 其他方法 ...

    /// 处理自定义请求
    fn custom_request(
        &mut self,
        method: &str,
        params: &serde_json::Value,
    ) -> Result<Option<serde_json::Value>> {
        match method {
            "cangjie/runCode" => {
                let params: RunCangjieCodeParams = serde_json::from_value(params.clone())?;
                let result = self.run_cangjie_code(params)?;
                Ok(Some(serde_json::to_value(result)?))
            }
            _ => Ok(None),
        }
    }

    /// 处理自定义通知
    fn custom_notification(
        &mut self,
        method: &str,
        params: &serde_json::Value,
    ) -> Result<()> {
        match method {
            "cangjie/executionProgress" => {
                let progress: CodeExecutionProgressNotification = serde_json::from_value(params.clone())?;
                self.handle_execution_progress(progress)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl CangjieLspServer {
    /// 运行 Cangjie 代码
    fn run_cangjie_code(&mut self, params: RunCangjieCodeParams) -> Result<RunCangjieCodeResponse> {
        let document = self.workspace.document(&params.text_document_uri)?;
        let content = document.text();
        let path = document.path()?;

        // 发送进度通知（开始）
        self.send_custom_notification(
            "cangjie/executionProgress",
            &CodeExecutionProgressNotification {
                text_document_uri: params.text_document_uri.clone(),
                progress: 0,
                message: "Starting code execution...".to_string(),
            },
        )?;

        // 模拟代码执行（实际应调用 Cangjie 编译器/解释器）
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        // 发送进度通知（中间）
        self.send_custom_notification(
            "cangjie/executionProgress",
            &CodeExecutionProgressNotification {
                text_document_uri: params.text_document_uri.clone(),
                progress: 50,
                message: "Executing main function...".to_string(),
            },
        )?;

        std::thread::sleep(std::time::Duration::from_millis(500));

        // 构造执行结果
        let result = RunCangjieCodeResponse {
            exit_code: 0,
            stdout: format!("Successfully executed '{}'\nOutput: Hello from Cangjie!", path.file_name().unwrap().to_string_lossy()),
            stderr: String::new(),
        };

        // 发送进度通知（完成）
        self.send_custom_notification(
            "cangjie/executionProgress",
            &CodeExecutionProgressNotification {
                text_document_uri: params.text_document_uri,
                progress: 100,
                message: "Execution completed successfully!".to_string(),
            },
        )?;

        // 显示输出（如果需要）
        if params.show_output {
            self.workspace.show_output_panel("Cangjie Execution", &result.stdout)?;
        }

        Ok(result)
    }

    /// 处理执行进度通知
    fn handle_execution_progress(&mut self, progress: CodeExecutionProgressNotification) -> Result<()> {
        // 更新状态栏或输出面板
        self.workspace.set_status_message(&format!(
            "Cangjie Execution: {} ({})",
            progress.message, progress.progress
        ))?;
        Ok(())
    }
}
```

3. **客户端调用自定义消息**（可在 Zed 插件或外部工具中）
```javascript
// 示例：JavaScript 客户端调用
async function runCangjieCode(uri, showOutput = true, args = []) {
    const result = await zed.languageClient.sendRequest("cangjie/runCode", {
        textDocumentUri: uri,
        showOutput,
        args
    });
    return result;
}

// 监听进度通知
zed.languageClient.onNotification("cangjie/executionProgress", (progress) => {
    console.log(`Execution progress: ${progress.progress}% - ${progress.message}`);
});
```

#### K.2 集成外部工具
Cangjie 扩展可集成外部工具（如编译器、格式化工具、测试运行器），以下是集成 Cangjie 编译器的示例：

##### 实现步骤：
1. **添加工具配置**（`src/config/mod.rs`）
```rust
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CangjieToolConfig {
    /// 编译器路径
    pub compiler_path: Option<String>,
    /// 额外编译参数
    pub compile_args: Vec<String>,
    /// 测试运行器路径
    pub test_runner_path: Option<String>,
    /// 额外测试参数
    pub test_args: Vec<String>,
    /// 是否自动检测工具路径
    pub auto_detect: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CangjieConfig {
    // ... 其他配置项 ...
    /// 外部工具配置
    pub tools: CangjieToolConfig,
}
```

2. **工具路径检测**（`src/utils/tool_detection.rs`）
```rust
//! 外部工具路径检测
use std::path::{Path, PathBuf};
use crate::{config::CangjieToolConfig, utils::{file::file_exists, error::CangjieError}};

/// 检测 Cangjie 编译器路径
pub fn detect_compiler_path(config: &CangjieToolConfig) -> Result<PathBuf, CangjieError> {
    // 优先使用配置中的路径
    if let Some(path) = &config.compiler_path {
        let path = Path::new(path);
        if file_exists(path) {
            return Ok(path.to_path_buf());
        } else {
            return Err(CangjieError::User(format!(
                "Compiler not found at specified path: {}",
                path.display()
            )));
        }
    }

    // 自动检测（仅当 auto_detect 为 true 时）
    if config.auto_detect {
        // 检查系统 PATH
        if let Some(path) = which::which("cangjiec").ok() {
            return Ok(path);
        }

        // 检查常见安装路径
        let common_paths = [
            "/usr/bin/cangjiec",
            "/usr/local/bin/cangjiec",
            "~/.cargo/bin/cangjiec",
            "C:\\Program Files\\Cangjie\\bin\\cangjiec.exe",
        ];

        for path in common_paths {
            let expanded_path = shellexpand::tilde(path).into_owned();
            let path = Path::new(&expanded_path);
            if file_exists(path) {
                return Ok(path.to_path_buf());
            }
        }
    }

    Err(CangjieError::User(
        "Cangjie compiler not found. Please specify compiler_path in configuration or install Cangjie compiler."
            .to_string()
    ))
}

/// 检测 Cangjie 测试运行器路径
pub fn detect_test_runner_path(config: &CangjieToolConfig) -> Result<PathBuf, CangjieError> {
    // 类似编译器路径检测逻辑...
}
```

3. **编译功能实现**（`src/lsp/compile.rs`）
```rust
//! 代码编译功能
use std::process::{Command, Stdio};
use zed_extension_api::{self as zed, Result, lsp::Url};
use crate::{
    config::CangjieConfig,
    utils::{tool_detection::detect_compiler_path, log::info},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompileParams {
    pub text_document_uri: Url,
    pub output_path: Option<String>,
    pub watch: bool, // 是否监听文件变化自动重新编译
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompileResult {
    pub success: bool,
    pub output_path: Option<String>,
    pub errors: Vec<CompileError>,
    pub warnings: Vec<CompileWarning>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompileError {
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompileWarning {
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub file: Option<String>,
}

pub fn compile_cangjie_code(
    config: &CangjieConfig,
    params: &CompileParams,
) -> Result<CompileResult> {
    // 检测编译器路径
    let compiler_path = detect_compiler_path(&config.tools)?;
    info!("Using compiler: {}", compiler_path.display());

    // 获取文档路径
    let document = zed::workspace::current().document(&params.text_document_uri)?;
    let file_path = document.path()?;
    let file_dir = file_path.parent().ok_or_else(|| {
        zed::Error::user("Cannot get parent directory of the file")
    })?;

    // 构建输出路径
    let output_path = if let Some(path) = &params.output_path {
        Path::new(path).to_path_buf()
    } else {
        file_dir.join(
            file_path.file_stem().unwrap().to_str().unwrap()
        ).with_extension(if cfg!(windows) { "exe" } else { "" })
    };

    // 构建编译参数
    let mut args = config.tools.compile_args.clone();
    args.push(file_path.to_string_lossy().into_owned());
    args.push(format!("-o{}", output_path.to_string_lossy()));

    // 执行编译命令
    info!("Compiling with command: {:?} {:?}", compiler_path, args);
    let output = Command::new(compiler_path)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    // 解析编译输出
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let (errors, warnings) = parse_compile_output(&stdout, &stderr, &file_path)?;

    // 检查编译是否成功
    let success = output.status.success();
    if success {
        info!("Compilation successful. Output: {}", output_path.display());
    } else {
        info!("Compilation failed with {} errors and {} warnings", errors.len(), warnings.len());
    }

    // 如果启用 watch 模式，监听文件变化
    if params.watch {
        start_compile_watcher(&params.text_document_uri, config, params)?;
    }

    Ok(CompileResult {
        success,
        output_path: if success { Some(output_path.to_string_lossy().into_owned()) } else { None },
        errors,
        warnings,
    })
}

/// 解析编译输出（提取错误和警告）
fn parse_compile_output(
    stdout: &str,
    stderr: &str,
    file_path: &Path,
) -> Result<(Vec<CompileError>, Vec<CompileWarning>)> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // 示例解析逻辑（需根据实际编译器输出格式调整）
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let output = format!("{}\n{}", stdout, stderr);

    for line in output.lines() {
        // 匹配错误格式："file.cang:5:10: error: message"
        if let Some(captures) = regex::Regex::new(r"(\S+):(\d+):(\d+): (error|warning): (.+)")?
            .captures(line)
        {
            let path = captures[1].to_string();
            let line: u32 = captures[2].parse()?;
            let column: u32 = captures[3].parse()?;
            let level = captures[4].to_string();
            let message = captures[5].to_string();

            // 只保留当前文件的错误/警告
            if path.ends_with(file_name) {
                if level == "error" {
                    errors.push(CompileError {
                        message,
                        line: Some(line),
                        column: Some(column),
                        file: Some(path),
                    });
                } else {
                    warnings.push(CompileWarning {
                        message,
                        line: Some(line),
                        column: Some(column),
                        file: Some(path),
                    });
                }
            }
        }
    }

    Ok((errors, warnings))
}

/// 启动文件监听自动编译
fn start_compile_watcher(
    uri: &Url,
    config: &CangjieConfig,
    params: &CompileParams,
) -> Result<()> {
    let uri = uri.clone();
    let config = config.clone();
    let params = params.clone();

    // 在后台线程中监听文件变化
    std::thread::spawn(move || {
        let mut watcher = notify::recommended_watcher(|res| {
            match res {
                Ok(event) => {
                    if let notify::EventKind::Modify(_) = event.kind {
                        info!("File changed, triggering recompile...");
                        // 重新编译（忽略结果，通过诊断和输出面板反馈）
                        let _ = compile_cangjie_code(&config, &params);
                    }
                }
                Err(e) => info!("Watcher error: {:?}", e),
            }
        })?;

        // 监听当前文件
        let file_path = zed::workspace::current().document(&uri)?.path()?;
        watcher.watch(&file_path, notify::RecursiveMode::NonRecursive)?;

        // 保持线程运行
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        Ok(())
    });

    Ok(())
}
```

#### K.3 扩展 Zed 编辑器功能
除了 LSP 功能，Zed 扩展还可通过 `zed_extension_api` 扩展编辑器本身的功能，如添加自定义命令、菜单、快捷键等。

##### 示例：添加自定义命令
1. **注册自定义命令**（`src/extension.rs`）
```rust
use zed_extension_api::{self as zed, commands::CommandContext};

/// 扩展入口
pub fn activate() -> Result<(), zed::Error> {
    // 注册自定义命令
    zed::commands::register_command(
        "cangjie.runCode",
        "Run Cangjie Code",
        |context: CommandContext| async move {
            let document = context.document.ok_or_else(|| {
                zed::Error::user("No active document")
            })?;

            // 检查文件类型
            if document.language_id() != "cangjie" {
                return Err(zed::Error::user("Current file is not a Cangjie file"));
            }

            // 调用 LSP 自定义请求
            let client = zed::language_client::get(&document)?;
            let result: RunCangjieCodeResponse = client.send_request(
                "cangjie/runCode",
                &RunCangjieCodeParams {
                    text_document_uri: document.uri(),
                    show_output: true,
                    args: Vec::new(),
                },
            ).await?;

            // 显示结果
            if result.exit_code == 0 {
                zed::workspace::current().show_info_message(&format!(
                    "Code executed successfully:\n{}",
                    result.stdout
                ))?;
            } else {
                zed::workspace::current().show_error_message(&format!(
                    "Code execution failed (exit code {}):\n{}",
                    result.exit_code,
                    result.stderr
                ))?;
            }

            Ok(())
        },
    )?;

    // 注册编译命令
    zed::commands::register_command(
        "cangjie.compile",
        "Compile Cangjie Code",
        |context: CommandContext| async move {
            // 类似实现...
            Ok(())
        },
    )?;

    Ok(())
}
```

2. **添加菜单和快捷键**（`package.json`）
```json
{
  "contributes": {
    "commands": [
      {
        "command": "cangjie.runCode",
        "title": "Cangjie: Run Code"
      },
      {
        "command": "cangjie.compile",
        "title": "Cangjie: Compile Code"
      }
    ],
    "menus": {
      "editor/context": [
        {
          "command": "cangjie.runCode",
          "group": "cangjie@1",
          "when": "editorLangId == cangjie"
        },
        {
          "command": "cangjie.compile",
          "group": "cangjie@2",
          "when": "editorLangId == cangjie"
        }
      ],
      "commandPalette": [
        {
          "command": "cangjie.runCode",
          "when": "editorLangId == cangjie"
        },
        {
          "command": "cangjie.compile",
          "when": "editorLangId == cangjie"
        }
      ]
    },
    "keybindings": [
      {
        "command": "cangjie.runCode",
        "key": "ctrl+shift+r",
        "mac": "cmd+shift+r",
        "when": "editorLangId == cangjie"
      },
      {
        "command": "cangjie.compile",
        "key": "ctrl+shift+c",
        "mac": "cmd+shift+c",
        "when": "editorLangId == cangjie"
      }
    ]
  }
}
```

### 附录 L：扩展安全最佳实践
开发 Zed 扩展时，需遵循以下安全最佳实践，避免安全漏洞和恶意行为：

#### L.1 代码安全
1. **避免未验证的输入**：
   - 对所有用户输入（配置、文件内容、外部命令参数）进行验证和清洗
   - 使用 `regex` 限制输入格式，避免注入攻击（如命令注入、路径遍历）

2. **安全的外部命令执行**：
   - 避免直接拼接命令字符串，使用 `Command::args` 传递参数
   - 限制外部命令的执行权限，避免使用 `sudo` 或管理员权限
   - 验证外部命令的路径，避免执行恶意程序

   ```rust
   // 不安全：直接拼接命令
   let unsafe_cmd = format!("cangjiec {} -o output", user_input);
   Command::new("sh").arg("-c").arg(unsafe_cmd);

   // 安全：使用 args 传递参数
   Command::new("cangjiec")
       .arg(user_input) // 用户输入作为单独参数，避免注入
       .arg("-o")
       .arg("output");
   ```

3. **权限最小化**：
   - 仅申请扩展必需的权限（如文件读写、进程执行）
   - 避免访问敏感目录（如 `/etc`、`C:\Windows`）
   - 临时文件使用系统临时目录（`std::env::temp_dir()`）

#### L.2 依赖安全
1. **定期更新依赖**：
   - 使用 `cargo audit` 检查依赖漏洞：
     ```bash
     cargo install cargo-audit
     cargo audit
     ```
   - 及时更新存在漏洞的依赖版本

2. **审查第三方依赖**：
   - 优先选择成熟、活跃维护的依赖
   - 避免使用来源不明或下载量极低的依赖
   - 对于核心功能，可考虑使用官方或知名组织维护的依赖

3. **锁定依赖版本**：
   - 使用 `Cargo.lock` 锁定依赖版本，避免构建时自动更新到存在漏洞的版本
   - 发布扩展时包含 `Cargo.lock` 文件

#### L.3 数据安全
1. **敏感数据处理**：
   - 不存储或传输敏感数据（密码、密钥、个人信息）
   - 如需处理敏感数据，使用加密存储（如 Zed 的安全存储 API）

2. **文件操作安全**：
   - 验证文件路径，避免路径遍历攻击：
     ```rust
     // 安全的路径验证
     fn safe_resolve_path(base: &Path, relative: &str) -> Result<PathBuf, CangjieError> {
         let resolved = base.join(relative).canonicalize()?;
         if !resolved.starts_with(base) {
             return Err(user_error("Path traversal detected"));
         }
         Ok(resolved)
     }
     ```
   - 限制文件读写权限，仅允许操作必要的文件类型（如 `.cang`、`.cj`）

3. **网络请求安全**：
   - 如需发起网络请求，仅使用 HTTPS 协议
   - 验证服务器证书，避免中间人攻击
   - 限制请求超时时间，避免无限等待

### 附录 M：扩展国际化完整实现
除了基础的本地化支持，以下是扩展国际化的完整实现，包括自动检测系统语言、动态切换语言等功能：

#### M.1 扩展本地化模块
```rust
// src/locale/mod.rs（完整实现）
//! 扩展国际化支持
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use toml::Value;
use zed_extension_api::{self as zed, Result};
use crate::utils::{file::read_toml_file, error::CangjieError};

/// 支持的语言列表及显示名称
pub const SUPPORTED_LANGUAGES: &[(&str, &str)] = &[
    ("en-US", "English (US)"),
    ("zh-CN", "中文（简体）"),
    ("ja-JP", "日本語"),
    ("ko-KR", "한국어"),
    ("de-DE", "Deutsch"),
    ("fr-FR", "Français"),
];

/// 本地化数据结构
#[derive(Debug, Clone)]
pub struct Locale {
    data: Arc<HashMap<String, String>>,
    language: String,
}

impl Locale {
    /// 加载指定语言的本地化文件
    pub fn load(language: &str) -> Result<Self, CangjieError> {
        // 语言代码映射（支持模糊匹配，如 "zh" -> "zh-CN"）
        let lang_code = match language.split('-').next().unwrap_or("en") {
            "zh" => "zh-CN",
            "ja" => "ja-JP",
            "ko" => "ko-KR",
            "de" => "de-DE",
            "fr" => "fr-FR",
            _ => "en-US",
        };

        // 检查是否支持该语言
        let lang = if SUPPORTED_LANGUAGES.iter().any(|(code, _)| *code == lang_code) {
            lang_code
        } else {
            "en-US"
        };

        // 加载本地化文件
        let path = format!("src/locale/{}.toml", lang);
        let toml_data: Value = read_toml_file(&std::path::Path::new(&path))?;

        let mut data = HashMap::new();
        flatten_toml(&toml_data, "", &mut data);

        Ok(Self {
            data: Arc::new(data),
            language: lang.to_string(),
        })
    }

    /// 获取本地化字符串
    pub fn get(&self, key: &str) -> String {
        self.data.get(key).cloned().unwrap_or_else(|| {
            crate::utils::log::warn!("Missing locale key: {}", key);
            key.to_string()
        })
    }

    /// 获取本地化字符串并格式化
    pub fn format(&self, key: &str, args: &[&str]) -> String {
        let template = self.get(key);
        self.format_template(&template, args)
    }

    /// 格式化模板字符串（支持位置参数和命名参数）
    fn format_template(&self, template: &str, args: &[&str]) -> String {
        let mut result = template.to_string();
        
        // 处理位置参数（{0}, {1}, ...）
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
        
        result
    }

    /// 获取当前语言代码
    pub fn language(&self) -> &str {
        &self.language
    }
}

/// 扁平化 TOML 数据为 key-value 结构
fn flatten_toml(value: &Value, prefix: &str, data: &mut HashMap<String, String>) {
    match value {
        Value::Table(table) => {
            for (key, val) in table {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                flatten_toml(val, &new_prefix, data);
            }
        }
        Value::String(s) => {
            if !prefix.is_empty() {
                data.insert(prefix.to_string(), s.clone());
            }
        }
        _ => {}
    }
}

/// 全局本地化实例（使用 Arc 支持多线程共享）
static LOCALE: Mutex<Option<Arc<Locale>>> = Mutex::new(None);

/// 初始化本地化（自动检测系统语言）
pub fn init_locale() -> Result<(), CangjieError> {
    // 获取系统语言
    let system_lang = zed::system::language()?;
    crate::utils::log::info!("Detected system language: {}", system_lang);
    
    // 加载本地化
    let locale = Locale::load(&system_lang)?;
    *LOCALE.lock()? = Some(Arc::new(locale));
    
    Ok(())
}

/// 手动切换语言
pub fn switch_locale(language: &str) -> Result<(), CangjieError> {
    let locale = Locale::load(language)?;
    *LOCALE.lock()? = Some(Arc::new(locale));
    crate::utils::log::info!("Switched to language: {}", language);
    Ok(())
}

/// 获取当前本地化实例
pub fn current_locale() -> Option<Arc<Locale>> {
    LOCALE.lock().ok().and_then(|l| l.clone())
}

/// 获取本地化字符串（简化调用）
pub fn t(key: &str) -> String {
    current_locale()
        .map(|loc| loc.get(key))
        .unwrap_or_else(|| key.to_string())
}

/// 获取本地化字符串并格式化（简化调用）
pub fn tf(key: &str, args: &[&str]) -> String {
    current_locale()
        .map(|loc| loc.format(key, args))
        .unwrap_or_else(|| {
            let mut result = key.to_string();
            for (i, arg) in args.iter().enumerate() {
                result = result.replace(&format!("{{{}}}", i), arg);
            }
            result
        })
}

/// 获取支持的语言列表（用于 UI 显示）
pub fn supported_languages() -> Vec<(String, String)> {
    SUPPORTED_LANGUAGES
        .iter()
        .map(|(code, name)| (code.to_string(), name.to_string()))
        .collect()
}
```

#### M.2 添加语言切换命令
```rust
// src/extension.rs
pub fn activate() -> Result<(), zed::Error> {
    // ... 其他初始化 ...

    // 初始化本地化
    crate::locale::init_locale()?;

    // 注册语言切换命令
    zed::commands::register_command(
        "cangjie.switchLanguage",
        "Cangjie: Switch Language",
        |_context: CommandContext| async move {
            // 获取支持的语言列表
            let languages = crate::locale::supported_languages();
            
            // 显示语言选择菜单
            let selected = zed::ui::show_select_menu(
                "Select Language",
                &languages.iter().map(|(code, name)| format!("{} ({})", name, code)).collect::<Vec<_>>(),
            ).await?;
            
            if let Some(index) = selected {
                let (lang_code, _) = &languages[index];
                crate::locale::switch_locale(lang_code)?;
                
                // 显示切换成功消息
                zed::workspace::current().show_info_message(&format!(
                    "Switched to language: {}",
                    crate::locale::t("locale.language_switched")
                ))?;
            }
            
            Ok(())
        },
    )?;

    Ok(())
}
```

### 最终总结
Cangjie 扩展作为 Zed 编辑器的一款功能完整的编程语言扩展，涵盖了从基础语法支持到高级 LSP 功能、从本地开发到国际化部署的全流程实现。本文档详细阐述了扩展的设计理念、实现细节、使用指南和扩展技巧，旨在为开发者提供清晰的参考和实用的工具。

通过本扩展的开发，我们积累了以下关键经验：
1. **模块化设计**：将扩展拆分为配置、LSP、语法、工具等独立模块，提高代码可维护性和可扩展性。
2. **性能优化**：通过解析树缓存、增量解析、请求节流等机制，确保扩展在大文件和高频操作场景下的流畅性。
3. **用户体验**：注重配置的灵活性、错误提示的友好性、功能的易用性，让开发者能够快速上手和自定义。
4. **生态集成**：支持与外部工具（编译器、测试运行器）的集成，扩展功能边界。
5. **社区友好**：提供详细的开发文档、贡献指南和问题模板，降低社区参与门槛。

未来，我们将继续优化扩展的性能和功能，跟进 Zed 编辑器和 LSP 协议的最新特性，同时积极响应用户反馈，不断提升 Cangjie 语言在 Zed 中的开发体验。

感谢您选择 Cangjie 扩展，期待您的使用和贡献！

---

**文档版本**：v1.0.0  
**最后更新**：2025-11-09  
**完整代码仓库**：https://github.com/your-username/zed-cangjie-extension  
**扩展市场地址**：https://extensions.zed.dev/extensions/your-username/cangjie