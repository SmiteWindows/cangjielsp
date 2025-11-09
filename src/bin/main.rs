//! 仓颉 LSP 独立运行入口（适配 zed_extension_api 0.7.0）
use log::{debug, error, info};
use std::sync::Arc;
use tree_sitter::Point as TsPoint;

use cangjie_lsp::{
    config::CangjieConfig, language_server::CangjieLanguageServer, tree_sitter_utils,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    info!(
        "仓颉 LSP 独立服务器启动（版本: {}）",
        cangjie_lsp::EXTENSION_VERSION
    );

    // 初始化 tree-sitter 解析器
    tree_sitter_utils::init_parser();

    // 加载配置
    let config = Arc::new(CangjieConfig::default());

    // 创建 LSP 服务器
    let mut lsp_server = CangjieLanguageServer::new(config.clone());

    // 初始化工作目录（当前目录）
    let worktree = zed_extension_api::Worktree::new(zed_extension_api::Path::new("."));
    lsp_server.initialize(worktree.clone())?;

    info!("LSP 服务器初始化完成，监听 stdio 通信...");

    // 启动 LSP 通信循环
    let (reader, writer) = (tokio::io::stdin(), tokio::io::stdout());
    let mut lsp_transport = zed_extension_api::lsp::Transport::new(reader, writer);

    loop {
        match lsp_transport.read_message().await {
            Ok(Some(message)) => {
                debug!("收到 LSP 消息: {:?}", message);
                let response = handle_lsp_message(&mut lsp_server, &worktree, message)?;
                if let Some(response) = response {
                    lsp_transport.write_message(response).await?;
                }
            }
            Ok(None) => {
                info!("客户端断开连接");
                break;
            }
            Err(e) => {
                error!("LSP 通信错误: {}", e);
                break;
            }
        }
    }

    info!("LSP 服务器退出");
    Ok(())
}

/// 处理 LSP 消息
fn handle_lsp_message(
    lsp_server: &mut CangjieLanguageServer,
    worktree: &zed_extension_api::Worktree,
    message: zed_extension_api::lsp::Message,
) -> zed_extension_api::Result<Option<zed_extension_api::lsp::Message>> {
    match message {
        zed_extension_api::lsp::Message::Request(request) => {
            handle_request(lsp_server, worktree, request)
        }
        zed_extension_api::lsp::Message::Response(_) => {
            debug!("收到 LSP 响应，忽略");
            Ok(None)
        }
        zed_extension_api::lsp::Message::Notification(notification) => {
            handle_notification(lsp_server, worktree, notification)?;
            Ok(None)
        }
    }
}

