//! cjfmt 代码格式化工具集成（遵循官方代码风格规范）
use serde::{Deserialize, Serialize};
use std::path::Path;
use zed_extension_api as zed;

/// cjfmt 配置（对应 cjfmt.toml）
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CjfmtConfig {
    /// 缩进配置
    #[serde(default)]
    pub indent: IndentConfig,
    /// 行宽配置
    #[serde(default)]
    pub line_width: LineWidthConfig,
    /// 换行符配置
    #[serde(default)]
    pub newline: NewlineConfig,
    /// 空格配置
    #[serde(default)]
    pub space: SpaceConfig,
    /// 命名风格配置
    #[serde(default)]
    pub naming: NamingConfig,
    /// 忽略配置
    #[serde(default)]
    pub ignore: IgnoreConfig,
    /// 高级配置
    #[serde(default)]
    pub advanced: AdvancedConfig,
}

/// 缩进配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct IndentConfig {
    /// 缩进类型
    #[serde(default = "default_indent_style")]
    pub style: IndentStyle,
    /// 缩进大小
    #[serde(default = "default_indent_size")]
    pub size: u32,
}

/// 缩进类型
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum IndentStyle {
    Space, // 空格（默认）
    Tab,   // 制表符
}

impl Default for IndentStyle {
    fn default() -> Self {
        Self::Space
    }
}

/// 行宽配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LineWidthConfig {
    /// 最大行宽（默认 120）
    #[serde(default = "default_max_line_width")]
    pub max: u32,
    /// 注释行宽
    #[serde(default)]
    pub comment: Option<u32>,
    /// 字符串行宽
    #[serde(default)]
    pub string: Option<u32>,
}

/// 换行符配置
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy, Default)]
#[serde(rename_all = "snake_case")]
pub enum NewlineStyle {
    #[default]
    Lf, // \n
    Crlf, // \r\n
    Cr,   // \r
    Auto, // 自动适配平台
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct NewlineConfig {
    #[serde(default)]
    pub style: NewlineStyle,
    /// 语句末尾强制换行
    #[serde(default = "default_force_newline_at_end")]
    pub force_at_end: bool,
}

/// 空格配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SpaceConfig {
    /// 运算符两侧空格
    #[serde(default = "default_space_around_operators")]
    pub around_operators: bool,
    /// 括号内侧空格
    #[serde(default = "default_space_inside_brackets")]
    pub inside_brackets: bool,
    /// 逗号后空格
    #[serde(default = "default_space_after_comma")]
    pub after_comma: bool,
    /// 函数参数括号空格
    #[serde(default = "default_space_inside_function_parens")]
    pub inside_function_parens: bool,
    /// 结构体字段冒号空格
    #[serde(default = "default_space_around_colon")]
    pub around_colon: bool,
}

/// 命名风格配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct NamingConfig {
    /// 变量命名风格
    #[serde(default = "default_naming_variable")]
    pub variable: NamingStyle,
    /// 函数命名风格
    #[serde(default = "default_naming_function")]
    pub function: NamingStyle,
    /// 类型命名风格
    #[serde(default = "default_naming_type")]
    pub r#type: NamingStyle,
    /// 常量命名风格
    #[serde(default = "default_naming_constant")]
    pub constant: NamingStyle,
    /// 模块命名风格
    #[serde(default = "default_naming_module")]
    pub module: NamingStyle,
}

/// 命名风格
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum NamingStyle {
    SnakeCase,      // snake_case
    PascalCase,     // PascalCase
    CamelCase,      // camelCase
    UpperSnakeCase, // UPPER_SNAKE_CASE
    KebabCase,      // kebab-case
    Preserve,       // 保留原有风格
}

/// 忽略配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct IgnoreConfig {
    /// 忽略的文件/目录
    #[serde(default)]
    pub files: Vec<String>,
    /// 忽略的代码块标记
    #[serde(default = "default_ignore_comment")]
    pub comment: String,
    /// 忽略的规则列表
    #[serde(default)]
    pub rules: Vec<String>,
}

/// 高级配置
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct AdvancedConfig {
    /// 保留注释格式
    #[serde(default = "default_preserve_comments")]
    pub preserve_comments: bool,
    /// 字符串自动换行
    #[serde(default = "default_wrap_strings")]
    pub wrap_strings: bool,
    /// 结构体字段对齐
    #[serde(default = "default_align_struct_fields")]
    pub align_struct_fields: bool,
    /// 导入语句排序
    #[serde(default = "default_sort_imports")]
    pub sort_imports: bool,
    /// 格式化预览模式
    #[serde(default)]
    pub preview: bool,
}

