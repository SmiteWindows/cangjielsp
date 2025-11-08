//! cjcov 覆盖率分析工具集成（行覆盖率、分支覆盖率、函数覆盖率）
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use zed_extension_api as zed;

/// cjcov 配置（对应 cjcov.toml）
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CjcovConfig {
    /// 收集配置
    #[serde(default)]
    pub collect: CollectConfig,
    /// 报告配置
    #[serde(default)]
    pub report: ReportConfig,
    /// 过滤配置
    #[serde(default)]
    pub filter: FilterConfig,
    /// 阈值配置（CI 校验）
    #[serde(default)]
    pub threshold: ThresholdConfig,
    /// 高级配置
    #[serde(default)]
    pub advanced: AdvancedConfig,
}

/// 收集配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CollectConfig {
    /// 收集模式（full/partial/fast）
    #[serde(default = "default_collect_mode")]
    pub mode: CollectMode,
    /// 收集目录（默认 src）
    #[serde(default = "default_collect_dir")]
    pub dir: String,
    /// 输出目录（默认 target/cjcov）
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    /// 启用分支覆盖率（默认 true）
    #[serde(default = "default_enable_branch_coverage")]
    pub enable_branch: bool,
    /// 启用函数覆盖率（默认 true）
    #[serde(default = "default_enable_function_coverage")]
    pub enable_function: bool,
    /// 采样频率（仅 partial 模式有效，默认 10）
    #[serde(default = "default_sample_rate")]
    pub sample_rate: u32,
}

/// 收集模式
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CollectMode {
    Full,    // 全量收集（精确）
    Partial, // 部分收集（平衡精度和性能）
    Fast,    // 快速收集（仅行覆盖率）
}

impl Default for CollectMode {
    fn default() -> Self {
        Self::Full
    }
}

/// 报告配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ReportConfig {
    /// 输出格式（支持多格式）
    #[serde(default = "default_report_formats")]
    pub formats: Vec<ReportFormat>,
    /// 报告目录（默认 target/cjcov/reports）
    #[serde(default = "default_report_dir")]
    pub dir: String,
    /// 显示未覆盖代码（默认 true）
    #[serde(default = "default_show_uncovered")]
    pub show_uncovered: bool,
    /// 生成 HTML 详细报告（默认 true）
    #[serde(default = "default_generate_html")]
    pub generate_html: bool,
    /// 生成 XML 报告（用于 CI 集成，默认 false）
    #[serde(default)]
    pub generate_xml: bool,
    /// 生成 SARIF 报告（默认 false）
    #[serde(default)]
    pub generate_sarif: bool,
}

/// 报告格式
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ReportFormat {
    Text,  // 文本格式
    Html,  // HTML 格式
    Json,  // JSON 格式
    Xml,   // XML 格式
    Sarif, // SARIF 格式
}

/// 过滤配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FilterConfig {
    /// 包含的文件/目录（glob 模式）
    #[serde(default)]
    pub include: Vec<String>,
    /// 排除的文件/目录（glob 模式）
    #[serde(default)]
    pub exclude: Vec<String>,
    /// 排除测试文件（默认 true）
    #[serde(default = "default_exclude_tests")]
    pub exclude_tests: bool,
    /// 排除生成的文件（默认 true）
    #[serde(default = "default_exclude_generated")]
    pub exclude_generated: bool,
}

/// 阈值配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ThresholdConfig {
    /// 行覆盖率阈值（%，默认 80）
    #[serde(default)]
    pub line: Option<u32>,
    /// 分支覆盖率阈值（%，默认 70）
    #[serde(default)]
    pub branch: Option<u32>,
    /// 函数覆盖率阈值（%，默认 90）
    #[serde(default)]
    pub function: Option<u32>,
    /// 未达标时阻断 CI（默认 true）
    #[serde(default = "default_fail_on_threshold")]
    pub fail_on_missing: bool,
}

/// 高级配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct AdvancedConfig {
    /// 保存原始覆盖数据（默认 false）
    #[serde(default)]
    pub save_raw_data: bool,
    /// 覆盖数据压缩（默认 true）
    #[serde(default = "default_compress_data")]
    pub compress_data: bool,
    /// 并行收集（默认 true）
    #[serde(default = "default_parallel_collect")]
    pub parallel_collect: bool,
    /// 忽略未执行的测试（默认 false）
    #[serde(default)]
    pub ignore_unrun_tests: bool,
}

