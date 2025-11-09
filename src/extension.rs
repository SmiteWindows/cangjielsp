//! 扩展命令处理（适配 zed_extension_api 0.7.0）
use log::{debug, info};
use std::sync::Arc;
use zed_extension_api::Extension as ZedExtension;

use crate::{
    cjcov::CjcovManager, cjdb::CjdbManager, cjfmt::CjfmtManager, cjlint::CjlintManager,
    cjpm::CjpmManager, cjprof::CjprofManager, config::CangjieConfig,
    language_server::CangjieLanguageServer, tree_sitter_utils,
};

/// 仓颉扩展主结构体
pub struct CangjieExtension {
    config: Arc<CangjieConfig>,
    lsp_server: CangjieLanguageServer,
    worktree: Option<zed_extension_api::Worktree>,
}

impl CangjieExtension {
    /// 创建扩展实例
    pub fn new(config: Arc<CangjieConfig>, lsp_server: CangjieLanguageServer) -> Self {
        Self {
            config,
            lsp_server,
            worktree: None,
        }
    }

    /// 格式化文档
    pub fn format_document(
        &mut self,
        document: &mut zed_extension_api::Document,
    ) -> zed_extension_api::Result<()> {
        info!(
            "执行代码格式化: {}",
            document.path().to_str().unwrap_or("未知文件")
        );
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed_extension_api::Error::NotFound("未初始化工作目录".to_string()))?;

        // 加载配置
        let cjfmt_config = CjfmtManager::load_config(worktree, &self.config)?;
        // 执行格式化
        let edits = CjfmtManager::format_document(worktree, document, &cjfmt_config)?;

        if let Some(edits) = edits {
            document.apply_edits(edits)?;
            info!("格式化完成");
        } else {
            info!("文档已符合格式规范，无需修改");
        }

