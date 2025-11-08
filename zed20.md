在 Zed 编辑器的 `zed_extension_api` 中，`Document` 结构体是 LSP 扩展与编辑器文档交互的核心接口，封装了文档的路径、文本内容、版本等关键信息。以下是基于最新 `zed_extension_api`（v0.7.0+）的 `Document` 完整定义、核心字段、方法及使用说明，结合你之前的 Cangjie LSP 场景补充示例：

### 一、`Document` 核心定义（简化版，对应源码逻辑）
```rust
//! 来自 zed_extension_api::Document
use std::path::PathBuf;
use zed_extension_api::lsp::{Position, Range, TextEdit};

#[derive(Debug, Clone)]
pub struct Document {
    /// 文档唯一标识符（Zed 内部使用，用于区分不同文档）
    pub id: String,
    /// 文档本地文件路径（若为未保存文件，路径可能为空或临时路径）
    pub path: PathBuf,
    /// 文档当前文本内容（完整字符串）
    pub content: String,
    /// 文档版本号（每次修改递增，用于 LSP 增量同步）
    pub version: u64,
    /// 文档语言类型（如 "Cangjie"、"Rust"、"TypeScript"，与语法包关联）
    pub language: String,
    /// 文档行结束符类型（"\n" 或 "\r\n"）
    pub line_ending: String,
}
```

### 二、核心字段说明
| 字段名          | 类型         | 作用                                                                 |
|-----------------|--------------|----------------------------------------------------------------------|
| `id`            | `String`     | 编辑器内部分配的唯一 ID，用于追踪文档生命周期（如区分多个打开的相同文件）。 |
| `path`          | `PathBuf`    | 文档对应的本地文件路径（关键！LSP 中用于生成 `Uri`、缓存文档等）。       |
| `content`       | `String`     | 文档完整文本内容（LSP 解析、语法检查、符号提取的核心数据源）。           |
| `version`       | `u64`        | 文档版本号（每次编辑操作递增，LSP 用于处理增量更新，避免重复解析）。     |
| `language`      | `String`     | 文档关联的语言标识（需与 `tree-sitter-cangjie` 语法包的 `language_name` 一致，如 "Cangjie"）。 |
| `line_ending`   | `String`     | 行结束符（`"\n"` 或 `"\r\n"`），格式化时需适配此配置。                 |

### 三、`Document` 核心方法（API 提供的关键交互能力）
`zed_extension_api` 通过 trait 为 `Document` 提供了常用方法（实际使用时直接调用实例方法即可）：

#### 1. 获取文档文本内容
```rust
/// 获取文档完整文本（等同于直接访问 `content` 字段，语义更清晰）
pub fn text(&self) -> &str {
    &self.content
}
```

#### 2. 获取文档路径字符串
```rust
/// 将 `path` 转换为字符串（处理非 UTF-8 路径的边缘情况）
pub fn path_str(&self) -> Option<&str> {
    self.path.to_str()
}
```

#### 3. 根据位置获取行内容
```rust
/// 获取指定行的文本（行号从 0 开始，与 LSP `Position.line` 一致）
/// 若行号超出范围，返回 `None`
pub fn line(&self, line_number: usize) -> Option<&str> {
    self.content.lines().nth(line_number)
}
```

#### 4. 获取文档总行数
```rust
/// 文档的总行数（用于 LSP `Range` 合法性检查）
pub fn line_count(&self) -> usize {
    self.content.lines().count()
}
```

#### 5. 根据 LSP 范围获取文本
```rust
/// 提取 `range` 对应的文本片段（严格遵循 LSP 左闭右开规则）
/// 常用于范围格式化、范围语法检查等场景
pub fn text_in_range(&self, range: &Range) -> Option<String> {
    let lines: Vec<&str> = self.content.lines().collect();
    let start_line = range.start.line as usize;
    let end_line = range.end.line as usize;
    
    if start_line >= lines.len() || end_line > lines.len() {
        return None;
    }
    
    let result = if start_line == end_line {
        // 同一行：截取从 start.character 到 end.character 的部分
        let line = lines[start_line];
        let start_char = range.start.character as usize;
        let end_char = range.end.character as usize;
        if start_char > line.len() || end_char > line.len() {
            return None;
        }
        line[start_char..end_char].to_string()
    } else {
        // 多行：拼接多行文本（处理每行的字符范围）
        let mut parts = Vec::new();
        // 第一行：从 start.character 到行尾
        let first_line = lines[start_line];
        let start_char = range.start.character as usize;
        if start_char <= first_line.len() {
            parts.push(first_line[start_char..].to_string());
        }
        // 中间行：完整添加
        parts.extend(
            lines[start_line + 1..end_line]
                .iter()
                .map(|line| line.to_string())
        );
        // 最后一行：从开头到 end.character
        let last_line = lines[end_line];
        let end_char = range.end.character as usize;
        if end_char <= last_line.len() {
            parts.push(last_line[..end_char].to_string());
        }
        parts.join(&self.line_ending)
    };
    
    Some(result)
}
```

