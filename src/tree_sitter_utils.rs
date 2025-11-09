//! Tree-sitter 语法解析工具，基于 tree-sitter-cangjie 解析器
use std::collections::HashMap;
use std::sync::OnceLock;
use tree_sitter::{Node, Parser, Point as TsPoint, Query, QueryCursor, Tree};

// 初始化 Tree-sitter 解析器（全局单例）
static PARSER: OnceLock<Parser> = OnceLock::new();

/// 初始化 Tree-sitter 解析器
pub fn init_parser() -> &'static Parser {
    PARSER.get_or_init(|| {
        let mut parser = Parser::new();
        // 设置 Cangjie 语法解析器
        parser
            .set_language(tree_sitter_cangjie::language())
            .expect("Failed to load tree-sitter-cangjie language");
        parser
    })
}

/// 解析文档内容生成语法树
pub fn parse_document(content: &str) -> Tree {
    let parser = init_parser();
    parser
        .parse(content, None)
        .expect("Failed to parse Cangjie document")
}

/// 定义符号查询（基于 Cangjie 语法树节点类型）
const SYMBOL_QUERY: &str = r#"
    ; 函数定义
    (function_definition
        name: (identifier) @function.name
        parameters: (parameter_list)? @function.params
        body: (block)? @function.body
    ) @function

    ; 变量定义
    (variable_declaration
        name: (identifier) @variable.name
        type: (type_annotation)? @variable.type
        value: (expression)? @variable.value
    ) @variable

    ; 结构体定义
    (struct_definition
        name: (identifier) @struct.name
        fields: (field_declaration_list)? @struct.fields
    ) @struct

    ; 枚举定义
    (enum_definition
        name: (identifier) @enum.name
        variants: (enum_variant_list)? @enum.variants
    ) @enum

    ; 模块导入
    (import_declaration
        path: (string_literal) @import.path
    ) @import

    ; 方法定义（结构体/枚举的关联函数）
    (method_definition
        name: (identifier) @method.name
        parameters: (parameter_list)? @method.params
        body: (block)? @method.body
    ) @method
"#;

/// 符号类型枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolType {
    Function,
    Variable,
    Struct,
    Enum,
    Import,
    Method,
}

/// 符号信息结构体（包含语法树节点详情）
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub r#type: SymbolType,
    pub range: zed_extension_api::Range,
    pub detail: Option<String>,
    pub node: Node,
}

/// 从语法树提取符号信息
pub fn extract_symbols(content: &str, tree: &Tree) -> Vec<SymbolInfo> {
    let mut symbols = Vec::new();
    let query =
        Query::new(tree_sitter_cangjie::language(), SYMBOL_QUERY).expect("Invalid symbol query");
    let mut cursor = QueryCursor::new();

    // 执行查询并处理结果
    for match_result in cursor.matches(&query, tree.root_node(), content.as_bytes()) {
        let mut captures = HashMap::new();
        for capture in match_result.captures {
            captures.insert(
                query
                    .capture_name_for_id(capture.index)
                    .unwrap()
                    .to_string(),
                capture.node,
            );
        }

        // 根据根节点类型判断符号类型
        let root_node = match_result.captures[0].node;
        match root_node.kind() {
            "function_definition" => handle_function_symbol(&mut symbols, content, &captures),
            "variable_declaration" => handle_variable_symbol(&mut symbols, content, &captures),
            "struct_definition" => handle_struct_symbol(&mut symbols, content, &captures),
            "enum_definition" => handle_enum_symbol(&mut symbols, content, &captures),
            "import_declaration" => handle_import_symbol(&mut symbols, content, &captures),
            "method_definition" => handle_method_symbol(&mut symbols, content, &captures),
            _ => continue,
        }
    }

    symbols
}

/// 处理函数符号
fn handle_function_symbol(
    symbols: &mut Vec<SymbolInfo>,
    content: &str,
    captures: &HashMap<String, Node>,
) {
    let name_node = captures.get("function.name").unwrap();
    let name = get_node_text(content, name_node);

    // 构建函数详情（参数列表摘要）
    let detail = captures
        .get("function.params")
        .map(|params_node| format!("fn {}(...)", name))
        .unwrap_or_else(|| format!("fn {}", name));

    // 转换为 Zed 范围（0 基索引）
    let range = node_to_zed_range(name_node);

    symbols.push(SymbolInfo {
        name,
        r#type: SymbolType::Function,
        range,
        detail: Some(detail),
        node: captures.get("function").unwrap().clone(),
    });
}

/// 处理变量符号
fn handle_variable_symbol(
    symbols: &mut Vec<SymbolInfo>,
    content: &str,
    captures: &HashMap<String, Node>,
) {
    let name_node = captures.get("variable.name").unwrap();
    let name = get_node_text(content, name_node);

    // 构建变量详情（类型+值摘要）
    let detail = captures
        .get("variable.type")
        .map(|type_node| format!("let {}: {}", name, get_node_text(content, type_node)))
        .unwrap_or_else(|| format!("let {}", name));

    let range = node_to_zed_range(name_node);

    symbols.push(SymbolInfo {
        name,
        r#type: SymbolType::Variable,
        range,
        detail: Some(detail),
        node: captures.get("variable").unwrap().clone(),
    });
}

