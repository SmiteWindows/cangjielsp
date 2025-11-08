use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use zed_extension_api as zed;
use zed_extension_api::{
    serde, serde_json, Command, Completion, DebugAdapterBinary, DebugConfig, DebugRequest,
    DebugScenario, DebugTaskDefinition, LanguageServerId, SlashCommand,
    SlashCommandArgumentCompletion, SlashCommandOutput, Symbol, Worktree,
};

// 严格遵循文档：使用常量定义核心标识（与配置文件强关联）
pub const CANGJIE_LS_ID: &str = "cangjie-language-server";
pub const CANGJIE_DAP_ID: &str = "cangjie-dap";
pub const CANGJIE_LANG_NAME: &str = "Cangjie";
pub const CANGJIE_FILE_EXTS: &[&str] = &["cj", "cj.d"];

/// 仓颉语言服务配置管理器（遵循文档：单一职责原则）
#[derive(Debug, Default)]
struct CangjieConfigManager;

impl CangjieConfigManager {
    /// 读取 CANGJIE_HOME 环境变量（文档推荐：优先环境变量配置）
    fn get_cangjie_home(&self) -> zed::Result<PathBuf> {
        env::var("CANGJIE_HOME")
            .map_err(|_| {
                zed::Error::InvalidConfig(
                    "未配置 CANGJIE_HOME 环境变量，请参考仓颉 SDK 安装文档".into(),
                )
            })
            .and_then(|home| {
                let path = PathBuf::from(home);
                if !path.exists() {
                    return Err(zed::Error::NotFound(format!(
                        "CANGJIE_HOME 路径不存在: {}",
                        path.display()
                    )));
                }
                path.canonicalize()
                    .map_err(|e| zed::Error::InvalidPath(format!("CANGJIE_HOME 路径无效: {}", e)))
            })
    }

    /// 获取 LSP 服务器二进制路径（文档要求：跨平台路径处理）
    fn get_lsp_binary(&self) -> zed::Result<PathBuf> {
        let home = self.get_cangjie_home()?;
        let bin_name = if cfg!(windows) {
            "LSPServer.exe"
        } else {
            "LSPServer"
        };

        let bin_path = home.join("tools").join("bin").join(bin_name);

        self.validate_file_exists(&bin_path, "LSP 服务器")
    }

    /// 获取 DAP 调试适配器路径（文档要求：显式验证文件存在）
    fn get_dap_binary(&self) -> zed::Result<PathBuf> {
        let home = self.get_cangjie_home()?;
        let bin_name = if cfg!(windows) {
            "CangjieDAP.exe"
        } else {
            "CangjieDAP"
        };

        let bin_path = home.join("tools").join("bin").join(bin_name);

        self.validate_file_exists(&bin_path, "调试适配器")
    }

    /// 获取运行时库路径（文档推荐：按 OS/架构动态适配）
    fn get_runtime_lib_path(&self) -> zed::Result<PathBuf> {
        let home = self.get_cangjie_home()?;
        let arch_dir = self.get_arch_directory()?;

        let lib_path = home.join("runtime").join("lib").join(arch_dir);

        self.validate_dir_exists(&lib_path, "运行时库")
    }

    /// 适配目标架构目录（文档规范：明确支持的平台/架构）
    fn get_arch_directory(&self) -> zed::Result<&'static str> {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("windows", "x86_64") => Ok("windows_x86_64_llvm"),
            ("windows", "aarch64") => Ok("windows_arm64_llvm"),
            ("macos", "x86_64") => Ok("macos_x86_64_llvm"),
            ("macos", "aarch64") => Ok("macos_arm64_llvm"),
            ("linux", "x86_64") => Ok("linux_x86_64_llvm"),
            ("linux", "aarch64") => Ok("linux_arm64_llvm"),
            (os, arch) => Err(zed::Error::UnsupportedPlatform(format!(
                "不支持的平台/架构: {}/{}（仅支持 Windows/macOS/Linux 的 x86_64/aarch64）",
                os, arch
            ))),
        }
    }

    /// 通用文件存在性校验（文档推荐：提取通用逻辑）
    fn validate_file_exists(&self, path: &Path, desc: &str) -> zed::Result<PathBuf> {
        if !path.exists() {
            return Err(zed::Error::NotFound(format!(
                "{}不存在: {}",
                desc,
                path.display()
            )));
        }
        if !path.is_file() {
            return Err(zed::Error::InvalidPath(format!(
                "{}路径不是文件: {}",
                desc,
                path.display()
            )));
        }
        Ok(path.to_path_buf())
    }

    /// 通用目录存在性校验（文档推荐：强类型路径校验）
    fn validate_dir_exists(&self, path: &Path, desc: &str) -> zed::Result<PathBuf> {
        if !path.exists() {
            return Err(zed::Error::NotFound(format!(
                "{}目录不存在: {}",
                desc,
                path.display()
            )));
        }
        if !path.is_dir() {
            return Err(zed::Error::InvalidPath(format!(
                "{}路径不是目录: {}",
                desc,
                path.display()
            )));
        }
        Ok(path.to_path_buf())
    }

    /// 路径转字符串（文档要求：处理非 UTF-8 路径）
    fn path_to_str(&self, path: &PathBuf) -> zed::Result<String> {
        path.to_str()
            .ok_or_else(|| {
                zed::Error::InvalidPath(format!("路径包含非 UTF-8 字符: {}", path.display()))
            })
            .map(|s| s.to_string())
    }
}