// 默认值
fn default_indent_style() -> IndentStyle {
    IndentStyle::Space
}
fn default_indent_size() -> u32 {
    4
}
fn default_max_line_width() -> u32 {
    120
}
fn default_force_newline_at_end() -> bool {
    true
}
fn default_space_around_operators() -> bool {
    true
}
fn default_space_inside_brackets() -> bool {
    false
}
fn default_space_after_comma() -> bool {
    true
}
fn default_space_inside_function_parens() -> bool {
    false
}
fn default_space_around_colon() -> bool {
    true
}
fn default_naming_variable() -> NamingStyle {
    NamingStyle::SnakeCase
}
fn default_naming_function() -> NamingStyle {
    NamingStyle::SnakeCase
}
fn default_naming_type() -> NamingStyle {
    NamingStyle::PascalCase
}
fn default_naming_constant() -> NamingStyle {
    NamingStyle::UpperSnakeCase
}
fn default_naming_module() -> NamingStyle {
    NamingStyle::SnakeCase
}
fn default_ignore_comment() -> String {
    "cjfmt-ignore".to_string()
}
fn default_preserve_comments() -> bool {
    true
}
fn default_wrap_strings() -> bool {
    true
}
fn default_align_struct_fields() -> bool {
    true
}
fn default_sort_imports() -> bool {
    true
}

/// cjfmt 管理器
#[derive(Debug, Default)]
pub struct CjfmtManager;

impl CjfmtManager {
    /// 检查 cjfmt 是否可用
    pub fn is_available() -> zed::Result<()> {
        Self::find_executable()?;
        Ok(())
    }

    /// 查找 cjfmt 可执行文件
    pub fn find_executable() -> zed::Result<zed::Path> {
        // 1. 从 CANGJIE_HOME 查找
        if let Ok(cangjie_home) = zed::env::var("CANGJIE_HOME") {
            let mut path = zed::Path::new(&cangjie_home);
            path.push("bin");
            path.push(if zed::platform::is_windows() {
                "cjfmt.exe"
            } else {
                "cjfmt"
            });

            if path.exists() && path.is_executable() {
                return Ok(path);
            }
        }

        // 2. 从 PATH 查找
        if let Some(path) = zed::env::find_executable(if zed::platform::is_windows() {
            "cjfmt.exe"
        } else {
            "cjfmt"
        }) {
            return Ok(path);
        }

        Err(zed::Error::NotFound(
            "未找到 cjfmt 工具，请配置 CANGJIE_HOME 或确保 cjfmt 在 PATH 中".to_string(),
        ))
    }

    /// 加载 cjfmt 配置
    pub fn load_config(
        worktree: &zed::Worktree,
        extension_config: &super::config::CangjieConfig,
    ) -> zed::Result<CjfmtConfig> {
        // 1. 项目根目录 cjfmt.toml
        let project_config = worktree.path().join("cjfmt.toml");
        if project_config.exists() {
            return Self::parse_config(&project_config);
        }

        // 2. 用户目录 .cjfmt.toml
        if let Some(user_config) = Self::user_config_path() {
            if user_config.exists() {
                return Self::parse_config(&user_config);
            }
        }

        // 3. 扩展配置
        Ok(extension_config.cjfmt.clone())
    }

    /// 解析配置文件
    fn parse_config(path: &zed::Path) -> zed::Result<CjfmtConfig> {
        let content = zed::fs::read_to_string(path)
            .map_err(|e| zed::Error::IoError(format!("读取 {} 失败: {}", path.to_str()?, e)))?;

        toml::from_str(&content)
            .map_err(|e| zed::Error::InvalidConfig(format!("解析 {} 失败: {}", path.to_str()?, e)))
    }

    /// 用户目录配置路径
    fn user_config_path() -> Option<zed::Path> {
        zed::env::home_dir().map(|home| home.join(".cjfmt.toml"))
    }

    /// 执行格式化
    pub fn format_document(
        worktree: &zed::Worktree,
        document: &zed::Document,
        config: &CjfmtConfig,
    ) -> zed::Result<Option<Vec<zed::TextEdit>>> {
        let file_path = document.path();
        let file_str = file_path.to_str()?;

        // 跳过忽略文件
        if config
            .ignore
            .files
            .iter()
            .any(|pattern| glob::Pattern::new(pattern).map_or(false, |p| p.matches(file_str)))
        {
            zed::log::debug!("文件 {} 在 cjfmt 忽略列表中", file_str);
            return Ok(None);
        }

        // 构建命令
        let cjfmt_path = Self::find_executable()?;
        let mut args = Vec::new();

        // 配置参数
        args.extend(Self::config_to_args(config));
        args.push("--stdin".to_string());
        args.push(format!("--stdin-filename={}", file_str));

        // 执行命令
        let mut child = zed::process::Command::new(cjfmt_path.to_str()?)
            .args(&args)
            .stdin(zed::process::Stdio::piped())
            .stdout(zed::process::Stdio::piped())
            .stderr(zed::process::Stdio::piped())
            .spawn()?;

        // 写入文件内容
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(document.text().as_bytes())?;

        let output = child.wait_with_output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(zed::Error::ProcessFailed(format!(
                "cjfmt 格式化失败: {}",
                stderr.trim()
            )));
        }

