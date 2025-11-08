//! 扩展命令处理（整合所有工具链命令）
use log::{debug, info};
use std::sync::Arc;
use zed_extension_api as zed;

use crate::{
    cjcov::CjcovManager, cjdb::CjdbManager, cjfmt::CjfmtManager, cjlint::CjlintManager,
    cjpm::CjpmManager, cjprof::CjprofManager, config::CangjieConfig,
    language_server::CangjieLanguageServer,
};

/// 仓颉扩展主结构体（整合 LSP 和命令处理）
pub struct CangjieExtension {
    config: Arc<CangjieConfig>,
    lsp_server: CangjieLanguageServer,
    worktree: Option<zed::Worktree>,
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

    /// 格式化文档（对应命令：cangjie.format）
    pub fn format_document(&mut self, document: &mut zed::Document) -> zed::Result<()> {
        info!(
            "执行代码格式化: {}",
            document.path().to_str().unwrap_or("未知文件")
        );
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed::Error::NotFound("未初始化工作目录".to_string()))?;

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

    /// 执行代码检查（对应命令：cangjie.lint）
    pub fn run_lint(&mut self, document: &zed::Document) -> zed::Result<Vec<zed::Diagnostic>> {
        info!(
            "执行代码检查: {}",
            document.path().to_str().unwrap_or("未知文件")
        );
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed::Error::NotFound("未初始化工作目录".to_string()))?;

        // 加载配置
        let cjlint_config = CjlintManager::load_config(worktree, &self.config)?;
        // 执行检查
        let diagnostics = CjlintManager::run_lint(worktree, document, &cjlint_config)?;