#### 6. 应用文本编辑（修改文档内容）
```rust
/// 根据 LSP `TextEdit` 修改文档内容（返回修改后的新文档实例，不直接修改原文档）
/// 常用于格式化、自动修复等场景（如 Cangjie LSP 的代码格式化功能）
pub fn apply_text_edits(&self, edits: &[TextEdit]) -> Result<Self, zed_extension_api::Error> {
    let mut new_content = self.content.clone();
    let mut offset_adjustment = 0; // 处理编辑后字符偏移量变化
    
    // 按编辑范围的起始位置排序（确保正确处理重叠编辑）
    let mut sorted_edits = edits.to_vec();
    sorted_edits.sort_by_key(|edit| edit.range.start);
    
    for edit in sorted_edits {
        // 将 LSP Range 转换为字节偏移量
        let start_offset = self.position_to_offset(&edit.range.start)?;
        let end_offset = self.position_to_offset(&edit.range.end)?;
        
        // 应用偏移量调整（处理之前编辑导致的位置变化）
        let adjusted_start = start_offset + offset_adjustment;
        let adjusted_end = end_offset + offset_adjustment;
        
        // 替换文本片段
        if adjusted_start > new_content.len() || adjusted_end > new_content.len() {
            return Err(zed_extension_api::Error::InvalidData(
                "TextEdit 范围超出文档边界".to_string()
            ));
        }
        new_content.replace_range(adjusted_start..adjusted_end, &edit.new_text);
        
        // 更新偏移量调整（新文本长度 - 原文本长度）
        offset_adjustment += edit.new_text.len() - (end_offset - start_offset);
    }
    
    // 返回修改后的新文档（版本号递增）
    Ok(Self {
        version: self.version + 1,
        content: new_content,
        ..self.clone()
    })
}
```

#### 7. 位置转换工具（LSP Position ↔ 字节偏移量）
```rust
/// 将 LSP `Position`（line, character）转换为文档内容的字节偏移量
/// 核心用于文本编辑、字符截取等底层操作
pub fn position_to_offset(&self, position: &Position) -> Result<usize, zed_extension_api::Error> {
    let line = position.line as usize;
    let character = position.character as usize;
    let lines: Vec<&str> = self.content.lines().collect();
    
    if line >= lines.len() {
        return Err(zed_extension_api::Error::InvalidData(
            format!("行号 {} 超出文档总行数 {}", line, lines.len())
        ));
    }
    
    // 计算前 line 行的总字节数 + 当前行的字符偏移量
    let mut offset = 0;
    for (i, l) in lines.iter().enumerate().take(line) {
        offset += l.len() + self.line_ending.len(); // 加上行结束符的字节数
    }
    
    let current_line = lines[line];
    // 确保 character 不超出当前行长度（处理多字节字符？Zed 内部已确保 character 是 UTF-8 代码点偏移）
    let char_offset = current_line
        .char_indices()
        .nth(character)
        .map(|(idx, _)| idx)
        .unwrap_or(current_line.len());
    
    Ok(offset + char_offset)
}

/// 将字节偏移量转换为 LSP `Position`
pub fn offset_to_position(&self, offset: usize) -> Position {
    let mut remaining = offset;
    let mut line = 0;
    let mut character = 0;
    
    for (i, l) in self.content.lines().enumerate() {
        let line_length = l.len() + self.line_ending.len();
        if remaining <= line_length {
            line = i;
            // 计算当前行的字符数（处理 UTF-8 多字节字符）
            character = l
                .char_indices()
                .take_while(|(idx, _)| *idx < remaining)
                .count() as u32;
            break;
        }
        remaining -= line_length;
    }
    
    Position { line: line as u32, character }
}
```