// 默认值
fn default_collect_mode() -> CollectMode {
    CollectMode::Full
}
fn default_collect_dir() -> String {
    "src".to_string()
}
fn default_output_dir() -> String {
    "target/cjcov".to_string()
}
fn default_enable_branch_coverage() -> bool {
    true
}
fn default_enable_function_coverage() -> bool {
    true
}
fn default_sample_rate() -> u32 {
    10
}
fn default_report_formats() -> Vec<ReportFormat> {
    vec![ReportFormat::Text, ReportFormat::Html]
}
fn default_report_dir() -> String {
    "target/cjcov/reports".to_string()
}
fn default_show_uncovered() -> bool {
    true
}
fn default_generate_html() -> bool {
    true
}
fn default_exclude_tests() -> bool {
    true
}
fn default_exclude_generated() -> bool {
    true
}
fn default_fail_on_threshold() -> bool {
    true
}
fn default_compress_data() -> bool {
    true
}
fn default_parallel_collect() -> bool {
    true
}

/// 覆盖率结果
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CoverageResult {
    /// 汇总信息
    pub summary: CoverageSummary,
    /// 文件详细覆盖率
    pub files: HashMap<String, FileCoverage>,
    /// 阈值校验结果
    pub threshold_check: ThresholdCheckResult,
}

/// 覆盖率汇总
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CoverageSummary {
    /// 总行数
    pub total_lines: u32,
    /// 覆盖行数
    pub covered_lines: u32,
    /// 行覆盖率（%）
    pub line_coverage: f64,
    /// 总分支数
    pub total_branches: u32,
    /// 覆盖分支数
    pub covered_branches: u32,
    /// 分支覆盖率（%）
    pub branch_coverage: f64,
    /// 总函数数
    pub total_functions: u32,
    /// 覆盖函数数
    pub covered_functions: u32,
    /// 函数覆盖率（%）
    pub function_coverage: f64,
    /// 收集时长（秒）
    pub collect_duration: f64,
}

/// 文件覆盖率详情
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileCoverage {
    /// 文件路径
    pub path: String,
    /// 行覆盖率详情
    pub lines: HashMap<u32, LineCoverage>,
    /// 分支覆盖率详情
    pub branches: HashMap<u32, Vec<BranchCoverage>>,
    /// 函数覆盖率详情
    pub functions: HashMap<String, FunctionCoverage>,
    /// 行覆盖率（%）
    pub line_coverage: f64,
    /// 分支覆盖率（%）
    pub branch_coverage: f64,
    /// 函数覆盖率（%）
    pub function_coverage: f64,
}

/// 行覆盖率状态
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
pub enum LineCoverageStatus {
    Covered,   // 已覆盖
    Uncovered, // 未覆盖
    Skipped,   // 跳过（如注释、空行）
}

/// 行覆盖率详情
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineCoverage {
    pub status: LineCoverageStatus,
    pub execution_count: u32, // 执行次数
}

/// 分支覆盖率详情
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BranchCoverage {
    pub branch_id: u32,
    pub status: LineCoverageStatus,
    pub execution_count: u32,
}

/// 函数覆盖率详情
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionCoverage {
    pub name: String,
    pub start_line: u32,
    pub end_line: u32,
    pub status: LineCoverageStatus,
    pub execution_count: u32,
}

/// 阈值校验结果
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThresholdCheckResult {
    pub passed: bool,
    pub failures: Vec<ThresholdFailure>,
}

/// 阈值未达标项
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThresholdFailure {
    pub metric: String, // line/branch/function
    pub actual: f64,    // 实际值（%）
    pub threshold: u32, // 阈值（%）
    pub message: String,
}

/// cjcov 管理器
#[derive(Debug, Default)]
pub struct CjcovManager;

impl CjcovManager {
    /// 检查 cjcov 是否可用
    pub fn is_available() -> zed::Result<()> {
        Self::find_executable()?;
        Ok(())
    }

    /// 查找 cjcov 可执行文件
    pub fn find_executable() -> zed::Result<zed::Path> {
        // 1. 从 CANGJIE_HOME 查找
        if let Ok(cangjie_home) = zed::env::var("CANGJIE_HOME") {
            let mut path = zed::Path::new(&cangjie_home);
            path.push("bin");
            path.push(if zed::platform::is_windows() {
                "cjcov.exe"
            } else {
                "cjcov"
            });

            if path.exists() && path.is_executable() {
                return Ok(path);
            }
        }

        // 2. 从 PATH 查找
        if let Some(path) = zed::env::find_executable(if zed::platform::is_windows() {
            "cjcov.exe"
        } else {
            "cjcov"
        }) {
            return Ok(path);
        }

        Err(zed::Error::NotFound(
            "未找到 cjcov 工具，请配置 CANGJIE_HOME 或确保 cjcov 在 PATH 中".to_string(),
        ))
    }

