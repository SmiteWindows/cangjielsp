//! 仓颉 LSP 核心实现（基于 tree-sitter-cangjie 解析器）
use crate::{
    config::CangjieConfig,
    tree_sitter_utils::{self, find_identifier_at_point, SymbolInfo, SymbolType},
};
use std::collections::HashMap;
use std::sync::Arc;
use tree_sitter::Tree;

/// 仓颉 LSP 服务器
pub struct CangjieLanguageServer {
    config: Arc<CangjieConfig>,
    /// 缓存的文档数据（路径 -> (语法树, 符号列表)）
    document_cache: HashMap<String, (Tree, Vec<SymbolInfo>)>,
}

impl CangjieLanguageServer {
    /// 创建新的 LSP 服务器
    pub fn new(config: Arc<CangjieConfig>) -> Self {
        // 初始化 tree-sitter 解析器
        tree_sitter_utils::init_parser();

        Self {
            config,
            document_cache: HashMap::new(),
        }
    }

    /// 初始化 LSP 服务器
    pub fn initialize(
        &mut self,
        worktree: zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<()> {
        // 加载工作目录下的文档符号（基于 tree-sitter 解析）
        let _ = self.scan_workspace_symbols(&worktree);
        Ok(())
    }

    /// 扫描工作区符号
    fn scan_workspace_symbols(
        &mut self,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<()> {
        let src_dir = worktree.path().join("src");
        if !src_dir.exists() {
            return Ok(());
        }

        // 递归扫描 .cj 文件
        let cj_files = glob::glob(&src_dir.join("**/*.cj").to_str().unwrap())
            .map_err(|e| zed_extension_api::Error::IoError(format!("扫描文件失败: {}", e)))?;

        for entry in cj_files {
            let path = entry.map_err(|e| {
                zed_extension_api::Error::IoError(format!("获取文件路径失败: {}", e))
            })?;
            let path_str = path
                .to_str()
                .ok_or_else(|| zed_extension_api::Error::InvalidData("文件路径无效".to_string()))?;

            let content = std::fs::read_to_string(&path).map_err(|e| {
                zed_extension_api::Error::IoError(format!("读取文件 {} 失败: {}", path_str, e))
            })?;

            // 使用 tree-sitter 解析并提取符号
            let tree = tree_sitter_utils::parse_document(&content);
            let symbols = tree_sitter_utils::extract_symbols(&content, &tree);
            self.document_cache
                .insert(path_str.to_string(), (tree, symbols));
        }

        Ok(())
    }

    /// 文档打开时解析并缓存符号
    pub fn did_open(
        &mut self,
        document: &zed_extension_api::Document,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::Diagnostic>> {
        let path_str = document
            .path()
            .to_str()
            .ok_or_else(|| zed_extension_api::Error::InvalidData("文档路径无效".to_string()))?;
        let content = document.text();

        // 解析文档生成语法树
        let tree = tree_sitter_utils::parse_document(content);
        // 提取符号
        let symbols = tree_sitter_utils::extract_symbols(content, &tree);
        // 检查语法错误
        let diagnostics = tree_sitter_utils::check_syntax_errors(&tree, content);

        // 缓存文档数据
        self.document_cache
            .insert(path_str.to_string(), (tree, symbols));

        Ok(diagnostics)
    }

    /// 文档变更时更新缓存
    pub fn did_change(
        &mut self,
        document: &zed_extension_api::Document,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::Diagnostic>> {
        self.did_open(document) // 复用 did_open 逻辑，重新解析
    }

    /// 文档关闭时移除缓存
    pub fn did_close(&mut self, document: &zed_extension_api::Document) {
        let path_str = document.path().to_str().unwrap_or("");
        self.document_cache.remove(path_str);
    }

    /// 获取代码补全
    pub fn completion(
        &self,
        document: &zed_extension_api::Document,
        position: zed_extension_api::Position,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::CompletionItem>> {
        let mut items = Vec::new();

        // 1. 添加当前文档符号补全
        let path_str = document
            .path()
            .to_str()
            .ok_or_else(|| zed_extension_api::Error::InvalidData("文档路径无效".to_string()))?;
        if let Some((_, symbols)) = self.document_cache.get(path_str) {
            for symbol in symbols {
                let (kind, insert_text) = match symbol.r#type {
                    SymbolType::Function => (
                        zed_extension_api::CompletionItemKind::Function,
                        format!("{}()", symbol.name), // 补全时自动添加括号
                    ),
                    SymbolType::Variable => (
                        zed_extension_api::CompletionItemKind::Variable,
                        symbol.name.clone(),
                    ),
                    SymbolType::Struct => (
                        zed_extension_api::CompletionItemKind::Struct,
                        symbol.name.clone(),
                    ),
                    SymbolType::Enum => (
                        zed_extension_api::CompletionItemKind::Enum,
                        symbol.name.clone(),
                    ),
                    SymbolType::Import => (
                        zed_extension_api::CompletionItemKind::Module,
                        symbol.name.clone(),
                    ),
                    SymbolType::Method => (
                        zed_extension_api::CompletionItemKind::Method,
                        format!("{}()", symbol.name),
                    ),
                };

                items.push(zed_extension_api::CompletionItem {
                    label: symbol.name.clone(),
                    kind: Some(kind),
                    detail: symbol.detail.clone(),
                    documentation: None,
                    sort_text: None,
                    filter_text: None,
                    insert_text: Some(insert_text),
                    insert_text_format: Some(zed_extension_api::InsertTextFormat::PlainText),
                    text_edit: None,
                    additional_text_edits: None,
                    commit_characters: None,
                    command: None,
                    deprecated: Some(false),
                    preselect: None,
                    tags: None,
                    data: None,
                });
            }
        }

        // 2. 添加标准库补全（硬编码示例，可改为从语法树动态提取）
        let std_lib_items = vec![
            (
                "println",
                "fn println(message: String) -> Void",
                zed_extension_api::CompletionItemKind::Function,
            ),
            (
                "read_file",
                "fn read_file(path: String) -> Result<String, Error>",
                zed_extension_api::CompletionItemKind::Function,
            ),
            (
                "Vec",
                "struct Vec<T>",
                zed_extension_api::CompletionItemKind::Struct,
            ),
            (
                "Option",
                "enum Option<T>",
                zed_extension_api::CompletionItemKind::Enum,
            ),
        ];
        for (name, detail, kind) in std_lib_items {
            items.push(zed_extension_api::CompletionItem {
                label: name.to_string(),
                kind: Some(kind),
                detail: Some(detail.to_string()),
                documentation: None,
                sort_text: None,
                filter_text: None,
                insert_text: Some(if kind == zed_extension_api::CompletionItemKind::Function {
                    format!("{}()", name)
                } else {
                    name.to_string()
                }),
                insert_text_format: Some(zed_extension_api::InsertTextFormat::PlainText),
                text_edit: None,
                additional_text_edits: None,
                commit_characters: None,
                command: None,
                deprecated: Some(false),
                preselect: None,
                tags: None,
                data: None,
            });
        }

        // 3. 添加代码片段补全
        let snippets = crate::syntax::get_snippets();
        if let Some(cangjie_snippets) = snippets.get("Cangjie") {
            for snippet in cangjie_snippets {
                items.push(zed_extension_api::CompletionItem {
                    label: snippet.name.clone(),
                    kind: Some(zed_extension_api::CompletionItemKind::Snippet),
                    detail: Some(snippet.description.clone()),
                    documentation: None,
                    sort_text: None,
                    filter_text: None,
                    insert_text: Some(snippet.body.clone()),
                    insert_text_format: Some(zed_extension_api::InsertTextFormat::Snippet),
                    text_edit: None,
                    additional_text_edits: None,
                    commit_characters: None,
                    command: None,
                    deprecated: Some(false),
                    preselect: None,
                    tags: None,
                    data: None,
                });
            }
        }

        Ok(items)
    }

    /// 获取文档符号
    pub fn document_symbols(
        &self,
        document: &zed_extension_api::Document,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::SymbolInformation>> {
        let path_str = document
            .path()
            .to_str()
            .ok_or_else(|| zed_extension_api::Error::InvalidData("文档路径无效".to_string()))?;

        let symbols = self
            .document_cache
            .get(path_str)
            .map(|(_, symbols)| symbols.clone())
            .unwrap_or_default();

        // 转换为 Zed 所需的 SymbolInformation 格式
        let zed_symbols = symbols
            .into_iter()
            .map(|symbol| {
                let kind = match symbol.r#type {
                    SymbolType::Function => zed_extension_api::SymbolKind::Function,
                    SymbolType::Variable => zed_extension_api::SymbolKind::Variable,
                    SymbolType::Struct => zed_extension_api::SymbolKind::Struct,
                    SymbolType::Enum => zed_extension_api::SymbolKind::Enum,
                    SymbolType::Import => zed_extension_api::SymbolKind::Module,
                    SymbolType::Method => zed_extension_api::SymbolKind::Method,
                };

                zed_extension_api::SymbolInformation {
                    name: symbol.name,
                    kind,
                    range: symbol.range,
                    selection_range: symbol.range,
                    detail: symbol.detail,
                    deprecated: false,
                    tags: None,
                    container_name: None,
                    location: zed_extension_api::Location {
                        uri: zed_extension_api::Uri::from_file_path(document.path()).unwrap(),
                        range: symbol.range,
                    },
                }
            })
            .collect();

        Ok(zed_symbols)
    }

    /// 跳转定义
    pub fn goto_definition(
        &self,
        document: &zed_extension_api::Document,
        position: zed_extension_api::Position,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::Location>> {
        let path_str = document
            .path()
            .to_str()
            .ok_or_else(|| zed_extension_api::Error::InvalidData("文档路径无效".to_string()))?;
        let content = document.text();

        // 1. 查找当前文档内的定义
        if let Some((tree, _)) = self.document_cache.get(path_str) {
            if let Some(target_symbol) =
                tree_sitter_utils::find_symbol_at_position(tree, content, position)
            {
                return Ok(vec![zed_extension_api::Location {
                    uri: zed_extension_api::Uri::from_file_path(document.path()).unwrap(),
                    range: target_symbol.range,
                }]);
            }
        }

        // 2. 查找工作区其他文档的定义（基于符号名匹配）
        let target_symbol_name = self.get_symbol_name_at_position(document, position)?;
        if target_symbol_name.is_empty() {
            return Ok(Vec::new());
        }

        let mut locations = Vec::new();
        for (file_path, (_, symbols)) in &self.document_cache {
            // 跳过当前文档（已在第一步检查）
            if file_path == path_str {
                continue;
            }

            // 匹配符号名
            for symbol in symbols {
                if symbol.name == target_symbol_name {
                    let uri = zed_extension_api::Uri::from_file_path(zed_extension_api::Path::new(
                        file_path,
                    ))
                    .unwrap();
                    locations.push(zed_extension_api::Location {
                        uri,
                        range: symbol.range,
                    });
                }
            }
        }

        Ok(locations)
    }

    /// 获取指定位置的符号名
    fn get_symbol_name_at_position(
        &self,
        document: &zed_extension_api::Document,
        position: zed_extension_api::Position,
    ) -> zed_extension_api::Result<String> {
        let path_str = document
            .path()
            .to_str()
            .ok_or_else(|| zed_extension_api::Error::InvalidData("文档路径无效".to_string()))?;
        let content = document.text();

        if let Some((tree, _)) = self.document_cache.get(path_str) {
            if let Some(symbol) =
                tree_sitter_utils::find_symbol_at_position(tree, content, position)
            {
                return Ok(symbol.name);
            }
        }

        // fallback: 提取标识符文本
        let tree = tree_sitter_utils::parse_document(content);
        let point = tree_sitter::Point {
            row: position.line as usize,
            column: position.column as usize,
        };
        let mut cursor = tree_sitter::TreeCursor::new(&tree.root_node());

        Ok(find_identifier_at_point(&mut cursor, content, point).unwrap_or_default())
    }
}
