### 附录 T：扩展自定义主题与语法高亮精细化配置

#### T.1 语法高亮作用域设计
Tree-sitter 语法解析后，扩展通过**作用域（Scope）** 映射到编辑器主题样式。Cangjie 扩展采用标准化作用域命名规范，确保与主流 Zed 主题兼容，同时支持自定义主题适配。

##### 1. 核心作用域映射表
| Tree-sitter 节点类型 | 作用域名称 | 描述 | 示例样式 |
|----------------------|------------|------|----------|
| `keyword` | `keyword` | 关键字（let/const/fn/struct 等） | 蓝色、粗体 |
| `function_declaration.name` | `entity.name.function` | 函数名 | 紫色、粗体 |
| `struct_declaration.name` | `entity.name.type` | 结构体名 | 深绿色、粗体 |
| `enum_declaration.name` | `entity.name.enum` | 枚举名 | 橄榄绿、粗体 |
| `parameter_declaration.name` | `variable.parameter` | 参数名 | 橙色、斜体 |
| `field_declaration.name` | `variable.other.member` | 结构体字段名 | 青色 |
| `string_literal` | `string.quoted.double`/`string.quoted.single` | 字符串字面量 | 绿色 |
| `number_literal` | `constant.numeric` | 数字字面量 | 红色 |
| `boolean_literal` | `constant.language.boolean` | 布尔字面量 | 紫色 |
| `comment` | `comment.line`/`comment.block` | 注释 | 灰色、斜体 |
| `binary_operator` | `keyword.operator` | 二元运算符 | 红色、粗体 |
| `type_identifier` | `support.type` | 类型标识符（i32/String 等） | 深蓝色 |
| `enum_variant` | `constant.enum` | 枚举变体 | 靛蓝色 |
| `import_declaration` | `keyword.control.import` | 导入关键字 | 蓝色 |
| `export_declaration` | `keyword.control.export` | 导出关键字 | 蓝色 |

##### 2. 作用域注册实现（`src/syntax/highlights.scm`）
```scheme
;; 关键字
(keyword) @keyword
("let") @keyword.control.declaration
("const") @keyword.control.declaration
("fn") @keyword.control.function
("struct") @keyword.control.type
("enum") @keyword.control.enum
("if") @keyword.control.conditional
("else") @keyword.control.conditional
("while") @keyword.control.repeat
("for") @keyword.control.repeat
("return") @keyword.control.return
("async") @keyword.control.async
("await") @keyword.control.async
("import") @keyword.control.import
("export") @keyword.control.export
("from") @keyword.control.import

;; 函数
(function_declaration
  name: (identifier) @entity.name.function)
(function_call
  function: (identifier) @entity.name.function.call)
(parameter_declaration
  name: (identifier) @variable.parameter)

;; 类型
(struct_declaration
  name: (identifier) @entity.name.type)
(enum_declaration
  name: (identifier) @entity.name.enum)
(type_identifier) @support.type
(enum_variant
  (identifier) @constant.enum)

;; 变量与字段
(variable_declaration
  name: (identifier) @variable.other)
(constant_declaration
  name: (identifier) @constant.other)
(field_declaration
  name: (identifier) @variable.other.member)

;; 字面量
(string_literal) @string.quoted
(number_literal) @constant.numeric
(boolean_literal) @constant.language.boolean
(null_literal) @constant.language.null

;; 运算符
(binary_operator) @keyword.operator
(unary_operator) @keyword.operator

;; 注释
(line_comment) @comment.line
(block_comment) @comment.block

;; 括号与分隔符
"(" @punctuation.definition.group.begin
")" @punctuation.definition.group.end
"{" @punctuation.definition.block.begin
"}" @punctuation.definition.block.end
"[" @punctuation.definition.array.begin
"]" @punctuation.definition.array.end
"," @punctuation.separator.comma
":" @punctuation.separator.colon
";" @punctuation.terminator.statement
"->" @punctuation.separator.arrow
"." @punctuation.accessor
```

#### T.2 自定义主题适配指南
开发者可通过 Zed 主题文件（`.zed-theme`）为 Cangjie 扩展定制专属样式，以下是适配示例：