    /// 加载 cjcov 配置
    pub fn load_config(
        worktree: &zed::Worktree,
        extension_config: &super::config::CangjieConfig,
    ) -> zed::Result<CjcovConfig> {
        // 1. 项目根目录 cjcov.toml
        let project_config = worktree.path().join("cjcov.toml");
        if project_config.exists() {
            return Self::parse_config(&project_config);
        }

        // 2. 用户目录 .cjcov.toml
        if let Some(user_config) = Self::user_config_path() {
            if user_config.exists() {
                return Self::parse_config(&user_config);
            }
        }

        // 3. 扩展配置
        Ok(extension_config.cjcov.clone())
    }

    /// 解析配置文件
    fn parse_config(path: &zed::Path) -> zed::Result<CjcovConfig> {
        let content = zed::fs::read_to_string(path)
            .map_err(|e| zed::Error::IoError(format!("读取 {} 失败: {}", path.to_str()?, e)))?;

        toml::from_str(&content)
            .map_err(|e| zed::Error::InvalidConfig(format!("解析 {} 失败: {}", path.to_str()?, e)))
    }

    /// 用户目录配置路径
    fn user_config_path() -> Option<zed::Path> {
        zed::env::home_dir().map(|home| home.join(".cjcov.toml"))
    }

    /// 执行覆盖率收集
    pub fn collect_coverage(
        worktree: &zed::Worktree,
        config: &CjcovConfig,
        test_command: &str,
        test_args: &[String],
    ) -> zed::Result<CoverageResult> {
        let cjcov_path = Self::find_executable()?;
        let mut args = Vec::new();

        // 收集配置
        args.push(format!(
            "--collect-mode={}",
            match config.collect.mode {
                CollectMode::Full => "full",
                CollectMode::Partial => "partial",
                CollectMode::Fast => "fast",
            }
        ));
        args.push(format!("--collect-dir={}", config.collect.dir));
        let output_dir = worktree.path().join(&config.collect.output_dir);
        args.push(format!("--output-dir={}", output_dir.to_str()?));

        if config.collect.enable_branch {
            args.push("--enable-branch-coverage".to_string());
        }
        if config.collect.enable_function {
            args.push("--enable-function-coverage".to_string());
        }
        if config.collect.mode == CollectMode::Partial {
            args.push(format!("--sample-rate={}", config.collect.sample_rate));
        }

        // 报告配置
        args.push("--report-formats".to_string());
        args.push(
            config
                .report
                .formats
                .iter()
                .map(|f| match f {
                    ReportFormat::Text => "text",
                    ReportFormat::Html => "html",
                    ReportFormat::Json => "json",
                    ReportFormat::Xml => "xml",
                    ReportFormat::Sarif => "sarif",
                })
                .collect::<Vec<_>>()
                .join(","),
        );

        let report_dir = worktree.path().join(&config.report.dir);
        args.push(format!("--report-dir={}", report_dir.to_str()?));

        if config.report.show_uncovered {
            args.push("--show-uncovered".to_string());
        }
        if config.report.generate_html {
            args.push("--generate-html".to_string());
        }
        if config.report.generate_xml {
            args.push("--generate-xml".to_string());
        }
        if config.report.generate_sarif {
            args.push("--generate-sarif".to_string());
        }

        // 过滤配置
        if !config.filter.include.is_empty() {
            args.push("--include".to_string());
            args.push(config.filter.include.join(","));
        }
        if !config.filter.exclude.is_empty() {
            args.push("--exclude".to_string());
            args.push(config.filter.exclude.join(","));
        }
        if config.filter.exclude_tests {
            args.push("--exclude-tests".to_string());
        }
        if config.filter.exclude_generated {
            args.push("--exclude-generated".to_string());
        }

        // 阈值配置
        if let Some(line) = config.threshold.line {
            args.push(format!("--threshold-line={}", line));
        }
        if let Some(branch) = config.threshold.branch {
            args.push(format!("--threshold-branch={}", branch));
        }
        if let Some(function) = config.threshold.function {
            args.push(format!("--threshold-function={}", function));
        }
        if config.threshold.fail_on_missing {
            args.push("--fail-on-missing".to_string());
        }

        // 高级配置
        if config.advanced.save_raw_data {
            args.push("--save-raw-data".to_string());
        }
        if config.advanced.compress_data {
            args.push("--compress-data".to_string());
        }
        if config.advanced.parallel_collect {
            args.push("--parallel-collect".to_string());
        }
        if config.advanced.ignore_unrun_tests {
            args.push("--ignore-unrun-tests".to_string());
        }

        // 测试命令（-- 后面跟测试命令和参数）
        args.push("--".to_string());
        args.push(test_command.to_string());
        args.extend(test_args.iter().cloned());

        // 执行覆盖率收集
        let output = zed::process::Command::new(cjcov_path.to_str()?)
            .args(&args)
            .current_dir(worktree.path())
            .stdout(zed::process::Stdio::piped())
            .stderr(zed::process::Stdio::piped())
            .output()
            .map_err(|e| zed::Error::ProcessFailed(format!("启动 cjcov 失败: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(zed::Error::ProcessFailed(format!(
                "覆盖率收集失败: {}",
                stderr.trim()
            )));
        }

