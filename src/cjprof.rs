//! 性能分析工具 cjprof 集成
use crate::config::CangjieConfig;
use serde::{Deserialize, Serialize};
use zed_extension_api;

/// 采样配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleConfig {
    /// 采样类型（cpu/memory/all）
    pub r#type: String,
    /// 采样频率（Hz）
    pub frequency: u32,
    /// 采样时长（秒）
    pub duration: Option<u32>,
    /// 包含的函数/模块
    pub include: Vec<String>,
    /// 排除的函数/模块
    pub exclude: Vec<String>,
    /// 启用协程分析
    pub enable_coroutine: bool,
    /// 启用内存泄漏检测
    pub enable_leak_detection: bool,
}

/// 分析配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeConfig {
    /// 热点阈值（%）
    pub hotspot_threshold: f64,
    /// 合并相同函数
    pub merge_same_functions: bool,
    /// 显示调用栈深度
    pub call_stack_depth: u32,
    /// 分析内存分配
    pub analyze_allocations: bool,
    /// 分析内存释放
    pub analyze_frees: bool,
}

/// 报告配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    /// 报告格式（flamegraph/json/text）
    pub formats: Vec<String>,
    /// 报告输出目录
    pub output_dir: String,
    /// 生成交互式报告
    pub interactive: bool,
    /// 显示详细统计
    pub detailed: bool,
}

/// 过滤配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    /// 忽略系统函数
    pub ignore_system: bool,
    /// 忽略测试函数
    pub ignore_tests: bool,
    /// 忽略生成的代码
    pub ignore_generated: bool,
}

/// 阈值配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfig {
    /// CPU 热点阈值（%）
    pub cpu_hotspot: f64,
    /// 内存热点阈值（MB）
    pub memory_hotspot: f64,
    /// 内存泄漏阈值（MB）
    pub leak_threshold: f64,
}

/// 高级配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedConfig {
    /// 采样缓冲区大小（MB）
    pub buffer_size: u32,
    /// 启用增量分析
    pub incremental: bool,
    /// 保存原始采样数据
    pub save_raw_data: bool,
    /// 原始数据文件路径
    pub raw_data_file: String,
}

/// 采样信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleInfo {
    /// 采样时长（秒）
    pub duration: f64,
    /// 采样总数
    pub sample_count: u32,
    /// CPU 采样数
    pub cpu_sample_count: u32,
    /// 内存采样数
    pub memory_sample_count: u32,
    /// 平均采样频率（Hz）
    pub avg_frequency: f64,
}

/// CPU 热点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuHotspot {
    /// 函数名
    pub function_name: String,
    /// 模块名
    pub module_name: String,
    /// 文件路径
    pub file_path: String,
    /// 行号
    pub line_number: u32,
    /// CPU 使用率（%）
    pub cpu_usage: f64,
    /// 平均执行时间（ms）
    pub avg_execution_time: f64,
    /// 调用次数
    pub call_count: u32,
}

/// 内存热点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHotspot {
    /// 函数名
    pub function_name: String,
    /// 模块名
    pub module_name: String,
    /// 文件路径
    pub file_path: String,
    /// 行号
    pub line_number: u32,
    /// 分配内存大小（MB）
    pub allocated_size_mb: f64,
    /// 分配次数
    pub allocation_count: u32,
    /// 平均分配大小（KB）
    pub avg_allocation_size_kb: f64,
}

/// 内存泄漏信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLeak {
    /// 对象类型
    pub object_type: String,
    /// 泄漏大小（MB）
    pub size_mb: f64,
    /// 对象数量
    pub object_count: u32,
    /// 主要分配位置
    pub allocation_location: String,
}

/// 性能分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingResult {
    /// 采样信息
    pub sample_info: SampleInfo,
    /// CPU 热点列表（按使用率排序）
    pub cpu_hotspots: Vec<CpuHotspot>,
    /// 内存热点列表（按分配大小排序）
    pub memory_hotspots: Vec<MemoryHotspot>,
    /// 协程数量
    pub coroutine_count: u32,
    /// 内存泄漏列表
    pub memory_leaks: Vec<MemoryLeak>,
    /// 报告文件路径列表
    pub report_files: Vec<String>,
}