        Ok(())
    }

    /// 执行代码检查
    pub fn run_lint(
        &mut self,
        document: &zed_extension_api::Document,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::Diagnostic>> {
        info!(
            "执行代码检查: {}",
            document.path().to_str().unwrap_or("未知文件")
        );
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed_extension_api::Error::NotFound("未初始化工作目录".to_string()))?;

        // 加载配置
        let cjlint_config = CjlintManager::load_config(worktree, &self.config)?;
        // 执行代码检查
        let diagnostics = CjlintManager::run_lint(worktree, document, &cjlint_config)?;

        info!("代码检查完成，发现 {} 个问题", diagnostics.len());
        Ok(diagnostics)
    }

    /// 构建项目
    pub fn build_project(&mut self) -> zed_extension_api::Result<()> {
        info!("开始构建项目");
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed_extension_api::Error::NotFound("未初始化工作目录".to_string()))?;

        // 检查 cjpm 是否可用
        CjpmManager::is_available()?;

        // 加载配置
        let cjpm_config = CjpmManager::load_config(worktree)?;

        // 安装依赖
        info!("安装项目依赖...");
        CjpmManager::install_dependencies(worktree)?;

        // 构建项目
        info!("开始编译项目（发布模式: {}）", cjpm_config.build.release);
        CjpmManager::build_project(worktree, &cjpm_config)?;

        info!("项目构建完成");
        Ok(())
    }

    /// 启动调试会话
    pub fn start_debug_session(&mut self, args: &[String]) -> zed_extension_api::Result<()> {
        info!("启动调试会话，参数: {:?}", args);
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed_extension_api::Error::NotFound("未初始化工作目录".to_string()))?;

        // 检查 cjdb 是否可用
        CjdbManager::is_available()?;

        // 加载配置
        let cjdb_config = CjdbManager::load_config(worktree)?;

        // 自动识别目标产物
        let target_binary = CjpmManager::auto_detect_target(worktree)?;
        info!("调试目标: {}", target_binary);

        // 启动调试会话
        let mut session =
            CjdbManager::start_debug_session(worktree, &cjdb_config, &target_binary, args)?;

        // 注册调试会话到 Zed
        zed_extension_api::debug::register_session(session).map_err(|e| {
            zed_extension_api::Error::ProcessFailed(format!("注册调试会话失败: {}", e))
        })?;

        info!("调试会话启动成功，端口: {}", cjdb_config.session.port);
        Ok(())
    }

    /// 收集代码覆盖率
    pub fn collect_coverage(
        &mut self,
        test_command: &str,
        test_args: &[String],
    ) -> zed_extension_api::Result<()> {
        info!("收集代码覆盖率，测试命令: {} {:?}", test_command, test_args);
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed_extension_api::Error::NotFound("未初始化工作目录".to_string()))?;

        // 检查 cjcov 是否可用
        CjcovManager::is_available()?;

        // 加载配置
        let cjcov_config = CjcovManager::load_config(worktree, &self.config)?;

        // 收集覆盖率
        let coverage_result =
            CjcovManager::collect_coverage(worktree, &cjcov_config, test_command, test_args)?;

        // 输出覆盖率汇总
        let summary = &coverage_result.summary;
        info!(
            "覆盖率收集完成:\n  行覆盖率: {:.2}% ({}/{})\n  分支覆盖率: {:.2}% ({}/{})\n  函数覆盖率: {:.2}% ({}/{})",
            summary.line_coverage,
            summary.covered_lines,
            summary.total_lines,
            summary.branch_coverage.unwrap_or(0.0),
            summary.covered_branches.unwrap_or(0),
            summary.total_branches.unwrap_or(0),
            summary.function_coverage.unwrap_or(0.0),
            summary.covered_functions.unwrap_or(0),
            summary.total_functions.unwrap_or(0)
        );

        // 检查阈值是否达标
        if !coverage_result.threshold_check.passed {
            info!("覆盖率未达阈值要求:");
            for failure in &coverage_result.threshold_check.failures {
                info!(
                    "  {}: 实际 {:.2}% < 要求 {}%",
                    failure.r#type, failure.actual, failure.required
                );
            }
            return Err(zed_extension_api::Error::ProcessFailed(
                "覆盖率未达阈值要求".to_string(),
            ));
        }

        // 打开 HTML 报告
        CjcovManager::open_html_report(worktree, &cjcov_config)?;

        Ok(())
    }

    /// 执行性能分析
    pub fn run_profiling(
        &mut self,
        target_binary: &str,
        args: &[String],
    ) -> zed_extension_api::Result<()> {
        info!("执行性能分析，目标: {} {:?}", target_binary, args);
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed_extension_api::Error::NotFound("未初始化工作目录".to_string()))?;

        // 检查 cjprof 是否可用
        CjprofManager::is_available()?;

        // 加载配置
        let cjprof_config = CjprofManager::load_config(worktree, &self.config)?;

        // 执行性能分析
        let profiling_result =
            CjprofManager::start_profiling(worktree, &cjprof_config, target_binary, args)?;

        // 输出性能分析汇总
        info!(
            "性能分析完成:\n  采样时长: {:.2}秒\n  CPU 热点数: {}\n  内存热点数: {}\n  协程数: {}\n  内存泄漏数: {}",
            profiling_result.sample_info.duration,
            profiling_result.cpu_hotspots.len(),
            profiling_result.memory_hotspots.len(),
            profiling_result.coroutine_count,
            profiling_result.memory_leaks.len()
        );

        // 输出 TOP 3 CPU 热点
        info!("TOP 3 CPU 热点:");
        for (i, hotspot) in profiling_result.cpu_hotspots.iter().take(3).enumerate() {
            info!(
                "  {}. {} ({}): {:.2}% | 平均耗时: {:.2}ms",
                i + 1,
                hotspot.function_name,
                hotspot.module_name,
                hotspot.cpu_usage,
                hotspot.avg_execution_time
            );
        }

        // 检查内存泄漏
        if !profiling_result.memory_leaks.is_empty() {
            info!("发现内存泄漏:");
            for leak in &profiling_result.memory_leaks {
                info!(
                    "  类型: {} | 大小: {:.2}MB | 数量: {}",
                    leak.object_type, leak.size_mb, leak.object_count
                );
            }
        }

        // 打开火焰图报告
        let cjprof_manager = CjprofManager::default();
        cjprof_manager.open_flamegraph(worktree, &cjprof_config)?;

        Ok(())
    }

    /// 生成性能优化建议
    pub fn generate_optimization_hints(
        &mut self,
        document: &zed_extension_api::Document,
    ) -> zed_extension_api::Result<Vec<String>> {
        info!(
            "生成性能优化建议: {}",
            document.path().to_str().unwrap_or("未知文件")
        );

        // 基于代码片段生成建议（示例实现）
        let suggestions = vec![
            "建议减少嵌套循环，可使用迭代器替代".to_string(),
            "大集合操作建议使用批量处理API".to_string(),
            "频繁字符串拼接建议使用 String::with_capacity 预分配空间".to_string(),
        ];

        Ok(suggestions)
    }
}

/// 扩展初始化
#[no_mangle]
pub extern "C" fn init() -> Box<dyn ZedExtension> {
    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    info!("仓颉 LSP 扩展初始化（版本: {}）", crate::EXTENSION_VERSION);

    // 加载配置（默认配置）
    let config = Arc::new(CangjieConfig::default());

    // 创建 LSP 服务器
    let lsp_server = CangjieLanguageServer::new(config.clone());

    // 创建扩展实例
    let extension = CangjieExtension::new(config, lsp_server);

    Box::new(extension)
}

/// 实现 Zed Extension 接口
impl ZedExtension for CangjieExtension {
    fn name(&self) -> &str {
        "cangjie-lsp"
    }

    fn version(&self) -> &str {
        crate::EXTENSION_VERSION
    }

