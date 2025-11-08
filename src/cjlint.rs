//! cjlint 代码检查工具集成（语法错误、风格规范、性能建议、安全校验）
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use zed_extension_api as zed;

/// cjlint 配置（对应 cjlint.toml）
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CjlintConfig {
    /// 检查规则配置
    #[serde(default)]
    pub rules: HashMap<String, RuleConfig>,
    /// 排除文件/目录
    #[serde(default)]
    pub exclude: Vec<String>,
    /// 包含文件/目录
    #[serde(default)]
    pub include: Vec<String>,
    /// 全局检查级别
    #[serde(default = "default_check_level")]
    pub check_level: CheckLevel,
    /// 自动修复配置
    #[serde(default)]
    pub fix: FixConfig,
    /// 输出格式
    #[serde(default)]
    pub output_format: OutputFormat,
}

/// 单个规则配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct RuleConfig {
    /// 是否启用
    #[serde(default = "default_rule_enabled")]
    pub enabled: bool,
    /// 规则级别（覆盖全局）
    #[serde(default)]
    pub level: Option<CheckLevel>,
    /// 规则额外配置
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
}

/// 检查级别
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CheckLevel {
    Error, // 错误（阻断构建）
    Warn,  // 警告
    Info,  // 信息
    Off,   // 关闭
}

impl Default for CheckLevel {
    fn default() -> Self {
        Self::Warn
    }
}

/// 自动修复配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FixConfig {
    /// 是否启用自动修复
    #[serde(default = "default_fix_enabled")]
    pub enabled: bool,
    /// 启用修复的规则列表
    #[serde(default)]
    pub rules: Vec<String>,
    /// 是否备份原始文件
    #[serde(default = "default_fix_backup")]
    pub backup: bool,
}

/// 输出格式
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    #[default]
    Human,
    Json,
    Xml,
    Sarif,
}

// 默认值
fn default_check_level() -> CheckLevel {
    CheckLevel::Warn
}
fn default_rule_enabled() -> bool {
    true
}
fn default_fix_enabled() -> bool {
    false
}
fn default_fix_backup() -> bool {
    true
}

/// 官方规则库
pub mod official_rules {
    use super::CheckLevel;

    /// 语法错误规则（Error 级别）
    pub const SYNTAX: &[(&str, &str)] = &[
        ("E001", "无效的语法格式，无法解析"),
        ("E002", "未闭合的括号/引号/注释"),
        ("E003", "引用未定义的标识符"),
        ("E004", "类型不匹配"),
        ("E005", "函数调用参数数量/类型不匹配"),
        ("E006", "导入不存在的模块或依赖"),
        ("E007", "重复定义标识符"),
    ];

    /// 代码风格规则（Warn 级别）
    pub const STYLE: &[(&str, &str)] = &[
        ("W001", "行宽超过限制（默认 120 字符）"),
        ("W002", "缩进不一致（建议 4 空格）"),
        ("W003", "多余的空格/空行"),
        (
            "W004",
            "命名不规范（变量/函数 snake_case，类型 PascalCase）",
        ),
        ("W005", "公共接口缺少文档注释"),
        ("W006", "冗余的分号"),
        ("W007", "未使用的变量/导入"),
        ("W008", "括号使用不规范"),
    ];

    /// 性能建议规则（Info 级别）
    pub const PERFORMANCE: &[(&str, &str)] = &[
        ("I001", "不必要的克隆操作（可改为引用）"),
        ("I002", "低效的循环写法（建议使用迭代器）"),
        ("I003", "未使用的函数返回值"),
        ("I004", "过度嵌套的代码块（建议拆分函数）"),
        ("I005", "使用更高效的标准库函数"),
    ];

    /// 安全规则（Error 级别）
    pub const SECURITY: &[(&str, &str)] = &[
        ("S001", "不安全的内存访问"),
        ("S002", "硬编码敏感信息"),
        ("S003", "未验证的用户输入"),
        ("S004", "不安全的并发操作"),
    ];