/// cjprof 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CjprofConfig {
    /// 采样配置
    pub sample: SampleConfig,
    /// 分析配置
    pub analyze: AnalyzeConfig,
    /// 报告配置
    pub report: ReportConfig,
    /// 过滤配置
    pub filter: FilterConfig,
    /// 阈值配置
    pub threshold: ThresholdConfig,
    /// 高级配置
    pub advanced: AdvancedConfig,
}

impl Default for SampleConfig {
    fn default() -> Self {
        Self {
            r#type: "all".to_string(),
            frequency: 100,
            duration: None,
            include: Vec::new(),
            exclude: vec!["std::*".to_string(), "sys::*".to_string()],
            enable_coroutine: true,
            enable_leak_detection: true,
        }
    }
}

impl Default for AnalyzeConfig {
    fn default() -> Self {
        Self {
            hotspot_threshold: 5.0,
            merge_same_functions: true,
            call_stack_depth: 10,
            analyze_allocations: true,
            analyze_frees: true,
        }
    }
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            formats: vec!["flamegraph".to_string(), "json".to_string()],
            output_dir: "target/profiling".to_string(),
            interactive: true,
            detailed: true,
        }
    }
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            ignore_system: true,
            ignore_tests: false,
            ignore_generated: true,
        }
    }
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        Self {
            cpu_hotspot: 5.0,
            memory_hotspot: 10.0,
            leak_threshold: 5.0,
        }
    }
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            buffer_size: 32,
            incremental: false,
            save_raw_data: false,
            raw_data_file: "target/profiling.raw".to_string(),
        }
    }
}

/// cjprof 管理器
#[derive(Debug, Default)]
pub struct CjprofManager;

impl CjprofManager {
    /// 检查 cjprof 是否可用
    pub fn is_available() -> zed_extension_api::Result<()> {
        if std::process::Command::new("cjprof")
            .arg("--version")
            .output()
            .is_err()
        {
            return Err(zed_extension_api::Error::NotFound(
                "cjprof 工具未找到，请安装并配置到 PATH 中".to_string(),
            ));
        }
        Ok(())
    }

    /// 加载配置
    pub fn load_config(
        worktree: &zed_extension_api::Worktree,
        config: &CangjieConfig,
    ) -> zed_extension_api::Result<CjprofConfig> {
        // 加载 cjprof.toml 配置（如果存在）
        let config_path = worktree.path().join("cjprof.toml");
        if config_path.exists() {
            let config_content = std::fs::read_to_string(&config_path).map_err(|e| {
                zed_extension_api::Error::IoError(format!("读取 cjprof 配置失败: {}", e))
            })?;
            let toml_config: CjprofConfig = toml::from_str(&config_content).map_err(|e| {
                zed_extension_api::Error::InvalidData(format!("解析 cjprof 配置失败: {}", e))
            })?;
            return Ok(toml_config);
        }

        // 使用默认配置
        Ok(config.cjprof.clone())
    }