    fn on_activate(
        &mut self,
        worktree: zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<()> {
        info!(
            "扩展激活，工作目录: {}",
            worktree.path().to_str().unwrap_or("未知路径")
        );
        self.worktree = Some(worktree.clone());

        // 初始化 LSP 服务器
        self.lsp_server.initialize(worktree)?;

        Ok(())
    }

    fn on_document_open(
        &mut self,
        document: &zed_extension_api::Document,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::Diagnostic>> {
        debug!(
            "文档打开: {}",
            document.path().to_str().unwrap_or("未知文件")
        );
        self.lsp_server.did_open(document)
    }

    fn on_document_change(
        &mut self,
        document: &zed_extension_api::Document,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::Diagnostic>> {
        debug!(
            "文档变更: {}",
            document.path().to_str().unwrap_or("未知文件")
        );
        self.lsp_server.did_change(document)
    }

    fn on_document_close(&mut self, document: &zed_extension_api::Document) {
        debug!(
            "文档关闭: {}",
            document.path().to_str().unwrap_or("未知文件")
        );
        self.lsp_server.did_close(document);
    }

    fn format(
        &mut self,
        document: &mut zed_extension_api::Document,
    ) -> zed_extension_api::Result<()> {
        self.format_document(document)
    }

    fn lint(
        &mut self,
        document: &zed_extension_api::Document,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::Diagnostic>> {
        self.run_lint(document)
    }

    fn completion(
        &mut self,
        document: &zed_extension_api::Document,
        position: zed_extension_api::Position,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::CompletionItem>> {
        self.lsp_server.completion(document, position)
    }

    fn goto_definition(
        &mut self,
        document: &zed_extension_api::Document,
        position: zed_extension_api::Position,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::Location>> {
        self.lsp_server.goto_definition(document, position)
    }

    fn document_symbols(
        &mut self,
        document: &zed_extension_api::Document,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::SymbolInformation>> {
        self.lsp_server.document_symbols(document)
    }

    fn run_command(&mut self, command: &str, args: &[String]) -> zed_extension_api::Result<()> {
        info!("执行命令: {} {:?}", command, args);

        match command {
            "cangjie.build_project" => self.build_project(),
            "cangjie.start_debug" => self.start_debug_session(args),
            "cangjie.collect_coverage" => {
                if args.is_empty() {
                    return Err(zed_extension_api::Error::InvalidData(
                        "测试命令不能为空".to_string(),
                    ));
                }
                let test_command = &args[0];
                let test_args = &args[1..];
                self.collect_coverage(test_command, test_args)
            }
            "cangjie.run_profiling" => {
                if args.is_empty() {
                    return Err(zed_extension_api::Error::InvalidData(
                        "目标程序不能为空".to_string(),
                    ));
                }
                let target_binary = &args[0];
                let target_args = &args[1..];
                self.run_profiling(target_binary, target_args)
            }
            "cangjie.generate_optimization_hints" => {
                let document_path = args.get(0).ok_or_else(|| {
                    zed_extension_api::Error::InvalidData("文档路径参数缺失".to_string())
                })?;
                let document =
                    zed_extension_api::Document::open(zed_extension_api::Path::new(document_path))?;
                let suggestions = self.generate_optimization_hints(&document)?;

                // 输出建议到控制台
                for hint in suggestions {
                    info!("\n{}", hint);
                }

                Ok(())
            }
            _ => Err(zed_extension_api::Error::NotFound(format!(
                "未知命令: {}",
                command
            ))),
        }
    }

    fn commands(&self) -> Vec<zed_extension_api::CommandDescription> {
        vec![
            zed_extension_api::CommandDescription {
                name: "cangjie.build_project".to_string(),
                description: "构建仓颉项目".to_string(),
                args: vec![],
            },
            zed_extension_api::CommandDescription {
                name: "cangjie.start_debug".to_string(),
                description: "启动仓颉调试会话".to_string(),
                args: vec![zed_extension_api::CommandArg {
                    name: "args".to_string(),
                    description: "调试目标参数".to_string(),
                    required: false,
                }],
            },
            zed_extension_api::CommandDescription {
                name: "cangjie.collect_coverage".to_string(),
                description: "收集代码覆盖率".to_string(),
                args: vec![
                    zed_extension_api::CommandArg {
                        name: "test_command".to_string(),
                        description: "测试命令".to_string(),
                        required: true,
                    },
                    zed_extension_api::CommandArg {
                        name: "test_args".to_string(),
                        description: "测试参数".to_string(),
                        required: false,
                    },
                ],
            },
            zed_extension_api::CommandDescription {
                name: "cangjie.run_profiling".to_string(),
                description: "执行性能分析".to_string(),
                args: vec![
                    zed_extension_api::CommandArg {
                        name: "target_binary".to_string(),
                        description: "目标程序路径".to_string(),
                        required: true,
                    },
                    zed_extension_api::CommandArg {
                        name: "args".to_string(),
                        description: "程序参数".to_string(),
                        required: false,
                    },
                ],
            },
            zed_extension_api::CommandDescription {
                name: "cangjie.generate_optimization_hints".to_string(),
                description: "生成性能优化建议".to_string(),
                args: vec![zed_extension_api::CommandArg {
                    name: "document_path".to_string(),
                    description: "文档路径".to_string(),
                    required: true,
                }],
            },
        ]
    }
}