/// 仓颉扩展主结构体（遵循文档：无状态设计）
#[derive(Default)]
struct CangjieExtension {
    config_manager: CangjieConfigManager,
}

impl zed::Extension for CangjieExtension {
    /// 初始化插件（文档要求：无副作用初始化）
    fn new() -> Self {
        Self::default()
    }

    /// 1. 启动语言服务器（文档核心 API：严格遵循 Command 结构体规范）
    fn language_server_command(
        &mut self,
        ls_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> zed::Result<Command> {
        if ls_id.as_ref() != CANGJIE_LS_ID {
            return Err(zed::Error::InvalidRequest(format!(
                "不支持的语言服务器 ID: {}（仅支持 {}）",
                ls_id, CANGJIE_LS_ID
            )));
        }

        // 读取核心路径
        let lsp_bin = self.config_manager.get_lsp_binary()?;
        let lib_path = self.config_manager.get_runtime_lib_path()?;
        let cangjie_home = self.config_manager.get_cangjie_home()?;

        // 路径转字符串（文档要求：Command 字段需为 String）
        let lsp_bin_str = self.config_manager.path_to_str(&lsp_bin)?;
        let lib_path_str = self.config_manager.path_to_str(&lib_path)?;
        let home_str = self.config_manager.path_to_str(&cangjie_home)?;

        // 配置环境变量（文档推荐：按 OS 差异化配置）
        let mut env = HashMap::new();
        env.insert("CANGJIE_HOME".to_string(), home_str);

        match std::env::consts::OS {
            "windows" => {
                let path = format!("{};{}", env::var("PATH").unwrap_or_default(), lib_path_str);
                env.insert("PATH".to_string(), path);
            }
            "macos" => {
                env.insert("DYLD_LIBRARY_PATH".to_string(), lib_path_str);
            }
            "linux" => {
                env.insert("LD_LIBRARY_PATH".to_string(), lib_path_str);
            }
            os => {
                return Err(zed::Error::UnsupportedPlatform(format!(
                    "不支持的 OS: {}",
                    os
                )))
            }
        }

        // 构建 LSP 命令（文档要求：参数清晰，日志可追踪）
        Ok(Command {
            command: lsp_bin_str,
            args: vec![
                "src".to_string(),
                "--disableAutoImport".to_string(),
                "--enable-log=true".to_string(),
                "--log-path".to_string(),
                self.config_manager
                    .path_to_str(&env::temp_dir().join("cangjie_lsp.log"))?,
            ],
            env: env.into_iter().collect(),
        })
    }

    /// 2. LSP 初始化选项（文档规范：JSON 配置结构化）
    fn language_server_initialization_options(
        &mut self,
        ls_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        if ls_id.as_ref() != CANGJIE_LS_ID {
            return Ok(None);
        }

        // 遵循文档：初始化选项与语言服务配置对齐
        Ok(Some(serde_json::json!({
            "logLevel": "info",
            "completion": {
                "enableSnippets": true,
                "enableAutoImport": false,
                "triggerOnTyping": true
            },
            "formatting": {
                "indentSize": 4,
                "newlineAfterBrace": true,
                "spaceAfterComma": true
            },
            "diagnostics": {
                "enable": true,
                "reportUnused": true
            }
        })))
    }

    /// 3. 工作区配置合并（文档要求：用户配置优先级高于默认配置）
    fn language_server_workspace_configuration(
        &mut self,
        ls_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        if ls_id.as_ref() != CANGJIE_LS_ID {
            return Ok(None);
        }

        // 读取用户配置（文档 API：LspSettings::for_worktree）
        let user_config = zed::settings::LspSettings::for_worktree(CANGJIE_LS_ID, worktree)
            .map(|settings| settings.settings)
            .unwrap_or_default();

        // 默认配置（文档推荐：提供合理默认值）
        let default_config = serde_json::json!({
            "cangjie": {
                "target": "native",
                "buildType": "debug",
                "enableHwasan": false,
                "sdkPath": self.config_manager.get_cangjie_home().ok().and_then(|p| p.to_str().map(|s| s.to_string()))
            }
        });

        // 合并配置（文档规范：用户配置覆盖默认配置）
        let merged = match user_config {
            serde_json::Value::Object(user_map) => {
                let mut default_map = default_config.as_object().unwrap().clone();
                default_map.extend(user_map);
                serde_json::Value::Object(default_map)
            }
            _ => default_config,
        };

        Ok(Some(merged))
    }

    /// 4. 调试适配器配置（文档核心 API：DAP 集成规范）
    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        _config: DebugTaskDefinition,
        user_provided_path: Option<String>,
        _worktree: &Worktree,
    ) -> zed::Result<DebugAdapterBinary> {
        if adapter_name != CANGJIE_DAP_ID {
            return Err(zed::Error::InvalidRequest(format!(
                "不支持的调试适配器: {}（仅支持 {}）",
                adapter_name, CANGJIE_DAP_ID
            )));
        }

        // 优先使用用户指定路径（文档推荐：用户可自定义）
        let dap_path = match user_provided_path {
            Some(path) => PathBuf::from(path),
            None => self.config_manager.get_dap_binary()?,
        };

        // 验证路径有效性（文档要求：显式校验）
        let dap_path = self
            .config_manager
            .validate_file_exists(&dap_path, "调试适配器")?;
        let dap_path_str = self.config_manager.path_to_str(&dap_path)?;

        Ok(DebugAdapterBinary {
            path: dap_path_str,
            args: vec![
                "--enable-log".to_string(),
                "--log-path".to_string(),
                self.config_manager
                    .path_to_str(&env::temp_dir().join("cangjie_dap.log"))?,
            ],
            env: HashMap::new(),
        })
    }

