//! 扩展配置管理，封装语言服务器、环境变量等配置逻辑
//! 职责：集中管理配置相关逻辑，避免配置散落在业务代码中

use super::utils;
use serde::Deserialize;
use zed_extension_api as zed;

/// 仓颉语言服务器配置（与 extension.toml 对齐）
#[derive(Debug, Deserialize, Default)]
pub struct CangjieLspConfig {
    /// 禁用自动导入
    pub disable_auto_import: bool,
    /// 启用日志
    pub enable_log: bool,
    /// 日志级别
    pub log_level: Option<String>,
}

impl CangjieLspConfig {
    /// 从Zed工作区配置加载
    pub fn from_worktree(worktree: &zed::Worktree) -> zed::Result<Self> {
        zed::settings::LspSettings::for_worktree(
            super::language_server::CangjieLanguageServer::LANGUAGE_SERVER_ID,
            worktree,
        )
        .map(|settings| {
            settings
                .settings
                .unwrap_or_else(|| serde_json::Value::Object(Default::default()))
        })
        .map(|value| serde_json::from_value(value).unwrap_or_default())
    }

    /// 生成语言服务器启动参数
    pub fn to_args(&self) -> Vec<String> {
        let mut args = vec!["src".to_string()];

        if self.disable_auto_import {
            args.push("--disableAutoImport".to_string());
        }

        args.push(format!("--enable-log={}", self.enable_log));

        if let Some(level) = &self.log_level {
            args.push(format!("--log-level={}", level));
        }

        args
    }
}

/// 环境变量配置
pub fn build_env_map(lib_path: &str) -> zed::Result<Vec<(String, String)>> {
    let os_type = utils::OsType::current();
    let mut env_map = Vec::new();

    // 必须添加 CANGJIE_HOME
    let cangjie_home = utils::get_env_var("CANGJIE_HOME")?;
    env_map.push(("CANGJIE_HOME".to_string(), cangjie_home));

    // 根据操作系统设置库路径
    if let Some(env_var) = os_type.lib_env_var() {
        let existing = env::var(env_var).ok();
        let separator = if os_type == utils::OsType::Windows {
            ";"
        } else {
            ":"
        };
        let new_value = utils::merge_env_var(existing, lib_path, separator);
        env_map.push((env_var.to_string(), new_value));
    } else {
        return Err(zed::Error::Unsupported(format!(
            "不支持的操作系统，无法设置动态库路径"
        )));
    }

    Ok(env_map)
}