        info!("代码检查完成，发现 {} 个问题", diagnostics.len());
        Ok(diagnostics)
    }

    /// 构建项目（对应命令：cangjie.build）
    pub fn build_project(&self) -> zed::Result<()> {
        info!("开始构建项目");
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed::Error::NotFound("未初始化工作目录".to_string()))?;

        // 安装依赖
        info!("安装项目依赖...");
        CjpmManager::install_dependencies(worktree)?;

        // 加载配置
        let cjpm_config = CjpmManager::load_config(worktree)?;
        // 执行构建
        info!("执行项目构建...");
        CjpmManager::build_project(worktree, &cjpm_config)?;

        info!("项目构建成功");
        Ok(())
    }

    /// 启动调试会话（对应命令：cangjie.debug.start）
    pub fn start_debug_session(&self, args: &[String]) -> zed::Result<zed::DebugSession> {
        info!("启动调试会话，参数: {:?}", args);
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed::Error::NotFound("未初始化工作目录".to_string()))?;

        // 自动识别目标产物
        let target_binary = CjpmManager::auto_detect_target(worktree)?;
        info!("调试目标: {}", target_binary);

        // 加载 cjdb 配置
        let cjdb_config = CjdbManager::load_config(worktree)?;
        // 启动调试
        let mut session =
            CjdbManager::start_debug_session(worktree, &cjdb_config, &target_binary, args)?;

        info!("调试会话启动成功，端口: {}", cjdb_config.session.port);
        Ok(session)
    }

    /// 收集覆盖率（对应命令：cangjie.coverage.collect）
    pub fn collect_coverage(&self, test_command: &str, test_args: &[String]) -> zed::Result<()> {
        info!("收集覆盖率，测试命令: {} {:?}", test_command, test_args);
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed::Error::NotFound("未初始化工作目录".to_string()))?;

        // 加载配置
        let cjcov_config = CjcovManager::load_config(worktree, &self.config)?;
        // 收集覆盖率
        let result =
            CjcovManager::collect_coverage(worktree, &cjcov_config, test_command, test_args)?;

        // 输出汇总信息
        info!("覆盖率收集完成:");
        info!(
            "  行覆盖率: {:.2}% ({} / {})",
            result.summary.line_coverage, result.summary.covered_lines, result.summary.total_lines
        );
        info!(
            "  分支覆盖率: {:.2}% ({} / {})",
            result.summary.branch_coverage,
            result.summary.covered_branches,
            result.summary.total_branches
        );
        info!(
            "  函数覆盖率: {:.2}% ({} / {})",
            result.summary.function_coverage,
            result.summary.covered_functions,
            result.summary.total_functions
        );

        // 检查阈值
        if !result.threshold_check.passed {
            info!("覆盖率阈值未达标:");
            for failure in result.threshold_check.failures {
                info!("  - {}", failure.message);
            }
            return Err(zed::Error::ProcessFailed("覆盖率阈值未达标".to_string()));
        }

        Ok(())
    }

    /// 打开覆盖率报告（对应命令：cangjie.coverage.open_report）
    pub fn open_coverage_report(&self) -> zed::Result<()> {
        info!("打开覆盖率报告");
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed::Error::NotFound("未初始化工作目录".to_string()))?;

        // 加载配置
        let cjcov_config = CjcovManager::load_config(worktree, &self.config)?;
        // 打开 HTML 报告
        CjcovManager::open_html_report(worktree, &cjcov_config)?;

        info!("覆盖率报告已打开");
        Ok(())
    }

    /// 启动性能分析（对应命令：cangjie.profiler.start）
    pub fn start_profiling(&self, target_binary: &str, args: &[String]) -> zed::Result<()> {
        info!("启动性能分析，目标: {} {:?}", target_binary, args);
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed::Error::NotFound("未初始化工作目录".to_string()))?;

        // 加载配置
        let cjprof_config = crate::cjprof::CjprofManager::load_config(worktree, &self.config)?;
        // 启动性能分析
        crate::cjprof::CjprofManager::start_profiling(
            worktree,
            &cjprof_config,
            target_binary,
            args,
        )?;

        info!("性能分析完成，报告已保存到: {}", cjprof_config.report.dir);
        Ok(())
    }

    /// 打开火焰图（对应命令：cangjie.profiler.open_flamegraph）
    pub fn open_flamegraph(&self) -> zed::Result<()> {
        info!("打开火焰图");
        let worktree = self
            .worktree
            .as_ref()
            .ok_or_else(|| zed::Error::NotFound("未初始化工作目录".to_string()))?;

        // 加载配置
        let cjprof_config = crate::cjprof::CjprofManager::load_config(worktree, &self.config)?;
        // 打开火焰图 HTML 文件
        let flamegraph_path = worktree
            .path()
            .join(&cjprof_config.report.dir)
            .join("flamegraph.html");

        if !flamegraph_path.exists() {
            return Err(zed::Error::NotFound(
                "未找到火焰图报告，请先执行性能分析".to_string(),
            ));
        }

        zed::shell::open(&flamegraph_path)?;
        info!("火焰图已打开");
        Ok(())
    }
}