### 四、在 Cangjie LSP 中的典型使用场景
结合你之前的 `tree_sitter_utils` 和 `language_server` 代码，补充 `Document` 的实际应用示例：

#### 1. 解析文档生成语法树
```rust
use crate::tree_sitter_utils;

fn parse_cangjie_document(document: &zed_extension_api::Document) -> tree_sitter::Tree {
    // 1. 获取文档文本内容（核心数据源）
    let content = document.text();
    // 2. 调用 Tree-sitter 解析（适配 Cangjie 语法）
    tree_sitter_utils::parse_document(content)
}
```

#### 2. 提取文档符号（依赖 `path` 和 `content`）
```rust
fn extract_document_symbols(document: &zed_extension_api::Document) -> Vec<tree_sitter_utils::SymbolInfo> {
    let content = document.text();
    let tree = parse_cangjie_document(document);
    
    // 提取符号（后续用于 LSP 文档符号、补全功能）
    tree_sitter_utils::extract_symbols(content, &tree)
}
```

#### 3. 范围格式化（使用 `text_in_range` 和 `apply_text_edits`）
```rust
use zed_extension_api::lsp::{Range, TextEdit};
use crate::cjfmt::CjfmtConfig;

fn format_document_range(
    document: &zed_extension_api::Document,
    range: &Range,
    config: &CjfmtConfig
) -> Result<zed_extension_api::Document, zed_extension_api::Error> {
    // 1. 获取范围文本
    let range_text = document.text_in_range(range)
        .ok_or_else(|| zed_extension_api::Error::InvalidData("范围超出文档边界".to_string()))?;
    
    // 2. 调用 cjfmt 格式化范围文本（你的格式化逻辑）
    let formatted_text = crate::cjfmt::format_range_text(&range_text, config)?;
    
    // 3. 构建 LSP TextEdit
    let edit = TextEdit {
        range: range.clone(),
        new_text: formatted_text,
    };
    
    // 4. 应用编辑，返回新文档
    document.apply_text_edits(&[edit])
}
```

#### 4. 生成 LSP Uri（依赖 `path`）
```rust
use zed_extension_api::lsp::Uri;

fn document_to_uri(document: &zed_extension_api::Document) -> Result<Uri, zed_extension_api::Error> {
    // 将 Document.path 转换为 LSP 标准 Uri（如 "file:///home/user/project/main.cj"）
    Uri::from_file_path(&document.path)
        .map_err(|_| zed_extension_api::Error::InvalidData(
            format!("无法将路径 {:?} 转换为 Uri", document.path)
        ))
}
```

### 五、关键注意事项
1. **路径合法性**：未保存的文档（如新建未保存文件）的 `path` 可能为空，此时 `path_str()` 返回 `None`，LSP 需处理临时文档场景（如仅缓存内存，不写入磁盘）。
2. **版本号使用**：每次文档修改后 `version` 递增，LSP 需通过 `version` 跟踪文档状态，避免基于旧版本解析导致的错误（如增量同步、诊断缓存）。
3. **字符编码**：`content` 是 UTF-8 编码的字符串，`Position.character` 是 UTF-8 代码点偏移（非字节偏移），需使用 `position_to_offset` 转换后再操作文本（避免多字节字符截断）。
4. **语言标识匹配**：`language` 字段需与 `tree-sitter-cangjie` 语法包的 `language_name` 一致（如语法包定义为 "cangjie"，则文档 `language` 需为 "cangjie"），否则 Zed 不会触发对应的 LSP 功能。

### 六、完整依赖与导入
在你的 Cangjie LSP 中使用 `Document` 时，需确保 `Cargo.toml` 包含正确的依赖：
```toml
[dependencies]
zed_extension_api = "0.7.0"
# 其他依赖...
```

导入方式：
```rust
use zed_extension_api::Document;
use zed_extension_api::lsp::{Position, Range, TextEdit, Uri};
```

以上就是 `zed_extension_api::Document` 的核心内容，结合你的 Cangjie LSP 场景，重点关注 `text()`、`path`、`version` 及位置转换方法的使用，即可实现语法解析、符号提取、格式化等核心 LSP 功能。