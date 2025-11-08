//! 仓颉 LSP 实现（整合语法分析、代码补全、诊断、跳转等核心功能）
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use zed_extension_api as zed;

/// 仓颉 LSP 服务器
#[derive(Debug, Default)]
pub struct CangjieLanguageServer {
    config: Arc<super::config::CangjieConfig>,
    worktree: Option<zed::Worktree>,
    // 缓存：文件路径 -> 语法分析结果
    syntax_cache: HashMap<String, SyntaxAnalysisResult>,
    // 缓存：代码补全候选
    completion_cache: HashMap<String, Vec<zed::CompletionItem>>,
}

/// 语法分析结果
#[derive(Debug, Clone)]
struct SyntaxAnalysisResult {
    ast: serde_json::Value,            // 抽象语法树（简化版）
    symbols: Vec<zed::Symbol>,         // 符号表
    diagnostics: Vec<zed::Diagnostic>, // 语法诊断
    last_updated: std::time::Instant,  // 最后更新时间
}

impl CangjieLanguageServer {
    /// 创建新的 LSP 服务器
    pub fn new(config: Arc<super::config::CangjieConfig>) -> Self {
        Self {
            config,
            worktree: None,
            syntax_cache: HashMap::new(),
            completion_cache: HashMap::new(),
        }
    }

    /// 初始化 LSP 服务器
    pub fn initialize(&mut self, worktree: zed::Worktree) -> zed::Result<()> {
        self.worktree = Some(worktree);
        Ok(())
    }

    /// 处理文档打开事件
    pub fn did_open(&mut self, document: &zed::Document) -> zed::Result<Vec<zed::Diagnostic>> {
        let file_path = document.path().to_str().unwrap().to_string();
        let (ast, symbols, diagnostics) = self.analyze_document(document)?;

        // 更新缓存
        self.syntax_cache.insert(
            file_path.clone(),
            SyntaxAnalysisResult {
                ast,
                symbols,
                diagnostics: diagnostics.clone(),
                last_updated: std::time::Instant::now(),
            },
        );

        // 执行 cjlint 检查并合并诊断
        let worktree = self.worktree.as_ref().unwrap();
        let cjlint_config = super::cjlint::CjlintManager::load_config(worktree, &self.config)?;
        let lint_diagnostics =
            super::cjlint::CjlintManager::run_lint(worktree, document, &cjlint_config)?;

        let mut all_diagnostics = diagnostics;
        all_diagnostics.extend(lint_diagnostics);

        Ok(all_diagnostics)
    }

    /// 处理文档变更事件
    pub fn did_change(&mut self, document: &zed::Document) -> zed::Result<Vec<zed::Diagnostic>> {
        self.did_open(document) // 复用打开逻辑，重新分析
    }

    /// 处理文档关闭事件
    pub fn did_close(&mut self, document: &zed::Document) {
        let file_path = document.path().to_str().unwrap().to_string();
        self.syntax_cache.remove(&file_path);
        self.completion_cache.remove(&file_path);
    }

    /// 处理代码补全请求
    pub fn completion(
        &mut self,
        document: &zed::Document,
        position: zed::Position,
    ) -> zed::Result<Vec<zed::CompletionItem>> {
        let file_path = document.path().to_str().unwrap().to_string();

        // 检查缓存
        if let Some(cache) = self.completion_cache.get(&file_path) {
            return Ok(cache.clone());
        }

        // 语法分析获取符号
        let syntax_result = self
            .syntax_cache
            .entry(file_path.clone())
            .or_insert_with(|| {
                let (ast, symbols, diagnostics) = self.analyze_document(document).unwrap();
                SyntaxAnalysisResult {
                    ast,
                    symbols,
                    diagnostics,
                    last_updated: std::time::Instant::now(),
                }
            });

        // 生成补全候选
        let mut completions = Vec::new();

        // 1. 文档内符号补全
        for symbol in &syntax_result.symbols {
            completions.push(zed::CompletionItem {
                label: symbol.name.clone(),
                kind: match symbol.kind {
                    zed::SymbolKind::Function => zed::CompletionItemKind::Function,
                    zed::SymbolKind::Variable => zed::CompletionItemKind::Variable,
                    zed::SymbolKind::Struct => zed::CompletionItemKind::Struct,
                    zed::SymbolKind::Enum => zed::CompletionItemKind::Enum,
                    zed::SymbolKind::Module => zed::CompletionItemKind::Module,
                    _ => zed::CompletionItemKind::Text,
                },
                detail: Some(symbol.detail.clone()),
                documentation: Some(symbol.documentation.clone()),
                insert_text: Some(symbol.name.clone()),
                insert_text_format: zed::InsertTextFormat::PlainText,
                sort_text: Some(symbol.name.clone()),
            });
        }

        // 2. 标准库符号补全（简化版）
        completions.extend(self.get_stdlib_completions());

        // 3. 代码片段补全
        completions.extend(
            super::syntax::get_snippets()
                .get("Cangjie")
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .map(|snippet| zed::CompletionItem {
                    label: snippet.name,
                    kind: zed::CompletionItemKind::Snippet,
                    detail: Some(snippet.description),
                    documentation: None,
                    insert_text: Some(snippet.body),
                    insert_text_format: zed::InsertTextFormat::Snippet,
                    sort_text: Some(format!("z{}", snippet.name)), // 片段排最后
                }),
        );

        // 更新缓存
        self.completion_cache.insert(file_path, completions.clone());

        Ok(completions)
    }