        // 解析格式化结果
        let formatted_text = String::from_utf8(output.stdout)
            .map_err(|e| zed::Error::InvalidData(format!("格式化结果不是 UTF-8: {}", e)))?;

        // 对比原始内容，生成编辑
        if formatted_text == document.text() {
            return Ok(None);
        }

        // 生成全文档替换的 TextEdit
        let text_edit = zed::TextEdit {
            range: zed::Range {
                start: zed::Position { line: 0, column: 0 },
                end: zed::Position {
                    line: document.line_count() as u32,
                    column: 0,
                },
            },
            new_text: formatted_text,
        };

        Ok(Some(vec![text_edit]))
    }

    /// 配置转命令行参数
    fn config_to_args(config: &CjfmtConfig) -> Vec<String> {
        let mut args = Vec::new();

        // 缩进配置
        args.push(format!(
            "--indent-style={}",
            match config.indent.style {
                IndentStyle::Space => "space",
                IndentStyle::Tab => "tab",
            }
        ));
        args.push(format!("--indent-size={}", config.indent.size));

        // 行宽配置
        args.push(format!("--line-width={}", config.line_width.max));
        if let Some(comment_width) = config.line_width.comment {
            args.push(format!("--comment-line-width={}", comment_width));
        }
        if let Some(string_width) = config.line_width.string {
            args.push(format!("--string-line-width={}", string_width));
        }

        // 换行符配置
        args.push(format!(
            "--newline-style={}",
            match config.newline.style {
                NewlineStyle::Lf => "lf",
                NewlineStyle::Crlf => "crlf",
                NewlineStyle::Cr => "cr",
                NewlineStyle::Auto => "auto",
            }
        ));
        if config.newline.force_at_end {
            args.push("--force-newline-at-end".to_string());
        }

        // 空格配置
        if config.space.around_operators {
            args.push("--space-around-operators".to_string());
        } else {
            args.push("--no-space-around-operators".to_string());
        }
        if config.space.inside_brackets {
            args.push("--space-inside-brackets".to_string());
        } else {
            args.push("--no-space-inside-brackets".to_string());
        }
        if config.space.after_comma {
            args.push("--space-after-comma".to_string());
        } else {
            args.push("--no-space-after-comma".to_string());
        }
        if config.space.inside_function_parens {
            args.push("--space-inside-function-parens".to_string());
        } else {
            args.push("--no-space-inside-function-parens".to_string());
        }
        if config.space.around_colon {
            args.push("--space-around-colon".to_string());
        } else {
            args.push("--no-space-around-colon".to_string());
        }

        // 命名风格配置
        args.push(format!(
            "--variable-naming={}",
            Self::naming_style_to_str(config.naming.variable)
        ));
        args.push(format!(
            "--function-naming={}",
            Self::naming_style_to_str(config.naming.function)
        ));
        args.push(format!(
            "--type-naming={}",
            Self::naming_style_to_str(config.naming.r#type)
        ));
        args.push(format!(
            "--constant-naming={}",
            Self::naming_style_to_str(config.naming.constant)
        ));
        args.push(format!(
            "--module-naming={}",
            Self::naming_style_to_str(config.naming.module)
        ));

        // 高级配置
        if config.advanced.preserve_comments {
            args.push("--preserve-comments".to_string());
        } else {
            args.push("--no-preserve-comments".to_string());
        }
        if config.advanced.wrap_strings {
            args.push("--wrap-strings".to_string());
        } else {
            args.push("--no-wrap-strings".to_string());
        }
        if config.advanced.align_struct_fields {
            args.push("--align-struct-fields".to_string());
        } else {
            args.push("--no-align-struct-fields".to_string());
        }
        if config.advanced.sort_imports {
            args.push("--sort-imports".to_string());
        } else {
            args.push("--no-sort-imports".to_string());
        }
        if config.advanced.preview {
            args.push("--preview".to_string());
        }

        args
    }

    /// 命名风格转字符串
    fn naming_style_to_str(style: NamingStyle) -> &'static str {
        match style {
            NamingStyle::SnakeCase => "snake_case",
            NamingStyle::PascalCase => "PascalCase",
            NamingStyle::CamelCase => "camelCase",
            NamingStyle::UpperSnakeCase => "UPPER_SNAKE_CASE",
            NamingStyle::KebabCase => "kebab-case",
            NamingStyle::Preserve => "preserve",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_basic() {
        let worktree = zed::Worktree::new(zed::Path::new("/tmp/test-cjfmt"));
        let config = CjfmtConfig::default();
        let document = zed::Document::new(
            zed::Path::new("/tmp/test-cjfmt/test.cj"),
            "fn add(a:Int,b:Int)->Int{return a+b;}".to_string(),
        );

        let result = CjfmtManager::format_document(&worktree, &document, &config).unwrap();
        assert!(result.is_some());

        let formatted_text = &result.unwrap()[0].new_text;
        assert_eq!(
            formatted_text,
            "fn add(a: Int, b: Int) -> Int {\n    return a + b;\n}"
        );
    }
}
