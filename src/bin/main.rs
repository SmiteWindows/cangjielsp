//! 仓颉 LSP 可执行文件入口（供 Zed 调用）
use cangjie_lsp::{
    config::CangjieConfig,
    language_server::CangjieLanguageServer,
    extension::CangjieExtension,
};
use zed_extension_api::{self as zed, LanguageServer};
use std::sync::Arc;
use log::{info, error};

#[tokio::main]
async fn main() -> zed::Result<()> {
    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Cangjie LSP v{} 启动", cangjie_lsp::EXTENSION_VERSION);

    // 加载默认配置
    let config = Arc::new(CangjieConfig::default());
    info!("加载默认配置完成");

    // 初始化 LSP 服务器
    let mut lsp_server = CangjieLanguageServer::new(config.clone());
    info!("LSP 服务器初始化完成");

    // 初始化扩展（整合命令处理）
    let extension = CangjieExtension::new(config, lsp_server);

    // 启动 LSP 服务（STDIO 通信模式）
    zed::lsp::run_stdio_server(extension).await?;

    info!("Cangjie LSP 正常退出");
    Ok(())
}

// 处理未捕获的恐慌
#[panic_handler]
fn panic_handler(panic: &core::panic::PanicInfo<'_>) {
    error!("LSP 发生恐慌: {}", panic);
    std::process::exit(1);
}