    /// 处理符号查找请求
    pub fn document_symbols(&self, document: &zed::Document) -> zed::Result<Vec<zed::Symbol>> {
        let file_path = document.path().to_str().unwrap().to_string();
        self.syntax_cache
            .get(&file_path)
            .map(|result| result.symbols.clone())
            .ok_or_else(|| zed::Error::NotFound("未找到文档符号，请先打开文档".to_string()))
    }

    /// 处理跳转定义请求
    pub fn goto_definition(
        &self,
        document: &zed::Document,
        position: zed::Position,
    ) -> zed::Result<Vec<zed::Location>> {
        let file_path = document.path().to_str().unwrap().to_string();
        let syntax_result = self
            .syntax_cache
            .get(&file_path)
            .ok_or_else(|| zed::Error::NotFound("未找到文档语法分析结果".to_string()))?;

        // 简化实现：查找当前位置的符号并返回其定义位置
        let token = self.get_token_at_position(document, position)?;
        for symbol in &syntax_result.symbols {
            if symbol.name == token && symbol.range.start.line == position.line {
                return Ok(vec![zed::Location {
                    path: document.path().clone(),
                    range: symbol.range.clone(),
                }]);
            }
        }

        Err(zed::Error::NotFound(format!("未找到 {} 的定义", token)))
    }

