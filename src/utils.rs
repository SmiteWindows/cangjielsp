//! 通用工具函数，封装跨平台、路径处理、错误处理等通用逻辑
//! 单一职责：提供可复用的辅助功能，避免代码重复

use std::env;
use std::path::{Path, PathBuf};
use zed_extension_api as zed;

/// 操作系统类型枚举（明确化平台判断）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsType {
    Windows,
    MacOs,
    Linux,
    Other,
}

impl OsType {
    /// 获取当前操作系统类型
    pub fn current() -> Self {
        #[cfg(windows)]
        return Self::Windows;
        #[cfg(target_os = "macos")]
        return Self::MacOs;
        #[cfg(target_os = "linux")]
        return Self::Linux;
        #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
        return Self::Other;
    }

    /// 获取对应平台的动态库路径环境变量名
    pub fn lib_env_var(&self) -> Option<&'static str> {
        match self {
            Self::Windows => Some("PATH"),
            Self::MacOs => Some("DYLD_LIBRARY_PATH"),
            Self::Linux => Some("LD_LIBRARY_PATH"),
            Self::Other => None,
        }
    }
}

/// 安全获取环境变量，返回结构化错误
pub fn get_env_var(name: &str) -> zed::Result<String> {
    env::var(name).map_err(|_| {
        zed::Error::NotFound(format!(
            "环境变量 `{name}` 未设置，请参考仓颉官方文档配置开发环境"
        ))
    })
}

/// 拼接路径并验证存在性
pub fn resolve_and_validate_path<P: AsRef<Path>>(
    base: P,
    segments: &[&str],
) -> zed::Result<PathBuf> {
    let mut path = PathBuf::from(base.as_ref());
    path.extend(segments);

    // 验证路径存在
    if !path.exists() {
        return Err(zed::Error::NotFound(format!(
            "路径不存在: {}",
            path.display()
        )));
    }

    // 验证路径是文件（如果是二进制文件路径）
    if segments
        .last()
        .map_or(false, |s| s.ends_with(".exe") || s == &"LSPServer")
    {
        if !path.is_file() {
            return Err(zed::Error::InvalidPath(format!(
                "不是有效文件: {}",
                path.display()
            )));
        }
    }

    Ok(path)
}

/// 将Path转换为String（处理无效UTF-8路径）
pub fn path_to_string(path: &Path) -> zed::Result<String> {
    path.to_str()
        .ok_or_else(|| {
            zed::Error::InvalidPath(format!("路径包含无效UTF-8字符: {}", path.display()))
        })
        .map(|s| s.to_string())
}

/// 合并环境变量（例如：在现有PATH中添加新路径）
pub fn merge_env_var(existing: Option<String>, new_path: &str, separator: &str) -> String {
    match existing {
        Some(prev) => format!("{new_path}{separator}{prev}"),
        None => new_path.to_string(),
    }
}