    /// 获取规则描述
    pub fn get_description(rule_id: &str) -> Option<&'static str> {
        SYNTAX
            .iter()
            .find(|(id, _)| *id == rule_id)
            .map(|(_, desc)| *desc)
            .or_else(|| {
                STYLE
                    .iter()
                    .find(|(id, _)| *id == rule_id)
                    .map(|(_, desc)| *desc)
            })
            .or_else(|| {
                PERFORMANCE
                    .iter()
                    .find(|(id, _)| *id == rule_id)
                    .map(|(_, desc)| *desc)
            })
            .or_else(|| {
                SECURITY
                    .iter()
                    .find(|(id, _)| *id == rule_id)
                    .map(|(_, desc)| *desc)
            })
    }

    /// 获取规则默认级别
    pub fn get_default_level(rule_id: &str) -> CheckLevel {
        if SYNTAX.iter().any(|(id, _)| *id == rule_id)
            || SECURITY.iter().any(|(id, _)| *id == rule_id)
        {
            CheckLevel::Error
        } else if STYLE.iter().any(|(id, _)| *id == rule_id) {
            CheckLevel::Warn
        } else if PERFORMANCE.iter().any(|(id, _)| *id == rule_id) {
            CheckLevel::Info
        } else {
            CheckLevel::Warn
        }
    }

    /// 检查规则是否支持自动修复
    pub fn supports_fix(rule_id: &str) -> bool {
        [
            "W001", "W002", "W003", "W004", "W006", "W007", "I001", "I005",
        ]
        .contains(&rule_id)
    }
}

/// 诊断结果
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CjlintDiagnostic {
    pub rule_id: String,
    pub message: String,
    pub level: CheckLevel,
    pub line: u32,
    pub column: u32,
    pub end_line: Option<u32>,
    pub end_column: Option<u32>,
    pub fixable: bool,
    pub fix: Option<CjlintFix>,
}

/// 自动修复方案
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CjlintFix {
    pub range: CjlintRange,
    pub new_text: String,
}