    /// 启动性能分析
    pub fn start_profiling(
        worktree: &zed_extension_api::Worktree,
        config: &CjprofConfig,
        target_binary: &str,
        args: &[String],
    ) -> zed_extension_api::Result<ProfilingResult> {
        Self::is_available()?;

        let mut command_args = vec!["profile".to_string()];

        // 添加采样配置参数
        command_args.push(format!("--type={}", config.sample.r#type));
        command_args.push(format!("--frequency={}", config.sample.frequency));
        if let Some(duration) = config.sample.duration {
            command_args.push(format!("--duration={}", duration));
        }
        for include in &config.sample.include {
            command_args.push(format!("--include={}", include));
        }
        for exclude in &config.sample.exclude {
            command_args.push(format!("--exclude={}", exclude));
        }
        if config.sample.enable_coroutine {
            command_args.push("--enable-coroutine".to_string());
        }
        if config.sample.enable_leak_detection {
            command_args.push("--enable-leak-detection".to_string());
        }

        // 添加分析配置参数
        command_args.push(format!(
            "--hotspot-threshold={}",
            config.analyze.hotspot_threshold
        ));
        if config.analyze.merge_same_functions {
            command_args.push("--merge-same-functions".to_string());
        }
        command_args.push(format!(
            "--call-stack-depth={}",
            config.analyze.call_stack_depth
        ));
        if config.analyze.analyze_allocations {
            command_args.push("--analyze-allocations".to_string());
        }
        if config.analyze.analyze_frees {
            command_args.push("--analyze-frees".to_string());
        }

        // 添加报告配置参数
        for format in &config.report.formats {
            command_args.push(format!("--format={}", format));
        }
        command_args.push(format!("--output-dir={}", config.report.output_dir));
        if config.report.interactive {
            command_args.push("--interactive".to_string());
        }
        if config.report.detailed {
            command_args.push("--detailed".to_string());
        }

        // 添加过滤配置参数
        if config.filter.ignore_system {
            command_args.push("--ignore-system".to_string());
        }
        if config.filter.ignore_tests {
            command_args.push("--ignore-tests".to_string());
        }
        if config.filter.ignore_generated {
            command_args.push("--ignore-generated".to_string());
        }

        // 添加阈值配置参数
        command_args.push(format!(
            "--cpu-hotspot-threshold={}",
            config.threshold.cpu_hotspot
        ));
        command_args.push(format!(
            "--memory-hotspot-threshold={}",
            config.threshold.memory_hotspot
        ));
        command_args.push(format!(
            "--leak-threshold={}",
            config.threshold.leak_threshold
        ));

        // 添加高级配置参数
        command_args.push(format!("--buffer-size={}", config.advanced.buffer_size));
        if config.advanced.incremental {
            command_args.push("--incremental".to_string());
        }
        if config.advanced.save_raw_data {
            command_args.push("--save-raw-data".to_string());
            command_args.push(format!("--raw-data-file={}", config.advanced.raw_data_file));
        }

        // 添加目标程序和参数（使用 -- 分隔）
        command_args.push("--".to_string());
        command_args.push(target_binary.to_string());
        command_args.extend_from_slice(args);

        // 执行性能分析命令
        let output = std::process::Command::new("cjprof")
            .args(&command_args)
            .current_dir(worktree.path())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(zed_extension_api::Error::ProcessFailed(format!(
                "性能分析失败: {}",
                stderr
            )));
        }

        // 解析 JSON 结果
        let profiling_result: ProfilingResult =
            serde_json::from_slice(&output.stdout).map_err(|e| {
                zed_extension_api::Error::InvalidData(format!("解析性能分析结果失败: {}", e))
            })?;

        Ok(profiling_result)
    }

    /// 打开火焰图报告
    pub fn open_flamegraph(
        worktree: &zed_extension_api::Worktree,
        config: &CjprofConfig,
    ) -> zed_extension_api::Result<()> {
        let report_dir = worktree.path().join(&config.report.output_dir);
        let flamegraph_path = if config.sample.r#type == "cpu" || config.sample.r#type == "all" {
            report_dir.join("cpu_flamegraph.html")
        } else {
            report_dir.join("memory_flamegraph.html")
        };

        if !flamegraph_path.exists() {
            return Err(zed_extension_api::Error::NotFound(format!(
                "火焰图报告未找到: {}",
                flamegraph_path.to_str().unwrap()
            )));
        }

        // 跨平台打开文件
        #[cfg(windows)]
        std::process::Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg(flamegraph_path)
            .spawn()?;

        #[cfg(unix)]
        {
            if cfg!(macos) {
                std::process::Command::new("open")
                    .arg(flamegraph_path)
                    .spawn()?;
            } else {
                std::process::Command::new("xdg-open")
                    .arg(flamegraph_path)
                    .spawn()?;
            }
        }

        Ok(())
    }
}
