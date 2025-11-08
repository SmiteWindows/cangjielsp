//! Zed扩展接口实现
//! 职责：严格实现Zed Extension API，对接语言服务器和Zed编辑器

use super::language_server::CangjieLanguageServer;
use serde_json::Value;
use zed::LanguageServerId;
use zed_extension_api as zed;

/// 仓颉扩展主结构体
#[derive(Debug, Default)]
pub struct CangjieExtension {
    /// 语言服务器实例（延迟初始化）
    language_server: Option<CangjieLanguageServer>,
}

impl zed::Extension for CangjieExtension {
    /// 初始化扩展（Zed API 必需）
    fn new() -> Self {
        Self::default()
    }

    /// 获取语言服务器启动命令（Zed API 核心接口）
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // 验证语言服务器ID
        if language_server_id.as_ref() != CangjieLanguageServer::LANGUAGE_SERVER_ID {
            return Err(zed::Error::InvalidRequest(format!(
                "未识别的语言服务器ID: {}",
                language_server_id.as_ref()
            )));
        }

        // 延迟初始化语言服务器
        let server = self
            .language_server
            .get_or_insert_with(CangjieLanguageServer::default);

        // 构建并返回命令
        server.build_command(worktree)
    }

    /// 获取工作区配置（Zed API 必需接口）
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<Value>> {
        // 仅处理仓颉语言服务器的配置请求
        if language_server_id.as_ref() != CangjieLanguageServer::LANGUAGE_SERVER_ID {
            return Ok(None);
        }

        // 从Zed工作区配置加载，与 config.rs 逻辑一致
        super::config::CangjieLspConfig::from_worktree(worktree)
            .map(|config| Some(serde_json::to_value(config).unwrap()))
    }

    /// 获取初始化选项（Zed API 可选接口，增强兼容性）
    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<Value>> {
        // 复用工作区配置逻辑，保持配置一致性
        self.language_server_workspace_configuration(language_server_id, worktree)
    }
}