/// 修复范围
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CjlintRange {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

/// cjlint 管理器
#[derive(Debug, Default)]
pub struct CjlintManager;

impl CjlintManager {
    /// 检查 cjlint 是否可用
    pub fn is_available() -> zed::Result<()> {
        Self::find_executable()?;
        Ok(())
    }

    /// 查找 cjlint 可执行文件
    pub fn find_executable() -> zed::Result<zed::Path> {
        // 1. 从 CANGJIE_HOME 查找
        if let Ok(cangjie_home) = zed::env::var("CANGJIE_HOME") {
            let mut path = zed::Path::new(&cangjie_home);
            path.push("bin");
            path.push(if zed::platform::is_windows() {
                "cjlint.exe"
            } else {
                "cjlint"
            });

            if path.exists() && path.is_executable() {
                return Ok(path);
            }
        }

        // 2. 从 PATH 查找
        if let Some(path) = zed::env::find_executable(if zed::platform::is_windows() {
            "cjlint.exe"
        } else {
            "cjlint"
        }) {
            return Ok(path);
        }

        Err(zed::Error::NotFound(
            "未找到 cjlint 工具，请配置 CANGJIE_HOME 或确保 cjlint 在 PATH 中".to_string(),
        ))
    }

    /// 加载 cjlint 配置
    pub fn load_config(
        worktree: &zed::Worktree,
        extension_config: &super::config::CangjieConfig,
    ) -> zed::Result<CjlintConfig> {
        // 1. 项目根目录 cjlint.toml
        let project_config = worktree.path().join("cjlint.toml");
        if project_config.exists() {
            return Self::parse_config(&project_config);
        }

        // 2. 用户目录 .cjlint.toml
        if let Some(user_config) = Self::user_config_path() {
            if user_config.exists() {
                return Self::parse_config(&user_config);
            }
        }

        // 3. 扩展配置
        Ok(extension_config.cjlint.clone())
    }

    /// 解析配置文件
    fn parse_config(path: &zed::Path) -> zed::Result<CjlintConfig> {
        let content = zed::fs::read_to_string(path)
            .map_err(|e| zed::Error::IoError(format!("读取 {} 失败: {}", path.to_str()?, e)))?;

        toml::from_str(&content)
            .map_err(|e| zed::Error::InvalidConfig(format!("解析 {} 失败: {}", path.to_str()?, e)))
    }

    /// 用户目录配置路径
    fn user_config_path() -> Option<zed::Path> {
        zed::env::home_dir().map(|home| home.join(".cjlint.toml"))
    }

    /// 执行代码检查
    pub fn run_lint(
        worktree: &zed::Worktree,
        document: &zed::Document,
        config: &CjlintConfig,
    ) -> zed::Result<Vec<zed::Diagnostic>> {
        let file_path = document.path();
        let file_str = file_path.to_str()?;

        // 跳过排除列表
        if config
            .exclude
            .iter()
            .any(|pattern| glob::Pattern::new(pattern).map_or(false, |p| p.matches(file_str)))
        {
            return Ok(vec![]);
        }

        // 构建命令
        let cjlint_path = Self::find_executable()?;
        let mut args = Vec::new();

        // 配置文件
        let project_config = worktree.path().join("cjlint.toml");
        if project_config.exists() {
            args.push(format!("--config={}", project_config.to_str()?));
        }

        // 检查级别、输出格式
        args.push(format!(
            "--level={}",
            Self::level_to_str(config.check_level)
        ));
        args.push("--format=json".to_string());
        args.push("--stdin".to_string());
        args.push(format!("--stdin-filename={}", file_str));

        // 执行命令
        let mut child = zed::process::Command::new(cjlint_path.to_str()?)
            .args(&args)
            .stdin(zed::process::Stdio::piped())
            .stdout(zed::process::Stdio::piped())
            .stderr(zed::process::Stdio::piped())
            .spawn()?;

        // 写入文件内容
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(document.text().as_bytes())?;

        let output = child.wait_with_output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(zed::Error::ProcessFailed(format!(
                "cjlint 检查失败: {}",
                stderr.trim()
            )));
        }

        // 解析诊断结果
        let diagnostics: Vec<CjlintDiagnostic> = serde_json::from_slice(&output.stdout)
            .map_err(|e| zed::Error::InvalidConfig(format!("解析诊断结果失败: {}", e)))?;

        // 转换为 Zed 诊断格式
        Ok(diagnostics
            .into_iter()
            .filter(|diag| {
                // 过滤禁用的规则
                let rule_config = config
                    .rules
                    .get(&diag.rule_id)
                    .unwrap_or(&RuleConfig::default());
                rule_config.enabled
                    && rule_config.level.unwrap_or(config.check_level) != CheckLevel::Off
            })
            .map(|diag| {
                let severity = match diag.level {
                    CheckLevel::Error => zed::DiagnosticSeverity::Error,
                    CheckLevel::Warn => zed::DiagnosticSeverity::Warn,
                    CheckLevel::Info => zed::DiagnosticSeverity::Info,
                    CheckLevel::Off => unreachable!(),
                };

                let mut fixes = None;
                if diag.fixable && config.fix.enabled {
                    let rule_config = config
                        .rules
                        .get(&diag.rule_id)
                        .unwrap_or(&RuleConfig::default());
                    let fix_enabled =
                        config.fix.rules.is_empty() || config.fix.rules.contains(&diag.rule_id);
                    if fix_enabled && official_rules::supports_fix(&diag.rule_id) {
                        if let Some(fix) = diag.fix {
                            fixes = Some(vec![zed::Fix {
                                title: format!("修复规则 {}", diag.rule_id),
                                edits: vec![zed::Edit {
                                    path: file_path.clone(),
                                    edits: vec![zed::TextEdit {
                                        range: zed::Range {
                                            start: zed::Position {
                                                line: fix.range.start_line - 1,
                                                column: fix.range.start_column - 1,
                                            },
                                            end: zed::Position {
                                                line: fix.range.end_line - 1,
                                                column: fix.range.end_column - 1,
                                            },
                                        },
                                        new_text: fix.new_text,
                                    }],
                                }],
                            }]);
                        }
                    }
                }

                zed::Diagnostic {
                    range: zed::Range {
                        start: zed::Position {
                            line: diag.line - 1,
                            column: diag.column - 1,
                        },
                        end: zed::Position {
                            line: diag.end_line.unwrap_or(diag.line) - 1,
                            column: diag.end_column.unwrap_or(diag.column) - 1,
                        },
                    },
                    severity,
                    code: Some(zed::DiagnosticCode {
                        value: diag.rule_id.clone(),
                        description: official_rules::get_description(&diag.rule_id)
                            .map(|s| s.to_string()),
                    }),
                    message: diag.message,
                    source: Some("cjlint".to_string()),
                    fixes,
                }
            })
            .collect())
    }

    /// 执行自动修复
    pub fn run_fix(
        worktree: &zed::Worktree,
        document: &mut zed::Document,
        config: &CjlintConfig,
    ) -> zed::Result<()> {
        if !config.fix.enabled {
            return Err(zed::Error::InvalidConfig("自动修复未启用".to_string()));
        }

        let diagnostics = self.run_lint(worktree, document, config)?;
        let mut edits = Vec::new();

        for diag in diagnostics {
            if let Some(fixes) = diag.fixes {
                for fix in fixes {
                    for edit in fix.edits {
                        edits.extend(edit.edits);
                    }
                }
            }
        }

        // 应用编辑
        document.apply_edits(edits)?;
        Ok(())
    }

    /// 检查级别转字符串
    fn level_to_str(level: CheckLevel) -> &'static str {
        match level {
            CheckLevel::Error => "error",
            CheckLevel::Warn => "warn",
            CheckLevel::Info => "info",
            CheckLevel::Off => "off",
        }
    }
}
