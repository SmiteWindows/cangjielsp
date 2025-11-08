//! 仓颉语言Zed扩展（严格遵循 Zed Extension API v0.7 规范）
//! API 文档：https://docs.rs/zed_extension_api/latest/zed_extension_api/

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(unused_imports)]
#![deny(clippy::all)]

// 仅导入 API 模块，拒绝标准库路径/环境变量等模块
use zed_extension_api as zed;

pub mod config;
pub mod extension;
pub mod language_server;

// 必须使用 API 提供的注册宏（唯一合法注册方式）
zed::register_extension!(extension::CangjieExtension);
