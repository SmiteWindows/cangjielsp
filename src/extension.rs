//! Zed 扩展接口实现（严格实现 API 定义的 Extension trait）
use serde_json::Value;
use zed::LanguageServerId;
use zed_extension_api as zed;

/// 仓颉扩展主结构体（API 规范要求的 Default 实现）
#[derive(Debug, Default)]
pub struct CangjieExtension {
    /// 语言服务器实例（延迟初始化，API 推荐实践）
    language_server: Option<language_server::CangjieLanguageServer>,
}

impl zed::Extension for CangjieExtension {
    /// 初始化扩展（API 必需接口）
    fn new() -> Self {
        // 初始化日志（API 日志系统）
        zed::log::info!("仓颉扩展初始化（遵循 Zed Extension API 规范）");
        Self::default()
    }

    /// 获取语言服务器启动命令（API 核心接口）
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // 1. 验证语言服务器 ID（必须完全匹配）
        if language_server_id.as_ref() != language_server::CangjieLanguageServer::LANGUAGE_SERVER_ID
        {
            return Err(zed::Error::InvalidRequest(format!(
                "未识别的语言服务器 ID: {}，仅支持 {}",
                language_server_id.as_ref(),
                language_server::CangjieLanguageServer::LANGUAGE_SERVER_ID
            )));
        }

        // 2. 延迟初始化语言服务器（API 推荐的惰性加载）
        let server = self
            .language_server
            .get_or_insert_with(language_server::CangjieLanguageServer::default);

        // 3. 构建命令并返回（完全依赖 LSP 模块逻辑）
        let command = server.build_command(worktree)?;
        zed::log::debug!("LSP 命令生成成功: {:?}", command);
        Ok(command)
    }

    /// 获取工作区配置（API 必需接口）
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<Value>> {
        // 仅处理仓颉 LSP 的配置请求
        if language_server_id.as_ref() != language_server::CangjieLanguageServer::LANGUAGE_SERVER_ID
        {
            return Ok(None);
        }

        // 复用配置加载逻辑，确保一致性
        config::CangjieLspConfig::from_worktree(worktree).map(|config| {
            Some(
                serde_json::to_value(config)
                    .map_err(|err| zed::Error::InvalidConfig(format!("配置序列化失败: {}", err)))?,
            )
        })
    }

    /// 获取初始化选项（API 推荐接口，增强兼容性）
    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<Value>> {
        // 初始化选项与工作区配置保持一致（API 推荐做法）
        self.language_server_workspace_configuration(language_server_id, worktree)
    }

    /// 扩展销毁（API 可选接口，资源清理）
    fn destroy(&mut self) -> zed::Result<()> {
        zed::log::info!("仓颉扩展销毁，清理资源");
        self.language_server.take();
        Ok(())
    }

    /// 扩展元信息（API 可选接口，增强可读性）
    fn meta(&self) -> &zed::ExtensionMeta {
        // 直接返回静态元信息，与 extension.toml 一致
        static META: zed::ExtensionMeta = zed::ExtensionMeta {
            name: "cangjie",
            display_name: "Cangjie",
            description: "仓颉语言官方 Zed 扩展（严格遵循 Zed Extension API 规范）",
            version: "0.4.0",
            author: "Smite Rust <https://github.com/SmiteWindows>",
            repository: "https://github.com/SmiteWindows/cangjielsp",
            license: "MIT",
            categories: &["languages", "debuggers", "build-tools"],
            keywords: &["cangjie", "仓颉", "lsp", "zed", "official"],
        };
        &META
    }
}