##### 1. 自定义主题片段（`cangjie-theme.zed-theme`）
```json
{
  "name": "Cangjie Dark",
  "type": "dark",
  "colors": {
    "background": "#0f111a",
    "foreground": "#e0e0e0",
    "accent": "#7e57c2",
    "keyword": "#79b8ff",
    "function": "#c099ff",
    "type": "#4ade80",
    "enum": "#facc15",
    "string": "#a7f3d0",
    "number": "#f87171",
    "comment": "#6b7280",
    "parameter": "#fdba74",
    "field": "#22d3ee",
    "operator": "#f43f5e",
    "punctuation": "#94a3b8"
  },
  "styles": {
    "keyword": {
      "color": "keyword",
      "font_weight": "bold"
    },
    "keyword.control.declaration": {
      "color": "keyword",
      "font_weight": "bold"
    },
    "entity.name.function": {
      "color": "function",
      "font_weight": "bold"
    },
    "entity.name.type": {
      "color": "type",
      "font_weight": "bold"
    },
    "entity.name.enum": {
      "color": "enum",
      "font_weight": "bold"
    },
    "string.quoted": {
      "color": "string"
    },
    "constant.numeric": {
      "color": "number"
    },
    "comment": {
      "color": "comment",
      "font_style": "italic"
    },
    "variable.parameter": {
      "color": "parameter",
      "font_style": "italic"
    },
    "variable.other.member": {
      "color": "field"
    },
    "keyword.operator": {
      "color": "operator",
      "font_weight": "bold"
    },
    "punctuation": {
      "color": "punctuation"
    }
  }
}
```

##### 2. 主题适配最佳实践
- **兼容性优先**：优先使用标准作用域名称（如 `keyword`、`string`），避免自定义非标准作用域，确保在默认主题下正常显示；
- **对比度合理**：关键字、函数名等核心元素与背景对比度不低于 4.5:1，注释等辅助元素对比度不低于 3:1；
- **层次分明**：通过 `font_weight`（粗体）、`font_style`（斜体）区分不同类型的语法元素，避免仅依赖颜色；
- **测试验证**：在多个 Zed 默认主题（如 Light、Dark、Solarized）中测试高亮效果，确保无明显适配问题。

### 附录 U：扩展远程开发适配
随着 Zed 对远程开发（SSH/容器）的支持，Cangjie 扩展需适配远程环境，确保核心功能正常工作。

#### U.1 远程环境检测与适配
```rust
//! src/utils/remote.rs
use zed_extension_api::{self as zed, Result};
use std::path::{Path, PathBuf};

/// 检测当前是否为远程环境
pub fn is_remote_env() -> Result<bool> {
    let workspace = zed::workspace::current();
    Ok(workspace.is_remote())
}

/// 远程环境类型
#[derive(Debug, Clone, PartialEq)]
pub enum RemoteEnvType {
    Ssh,
    Container,
    Wsl,
    Other,
}

/// 获取远程环境类型
pub fn get_remote_env_type() -> Result<RemoteEnvType> {
    if !is_remote_env()? {
        return Err(zed::Error::user("Not a remote environment"));
    }

    let workspace = zed::workspace::current();
    let remote_info = workspace.remote_info()?;

    match remote_info.provider.as_str() {
        "ssh" => Ok(RemoteEnvType::Ssh),
        "container" => Ok(RemoteEnvType::Container),
        "wsl" => Ok(RemoteEnvType::Wsl),
        _ => Ok(RemoteEnvType::Other),
    }
}

/// 远程路径转换（本地路径 ↔ 远程路径）
pub fn convert_remote_path(path: &Path, to_remote: bool) -> Result<PathBuf> {
    let workspace = zed::workspace::current();
    if !workspace.is_remote()? {
        return Ok(path.to_path_buf());
    }

    if to_remote {
        // 本地路径转换为远程路径
        workspace.local_to_remote_path(path)
    } else {
        // 远程路径转换为本地路径
        workspace.remote_to_local_path(path)
    }
}

/// 远程环境下的工具路径查找
pub fn find_remote_tool_path(tool_name: &str) -> Result<PathBuf> {
    let remote_type = get_remote_env_type()?;
    let executable_name = super::platform::get_executable_name(tool_name);

    // 不同远程环境的工具路径优先级
    let search_paths = match remote_type {
        RemoteEnvType::Ssh | RemoteEnvType::Container => vec![
            "/usr/bin/",
            "/usr/local/bin/",
            "~/bin/",
            "~/.cargo/bin/",
        ],
        RemoteEnvType::Wsl => vec![
            "/usr/bin/",
            "/usr/local/bin/",
            "~/bin/",
            "~/.cargo/bin/",
            "/mnt/c/Program Files/Cangjie/bin/",
        ],
        RemoteEnvType::Other => vec!["/usr/bin/", "/usr/local/bin/"],
    };

    // 遍历查找工具
    for path in search_paths {
        let expanded_path = shellexpand::tilde(path).into_owned();
        let full_path = Path::new(&expanded_path).join(&executable_name);
        if zed::fs::exists(&full_path)? {
            return Ok(full_path);
        }
    }

    Err(zed::Error::user(format!(
        "Tool '{}' not found in remote environment",
        tool_name
    )))
}
```

