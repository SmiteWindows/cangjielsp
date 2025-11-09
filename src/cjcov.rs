//! 代码覆盖率工具 cjcov 集成
use crate::config::CangjieConfig;
use serde::{Deserialize, Serialize};
use zed_extension_api;

/// 采样配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleConfig {
    /// 采样模式（full/partial）
    pub mode: String,
    /// 包含的源文件路径
    pub include: Vec<String>,
    /// 排除的文件路径
    pub exclude: Vec<String>,
    /// 启用分支覆盖率
    pub enable_branch: bool,
    /// 启用函数覆盖率
    pub enable_function: bool,
}

/// 报告配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    /// 报告格式（html/json/lcov）
    pub formats: Vec<String>,
    /// 报告输出目录
    pub output_dir: String,
    /// 生成详细报告
    pub detailed: bool,
    /// 显示未覆盖的代码
    pub show_uncovered: bool,
}

/// 过滤配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    /// 忽略测试文件
    pub ignore_tests: bool,
    /// 忽略生成的代码
    pub ignore_generated: bool,
    /// 忽略注释行
    pub ignore_comments: bool,
}

/// 阈值配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfig {
    /// 行覆盖率阈值（%）
    pub line: f64,
    /// 分支覆盖率阈值（%）
    pub branch: Option<f64>,
    /// 函数覆盖率阈值（%）
    pub function: Option<f64>,
    /// 严格模式（不达标则失败）
    pub strict: bool,
}

/// 高级配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedConfig {
    /// 采样缓冲区大小（MB）
    pub buffer_size: u32,
    /// 启用增量覆盖率
    pub incremental: bool,
    /// 覆盖率数据文件路径
    pub data_file: String,
}

/// cjcov 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CjcovConfig {
    /// 采样配置
    pub collect: SampleConfig,
    /// 报告配置
    pub report: ReportConfig,
    /// 过滤配置
    pub filter: FilterConfig,
    /// 阈值配置
    pub threshold: ThresholdConfig,
    /// 高级配置
    pub advanced: AdvancedConfig,
}

/// 覆盖率汇总结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageSummary {
    /// 行覆盖率（%）
    pub line_coverage: f64,
    /// 已覆盖行数
    pub covered_lines: u32,
    /// 总行数
    pub total_lines: u32,
    /// 分支覆盖率（%）
    pub branch_coverage: Option<f64>,
    /// 已覆盖分支数
    pub covered_branches: Option<u32>,
    /// 总分支数
    pub total_branches: Option<u32>,
    /// 函数覆盖率（%）
    pub function_coverage: Option<f64>,
    /// 已覆盖函数数
    pub covered_functions: Option<u32>,
    /// 总函数数
    pub total_functions: Option<u32>,
}

/// 阈值检查失败项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdFailure {
    /// 类型（line/branch/function）
    pub r#type: String,
    /// 实际覆盖率（%）
    pub actual: f64,
    /// 要求覆盖率（%）
    pub required: f64,
}

/// 阈值检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdCheckResult {
    /// 是否通过
    pub passed: bool,
    /// 失败项列表
    pub failures: Vec<ThresholdFailure>,
}

/// 覆盖率收集结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageResult {
    /// 覆盖率汇总
    pub summary: CoverageSummary,
    /// 阈值检查结果
    pub threshold_check: ThresholdCheckResult,
    /// 报告文件路径列表
    pub report_files: Vec<String>,
}

impl Default for SampleConfig {
    fn default() -> Self {
        Self {
            mode: "full".to_string(),
            include: vec!["src/**/*.cj".to_string()],
            exclude: vec!["src/test/**/*.cj".to_string()],
            enable_branch: true,
            enable_function: true,
        }
    }
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            formats: vec!["html".to_string(), "json".to_string()],
            output_dir: "target/coverage".to_string(),
            detailed: true,
            show_uncovered: true,
        }
    }
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            ignore_tests: true,
            ignore_generated: true,
            ignore_comments: true,
        }
    }
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        Self {
            line: 80.0,
            branch: Some(70.0),
            function: Some(75.0),
            strict: true,
        }
    }
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            buffer_size: 16,
            incremental: false,
            data_file: "target/coverage.data".to_string(),
        }
    }
}

/// cjcov 管理器
#[derive(Debug, Default)]
pub struct CjcovManager;

impl CjcovManager {
    /// 检查 cjcov 是否可用
    pub fn is_available() -> zed_extension_api::Result<()> {
        if std::process::Command::new("cjcov")
            .arg("--version")
            .output()
            .is_err()
        {
            return Err(zed_extension_api::Error::NotFound(
                "cjcov 工具未找到，请安装并配置到 PATH 中".to_string(),
            ));
        }
        Ok(())
    }