/// 处理结构体符号
fn handle_struct_symbol(
    symbols: &mut Vec<SymbolInfo>,
    content: &str,
    captures: &HashMap<String, Node>,
) {
    let name_node = captures.get("struct.name").unwrap();
    let name = get_node_text(content, name_node);

    // 构建结构体详情（字段数量）
    let detail = captures
        .get("struct.fields")
        .map(|fields_node| format!("struct {} ({} fields)", name, fields_node.child_count()))
        .unwrap_or_else(|| format!("struct {}", name));

    let range = node_to_zed_range(name_node);

    symbols.push(SymbolInfo {
        name,
        r#type: SymbolType::Struct,
        range,
        detail: Some(detail),
        node: captures.get("struct").unwrap().clone(),
    });
}

/// 处理枚举符号
fn handle_enum_symbol(
    symbols: &mut Vec<SymbolInfo>,
    content: &str,
    captures: &HashMap<String, Node>,
) {
    let name_node = captures.get("enum.name").unwrap();
    let name = get_node_text(content, name_node);

    // 构建枚举详情（变体数量）
    let detail = captures
        .get("enum.variants")
        .map(|variants_node| format!("enum {} ({} variants)", name, variants_node.child_count()))
        .unwrap_or_else(|| format!("enum {}", name));

    let range = node_to_zed_range(name_node);

    symbols.push(SymbolInfo {
        name,
        r#type: SymbolType::Enum,
        range,
        detail: Some(detail),
        node: captures.get("enum").unwrap().clone(),
    });
}

/// 处理导入符号
fn handle_import_symbol(
    symbols: &mut Vec<SymbolInfo>,
    content: &str,
    captures: &HashMap<String, Node>,
) {
    let path_node = captures.get("import.path").unwrap();
    let path = get_node_text(content, path_node)
        .trim_matches('"')
        .to_string();

    let range = node_to_zed_range(captures.get("import").unwrap());

    symbols.push(SymbolInfo {
        name: path.clone(),
        r#type: SymbolType::Import,
        range,
        detail: Some(format!("import {}", path)),
        node: captures.get("import").unwrap().clone(),
    });
}

/// 处理方法符号
fn handle_method_symbol(
    symbols: &mut Vec<SymbolInfo>,
    content: &str,
    captures: &HashMap<String, Node>,
) {
    let name_node = captures.get("method.name").unwrap();
    let name = get_node_text(content, name_node);

    let detail = captures
        .get("method.params")
        .map(|params_node| format!("method {}(...)", name))
        .unwrap_or_else(|| format!("method {}", name));

    let range = node_to_zed_range(name_node);

    symbols.push(SymbolInfo {
        name,
        r#type: SymbolType::Method,
        range,
        detail: Some(detail),
        node: captures.get("method").unwrap().clone(),
    });
}

/// 获取节点文本内容
pub fn get_node_text(content: &str, node: &Node) -> String {
    let start = node.start_byte();
    let end = node.end_byte();
    content[start..end].to_string()
}

/// 将 tree-sitter 节点范围转换为 Zed 范围（0 基索引）
pub fn node_to_zed_range(node: &Node) -> zed_extension_api::Range {
    zed_extension_api::Range {
        start: zed_extension_api::Position {
            line: node.start_point().row as u32,
            column: node.start_point().column as u32,
        },
        end: zed_extension_api::Position {
            line: node.end_point().row as u32,
            column: node.end_point().column as u32,
        },
    }
}

/// 语法错误检查（基于语法树的错误节点）
pub fn check_syntax_errors(tree: &Tree, content: &str) -> Vec<zed_extension_api::Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut cursor = tree_sitter::TreeCursor::new(&tree.root_node());

    // 递归遍历语法树查找错误节点
    find_error_nodes(&mut cursor, content, &mut diagnostics);
    diagnostics
}