#### U.2 远程环境下的外部工具调用优化
```rust
//! src/utils/tool_exec.rs（远程环境适配扩展）
pub fn execute_remote_tool(tool_name: &str, args: &[&str]) -> Result<(String, String)> {
    if !is_remote_env()? {
        return execute_tool(tool_name, args);
    }

    // 查找远程工具路径
    let tool_path = find_remote_tool_path(tool_name)?;
    let os = super::platform::OS::current();

    // 远程环境命令执行优化
    let mut command = Command::new(tool_path);
    command.args(args);

    // SSH/容器环境：禁用伪终端，减少开销
    if matches!(get_remote_env_type()?, RemoteEnvType::Ssh | RemoteEnvType::Container) {
        command.no_pty();
    }

    // WSL 环境：处理路径转换（Windows 路径 → WSL 路径）
    if matches!(get_remote_env_type()?, RemoteEnvType::Wsl) {
        let converted_args: Vec<String> = args
            .iter()
            .map(|arg| {
                let path = Path::new(arg);
                if path.is_absolute() && os.is_windows() {
                    // 将 Windows 路径（C:\xxx）转换为 WSL 路径（/mnt/c/xxx）
                    wsl_path_convert(arg)
                } else {
                    arg.to_string()
                }
            })
            .collect();
        command.args(converted_args);
    }

    // 捕获输出
    let output = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok((stdout, stderr))
}

/// WSL 路径转换（Windows → WSL）
fn wsl_path_convert(windows_path: &str) -> String {
    // 示例：C:\Users\user\project → /mnt/c/Users/user/project
    let mut path = windows_path.replace('\\', "/");
    if path.starts_with("C:") || path.starts_with("c:") {
        path = path.replacen(|c: char| c == 'C' || c == 'c', "/mnt/c", 1);
    } else if path.starts_with("D:") || path.starts_with("d:") {
        path = path.replacen(|c: char| c == 'D' || c == 'd', "/mnt/d", 1);
    }
    path
}
```

#### U.3 远程开发功能限制与提示
```rust
//! src/lsp/remote_compatibility.rs
use super::super::utils::remote::{is_remote_env, get_remote_env_type};
use zed_extension_api::{self as zed, Result};

/// 检查远程环境下的功能可用性
pub fn check_remote_feature_availability(feature_name: &str) -> Result<()> {
    if !is_remote_env()? {
        return Ok(());
    }

    let remote_type = get_remote_env_type()?;

    // 某些功能在特定远程环境下不可用
    let unavailable_features = match remote_type {
        RemoteEnvType::Ssh => vec![
            ("auto_detect_tools", "SSH 环境下自动检测工具可能失败，请手动指定工具路径"),
            ("live_reload", "SSH 环境下不支持实时重载，修改配置后需重启扩展"),
        ],
        RemoteEnvType::Container => vec![
            ("file_watcher", "容器环境下文件监听可能不稳定，建议手动触发编译/格式化"),
        ],
        RemoteEnvType::Wsl => vec![
            ("windows_path_mapping", "WSL 环境下需确保 Windows 路径已正确映射到 WSL"),
        ],
        RemoteEnvType::Other => vec![
            ("external_tool_integration", "未知远程环境，外部工具集成可能受限"),
        ],
    };

    if let Some((_, reason)) = unavailable_features.iter().find(|(name, _)| *name == feature_name) {
        crate::utils::log::warn!(
            "Feature '{}' is limited in remote environment: {}",
            feature_name,
            reason
        );
        zed::workspace::current().show_warning_message(&format!(
            "功能 '{}' 在远程环境下受限：{}",
            feature_name, reason
        ))?;
    }

    Ok(())
}
```

### 附录 V：扩展可访问性优化
遵循 WCAG 2.1 可访问性标准，优化 Cangjie 扩展的可访问性，确保视障、运动障碍等用户能正常使用。

