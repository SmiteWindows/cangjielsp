//! cjpm 包管理工具集成（构建、依赖管理、目标产物识别）
use serde::{Deserialize, Serialize};
use std::path::Path;
use zed_extension_api as zed;

/// cjpm 配置（对应 cjpm.toml）
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CjpmConfig {
    pub package: PackageConfig,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
    pub build: BuildConfig,
}

/// 包信息配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
}

/// 构建配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct BuildConfig {
    pub target: String,
    pub release: bool,
    pub features: Vec<String>,
    pub rustc_flags: Vec<String>,
}

/// cjpm 管理器
#[derive(Debug, Default)]
pub struct CjpmManager;

impl CjpmManager {
    /// 检查 cjpm 是否可用
    pub fn is_available() -> zed::Result<()> {
        Self::find_executable()?;
        Ok(())
    }

    /// 查找 cjpm 可执行文件
    pub fn find_executable() -> zed::Result<zed::Path> {
        // 1. 从 CANGJIE_HOME 查找
        if let Ok(cangjie_home) = zed::env::var("CANGJIE_HOME") {
            let mut path = zed::Path::new(&cangjie_home);
            path.push("bin");
            path.push(if zed::platform::is_windows() {
                "cjpm.exe"
            } else {
                "cjpm"
            });

            if path.exists() && path.is_executable() {
                return Ok(path);
            }
        }

        // 2. 从 PATH 查找
        if let Some(path) = zed::env::find_executable(if zed::platform::is_windows() {
            "cjpm.exe"
        } else {
            "cjpm"
        }) {
            return Ok(path);
        }

        Err(zed::Error::NotFound(
            "未找到 cjpm 工具，请配置 CANGJIE_HOME 或确保 cjpm 在 PATH 中".to_string(),
        ))
    }

    /// 加载 cjpm 配置
    pub fn load_config(worktree: &zed::Worktree) -> zed::Result<CjpmConfig> {
        let config_path = worktree.path().join("cjpm.toml");
        if !config_path.exists() {
            return Err(zed::Error::NotFound(
                "未找到 cjpm.toml 配置文件".to_string(),
            ));
        }

        let content = zed::fs::read_to_string(&config_path)
            .map_err(|e| zed::Error::IoError(format!("读取 cjpm.toml 失败: {}", e)))?;

        toml::from_str(&content)
            .map_err(|e| zed::Error::InvalidConfig(format!("解析 cjpm.toml 失败: {}", e)))
    }

    /// 自动识别构建目标产物路径
    pub fn auto_detect_target(worktree: &zed::Worktree) -> zed::Result<String> {
        let config = Self::load_config(worktree)?;
        let target_name = &config.package.name;
        let build_type = if config.build.release {
            "release"
        } else {
            "debug"
        };

        // 构建目标路径：target/{build_type}/{target_name}[.exe]
        let mut target_path = worktree.path().join("target");
        target_path.push(build_type);
        target_path.push(if zed::platform::is_windows() {
            format!("{}.exe", target_name)
        } else {
            target_name.to_string()
        });

        if !target_path.exists() {
            // 尝试构建项目
            self.build_project(worktree, &config)?;
        }

        target_path
            .to_str()
            .ok_or_else(|| zed::Error::InvalidPath("目标产物路径无效".to_string()))
            .map(|s| s.to_string())
    }

    /// 构建项目（执行 cjpm build）
    pub fn build_project(worktree: &zed::Worktree, config: &CjpmConfig) -> zed::Result<()> {
        let cjpm_path = Self::find_executable()?;
        let mut args = vec!["build".to_string()];

        // 添加构建参数
        if config.build.release {
            args.push("--release".to_string());
        }
        if !config.build.features.is_empty() {
            args.push(format!("--features={}", config.build.features.join(",")));
        }

        // 执行构建命令
        let output = zed::process::Command::new(cjpm_path.to_str()?)
            .args(&args)
            .current_dir(worktree.path())
            .output()
            .map_err(|e| zed::Error::ProcessFailed(format!("启动 cjpm build 失败: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(zed::Error::ProcessFailed(format!(
                "项目构建失败: {}",
                stderr.trim()
            )));
        }

        Ok(())
    }

    /// 安装依赖（执行 cjpm install）
    pub fn install_dependencies(worktree: &zed::Worktree) -> zed::Result<()> {
        let cjpm_path = Self::find_executable()?;

        let output = zed::process::Command::new(cjpm_path.to_str()?)
            .arg("install")
            .current_dir(worktree.path())
            .output()
            .map_err(|e| zed::Error::ProcessFailed(format!("启动 cjpm install 失败: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(zed::Error::ProcessFailed(format!(
                "依赖安装失败: {}",
                stderr.trim()
            )));
        }

        Ok(())
    }
}
