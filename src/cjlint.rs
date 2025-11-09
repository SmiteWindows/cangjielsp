//! 代码检查工具 cjlint 集成
use crate::config::CangjieConfig;
use crate::tree_sitter_utils;
use serde::{Deserialize, Serialize};
use zed_extension_api;

/// cjlint 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CjlintConfig {
    /// 检查级别（error/warn/info/off）
    pub check_level: String,
    /// 启用风格检查
    pub enable_style_check: bool,
    /// 启用语法检查
    pub enable_syntax_check: bool,
    /// 忽略的规则列表
    pub ignore_rules: Vec<String>,
    /// 自定义规则路径
    pub custom_rules_path: Option<String>,
}

impl Default for CjlintConfig {
    fn default() -> Self {
        Self {
            check_level: "warn".to_string(),
            enable_style_check: true,
            enable_syntax_check: true,
            ignore_rules: Vec::new(),
            custom_rules_path: None,
        }
    }
}

/// 代码检查问题严重级别
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LintSeverity {
    Error,
    Warn,
    Info,
}

/// 代码检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintIssue {
    /// 规则ID
    pub rule_id: String,
    /// 问题描述
    pub message: String,
    /// 严重级别
    pub severity: LintSeverity,
    /// 代码范围
    pub range: zed_extension_api::Range,
    /// 修复建议
    pub fix: Option<String>,
}

/// cjlint 管理器
#[derive(Debug, Default)]
pub struct CjlintManager;

impl CjlintManager {
    /// 检查 cjlint 是否可用
    pub fn is_available() -> zed_extension_api::Result<()> {
        if std::process::Command::new("cjlint")
            .arg("--version")
            .output()
            .is_err()
        {
            return Err(zed_extension_api::Error::NotFound(
                "cjlint 工具未找到，请安装并配置到 PATH 中".to_string(),
            ));
        }
        Ok(())
    }

    /// 加载配置
    pub fn load_config(
        worktree: &zed_extension_api::Worktree,
        config: &CangjieConfig,
    ) -> zed_extension_api::Result<CjlintConfig> {
        // 优先加载工作目录下的 cjlint.toml 配置
        let config_path = worktree.path().join("cjlint.toml");
        if config_path.exists() {
            let config_content = std::fs::read_to_string(&config_path).map_err(|e| {
                zed_extension_api::Error::IoError(format!("读取 cjlint 配置失败: {}", e))
            })?;
            let toml_config: CjlintConfig = toml::from_str(&config_content).map_err(|e| {
                zed_extension_api::Error::InvalidData(format!("解析 cjlint 配置失败: {}", e))
            })?;
            return Ok(toml_config);
        }

        // 未找到配置文件时使用默认配置
        Ok(config.cjlint.clone())
    }

    /// 执行代码检查
    pub fn run_lint(
        worktree: &zed_extension_api::Worktree,
        document: &zed_extension_api::Document,
        config: &CjlintConfig,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::Diagnostic>> {
        Self::is_available()?;

        // 1. 先通过 tree-sitter 进行语法错误检查（快速前置检查）
        let content = document.text();
        let tree = tree_sitter_utils::parse_document(content);
        let mut diagnostics = tree_sitter_utils::check_syntax_errors(&tree, content);

        // 如果禁用语法检查，过滤掉语法错误诊断
        if !config.enable_syntax_check {
            diagnostics.retain(|d| {
                d.code
                    .as_ref()
                    .map(|c| c.value != "SYNTAX_ERROR")
                    .unwrap_or(true)
            });
        }

        // 2. 执行 cjlint 进行风格和语义检查
        if config.enable_style_check || (config.enable_syntax_check && diagnostics.is_empty()) {
            let mut args = vec!["check".to_string()];

            // 添加配置参数
            args.push(format!("--level={}", config.check_level));
            if !config.enable_style_check {
                args.push("--no-style".to_string());
            }
            if !config.enable_syntax_check {
                args.push("--no-syntax".to_string());
            }
            for rule in &config.ignore_rules {
                args.push(format!("--ignore={}", rule));
            }
            if let Some(custom_rules) = &config.custom_rules_path {
                args.push(format!("--rules={}", custom_rules));
            }
            // 输出 JSON 格式结果
            args.push("--format=json".to_string());
            // 添加文件路径
            args.push(document.path().to_str().unwrap().to_string());

            // 执行 cjlint 命令
            let output = std::process::Command::new("cjlint")
                .args(&args)
                .current_dir(worktree.path())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?
                .wait_with_output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(zed_extension_api::Error::ProcessFailed(format!(
                    "cjlint 检查失败: {}",
                    stderr
                )));
            }

            // 解析 JSON 结果
            let lint_issues: Vec<LintIssue> =
                serde_json::from_slice(&output.stdout).map_err(|e| {
                    zed_extension_api::Error::InvalidData(format!("解析 cjlint 结果失败: {}", e))
                })?;

            // 转换为 Zed 诊断格式
            for issue in lint_issues {
                let severity = match issue.severity {
                    LintSeverity::Error => zed_extension_api::DiagnosticSeverity::Error,
                    LintSeverity::Warn => zed_extension_api::DiagnosticSeverity::Warn,
                    LintSeverity::Info => zed_extension_api::DiagnosticSeverity::Info,
                };

                let mut diagnostic = zed_extension_api::Diagnostic {
                    range: issue.range,
                    severity,
                    code: Some(zed_extension_api::DiagnosticCode {
                        value: issue.rule_id,
                        description: Some(issue.message.clone()),
                    }),
                    message: issue.message,
                    source: Some("cjlint".to_string()),
                    fixes: None,
                };

                // 添加修复建议（如果有）
                if let Some(fix) = issue.fix {
                    let text_edit = zed_extension_api::TextEdit {
                        range: diagnostic.range.clone(),
                        new_text: fix,
                    };
                    diagnostic.fixes = Some(vec![zed_extension_api::Fix {
                        title: "应用 cjlint 修复建议".to_string(),
                        edits: vec![(document.uri().clone(), vec![text_edit])],
                    }]);
                }

                diagnostics.push(diagnostic);
            }
        }

        Ok(diagnostics)
    }
}
