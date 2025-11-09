//! 调试工具 cjdb 集成
use serde::{Deserialize, Serialize};
use zed_extension_api;

/// 调试会话配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// 调试端口
    pub port: u16,
    /// 等待客户端连接（阻塞模式）
    pub wait_for_client: bool,
    /// 启用日志
    pub enable_log: bool,
    /// 日志路径
    pub log_path: Option<String>,
}

/// 断点配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointConfig {
    /// 启用条件断点
    pub enable_conditional: bool,
    /// 启用日志断点
    pub enable_log: bool,
    /// 忽略异常断点
    pub ignore_exceptions: bool,
}

/// cjdb 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CjdbConfig {
    /// 调试会话配置
    pub session: SessionConfig,
    /// 断点配置
    pub breakpoint: BreakpointConfig,
    /// 启用源码映射
    pub enable_source_map: bool,
    /// 调试超时时间（秒）
    pub timeout: u32,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            port: 5678,
            wait_for_client: true,
            enable_log: false,
            log_path: None,
        }
    }
}

impl Default for BreakpointConfig {
    fn default() -> Self {
        Self {
            enable_conditional: true,
            enable_log: true,
            ignore_exceptions: false,
        }
    }
}

impl Default for CjdbConfig {
    fn default() -> Self {
        Self {
            session: SessionConfig::default(),
            breakpoint: BreakpointConfig::default(),
            enable_source_map: true,
            timeout: 300,
        }
    }
}

/// cjdb 管理器
#[derive(Debug, Default)]
pub struct CjdbManager;

impl CjdbManager {
    /// 检查 cjdb 是否可用
    pub fn is_available() -> zed_extension_api::Result<()> {
        if std::process::Command::new("cjdb")
            .arg("--version")
            .output()
            .is_err()
        {
            return Err(zed_extension_api::Error::NotFound(
                "cjdb 工具未找到，请安装并配置到 PATH 中".to_string(),
            ));
        }
        Ok(())
    }

    /// 加载配置
    pub fn load_config(
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<CjdbConfig> {
        // 加载 .cjdb.toml 配置（如果存在）
        let config_path = worktree.path().join(".cjdb.toml");
        if config_path.exists() {
            let config_content = std::fs::read_to_string(&config_path).map_err(|e| {
                zed_extension_api::Error::IoError(format!("读取 cjdb 配置失败: {}", e))
            })?;
            let config: CjdbConfig = toml::from_str(&config_content).map_err(|e| {
                zed_extension_api::Error::InvalidData(format!("解析 cjdb 配置失败: {}", e))
            })?;
            return Ok(config);
        }

        // 使用默认配置
        Ok(CjdbConfig::default())
    }

    /// 启动调试会话
    pub fn start_debug_session(
        worktree: &zed_extension_api::Worktree,
        config: &CjdbConfig,
        target_binary: &str,
        args: &[String],
    ) -> zed_extension_api::Result<zed_extension_api::debug::Session> {
        Self::is_available()?;

        let mut command_args = vec!["debug".to_string()];

        // 添加调试配置参数
        command_args.push(format!("--port={}", config.session.port));
        if config.session.wait_for_client {
            command_args.push("--wait".to_string());
        }
        if config.session.enable_log {
            command_args.push("--log".to_string());
            if let Some(log_path) = &config.session.log_path {
                command_args.push(format!("--log-path={}", log_path));
            }
        }
        if !config.breakpoint.enable_conditional {
            command_args.push("--no-conditional-breakpoints".to_string());
        }
        if !config.breakpoint.enable_log {
            command_args.push("--no-log-breakpoints".to_string());
        }
        if config.breakpoint.ignore_exceptions {
            command_args.push("--ignore-exceptions".to_string());
        }
        if !config.enable_source_map {
            command_args.push("--no-source-map".to_string());
        }
        command_args.push(format!("--timeout={}", config.timeout));

        // 添加目标程序和参数
        command_args.push(target_binary.to_string());
        command_args.extend_from_slice(args);

        // 创建调试会话
        let session = zed_extension_api::debug::Session::new(
            "cjdb".to_string(),
            worktree.path().to_path_buf(),
            "cjdb".to_string(),
            command_args,
            zed_extension_api::debug::SessionOptions {
                port: config.session.port,
                ..zed_extension_api::debug::SessionOptions::default()
            },
        )?;

        Ok(session)
    }
}