    /// 加载配置
    pub fn load_config(
        worktree: &zed_extension_api::Worktree,
        config: &CangjieConfig,
    ) -> zed_extension_api::Result<CjcovConfig> {
        // 加载 cjcov.toml 配置（如果存在）
        let config_path = worktree.path().join("cjcov.toml");
        if config_path.exists() {
            let config_content = std::fs::read_to_string(&config_path).map_err(|e| {
                zed_extension_api::Error::IoError(format!("读取 cjcov 配置失败: {}", e))
            })?;
            let toml_config: CjcovConfig = toml::from_str(&config_content).map_err(|e| {
                zed_extension_api::Error::InvalidData(format!("解析 cjcov 配置失败: {}", e))
            })?;
            return Ok(toml_config);
        }

        // 使用默认配置
        Ok(config.cjcov.clone())
    }

    /// 收集代码覆盖率
    pub fn collect_coverage(
        worktree: &zed_extension_api::Worktree,
        config: &CjcovConfig,
        test_command: &str,
        test_args: &[String],
    ) -> zed_extension_api::Result<CoverageResult> {
        Self::is_available()?;

        let mut args = vec!["collect".to_string()];

        // 添加采样配置参数
        args.push(format!("--mode={}", config.collect.mode));
        for include in &config.collect.include {
            args.push(format!("--include={}", include));
        }
        for exclude in &config.collect.exclude {
            args.push(format!("--exclude={}", exclude));
        }
        if config.collect.enable_branch {
            args.push("--enable-branch".to_string());
        }
        if config.collect.enable_function {
            args.push("--enable-function".to_string());
        }

        // 添加报告配置参数
        for format in &config.report.formats {
            args.push(format!("--format={}", format));
        }
        args.push(format!("--output-dir={}", config.report.output_dir));
        if config.report.detailed {
            args.push("--detailed".to_string());
        }
        if config.report.show_uncovered {
            args.push("--show-uncovered".to_string());
        }

        // 添加过滤配置参数
        if config.filter.ignore_tests {
            args.push("--ignore-tests".to_string());
        }
        if config.filter.ignore_generated {
            args.push("--ignore-generated".to_string());
        }
        if config.filter.ignore_comments {
            args.push("--ignore-comments".to_string());
        }

        // 添加阈值配置参数
        args.push(format!("--line-threshold={}", config.threshold.line));
        if let Some(branch) = config.threshold.branch {
            args.push(format!("--branch-threshold={}", branch));
        }
        if let Some(function) = config.threshold.function {
            args.push(format!("--function-threshold={}", function));
        }
        if config.threshold.strict {
            args.push("--strict".to_string());
        }

        // 添加高级配置参数
        args.push(format!("--buffer-size={}", config.advanced.buffer_size));
        if config.advanced.incremental {
            args.push("--incremental".to_string());
        }
        args.push(format!("--data-file={}", config.advanced.data_file));

        // 添加测试命令和参数（使用 -- 分隔）
        args.push("--".to_string());
        args.push(test_command.to_string());
        args.extend_from_slice(test_args);

        // 执行覆盖率收集命令
        let output = std::process::Command::new("cjcov")
            .args(&args)
            .current_dir(worktree.path())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(zed_extension_api::Error::ProcessFailed(format!(
                "覆盖率收集失败: {}",
                stderr
            )));
        }

        // 解析 JSON 结果
        let coverage_result: CoverageResult =
            serde_json::from_slice(&output.stdout).map_err(|e| {
                zed_extension_api::Error::InvalidData(format!("解析覆盖率结果失败: {}", e))
            })?;

        Ok(coverage_result)
    }

    /// 打开 HTML 覆盖率报告
    pub fn open_html_report(
        worktree: &zed_extension_api::Worktree,
        config: &CjcovConfig,
    ) -> zed_extension_api::Result<()> {
        let report_dir = worktree.path().join(&config.report.output_dir);
        let index_path = report_dir.join("index.html");

        if !index_path.exists() {
            return Err(zed_extension_api::Error::NotFound(format!(
                "HTML 覆盖率报告未找到: {}",
                index_path.to_str().unwrap()
            )));
        }

        // 跨平台打开文件
        #[cfg(windows)]
        std::process::Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg(index_path)
            .spawn()?;

        #[cfg(unix)]
        {
            if cfg!(macos) {
                std::process::Command::new("open").arg(index_path).spawn()?;
            } else {
                std::process::Command::new("xdg-open")
                    .arg(index_path)
                    .spawn()?;
            }
        }

        Ok(())
    }
}
