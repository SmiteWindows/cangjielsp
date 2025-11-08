//! 仓颉语言服务器逻辑（完全依赖 API 接口实现）
use zed_extension_api as zed;

/// 仓颉语言服务器（严格遵循 API 语言服务器规范）
#[derive(Debug, Default)]
pub struct CangjieLanguageServer;

impl CangjieLanguageServer {
    /// 语言服务器唯一ID（必须与 extension.toml 中配置完全一致）
    pub const LANGUAGE_SERVER_ID: &'static str = "cangjie-language-server";

    /// 获取 LSP 二进制文件路径（API 推荐的路径查找流程）
    pub fn binary_path(&self) -> zed::Result<zed::Path> {
        // 1. 必须通过 API 获取 CANGJIE_HOME 环境变量
        let cangjie_home = zed::env::var("CANGJIE_HOME")
            .map_err(|_| zed::Error::NotFound(
                "环境变量 CANGJIE_HOME 未设置，请参考仓颉官方文档配置".to_string()
            ))?;

        // 2. 使用 API 提供的 Path 类型拼接路径（跨平台安全）
        let mut binary_path = zed::Path::new(&cangjie_home);
        binary_path.push("tools");
        binary_path.push("bin");
        binary_path.push(self.binary_filename());

        // 3. 使用 API 路径验证方法（确保路径有效）
        self.validate_binary_path(&binary_path)?;

        Ok(binary_path)
    }

    /// 获取运行时库路径（API 平台适配逻辑）
    pub fn lib_path(&self) -> zed::Result<zed::Path> {
        let cangjie_home = zed::env::var("CANGJIE_HOME")?;
        let mut lib_path = zed::Path::new(&cangjie_home);

        lib_path.push("runtime");
        lib_path.push("lib");
        lib_path.push(self.platform_lib_dir());

        // 验证路径是合法目录
        if !lib_path.exists() {
            return Err(zed::Error::NotFound(format!(
                "运行时库目录不存在: {}",
                lib_path.to_str()?
            )));
        }

        if !lib_path.is_dir() {
            return Err(zed::Error::InvalidPath(format!(
                "不是合法目录: {}",
                lib_path.to_str()?
            )));
        }

        Ok(lib_path)
    }

    /// 构建 LSP 启动命令（完全使用 API 定义的 Command 类型）
    pub fn build_command(&self, worktree: &zed::Worktree) -> zed::Result<zed::Command> {
        // 1. 加载配置
        let config = config::CangjieLspConfig::from_worktree(worktree)?;
        let args = config.to_args(worktree)?;

        // 2. 获取二进制路径和库路径
        let binary_path = self.binary_path()?;
        let lib_path = self.lib_path()?;

        // 3. 构建环境变量（使用 API 平台工具）
        let env = self.build_env_map(&lib_path)?;

        // 4. 生成 API 标准 Command 结构体
        Ok(zed::Command {
            command: binary_path.to_str()?.to_string(),
            args,
            env,
        })
    }

    /// 构建环境变量映射（API 原生 EnvMap 类型）
    fn build_env_map(&self, lib_path: &zed::Path) -> zed::Result<zed::EnvMap> {
        let mut env = zed::EnvMap::new();

        // 必须添加 CANGJIE_HOME
        env.insert(
            "CANGJIE_HOME".to_string(),
            zed::env::var("CANGJIE_HOME")?,
        );

        // 按平台设置动态库路径（使用 API 提供的平台工具）
        let lib_env_var = zed::platform::lib_path_env_var();
        let existing_lib_path = zed::env::var(lib_env_var).ok();
        let separator = zed::platform::path_separator();

        let new_lib_path = match existing_lib_path {
            Some(prev) => format!(
                "{}{}{}",
                lib_path.to_str()?,
                separator,
                prev
            ),
            None => lib_path.to_str()?.to_string(),
        };

        env.insert(lib_env_var.to_string(), new_lib_path);

        Ok(env)
    }

    /// 根据平台获取 LSP 二进制文件名（API 平台判断）
    fn binary_filename(&self) -> &'static str {
        if zed::platform::is_windows() {
            "LSPServer.exe"
        } else {
            "LSPServer"
        }
    }

    /// 根据平台获取库目录（API 平台枚举）
    fn platform_lib_dir(&self) -> &'static str {
        match zed::platform::OS::current() {
            zed::platform::OS::Windows => "windows_x86_64_llvm",
            zed::platform::OS::MacOS => "macos_x86_64_llvm",
            zed::platform::OS::Linux => "linux_x86_64_llvm",
            zed::platform::OS::Other => {
                // 直接返回错误，API 不支持的平台无法运行
                return Err(zed::Error::Unsupported(
                    "当前操作系统不支持仓颉LSP，请使用 Windows/macOS/Linux".to_string()
                ).unwrap_err();
            }
        }
    }

    /// 验证二进制路径有效性（API 路径方法）
    fn validate_binary_path(&self, path: &zed::Path) -> zed::Result<()> {
        if !path.exists() {
            return Err(zed::Error::NotFound(format!(
                "LSP 二进制文件不存在: {}",
                path.to_str()?
            )));
        }

        if !path.is_file() {
            return Err(zed::Error::InvalidPath(format!(
                "路径不是合法文件: {}",
                path.to_str()?
            )));
        }

        if !path.is_executable() {
            return Err(zed::Error::PermissionDenied(format!(
                "LSP 二进制文件无执行权限: {}",
                path.to_str()?
            )));
        }

        Ok(())
    }
}
