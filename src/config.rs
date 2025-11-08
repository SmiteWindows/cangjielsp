//! 扩展配置管理（完全基于 API 配置系统）
use serde::{Deserialize, Serialize};
use zed_extension_api as zed;

/// 仓颉LSP配置（与 API 配置系统强绑定）
/// 字段类型、默认值均遵循 API 推荐规范
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct CangjieLspConfig {
    /// 禁用自动导入（布尔类型，API 配置系统原生支持）
    #[serde(default = "default_disable_auto_import")]
    pub disable_auto_import: bool,

    /// 启用日志（与 API 日志系统对齐）
    #[serde(default = "default_enable_log")]
    pub enable_log: bool,

    /// 日志级别（直接使用 API 定义的 LogLevel 枚举）
    #[serde(default = "default_log_level")]
    pub log_level: zed::LogLevel,

    /// LSP 工作目录（使用 API 提供的 Path 类型）
    #[serde(default)]
    pub workspace_dir: Option<zed::Path>,
}

// 配置默认值（严格遵循 API 类型约束）
fn default_disable_auto_import() -> bool {
    true
}
fn default_enable_log() -> bool {
    true
}
fn default_log_level() -> zed::LogLevel {
    zed::LogLevel::Info
}

impl CangjieLspConfig {
    /// 从 Zed 工作区加载配置（API 推荐的配置加载流程）
    pub fn from_worktree(worktree: &zed::Worktree) -> zed::Result<Self> {
        // 必须使用 API 提供的 LspSettings::for_worktree 加载配置
        let lsp_settings = zed::settings::LspSettings::for_worktree(
            language_server::CangjieLanguageServer::LANGUAGE_SERVER_ID,
            worktree,
        )?;

        // 配置反序列化必须使用 API 内置的 serde 工具链
        match lsp_settings.settings {
            Some(config_val) => serde_json::from_value(config_val)
                .map_err(|err| zed::Error::InvalidConfig(format!("配置解析失败: {}", err))),
            None => Ok(Self::default()),
        }
    }

    /// 生成 LSP 启动参数（API 命令参数格式要求）
    pub fn to_args(&self, worktree: &zed::Worktree) -> zed::Result<Vec<String>> {
        let mut args = Vec::new();

        // 工作目录参数（优先使用配置，其次使用工作区根目录）
        let workspace_dir = match &self.workspace_dir {
            Some(dir) => dir,
            None => worktree.path(),
        };
        args.push("--workspace".to_string());
        args.push(workspace_dir.to_str()?.to_string());

        // 自动导入开关
        if self.disable_auto_import {
            args.push("--disable-auto-import".to_string());
        }

        // 日志配置（与 API 日志级别字符串格式对齐）
        args.push(format!("--log-enabled={}", self.enable_log));
        args.push(format!("--log-level={}", self.log_level.as_str()));

        Ok(args)
    }
}