/// 递归查找语法错误节点
fn find_error_nodes(
    cursor: &mut tree_sitter::TreeCursor,
    content: &str,
    diagnostics: &mut Vec<zed_extension_api::Diagnostic>,
) {
    let node = cursor.node();

    // 检查当前节点是否为错误节点
    if node.is_error() {
        let range = node_to_zed_range(&node);
        let error_text = get_node_text(content, &node);

        diagnostics.push(zed_extension_api::Diagnostic {
            range,
            severity: zed_extension_api::DiagnosticSeverity::Error,
            code: Some(zed_extension_api::DiagnosticCode {
                value: "SYNTAX_ERROR".to_string(),
                description: Some("语法错误".to_string()),
            }),
            message: format!("无效的语法: '{}'", error_text.trim()),
            source: Some("tree-sitter-cangjie".to_string()),
            fixes: None,
        });
    }

    // 递归遍历子节点
    if cursor.goto_first_child() {
        loop {
            find_error_nodes(cursor, content, diagnostics);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

/// 根据位置查找对应的符号节点
pub fn find_symbol_at_position(
    tree: &Tree,
    content: &str,
    position: zed_extension_api::Position,
) -> Option<SymbolInfo> {
    let point = TsPoint {
        row: position.line as usize,
        column: position.column as usize,
    };

    // 查找包含目标位置的符号节点
    let mut cursor = tree_sitter::TreeCursor::new(&tree.root_node());
    find_symbol_node(&mut cursor, content, point)
}

/// 递归查找包含目标位置的符号节点
fn find_symbol_node(
    cursor: &mut tree_sitter::TreeCursor,
    content: &str,
    point: TsPoint,
) -> Option<SymbolInfo> {
    let node = cursor.node();

    // 检查当前节点是否包含目标位置且是符号节点
    if node.contains_point(point) {
        match node.kind() {
            "function_definition"
            | "variable_declaration"
            | "struct_definition"
            | "enum_definition"
            | "import_declaration"
            | "method_definition" => {
                // 提取该节点的符号信息
                let query = Query::new(tree_sitter_cangjie::language(), SYMBOL_QUERY)
                    .expect("Invalid symbol query");
                let mut query_cursor = QueryCursor::new();

                for match_result in query_cursor.matches(&query, node, content.as_bytes()) {
                    let mut captures = HashMap::new();
                    for capture in match_result.captures {
                        captures.insert(
                            query
                                .capture_name_for_id(capture.index)
                                .unwrap()
                                .to_string(),
                            capture.node,
                        );
                    }

                    return match node.kind() {
                        "function_definition" => {
                            let name_node = captures.get("function.name").unwrap();
                            Some(SymbolInfo {
                                name: get_node_text(content, name_node),
                                r#type: SymbolType::Function,
                                range: node_to_zed_range(name_node),
                                detail: Some(format!("fn {}", get_node_text(content, name_node))),
                                node: node.clone(),
                            })
                        }
                        "variable_declaration" => {
                            let name_node = captures.get("variable.name").unwrap();
                            Some(SymbolInfo {
                                name: get_node_text(content, name_node),
                                r#type: SymbolType::Variable,
                                range: node_to_zed_range(name_node),
                                detail: Some(format!("let {}", get_node_text(content, name_node))),
                                node: node.clone(),
                            })
                        }
                        "struct_definition" => {
                            let name_node = captures.get("struct.name").unwrap();
                            Some(SymbolInfo {
                                name: get_node_text(content, name_node),
                                r#type: SymbolType::Struct,
                                range: node_to_zed_range(name_node),
                                detail: Some(format!(
                                    "struct {}",
                                    get_node_text(content, name_node)
                                )),
                                node: node.clone(),
                            })
                        }
                        "enum_definition" => {
                            let name_node = captures.get("enum.name").unwrap();
                            Some(SymbolInfo {
                                name: get_node_text(content, name_node),
                                r#type: SymbolType::Enum,
                                range: node_to_zed_range(name_node),
                                detail: Some(format!("enum {}", get_node_text(content, name_node))),
                                node: node.clone(),
                            })
                        }
                        "import_declaration" => {
                            let path_node = captures.get("import.path").unwrap();
                            let path = get_node_text(content, path_node)
                                .trim_matches('"')
                                .to_string();
                            Some(SymbolInfo {
                                name: path.clone(),
                                r#type: SymbolType::Import,
                                range: node_to_zed_range(&node),
                                detail: Some(format!("import {}", path)),
                                node: node.clone(),
                            })
                        }
                        "method_definition" => {
                            let name_node = captures.get("method.name").unwrap();
                            Some(SymbolInfo {
                                name: get_node_text(content, name_node),
                                r#type: SymbolType::Method,
                                range: node_to_zed_range(name_node),
                                detail: Some(format!(
                                    "method {}",
                                    get_node_text(content, name_node)
                                )),
                                node: node.clone(),
                            })
                        }
                        _ => None,
                    };
                }
            }
            _ => {
                // 递归查找子节点
                if cursor.goto_first_child() {
                    loop {
                        if let Some(symbol) = find_symbol_node(cursor, content, point) {
                            return Some(symbol);
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                    cursor.goto_parent();
                }
                None
            }
        }
    }

    None
}

/// 查找指定位置的标识符文本
pub fn find_identifier_at_point(
    cursor: &mut tree_sitter::TreeCursor,
    content: &str,
    point: TsPoint,
) -> Option<String> {
    let node = cursor.node();

    if node.kind() == "identifier" && node.contains_point(point) {
        return Some(get_node_text(content, &node));
    }

    if cursor.goto_first_child() {
        loop {
            if let Some(ident) = find_identifier_at_point(cursor, content, point) {
                return Some(ident);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }

    None
}