    /// 5. 调试配置转换（文档规范：DAP 协议适配）
    fn dap_config_to_scenario(&mut self, config: DebugConfig) -> zed::Result<DebugScenario> {
        // 提取必填配置（文档要求：强校验必填字段）
        let program = config
            .get("program")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                zed::Error::InvalidConfig(
                    "调试配置缺少必填字段 `program`（需指定可执行文件路径）".into(),
                )
            })?;

        // 可选配置（文档推荐：提供合理默认值）
        let args: Vec<String> = config
            .get("args")
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let cwd = config
            .get("cwd")
            .and_then(|v| v.as_str())
            .unwrap_or(".")
            .to_string();

        let stop_on_entry = config
            .get("stopOnEntry")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let target = config
            .get("target")
            .and_then(|v| v.as_str())
            .unwrap_or("native");

        // 构建 DAP 场景（文档要求：严格遵循 DAP 协议结构）
        Ok(DebugScenario {
            adapter_name: CANGJIE_DAP_ID.to_string(),
            request: DebugRequest::Launch(serde_json::json!({
                "program": program,
                "args": args,
                "cwd": cwd,
                "stopOnEntry": stop_on_entry,
                "target": target,
                "sourceLanguages": [CANGJIE_LANG_NAME],
                "logging": {
                    "enable": true,
                    "file": self.config_manager.path_to_str(&env::temp_dir().join("cangjie_debug.log"))?
                }
            })),
            source_file_map: HashMap::new(),
        })
    }

    /// 6. Slash 命令执行（文档核心 API：命令标准化）
    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> zed::Result<SlashCommandOutput> {
        // 校验工作区（文档要求：命令需在工作区内执行）
        let worktree = worktree.ok_or_else(|| {
            zed::Error::InvalidRequest("仓颉命令需在工作区内执行，请先打开项目目录".into())
        })?;

        let cwd = self
            .config_manager
            .path_to_str(&worktree.path().to_path_buf())?;

        match command.as_str() {
            "cangjie: build" => self.run_build_command(&args, &cwd),
            "cangjie: run" => self.run_execute_command(&args, &cwd),
            "cangjie: test" => self.run_test_command(&args, &cwd),
            "cangjie: clean" => self.run_clean_command(&cwd),
            "cangjie: check-env" => self.run_check_env_command(),
            _ => Err(zed::Error::InvalidRequest(format!(
                "不支持的命令: {}（支持的命令：cangjie: build/run/test/clean/check-env）",
                command
            ))),
        }
    }

    /// 7. Slash 命令参数补全（文档推荐：提升用户体验）
    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        args: Vec<String>,
    ) -> zed::Result<Vec<SlashCommandArgumentCompletion>> {
        match command.as_str() {
            "cangjie: build" => self.complete_build_args(args.len()),
            "cangjie: test" => self.complete_test_args(args.len()),
            _ => Ok(vec![]),
        }
    }

    /// 8. 代码补全标签美化（文档 API：视觉一致性优化）
    fn label_for_completion(
        &self,
        _ls_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<zed::CodeLabel> {
        // 遵循文档：使用 Zed 推荐的图标和颜色体系
        let (icon, color) = match completion.kind.as_deref() {
            Some("function") => ("⨍", "text-blue-500"),
            Some("method") => ("⨍", "text-blue-600"),
            Some("class") => ("🅒", "text-purple-500"),
            Some("struct") => ("🅢", "text-purple-600"),
            Some("enum") => ("🅔", "text-yellow-500"),
            Some("enumMember") => ("🅔", "text-yellow-600"),
            Some("variable") => ("ⓥ", "text-green-500"),
            Some("constant") => ("ⓒ", "text-green-600"),
            Some("type") => ("ⓣ", "text-pink-500"),
            Some("interface") => ("🅘", "text-cyan-500"),
            Some("module") => ("🅜", "text-orange-500"),
            _ => return None,
        };

        Some(zed::CodeLabel {
            label: format!("{} {}", icon, completion.label),
            detail: completion.detail,
            color: Some(color.to_string()),
            ..Default::default()
        })
    }

    /// 9. 符号标签美化（文档 API：侧边栏符号树优化）
    fn label_for_symbol(
        &self,
        _ls_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<zed::CodeLabel> {
        let (icon, color) = match symbol.kind.as_deref() {
            Some("Class") => ("🅒", "text-purple-500"),
            Some("Struct") => ("🅢", "text-purple-600"),
            Some("Enum") => ("🅔", "text-yellow-500"),
            Some("Function") => ("⨍", "text-blue-500"),
            Some("Method") => ("⨍", "text-blue-600"),
            Some("Variable") => ("ⓥ", "text-green-500"),
            Some("Constant") => ("ⓒ", "text-green-600"),
            Some("Type") => ("ⓣ", "text-pink-500"),
            Some("Interface") => ("🅘", "text-cyan-500"),
            Some("Module") => ("🅜", "text-orange-500"),
            Some("Test") => ("✅", "text-green-400"),
            _ => return None,
        };

        Some(zed::CodeLabel {
            label: format!("{} {}", icon, symbol.name),
            detail: symbol.detail,
            color: Some(color.to_string()),
            ..Default::default()
        })
    }
}

/// Slash 命令实现（文档推荐：提取独立方法，便于维护）
impl CangjieExtension {
    /// 构建命令（支持目标架构和构建类型）
    fn run_build_command(&self, args: &[String], cwd: &str) -> zed::Result<SlashCommandOutput> {
        let target = args.get(0).cloned().unwrap_or("native".to_string());
        let build_type = args.get(1).cloned().unwrap_or("debug".to_string());

        // 校验构建类型（文档要求：参数合法性校验）
        if !["debug", "release"].contains(&build_type.as_str()) {
            return Err(zed::Error::InvalidArgument(format!(
                "无效的构建类型: {}（仅支持 debug/release）",
                build_type
            )));
        }

        let status = std::process::Command::new("cjpm")
            .args(["build", "--target", &target, "--build-type", &build_type])
            .current_dir(cwd)
            .status()
            .map_err(|e| zed::Error::ExecutionFailed(format!("构建失败: {}", e)))?;

        if status.success() {
            Ok(SlashCommandOutput::Message(format!(
                "✅ 构建成功\n目标架构: {}\n构建类型: {}",
                target, build_type
            )))
        } else {
            Err(zed::Error::ExecutionFailed(format!(
                "❌ 构建失败（退出码: {}）\n目标架构: {}\n构建类型: {}",
                status.code().unwrap_or(-1),
                target,
                build_type
            )))
        }
    }

    /// 运行命令（执行编译产物）
    fn run_execute_command(&self, args: &[String], cwd: &str) -> zed::Result<SlashCommandOutput> {
        let program = args.first().ok_or_else(|| {
            zed::Error::InvalidArgument("请指定运行的程序路径（如：target/debug/main）".into())
        })?;

        let status = std::process::Command::new(program)
            .current_dir(cwd)
            .status()
            .map_err(|e| zed::Error::ExecutionFailed(format!("运行失败: {}", e)))?;

        if status.success() {
            Ok(SlashCommandOutput::Message(format!(
                "✅ 程序运行成功\n路径: {}",
                program
            )))
        } else {
            Err(zed::Error::ExecutionFailed(format!(
                "❌ 程序运行失败（退出码: {}）\n路径: {}",
                status.code().unwrap_or(-1),
                program
            )))
        }
    }

    /// 测试命令（运行测试用例）
    fn run_test_command(&self, args: &[String], cwd: &str) -> zed::Result<SlashCommandOutput> {
        let test_filter = args.get(0).cloned().unwrap_or("*".to_string());

        let status = std::process::Command::new("cjpm")
            .args(["test", "--test-filter", &test_filter])
            .current_dir(cwd)
            .status()
            .map_err(|e| zed::Error::ExecutionFailed(format!("测试失败: {}", e)))?;

        if status.success() {
            Ok(SlashCommandOutput::Message(format!(
                "✅ 测试执行成功\n过滤规则: {}",
                test_filter
            )))
        } else {
            Err(zed::Error::ExecutionFailed(format!(
                "❌ 测试执行失败（部分用例未通过）\n过滤规则: {}",
                test_filter
            )))
        }
    }

    /// 清理命令（删除构建产物）
    fn run_clean_command(&self, cwd: &str) -> zed::Result<SlashCommandOutput> {
        let status = std::process::Command::new("cjpm")
            .arg("clean")
            .current_dir(cwd)
            .status()
            .map_err(|e| zed::Error::ExecutionFailed(format!("清理失败: {}", e)))?;

        if status.success() {
            Ok(SlashCommandOutput::Message("✅ 构建产物清理成功".into()))
        } else {
            Err(zed::Error::ExecutionFailed("❌ 构建产物清理失败".into()))
        }
    }

    /// 环境检查命令（文档推荐：提供环境诊断功能）
    fn run_check_env_command(&self) -> zed::Result<SlashCommandOutput> {
        let home = self.config_manager.get_cangjie_home()?;
        let lsp_bin = self.config_manager.get_lsp_binary()?;
        let dap_bin = self.config_manager.get_dap_binary()?;
        let lib_path = self.config_manager.get_runtime_lib_path()?;

        Ok(SlashCommandOutput::Message(format!(
            "✅ 仓颉环境检查通过\nCANGJIE_HOME: {}\nLSP 服务器: {}\n调试适配器: {}\n运行时库: {}",
            home.display(),
            lsp_bin.display(),
            dap_bin.display(),
            lib_path.display()
        )))
    }

    /// 构建命令参数补全（文档推荐：上下文感知补全）
    fn complete_build_args(
        &self,
        arg_index: usize,
    ) -> zed::Result<Vec<SlashCommandArgumentCompletion>> {
        match arg_index {
            // 第一个参数：目标架构
            1 => Ok(vec![
                self.create_completion("native", "本地架构（默认）"),
                self.create_completion("ohos-aarch64", "鸿蒙 ARM64 架构"),
                self.create_completion("ohos-x86_64", "鸿蒙 X86_64 架构"),
                self.create_completion("windows-x86_64", "Windows X86_64 架构"),
                self.create_completion("linux-x86_64", "Linux X86_64 架构"),
            ]),
            // 第二个参数：构建类型
            2 => Ok(vec![
                self.create_completion("debug", "调试模式（含调试信息，默认）"),
                self.create_completion("release", "发布模式（优化编译）"),
            ]),
            _ => Ok(vec![]),
        }
    }

    /// 测试命令参数补全
    fn complete_test_args(
        &self,
        arg_index: usize,
    ) -> zed::Result<Vec<SlashCommandArgumentCompletion>> {
        if arg_index == 1 {
            Ok(vec![
                self.create_completion("*", "运行所有测试用例（默认）"),
                self.create_completion("test_*", "运行前缀为 test_ 的用例"),
                self.create_completion("*_integ", "运行后缀为 _integ 的集成测试"),
            ])
        } else {
            Ok(vec![])
        }
    }

    /// 创建补全项（文档推荐：统一补全项格式）
    fn create_completion(&self, label: &str, detail: &str) -> SlashCommandArgumentCompletion {
        SlashCommandArgumentCompletion {
            label: label.to_string(),
            detail: Some(detail.to_string()),
            insert_text: Some(label.to_string()),
        }
    }
}

// 注册插件（文档要求：必须调用 register_extension！ 宏）
zed::register_extension!(CangjieExtension);
