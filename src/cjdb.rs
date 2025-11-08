//! cjdb 调试工具集成（调试会话管理、断点、调试-性能联动）
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zed_extension_api as zed;

/// cjdb 配置（对应 cjdb.toml）
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CjdbConfig {
    /// 调试会话配置
    pub session: SessionConfig,
    /// 断点配置
    pub breakpoints: BreakpointConfig,
    /// 日志配置
    pub logging: LoggingConfig,
    /// 性能联动配置
    pub performance: PerformanceLinkConfig,
}

/// 会话配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SessionConfig {
    /// 调试端口（默认 50051）
    #[serde(default = "default_debug_port")]
    pub port: u16,
    /// 超时时间（秒，默认 30）
    #[serde(default = "default_timeout")]
    pub timeout: u32,
    /// 自动附加子进程（默认 false）
    #[serde(default)]
    pub attach_child_processes: bool,
    /// 协程调试启用（默认 true）
    #[serde(default = "default_coroutine_debug")]
    pub enable_coroutine_debug: bool,
}

/// 断点配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct BreakpointConfig {
    /// 启用条件断点（默认 true）
    #[serde(default = "default_condition_breakpoints")]
    pub enable_conditional: bool,
    /// 启用日志断点（默认 true）
    #[serde(default = "default_log_breakpoints")]
    pub enable_log: bool,
    /// 忽略的断点文件（glob 模式）
    #[serde(default)]
    pub ignore_files: Vec<String>,
}

/// 日志配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LoggingConfig {
    /// 日志级别（trace/debug/info/warn/error）
    #[serde(default = "default_log_level")]
    pub level: String,
    /// 日志输出路径（默认 target/cjdb/logs）
    #[serde(default = "default_log_dir")]
    pub dir: String,
    /// 启用文件日志（默认 true）
    #[serde(default = "default_enable_file_log")]
    pub enable_file: bool,
    /// 启用控制台日志（默认 false）
    #[serde(default)]
    pub enable_console: bool,
}

/// 性能联动配置（与 cjprof 联动）
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PerformanceLinkConfig {
    /// 调试时启用性能采样（默认 false）
    #[serde(default)]
    pub enable_profiling: bool,
    /// 采样类型（默认 CPU+Coroutine）
    #[serde(default = "default_profiling_types")]
    pub profiling_types: Vec<String>,
    /// 采样频率（毫秒，默认 20）
    #[serde(default = "default_profiling_interval")]
    pub profiling_interval: u32,
}

// 默认值
fn default_debug_port() -> u16 {
    50051
}
fn default_timeout() -> u32 {
    30
}
fn default_coroutine_debug() -> bool {
    true
}
fn default_condition_breakpoints() -> bool {
    true
}
fn default_log_breakpoints() -> bool {
    true
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_log_dir() -> String {
    "target/cjdb/logs".to_string()
}
fn default_enable_file_log() -> bool {
    true
}
fn default_profiling_types() -> Vec<String> {
    vec!["Cpu".to_string(), "Coroutine".to_string()]
}
fn default_profiling_interval() -> u32 {
    20
}

/// cjdb 调试管理器
#[derive(Debug, Default)]
pub struct CjdbManager;

impl CjdbManager {
    /// 检查 cjdb 是否可用
    pub fn is_available() -> zed::Result<()> {
        Self::find_executable()?;
        Ok(())
    }

    /// 查找 cjdb 可执行文件
    pub fn find_executable() -> zed::Result<zed::Path> {
        // 1. 从 CANGJIE_HOME 查找
        if let Ok(cangjie_home) = zed::env::var("CANGJIE_HOME") {
            let mut path = zed::Path::new(&cangjie_home);
            path.push("bin");
            path.push(if zed::platform::is_windows() {
                "cjdb.exe"
            } else {
                "cjdb"
            });

            if path.exists() && path.is_executable() {
                return Ok(path);
            }
        }

        // 2. 从 PATH 查找
        if let Some(path) = zed::env::find_executable(if zed::platform::is_windows() {
            "cjdb.exe"
        } else {
            "cjdb"
        }) {
            return Ok(path);
        }

        Err(zed::Error::NotFound(
            "未找到 cjdb 工具，请配置 CANGJIE_HOME 或确保 cjdb 在 PATH 中".to_string(),
        ))
    }

    /// 加载 cjdb 配置
    pub fn load_config(worktree: &zed::Worktree) -> zed::Result<CjdbConfig> {
        // 1. 项目根目录 cjdb.toml
        let project_config = worktree.path().join("cjdb.toml");
        if project_config.exists() {
            return Self::parse_config(&project_config);
        }

        // 2. 用户目录 .cjdb.toml
        if let Some(user_config) = Self::user_config_path() {
            if user_config.exists() {
                return Self::parse_config(&user_config);
            }
        }

        // 3. 默认配置
        Ok(CjdbConfig::default())
    }

    /// 解析配置文件
    fn parse_config(path: &zed::Path) -> zed::Result<CjdbConfig> {
        let content = zed::fs::read_to_string(path)
            .map_err(|e| zed::Error::IoError(format!("读取 {} 失败: {}", path.to_str()?, e)))?;

        toml::from_str(&content)
            .map_err(|e| zed::Error::InvalidConfig(format!("解析 {} 失败: {}", path.to_str()?, e)))
    }

    /// 用户目录配置路径
    fn user_config_path() -> Option<zed::Path> {
        zed::env::home_dir().map(|home| home.join(".cjdb.toml"))
    }

    /// 启动调试会话（支持性能采样联动）
    pub fn start_debug_session(
        worktree: &zed::Worktree,
        config: &CjdbConfig,
        target_binary: &str,
        args: &[String],
    ) -> zed::Result<zed::DebugSession> {
        let cjdb_path = Self::find_executable()?;
        let mut command_args = vec![
            "debug".to_string(),
            target_binary.to_string(),
            "--port".to_string(),
            config.session.port.to_string(),
        ];

        // 添加目标程序参数
        if !args.is_empty() {
            command_args.push("--".to_string());
            command_args.extend(args.iter().cloned());
        }

        // 启用协程调试
        if config.session.enable_coroutine_debug {
            command_args.push("--enable-coroutine-debug".to_string());
        }

        // 性能采样联动
        if config.performance.enable_profiling {
            command_args.push("--enable-profiling".to_string());
            command_args.push(format!(
                "--profiling-types={}",
                config.performance.profiling_types.join(",")
            ));
            command_args.push(format!(
                "--profiling-interval={}",
                config.performance.profiling_interval
            ));
        }

        // 构建调试命令
        let debug_command = zed::DebugCommand {
            command: cjdb_path.to_str()?.to_string(),
            args: command_args,
            env: zed::EnvMap::new(),
            cwd: worktree.path().to_str()?.to_string(),
        };

        // 创建调试会话
        zed::debug::start_session(zed::DebugSessionConfig {
            command: debug_command,
            port: config.session.port,
            timeout_ms: config.session.timeout * 1000,
            adapter_type: "Cangjie".to_string(),
        })
    }

    /// 设置断点
    pub fn set_breakpoint(
        session: &mut zed::DebugSession,
        file_path: &str,
        line: u32,
        condition: Option<&str>,
    ) -> zed::Result<zed::BreakpointId> {
        session.set_breakpoint(zed::Breakpoint {
            file: file_path.to_string(),
            line,
            column: 0,
            condition: condition.map(|s| s.to_string()),
            log_message: None,
            enabled: true,
        })
    }
}