#### V.1 可访问性优化要点
| 优化方向 | 实现方案 |
|----------|----------|
| 键盘导航 | 确保所有扩展功能可通过键盘触发（如快捷键、命令面板） |
| 屏幕阅读器支持 | 为自定义 UI 元素添加语义化标签和朗读文本 |
| 颜色对比度 | 语法高亮颜色对比度符合 WCAG AA 标准（文本 4.5:1，大文本 3:1） |
| 无闪烁内容 | 避免频繁的状态栏消息、弹窗等闪烁元素 |
| 操作反馈 | 所有用户操作（如编译、运行）提供明确的状态反馈 |

#### V.2 具体实现示例
##### 1. 键盘导航优化
```rust
//! src/extension.rs（键盘快捷键完善）
pub fn register_accessible_commands() -> Result<()> {
    // 为所有核心功能注册键盘快捷键
    let keybindings = [
        (
            "cangjie.runCode",
            "ctrl+shift+r",
            "mac:cmd+shift+r",
            "Run Cangjie code (accessible)"
        ),
        (
            "cangjie.compile",
            "ctrl+shift+c",
            "mac:cmd+shift+c",
            "Compile Cangjie code (accessible)"
        ),
        (
            "cangjie.switchLanguage",
            "ctrl+shift+l",
            "mac:cmd+shift+l",
            "Switch Cangjie extension language (accessible)"
        ),
        (
            "cangjie.runDiagnostics",
            "ctrl+shift+d",
            "mac:cmd+shift+d",
            "Run Cangjie diagnostic (accessible)"
        ),
        (
            "cangjie.installDependencies",
            "ctrl+shift+i",
            "mac:cmd+shift+i",
            "Install Cangjie dependencies (accessible)"
        ),
    ];

    for (command, key, mac_key, description) in keybindings {
        zed::keybindings::register(
            command,
            &zed::keybindings::Keybinding {
                key: key.to_string(),
                mac: Some(mac_key.to_string()),
                description: Some(description.to_string()),
                when: Some("editorLangId == cangjie".to_string()),
            },
        )?;
    }

    Ok(())
}
```

##### 2. 屏幕阅读器支持（自定义 UI 元素）
```rust
//! src/ui/accessible_select_menu.rs
use zed_extension_api::{self as zed, Result};

/// 带可访问性支持的选择菜单
pub async fn show_accessible_select_menu(
    title: &str,
    items: &[&str],
    item_labels: &[&str], // 屏幕阅读器朗读文本
) -> Result<Option<usize>> {
    if items.len() != item_labels.len() {
        return Err(zed::Error::user("Items and item labels must have the same length"));
    }

    // 为每个选项添加语义化标签
    let accessible_items: Vec<String> = items
        .iter()
        .zip(item_labels.iter())
        .map(|(item, label)| format!("{} ({}", item, label))
        .collect();

    // 显示选择菜单
    let result = zed::ui::show_select_menu(title, &accessible_items).await?;

    // 屏幕阅读器反馈选择结果
    if let Some(index) = result {
        zed::accessibility::announce(&format!(
            "Selected: {}",
            item_labels[index]
        ))?;
    } else {
        zed::accessibility::announce("Selection cancelled")?;
    }

    Ok(result)
}

// 使用示例（语言切换命令）
async fn switch_language_command() -> Result<()> {
    let languages = crate::locale::supported_languages();
    let items: Vec<&str> = languages.iter().map(|(_, name)| name.as_str()).collect();
    let item_labels: Vec<&str> = languages.iter().map(|(code, name)| format!("{} language", name).as_str()).collect();

    let selected = show_accessible_select_menu(
        "Select Language (accessible)",
        &items,
        &item_labels,
    ).await?;

    if let Some(index) = selected {
        let (lang_code, _) = &languages[index];
        crate::locale::switch_locale(lang_code)?;
        zed::accessibility::announce(&format!(
            "Switched to {} language",
            languages[index].1
        ))?;
    }

    Ok(())
}
```