    /// 语法分析文档（简化版，实际应基于完整 parser）
    fn analyze_document(
        &self,
        document: &zed::Document,
    ) -> zed::Result<(serde_json::Value, Vec<zed::Symbol>, Vec<zed::Diagnostic>)> {
        let text = document.text();
        let mut symbols = Vec::new();
        let mut diagnostics = Vec::new();

        // 1. 简单符号提取（函数、变量、结构体）
        let function_regex = regex::Regex::new(r"\bfn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
        let struct_regex = regex::Regex::new(r"\bstruct\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{").unwrap();
        let var_regex =
            regex::Regex::new(r"\b(let|const|mut)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[:=]").unwrap();

        // 提取函数
        for cap in function_regex.captures_iter(text) {
            let name = cap[1].to_string();
            let line_num = text[0..cap.get(0).unwrap().start()].matches('\n').count() as u32;
            symbols.push(zed::Symbol {
                name: name.clone(),
                kind: zed::SymbolKind::Function,
                range: zed::Range {
                    start: zed::Position {
                        line: line_num,
                        column: 0,
                    },
                    end: zed::Position {
                        line: line_num,
                        column: name.len() as u32,
                    },
                },
                detail: "函数".to_string(),
                documentation: format!("函数 {}", name),
            });
        }

        // 提取结构体
        for cap in struct_regex.captures_iter(text) {
            let name = cap[1].to_string();
            let line_num = text[0..cap.get(0).unwrap().start()].matches('\n').count() as u32;
            symbols.push(zed::Symbol {
                name: name.clone(),
                kind: zed::SymbolKind::Struct,
                range: zed::Range {
                    start: zed::Position {
                        line: line_num,
                        column: 0,
                    },
                    end: zed::Position {
                        line: line_num,
                        column: name.len() as u32,
                    },
                },
                detail: "结构体".to_string(),
                documentation: format!("结构体 {}", name),
            });
        }

        // 提取变量
        for cap in var_regex.captures_iter(text) {
            let name = cap[2].to_string();
            let line_num = text[0..cap.get(0).unwrap().start()].matches('\n').count() as u32;
            symbols.push(zed::Symbol {
                name: name.clone(),
                kind: zed::SymbolKind::Variable,
                range: zed::Range {
                    start: zed::Position {
                        line: line_num,
                        column: 0,
                    },
                    end: zed::Position {
                        line: line_num,
                        column: name.len() as u32,
                    },
                },
                detail: "变量".to_string(),
                documentation: format!("变量 {}", name),
            });
        }

        // 2. 简单语法错误检查
        if text.contains("fn fn") {
            diagnostics.push(zed::Diagnostic {
                range: zed::Range {
                    start: zed::Position { line: 0, column: 0 },
                    end: zed::Position { line: 0, column: 5 },
                },
                severity: zed::DiagnosticSeverity::Error,
                code: Some(zed::DiagnosticCode {
                    value: "SYNTAX-001".to_string(),
                    description: Some("重复的 fn 关键字".to_string()),
                }),
                message: "发现重复的 fn 关键字，可能是语法错误".to_string(),
                source: Some("cangjie-lsp".to_string()),
                fixes: None,
            });
        }

        // 3. 生成简化 AST
        let ast = serde_json::json!({
            "type": "Program",
            "children": symbols.iter().map(|s| {
                serde_json::json!({
                    "type": match s.kind {
                        zed::SymbolKind::Function => "FunctionDeclaration",
                        zed::SymbolKind::Struct => "StructDeclaration",
                        zed::SymbolKind::Variable => "VariableDeclaration",
                        _ => "Unknown",
                    },
                    "name": s.name,
                    "line": s.range.start.line,
                })
            }).collect::<Vec<_>>()
        });

        Ok((ast, symbols, diagnostics))
    }

    /// 获取指定位置的 token
    fn get_token_at_position(
        &self,
        document: &zed::Document,
        position: zed::Position,
    ) -> zed::Result<String> {
        let text = document.text();
        let line_text = text
            .lines()
            .nth(position.line as usize)
            .ok_or_else(|| zed::Error::NotFound("行不存在".to_string()))?;

        // 简单实现：获取光标前的标识符
        let column = position.column as usize;
        let prefix = &line_text[0..column];
        let token = prefix
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .last()
            .unwrap_or("")
            .to_string();

        Ok(token)
    }

    /// 获取标准库补全候选（简化版）
    fn get_stdlib_completions(&self) -> Vec<zed::CompletionItem> {
        vec![
            // 基础类型
            zed::CompletionItem {
                label: "Int",
                kind: zed::CompletionItemKind::Type,
                detail: Some("整数类型".to_string()),
                documentation: Some("32/64 位整数类型，根据平台自动适配".to_string()),
                insert_text: Some("Int".to_string()),
                insert_text_format: zed::InsertTextFormat::PlainText,
                sort_text: Some("aInt".to_string()),
            },
            zed::CompletionItem {
                label: "String",
                kind: zed::CompletionItemKind::Type,
                detail: Some("字符串类型".to_string()),
                documentation: Some("UTF-8 编码的字符串类型".to_string()),
                insert_text: Some("String".to_string()),
                insert_text_format: zed::InsertTextFormat::PlainText,
                sort_text: Some("aString".to_string()),
            },
            // 标准库函数
            zed::CompletionItem {
                label: "println",
                kind: zed::CompletionItemKind::Function,
                detail: Some("fn println(value: Any)".to_string()),
                documentation: Some("打印值并换行".to_string()),
                insert_text: Some("println(${1:value})".to_string()),
                insert_text_format: zed::InsertTextFormat::Snippet,
                sort_text: Some("bprintln".to_string()),
            },
            zed::CompletionItem {
                label: "read_file",
                kind: zed::CompletionItemKind::Function,
                detail: Some("fn read_file(path: String) -> Result<String, Error>".to_string()),
                documentation: Some("读取文件内容".to_string()),
                insert_text: Some("read_file(${1:path})".to_string()),
                insert_text_format: zed::InsertTextFormat::Snippet,
                sort_text: Some("bread_file".to_string()),
            },
        ]
    }
}

// LSP 服务器实现 Zed 的 LanguageServer trait
impl zed::LanguageServer for CangjieLanguageServer {
    fn initialize(&mut self, worktree: zed::Worktree) -> zed::Result<()> {
        CangjieLanguageServer::initialize(self, worktree)
    }

    fn did_open(&mut self, document: &zed::Document) -> zed::Result<Vec<zed::Diagnostic>> {
        CangjieLanguageServer::did_open(self, document)
    }

    fn did_change(&mut self, document: &zed::Document) -> zed::Result<Vec<zed::Diagnostic>> {
        CangjieLanguageServer::did_change(self, document)
    }

    fn did_close(&mut self, document: &zed::Document) {
        CangjieLanguageServer::did_close(self, document)
    }

    fn completion(
        &mut self,
        document: &zed::Document,
        position: zed::Position,
    ) -> zed::Result<Vec<zed::CompletionItem>> {
        CangjieLanguageServer::completion(self, document, position)
    }

    fn document_symbols(&self, document: &zed::Document) -> zed::Result<Vec<zed::Symbol>> {
        CangjieLanguageServer::document_symbols(self, document)
    }

    fn goto_definition(
        &self,
        document: &zed::Document,
        position: zed::Position,
    ) -> zed::Result<Vec<zed::Location>> {
        CangjieLanguageServer::goto_definition(self, document, position)
    }
}
