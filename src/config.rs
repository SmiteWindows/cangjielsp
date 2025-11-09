//! 扩展配置定义
use serde::{Deserialize, Serialize};

/// 扩展主配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CangjieConfig {
    /// LSP 相关配置
    pub lsp: LspConfig,
    /// 格式化工具配置
    pub cjfmt: CjfmtConfig,
    /// 代码检查配置
    pub cjlint: CjlintConfig,
    /// 覆盖率工具配置
    pub cjcov: cjcov::CjcovConfig,
    /// 性能分析工具配置
    pub cjprof: cjprof::CjprofConfig,
}

/// LSP 配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LspConfig {
    /// 请求超时时间（毫秒）
    pub timeout_ms: u32,
    /// 实时诊断开关
    pub realtime_diagnostics: bool,
}

/// 格式化工具配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CjfmtConfig {
    /// 缩进风格（space/tab）
    pub indent_style: String,
    /// 缩进大小（空格数）
    pub indent_size: u8,
    /// 行尾分号自动补全
    pub auto_semicolon: bool,
}

/// 代码检查配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CjlintConfig {
    /// 检查级别（error/warn/info/off）
    pub check_level: String,
    /// 启用风格检查
    pub enable_style_check: bool,
    /// 启用语法检查
    pub enable_syntax_check: bool,
}

// 为子模块配置提供默认实现
impl Default for cjcov::CjcovConfig {
    fn default() -> Self {
        cjcov::CjcovConfig {
            collect: cjcov::SampleConfig::default(),
            report: cjcov::ReportConfig::default(),
            filter: cjcov::FilterConfig::default(),
            threshold: cjcov::ThresholdConfig::default(),
            advanced: cjcov::AdvancedConfig::default(),
        }
    }
}

impl Default for cjprof::CjprofConfig {
    fn default() -> Self {
        cjprof::CjprofConfig {
            sample: cjprof::SampleConfig::default(),
            analyze: cjprof::AnalyzeConfig::default(),
            report: cjprof::ReportConfig::default(),
            filter: cjprof::FilterConfig::default(),
            threshold: cjprof::ThresholdConfig::default(),
            advanced: cjprof::AdvancedConfig::default(),
        }
    }
}
