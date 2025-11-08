//! 仓颉语言 Zed 扩展（cangjielsp）
//! 整合 cjpm/cjdb/cjlint/cjfmt/cjcov/cjprof 工具链，提供全流程开发支持
// #![warn(missing_docs, unused_imports, unused_variables)]

// 导出核心模块供 Zed 调用
pub mod cjcov;
pub mod cjdb;
pub mod cjfmt;
pub mod cjlint;
pub mod cjpm;
pub mod cjprof;
pub mod config;
pub mod corpus;
pub mod extension;
pub mod language_server;
pub mod rag_utils;
pub mod syntax;

// 暴露扩展入口
pub use extension::CangjieExtension;

/// 扩展版本（与 Cargo.toml 同步）
pub const EXTENSION_VERSION: &str = "0.1.0";

/// 仓颉工具链最低支持版本
pub const MIN_CANGJIE_VERSION: &str = "1.0.3";
