use std::collections::HashMap;
use std::env;
use zed::LanguageServerId;
use zed::serde_json::Value;
use zed_extension_api::Worktree;
use zed_extension_api::{self as zed, settings::LspSettings};

// 仓颉语言服务器处理器
struct CangjieLanguageServer;

impl CangjieLanguageServer {
    // 语言服务器唯一标识（保留原始ID）
    const LANGUAGE_SERVER_ID: &'static str = "cangjie-language-server";

    fn new() -> Self {
        CangjieLanguageServer {}
    }

    // 获取语言服务器二进制路径（保留原始路径拼接逻辑）
    fn language_server_binary_path(&self, _: &LanguageServerId) -> zed::Result<String> {
        // 保留原始环境变量获取方式，添加错误处理
        let cangjie_home =
            env::var("CANGJIE_HOME").map_err(|_| "CANGJIE_HOME environment variable not set")?;
        let binary_path = format!("{}/tools/bin/LSPServer.exe", cangjie_home);
        Ok(binary_path)
    }
}

struct CangjieExtension {
    cangjie_language_server: Option<CangjieLanguageServer>, // 保留单例模式
}

impl zed::Extension for CangjieExtension {
    fn new() -> Self {
        Self {
            cangjie_language_server: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        match language_server_id.as_ref() {
            CangjieLanguageServer::LANGUAGE_SERVER_ID => {
                let cangjie_language_server = self
                    .cangjie_language_server
                    .get_or_insert_with(CangjieLanguageServer::new);

                // 保留原始二进制路径获取逻辑
                let binary_path =
                    cangjie_language_server.language_server_binary_path(language_server_id)?;

                // 保留动态库路径构建逻辑
                let cangjie_home = env::var("CANGJIE_HOME")
                    .map_err(|_| "CANGJIE_HOME environment variable not set")?;
                let lib_path = format!("{}/runtime/lib/windows_x86_64_llvm", cangjie_home);

                // 保留环境变量设置
                let mut env_map = HashMap::new();
                env_map.insert("CANGJIE_HOME".to_string(), cangjie_home);
                env_map.insert("DYLD_LIBRARY_PATH".to_string(), lib_path.clone());
                env_map.insert("LD_LIBRARY_PATH".to_string(), lib_path);

                Ok(zed::Command {
                    command: binary_path,
                    args: vec![
                        // 保留原始参数
                        "src".to_string(),
                        "--disableAutoImport".to_string(),
                        "--enable-log=true".to_string(),
                    ],
                    env: env_map.into_iter().collect(),
                })
            }
            _ => Err(format!(
                "Unrecognized language server for Cangjie: {language_server_id}"
            )),
        }
    }

    // 保留工作区配置逻辑，恢复原始注释
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        if let Ok(Some(settings)) = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.settings)
        {
            Ok(Some(settings))
        } else {
            self.language_server_initialization_options(language_server_id, worktree)
                .map(|init_options| {
                    init_options.and_then(|init_options| init_options.get("settings").cloned())
                })
        }
    }

    // 保留原始注释的配置逻辑（作为可选实现）
    // fn language_server_workspace_configuration(
    //     &mut self,
    //     language_server_id: &LanguageServerId,
    //     worktree: &zed_extension_api::Worktree,
    // ) -> Result<Option<serde_json::Value>> {
    //     let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
    //         .ok()
    //         .and_then(|lsp_settings| lsp_settings.settings.clone())
    //         .unwrap_or_default();
    //     Ok(Some(serde_json::json!({
    //         "cangjie": settings
    //     })))
    // }
}

zed::register_extension!(CangjieExtension);
