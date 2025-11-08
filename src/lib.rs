//! 仓颉语言Zed扩展（基于 Zed Extension API 原生能力）
//! API 文档：https://docs.rs/zed_extension_api/latest/zed_extension_api/

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(unused_imports)]

pub mod config;
pub mod extension;
pub mod language_server;

use zed_extension_api as zed;

// 注册扩展（使用 API 提供的宏）
zed::register_extension!(extension::CangjieExtension);