        // 解析 JSON 结果
        let json_report_path = report_dir.join("coverage_result.json");
        let json_content = zed::fs::read_to_string(&json_report_path)
            .map_err(|e| zed::Error::IoError(format!("读取覆盖率报告失败: {}", e)))?;

        serde_json::from_str(&json_content)
            .map_err(|e| zed::Error::InvalidConfig(format!("解析覆盖率报告失败: {}", e)))
    }

    /// 生成 Zed 诊断信息（未覆盖代码标黄）
    pub fn convert_to_diagnostics(&self, result: &CoverageResult) -> Vec<zed::Diagnostic> {
        let mut diagnostics = Vec::new();

        for (file_path, file_coverage) in &result.files {
            // 行覆盖率诊断
            for (line_num, line_coverage) in &file_coverage.lines {
                if line_coverage.status == LineCoverageStatus::Uncovered {
                    diagnostics.push(zed::Diagnostic {
                        range: zed::Range {
                            start: zed::Position {
                                line: line_num - 1,
                                column: 0,
                            },
                            end: zed::Position {
                                line: line_num - 1,
                                column: 1000,
                            },
                        },
                        severity: zed::DiagnosticSeverity::Warn,
                        code: Some(zed::DiagnosticCode {
                            value: "CJCOV-UNCOVERED-LINE".to_string(),
                            description: Some("未覆盖的代码行".to_string()),
                        }),
                        message: format!(
                            "代码行未被测试覆盖（执行次数：{}）",
                            line_coverage.execution_count
                        ),
                        source: Some("cjcov".to_string()),
                        fixes: None,
                    });
                }
            }

            // 分支覆盖率诊断
            for (line_num, branches) in &file_coverage.branches {
                for branch in branches {
                    if branch.status == LineCoverageStatus::Uncovered {
                        diagnostics.push(zed::Diagnostic {
                            range: zed::Range {
                                start: zed::Position {
                                    line: line_num - 1,
                                    column: 0,
                                },
                                end: zed::Position {
                                    line: line_num - 1,
                                    column: 1000,
                                },
                            },
                            severity: zed::DiagnosticSeverity::Info,
                            code: Some(zed::DiagnosticCode {
                                value: "CJCOV-UNCOVERED-BRANCH".to_string(),
                                description: Some("未覆盖的分支".to_string()),
                            }),
                            message: format!(
                                "分支 {} 未被测试覆盖（执行次数：{}）",
                                branch.branch_id, branch.execution_count
                            ),
                            source: Some("cjcov".to_string()),
                            fixes: None,
                        });
                    }
                }
            }
        }

        // 阈值未达标诊断
        if !result.threshold_check.passed {
            for failure in &result.threshold_check.failures {
                diagnostics.push(zed::Diagnostic {
                    range: zed::Range {
                        start: zed::Position { line: 0, column: 0 },
                        end: zed::Position {
                            line: 0,
                            column: 1000,
                        },
                    },
                    severity: zed::DiagnosticSeverity::Error,
                    code: Some(zed::DiagnosticCode {
                        value: "CJCOV-THRESHOLD-FAIL".to_string(),
                        description: Some("覆盖率阈值未达标".to_string()),
                    }),
                    message: failure.message.clone(),
                    source: Some("cjcov".to_string()),
                    fixes: None,
                });
            }
        }

        diagnostics
    }

    /// 打开 HTML 覆盖率报告
    pub fn open_html_report(worktree: &zed::Worktree, config: &CjcovConfig) -> zed::Result<()> {
        let report_dir = worktree.path().join(&config.report.dir);
        let html_report_path = report_dir.join("index.html");

        if !html_report_path.exists() {
            return Err(zed::Error::NotFound(
                "未找到 HTML 覆盖率报告，请先执行覆盖率收集".to_string(),
            ));
        }

        zed::shell::open(&html_report_path)
            .map_err(|e| zed::Error::ProcessFailed(format!("打开报告失败: {}", e)))?;

        Ok(())
    }
}
