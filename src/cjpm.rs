//! 包管理工具 cjpm 集成
use serde::{Deserialize, Serialize};
use zed_extension_api;

/// cjpm 构建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// 是否发布模式
    pub release: bool,
    /// 目标架构
    pub target: String,
    /// 启用调试信息
    pub debug_info: bool,
    /// 优化级别（0-3）
    pub opt_level: u8,
    /// 链接器参数
    pub linker_args: Vec<String>,
}

/// cjpm 依赖配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyConfig {
    /// 依赖名称
    pub name: String,
    /// 版本号
    pub version: String,
    /// 依赖来源（crates.io/git/local）
    pub source: Option<String>,
    /// 仅开发环境依赖
    pub dev: bool,
}

/// cjpm 项目配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CjpmConfig {
    /// 项目名称
    pub name: String,
    /// 项目版本
    pub version: String,
    /// 作者
    pub authors: Vec<String>,
    /// 构建配置
    pub build: BuildConfig,
    /// 依赖列表
    pub dependencies: Vec<DependencyConfig>,
    /// 开发依赖列表
    pub dev_dependencies: Vec<DependencyConfig>,
    /// 目标产物名称
    pub target_name: Option<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            release: false,
            target: "x86_64-unknown-linux-gnu".to_string(),
            debug_info: true,
            opt_level: 0,
            linker_args: Vec::new(),
        }
    }
}

impl Default for CjpmConfig {
    fn default() -> Self {
        Self {
            name: "untitled".to_string(),
            version: "0.1.0".to_string(),
            authors: Vec::new(),
            build: BuildConfig::default(),
            dependencies: Vec::new(),
            dev_dependencies: Vec::new(),
            target_name: None,
        }
    }
}

/// cjpm 管理器
#[derive(Debug, Default)]
pub struct CjpmManager;

impl CjpmManager {
    /// 检查 cjpm 是否可用
    pub fn is_available() -> zed_extension_api::Result<()> {
        if std::process::Command::new("cjpm")
            .arg("--version")
            .output()
            .is_err()
        {
            return Err(zed_extension_api::Error::NotFound(
                "cjpm 工具未找到，请安装并配置到 PATH 中".to_string(),
            ));
        }
        Ok(())
    }

    /// 加载 cjpm.toml 配置
    pub fn load_config(
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<CjpmConfig> {
        let config_path = worktree.path().join("cjpm.toml");
        if !config_path.exists() {
            return Err(zed_extension_api::Error::NotFound(
                "未找到 cjpm.toml 项目配置文件".to_string(),
            ));
        }

        let config_content = std::fs::read_to_string(&config_path)
            .map_err(|e| zed_extension_api::Error::IoError(format!("读取 cjpm 配置失败: {}", e)))?;

        let config: CjpmConfig = toml::from_str(&config_content).map_err(|e| {
            zed_extension_api::Error::InvalidData(format!("解析 cjpm 配置失败: {}", e))
        })?;

        Ok(config)
    }

    /// 安装项目依赖
    pub fn install_dependencies(
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<()> {
        Self::is_available()?;

        let output = std::process::Command::new("cjpm")
            .arg("install")
            .current_dir(worktree.path())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(zed_extension_api::Error::ProcessFailed(format!(
                "依赖安装失败: {}",
                stderr
            )));
        }

        Ok(())
    }

    /// 构建项目
    pub fn build_project(
        worktree: &zed_extension_api::Worktree,
        config: &CjpmConfig,
    ) -> zed_extension_api::Result<()> {
        Self::is_available()?;

        let mut args = vec!["build".to_string()];

        // 添加构建参数
        if config.build.release {
            args.push("--release".to_string());
        }
        args.push(format!("--target={}", config.build.target));
        args.push(format!("--opt-level={}", config.build.opt_level));
        if !config.build.debug_info {
            args.push("--no-debug".to_string());
        }
        for arg in &config.build.linker_args {
            args.push(format!("-C linker-args={}", arg));
        }

        let output = std::process::Command::new("cjpm")
            .args(&args)
            .current_dir(worktree.path())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(zed_extension_api::Error::ProcessFailed(format!(
                "项目构建失败: {}",
                stderr
            )));
        }

        Ok(())
    }

    /// 自动识别构建目标产物路径
    pub fn auto_detect_target(
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<String> {
        let config = Self::load_config(worktree)?;
        let target_name = config.target_name.as_ref().unwrap_or(&config.name);

        // 构建产物路径：target/<target>/<release|debug>/<target_name>
        let build_dir = if config.build.release {
            "release"
        } else {
            "debug"
        };

        let target_path = worktree
            .path()
            .join("target")
            .join(&config.build.target)
            .join(build_dir)
            .join(target_name)
            // 添加系统后缀
            .with_extension(if cfg!(windows) { "exe" } else { "" });

        let target_str = target_path
            .to_str()
            .ok_or_else(|| zed_extension_api::Error::InvalidData("目标产物路径无效".to_string()))?;

        if !target_path.exists() {
            return Err(zed_extension_api::Error::NotFound(format!(
                "未找到目标产物: {}",
                target_str
            )));
        }

        Ok(target_str.to_string())
    }
}