/// 处理 LSP 请求
fn handle_request(
    lsp_server: &mut CangjieLanguageServer,
    worktree: &zed_extension_api::Worktree,
    request: zed_extension_api::lsp::Request,
) -> zed_extension_api::Result<Option<zed_extension_api::lsp::Message>> {
    match request.method.as_str() {
        "initialize" => {
            // 处理初始化请求
            let response = zed_extension_api::lsp::Response {
                id: request.id,
                result: Some(serde_json::to_value(
                    zed_extension_api::lsp::InitializeResult {
                        capabilities: zed_extension_api::lsp::ServerCapabilities {
                            document_formatting_provider: Some(true),
                            document_symbol_provider: Some(true),
                            workspace_symbol_provider: Some(true),
                            completion_provider: Some(zed_extension_api::lsp::CompletionOptions {
                                trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                                ..zed_extension_api::lsp::CompletionOptions::default()
                            }),
                            definition_provider: Some(true),
                            ..zed_extension_api::lsp::ServerCapabilities::default()
                        },
                        server_info: Some(zed_extension_api::lsp::ServerInfo {
                            name: "cangjie-lsp".to_string(),
                            version: Some(cangjie_lsp::EXTENSION_VERSION.to_string()),
                        }),
                    },
                )?),
                error: None,
            };
            Ok(Some(zed_extension_api::lsp::Message::Response(response)))
        }
        "textDocument/completion" => {
            // 处理补全请求
            let params: zed_extension_api::lsp::CompletionParams =
                serde_json::from_value(request.params)?;
            let document_uri = &params.text_document_position.text_document.uri;
            let document =
                zed_extension_api::Document::open(zed_extension_api::Path::from_uri(document_uri))?;
            let position = params.text_document_position.position;

            let completion_items = lsp_server.completion(&document, position)?;
            let response = zed_extension_api::lsp::Response {
                id: request.id,
                result: Some(serde_json::to_value(
                    zed_extension_api::lsp::CompletionList {
                        is_incomplete: false,
                        items: completion_items,
                    },
                )?),
                error: None,
            };
            Ok(Some(zed_extension_api::lsp::Message::Response(response)))
        }
        "textDocument/definition" => {
            // 处理跳转定义请求
            let params: zed_extension_api::lsp::DefinitionParams =
                serde_json::from_value(request.params)?;
            let document_uri = &params.text_document_position_params.text_document.uri;
            let document =
                zed_extension_api::Document::open(zed_extension_api::Path::from_uri(document_uri))?;
            let position = params.text_document_position_params.position;

            let locations = lsp_server.goto_definition(&document, position)?;
            let response = zed_extension_api::lsp::Response {
                id: request.id,
                result: Some(serde_json::to_value(locations)?),
                error: None,
            };
            Ok(Some(zed_extension_api::lsp::Message::Response(response)))
        }
        "textDocument/documentSymbol" => {
            // 处理文档符号请求
            let params: zed_extension_api::lsp::DocumentSymbolParams =
                serde_json::from_value(request.params)?;
            let document_uri = &params.text_document.uri;
            let document =
                zed_extension_api::Document::open(zed_extension_api::Path::from_uri(document_uri))?;

            let symbols = lsp_server.document_symbols(&document)?;
            let response = zed_extension_api::lsp::Response {
                id: request.id,
                result: Some(serde_json::to_value(symbols)?),
                error: None,
            };
            Ok(Some(zed_extension_api::lsp::Message::Response(response)))
        }
        "textDocument/formatting" => {
            // 处理格式化请求
            let params: zed_extension_api::lsp::DocumentFormattingParams =
                serde_json::from_value(request.params)?;
            let document_uri = &params.text_document.uri;
            let mut document =
                zed_extension_api::Document::open(zed_extension_api::Path::from_uri(document_uri))?;

            let cjfmt_config =
                cangjie_lsp::cjfmt::CjfmtManager::load_config(worktree, &CangjieConfig::default())?;
            let edits = cangjie_lsp::cjfmt::CjfmtManager::format_document(
                worktree,
                &document,
                &cjfmt_config,
            )?;

            let response = zed_extension_api::lsp::Response {
                id: request.id,
                result: Some(serde_json::to_value(edits.unwrap_or_default())?),
                error: None,
            };
            Ok(Some(zed_extension_api::lsp::Message::Response(response)))
        }
        "shutdown" => {
            // 处理关闭请求
            let response = zed_extension_api::lsp::Response {
                id: request.id,
                result: Some(serde_json::Value::Null),
                error: None,
            };
            Ok(Some(zed_extension_api::lsp::Message::Response(response)))
        }
        _ => {
            error!("不支持的 LSP 请求: {}", request.method);
            let response = zed_extension_api::lsp::Response {
                id: request.id,
                result: None,
                error: Some(zed_extension_api::lsp::ResponseError {
                    code: zed_extension_api::lsp::ErrorCode::MethodNotFound as i32,
                    message: format!("不支持的方法: {}", request.method),
                    data: None,
                }),
            };
            Ok(Some(zed_extension_api::lsp::Message::Response(response)))
        }
    }
}

/// 处理 LSP 通知
fn handle_notification(
    lsp_server: &mut CangjieLanguageServer,
    _worktree: &zed_extension_api::Worktree,
    notification: zed_extension_api::lsp::Notification,
) -> zed_extension_api::Result<()> {
    match notification.method.as_str() {
        "textDocument/didOpen" => {
            let params: zed_extension_api::lsp::DidOpenTextDocumentParams =
                serde_json::from_value(notification.params)?;
            let document_uri = &params.text_document.uri;
            let document =
                zed_extension_api::Document::open(zed_extension_api::Path::from_uri(document_uri))?;
            lsp_server.did_open(&document)?;
        }
        "textDocument/didChange" => {
            let params: zed_extension_api::lsp::DidChangeTextDocumentParams =
                serde_json::from_value(notification.params)?;
            let document_uri = &params.text_document.uri;
            let document =
                zed_extension_api::Document::open(zed_extension_api::Path::from_uri(document_uri))?;
            lsp_server.did_change(&document)?;
        }
        "textDocument/didClose" => {
            let params: zed_extension_api::lsp::DidCloseTextDocumentParams =
                serde_json::from_value(notification.params)?;
            let document_uri = &params.text_document.uri;
            let document =
                zed_extension_api::Document::open(zed_extension_api::Path::from_uri(document_uri))?;
            lsp_server.did_close(&document);
        }
        "exit" => {
            info!("收到退出通知，关闭服务器");
            std::process::exit(0);
        }
        _ => {
            debug!("忽略 LSP 通知: {}", notification.method);
        }
    }
    Ok(())
}
