//! 代码格式化工具 cjfmt 集成
use crate::config::CangjieConfig;
use zed_extension_api;

/// cjfmt 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CjfmtConfig {
    /// 缩进风格（space/tab）
    pub indent_style: String,
    /// 缩进大小（空格数）
    pub indent_size: u8,
    /// 行尾分号自动补全
    pub auto_semicolon: bool,
    /// 换行符类型（lf/crlf）
    pub line_ending: String,
}

impl Default for CjfmtConfig {
    fn default() -> Self {
        Self {
            indent_style: "space".to_string(),
            indent_size: 4,
            auto_semicolon: true,
            line_ending: "lf".to_string(),
        }
    }
}

/// cjfmt 管理器
#[derive(Debug, Default)]
pub struct CjfmtManager;

impl CjfmtManager {
    /// 检查 cjfmt 是否可用
    pub fn is_available() -> zed_extension_api::Result<()> {
        // 检查 cjfmt 命令是否存在
        if std::process::Command::new("cjfmt")
            .arg("--version")
            .output()
            .is_err()
        {
            return Err(zed_extension_api::Error::NotFound(
                "cjfmt 工具未找到，请安装并配置到 PATH 中".to_string(),
            ));
        }
        Ok(())
    }

    /// 加载配置
    pub fn load_config(
        _worktree: &zed_extension_api::Worktree,
        config: &CangjieConfig,
    ) -> zed_extension_api::Result<CjfmtConfig> {
        Ok(config.cjfmt.clone())
    }

    /// 格式化文档
    pub fn format_document(
        _worktree: &zed_extension_api::Worktree,
        document: &zed_extension_api::Document,
        config: &CjfmtConfig,
    ) -> zed_extension_api::Result<Option<Vec<zed_extension_api::TextEdit>>> {
        Self::is_available()?;

        // 构建格式化命令参数
        let mut args = vec!["format".to_string()];

        // 添加配置参数
        args.push(format!("--indent-style={}", config.indent_style));
        args.push(format!("--indent-size={}", config.indent_size));
        if config.auto_semicolon {
            args.push("--auto-semicolon".to_string());
        }
        args.push(format!("--line-ending={}", config.line_ending));

        // 读取文档内容
        let content = document.text();

        // 执行格式化命令（通过 stdin 传入内容，stdout 获取结果）
        let output = std::process::Command::new("cjfmt")
            .args(&args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(zed_extension_api::Error::ProcessFailed(format!(
                "cjfmt 格式化失败: {}",
                stderr
            )));
        }

        let formatted_content = String::from_utf8(output.stdout)?;
        if formatted_content == content {
            // 内容未变更，返回 None
            return Ok(None);
        }

        // 生成全文档替换的 TextEdit
        let full_range = zed_extension_api::Range {
            start: zed_extension_api::Position { line: 0, column: 0 },
            end: zed_extension_api::Position {
                line: content.lines().count() as u32,
                column: 0,
            },
        };

        Ok(Some(vec![zed_extension_api::TextEdit {
            range: full_range,
            new_text: formatted_content,
        }]))
    }
}