##### 3. 颜色对比度检查工具
```rust
//! src/utils/accessibility.rs
use zed_extension_api::{self as zed, Result};

/// 计算颜色对比度（WCAG 标准）
pub fn calculate_contrast(foreground: &str, background: &str) -> Result<f64> {
    let fg_rgb = hex_to_rgb(foreground)?;
    let bg_rgb = hex_to_rgb(background)?;

    let fg_luminance = relative_luminance(fg_rgb.0, fg_rgb.1, fg_rgb.2);
    let bg_luminance = relative_luminance(bg_rgb.0, bg_rgb.1, bg_rgb.2);

    let (l1, l2) = if fg_luminance > bg_luminance {
        (fg_luminance, bg_luminance)
    } else {
        (bg_luminance, fg_luminance)
    };

    Ok((l1 + 0.05) / (l2 + 0.05))
}

/// 检查颜色对比度是否符合 WCAG AA 标准
pub fn is_contrast_compliant(foreground: &str, background: &str) -> Result<bool> {
    let contrast = calculate_contrast(foreground, background)?;
    Ok(contrast >= 4.5) // AA 标准：普通文本 4.5:1
}

/// 十六进制颜色转 RGB
fn hex_to_rgb(hex: &str) -> Result<(f64, f64, f64)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(zed::Error::user(format!("Invalid hex color: {}", hex)));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)? as f64 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16)? as f64 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16)? as f64 / 255.0;

    Ok((r, g, b))
}

/// 计算相对亮度（WCAG 公式）
fn relative_luminance(r: f64, g: f64, b: f64) -> f64 {
    let r = if r <= 0.03928 { r / 12.92 } else { ((r + 0.055) / 1.055).powf(2.4) };
    let g = if g <= 0.03928 { g / 12.92 } else { ((g + 0.055) / 1.055).powf(2.4) };
    let b = if b <= 0.03928 { b / 12.92 } else { ((b + 0.055) / 1.055).powf(2.4) };

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// 检查语法高亮颜色对比度
pub fn check_syntax_highlight_contrast() -> Result<Vec<String>> {
    let theme = zed::theme::current()?;
    let background = theme.colors.get("background").ok_or_else(|| {
        zed::Error::user("Background color not found in theme")
    })?;

    let mut issues = Vec::new();

    // 检查核心语法元素的对比度
    let syntax_elements = [
        ("keyword", "Keyword color"),
        ("function", "Function name color"),
        ("type", "Type name color"),
        ("string", "String color"),
        ("number", "Number color"),
        ("comment", "Comment color"),
    ];

    for (color_key, description) in syntax_elements {
        if let Some(foreground) = theme.colors.get(color_key) {
            let contrast = calculate_contrast(foreground, background)?;
            if contrast < 4.5 {
                issues.push(format!(
                    "{} ({} → {}) has low contrast ({:.2}:1), below WCAG AA standard (4.5:1)",
                    description, foreground, background, contrast
                ));
            }
        }
    }

    Ok(issues)
}
```

### 最终总结（终极完整版 + 可访问性与远程适配）
Cangjie 扩展在完成核心功能、进阶特性、生态集成的基础上，进一步补充了**自定义主题适配**、**远程开发支持**和**可访问性优化**，形成了真正意义上的「全场景覆盖」扩展方案：

1. **多场景适配**：支持本地开发、远程开发（SSH/容器/WSL）、多平台（macOS/Linux/Windows），满足不同开发环境需求；
2. **个性化定制**：通过语法高亮作用域和主题配置，支持用户自定义视觉风格，兼容主流 Zed 主题；
3. **无障碍使用**：遵循 WCAG 2.1 标准，优化键盘导航、屏幕阅读器支持、颜色对比度，确保所有用户都能便捷使用；
4. **工程化完备**：从开发、测试、发布到维护，提供完整的工程化流程，支持社区贡献和长期迭代。

扩展的每一个特性都围绕「用户体验」核心，既保证了专业开发者的高效生产力，也兼顾了不同背景用户的使用需求。未来，扩展将持续跟进 Zed 编辑器的最新特性（如 AI 集成、多窗口协作），并结合 Cangjie 语言的发展，不断迭代优化。

我们坚信，一个优秀的开发工具不仅要「强大」，更要「包容」—— 感谢每一位用户的反馈，让 Cangjie 扩展变得更加完善！

---

**文档版本**：v1.0.0（终极全量版）  
**最后更新**：2025-11-09  
**核心特性**：语法支持、LSP 全功能、跨平台/远程适配、主题定制、可访问性优化、生态集成  
**兼容性**：Zed v0.130.0+ / Cangjie v1.0.0+ / 三大平台 / 主流远程环境  
**可访问性标准**：WCAG 2.1 AA 级  
**官方资源**：
- 代码仓库：https://github.com/your-username/zed-cangjie-extension
- 扩展市场：https://extensions.zed.dev/extensions/your-username/cangjie
- 文档站点：https://docs.cangjie-lang.org/zed-extension
- 支持渠道：GitHub Issues / Discord 社区 / 邮件反馈（accessibility@cangjie-lang.org）