// 实现 Zed 的 Extension trait，处理命令和 LSP 回调
impl zed::Extension for CangjieExtension {
    fn initialize(&mut self, worktree: zed::Worktree) -> zed::Result<()> {
        info!(
            "扩展初始化，工作目录: {}",
            worktree.path().to_str().unwrap_or("未知")
        );
        self.worktree = Some(worktree.clone());
        self.lsp_server.initialize(worktree)?;
        Ok(())
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: &[serde_json::Value],
    ) -> zed::Result<serde_json::Value> {
        info!("处理命令: {}，参数: {:?}", command, args);

        match command {
            "cangjie.format" => {
                let mut document = args[0]
                    .as_document()
                    .ok_or_else(|| zed::Error::InvalidData("参数不是 Document 类型".to_string()))?;
                self.format_document(&mut document)?;
                Ok(serde_json::Value::Null)
            }
            "cangjie.lint" => {
                let document = args[0]
                    .as_document()
                    .ok_or_else(|| zed::Error::InvalidData("参数不是 Document 类型".to_string()))?;
                let diagnostics = self.run_lint(&document)?;
                Ok(serde_json::to_value(diagnostics)?)
            }
            "cangjie.build" => {
                self.build_project()?;
                Ok(serde_json::Value::String("构建成功".to_string()))
            }
            "cangjie.debug.start" => {
                let args: Vec<String> = args
                    .iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect();
                let session = self.start_debug_session(&args)?;
                Ok(serde_json::to_value(session.id())?)
            }
            "cangjie.coverage.collect" => {
                let test_command = args[0]
                    .as_str()
                    .ok_or_else(|| zed::Error::InvalidData("缺少测试命令参数".to_string()))?;
                let test_args: Vec<String> = args[1]
                    .as_array()
                    .ok_or_else(|| zed::Error::InvalidData("测试参数格式错误".to_string()))?
                    .iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect();
                self.collect_coverage(test_command, &test_args)?;
                Ok(serde_json::Value::String("覆盖率收集成功".to_string()))
            }
            "cangjie.coverage.open_report" => {
                self.open_coverage_report()?;
                Ok(serde_json::Value::String("覆盖率报告已打开".to_string()))
            }
            "cangjie.profiler.start" => {
                let target_binary = args[0]
                    .as_str()
                    .ok_or_else(|| zed::Error::InvalidData("缺少目标二进制文件参数".to_string()))?;
                let args: Vec<String> = args[1]
                    .as_array()
                    .ok_or_else(|| zed::Error::InvalidData("参数格式错误".to_string()))?
                    .iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect();
                self.start_profiling(target_binary, &args)?;
                Ok(serde_json::Value::String("性能分析完成".to_string()))
            }
            "cangjie.profiler.open_flamegraph" => {
                self.open_flamegraph()?;
                Ok(serde_json::Value::String("火焰图已打开".to_string()))
            }
            _ => Err(zed::Error::NotFound(format!("未知命令: {}", command))),
        }
    }
}

// 实现 Zed 的 LanguageServer trait，转发 LSP 回调到内部 LSP 服务器
impl zed::LanguageServer for CangjieExtension {
    fn did_open(&mut self, document: &zed::Document) -> zed::Result<Vec<zed::Diagnostic>> {
        self.lsp_server.did_open(document)
    }

    fn did_change(&mut self, document: &zed::Document) -> zed::Result<Vec<zed::Diagnostic>> {
        self.lsp_server.did_change(document)
    }

    fn did_close(&mut self, document: &zed::Document) {
        self.lsp_server.did_close(document)
    }

    fn completion(
        &mut self,
        document: &zed::Document,
        position: zed::Position,
    ) -> zed::Result<Vec<zed::CompletionItem>> {
        self.lsp_server.completion(document, position)
    }

    fn document_symbols(&self, document: &zed::Document) -> zed::Result<Vec<zed::Symbol>> {
        self.lsp_server.document_symbols(document)
    }

    fn goto_definition(
        &self,
        document: &zed::Document,
        position: zed::Position,
    ) -> zed::Result<Vec<zed::Location>> {
        self.lsp_server.goto_definition(document, position)
    }

    // 可选实现：hover 提示、代码重构等
    fn hover(
        &self,
        document: &zed::Document,
        position: zed::Position,
    ) -> zed::Result<Option<zed::Hover>> {
        let token = self.lsp_server.get_token_at_position(document, position)?;
        Ok(Some(zed::Hover {
            contents: zed::HoverContents::Markup(zed::MarkupContent {
                kind: zed::MarkupKind::Markdown,
                value: format!("# `{}`\n\n暂无详细文档（可通过扩展配置添加）", token),
            }),
            range: Some(zed::Range {
                start: position,
                end: zed::Position {
                    line: position.line,
                    column: position.column + token.len() as u32,
                },
            }),
        }))
    }
}
