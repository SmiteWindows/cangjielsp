//! 仓颉语言服务器核心逻辑
//! 职责：封装语言服务器路径查找、验证、初始化等逻辑

use super::{config, utils};
use std::path::PathBuf;
use zed_extension_api as zed;

/// 仓颉语言服务器处理器
#[derive(Debug, Default)]
pub struct CangjieLanguageServer;

impl CangjieLanguageServer {
    /// 语言服务器唯一标识（必须与 extension.toml 中配置一致）
    pub const LANGUAGE_SERVER_ID: &'static str = "cangjie-language-server";

    /// 语言服务器二进制文件名（跨平台）
    const BINARY_NAME: &'static str = if cfg!(windows) {
        "LSPServer.exe"
    } else {
        "LSPServer"
    };

    /// 平台对应的运行时库目录（与仓颉SDK目录结构一致）
    const PLATFORM_LIB_DIR: &'static str = match utils::OsType::current() {
        utils::OsType::Windows => "windows_x86_64_llvm",
        utils::OsType::MacOs => "macos_x86_64_llvm",
        utils::OsType::Linux => "linux_x86_64_llvm",
        utils::OsType::Other => panic!("不支持的操作系统"),
    };

    /// 获取语言服务器二进制文件路径
    pub fn binary_path(&self) -> zed::Result<PathBuf> {
        let cangjie_home = utils::get_env_var("CANGJIE_HOME")?;

        // 仓颉SDK标准路径：$CANGJIE_HOME/tools/bin/LSPServer
        utils::resolve_and_validate_path(&cangjie_home, &["tools", "bin", Self::BINARY_NAME])
    }

    /// 获取运行时库路径
    pub fn lib_path(&self) -> zed::Result<PathBuf> {
        let cangjie_home = utils::get_env_var("CANGJIE_HOME")?;

        // 仓颉SDK标准路径：$CANGJIE_HOME/runtime/lib/<platform>
        utils::resolve_and_validate_path(&cangjie_home, &["runtime", "lib", Self::PLATFORM_LIB_DIR])
    }

    /// 生成语言服务器启动命令
    pub fn build_command(&self, worktree: &zed::Worktree) -> zed::Result<zed::Command> {
        // 1. 获取并验证路径
        let binary_path = self.binary_path()?;
        let lib_path = self.lib_path()?;

        // 2. 加载配置并生成参数
        let lsp_config = config::CangjieLspConfig::from_worktree(worktree)?;
        let args = lsp_config.to_args();

        // 3. 构建环境变量
        let env_map = config::build_env_map(&utils::path_to_string(&lib_path)?)?;

        // 4. 生成最终命令
        Ok(zed::Command {
            command: utils::path_to_string(&binary_path)?,
            args,
            env: env_map,
        })
    }
}

// 安全校验：确保平台库目录在不支持的系统上编译失败
static_assertions::const_assert!(
    matches!(
        utils::OsType::current(),
        utils::OsType::Windows | utils::OsType::MacOs | utils::OsType::Linux
    ),
    "不支持的操作系统，仅支持 Windows、macOS、Linux"
);
