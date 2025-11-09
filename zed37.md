# Cangjie Theme for Zed: 仓颉语言专属语法主题
基于 [Zed 主题扩展规范](https://zed.dev/docs/extensions/themes)，为 **仓颉编程语言（Cangjie Lang）** 设计一套深度定制的语法主题。主题围绕仓颉「本源归一」「无界创世」的核心理念，采用「科技感+宇宙法则」的视觉语言，针对仓颉独特的语法元素（法则、宇宙、载体、本质等）优化色彩映射，确保代码层次清晰、视觉舒适，适配长时间开发场景。

## 主题核心设计理念
1. **本源归一**：以「靛蓝 #4F46E5」为核心主色，象征归一法则的统一性，所有语法色彩均围绕主色衍生，保持视觉一致性；
2. **无界层次**：通过「冷色基调+暖色强调」的色彩体系，区分代码结构（冷色）与核心语义（暖色），呼应「载体无界、宇宙分层」的设计；
3. **语义关联**：仓颉专属语法元素（法则、宇宙、载体等）采用独特且易记忆的色彩，形成「语义-色彩」强关联，降低认知成本；
4. **舒适适配**：支持 Zed 深色/浅色/高对比度三种模式，优化文字对比度（符合 WCAG 2.1 AA 标准），减少长时间编码疲劳。

## 主题结构（遵循 Zed 主题规范）
```
cangjie-theme/
├── Cargo.toml                # 扩展配置（依赖、元信息）
├── LICENSE                   # 许可证（MIT，与 Zed 一致）
├── README.md                 # 主题说明文档
├── themes/                   # 主题资源目录
│   ├── cangjie-dark.toml     # 深色模式主题配置
│   ├── cangjie-light.toml    # 浅色模式主题配置
│   └── cangjie-high-contrast.toml  # 高对比度模式配置
└── src/
    ├── lib.rs                # 主题注册逻辑
    └── theme.rs              # 主题元数据定义（名称、作者、支持模式等）
```

## 核心色彩体系
### 1. 基础色彩（跨模式通用逻辑）
| 色彩类型       | 深色模式       | 浅色模式       | 高对比度模式   | 作用说明                     |
|----------------|----------------|----------------|----------------|------------------------------|
| 背景色         | #0F172A（深蓝灰） | #FAFAFA（近白） | #000000（纯黑） | 代码编辑区背景，降低视觉疲劳 |
| 前景色         | #E2E8F0（浅灰） | #1E293B（深灰） | #FFFFFF（纯白） | 普通文本、注释等非核心内容   |
| 主色（核心）   | #4F46E5（靛蓝） | #4338CA（深靛蓝） | #818CF8（亮靛蓝） | 关键字、核心语法元素         |
| 辅助色1（逻辑） | #06B6D4（青蓝） | #0891B2（深青蓝） | #22D3EE（亮青蓝） | 逻辑运算符、条件语句         |
| 辅助色2（数据） | #10B981（翠绿） | #059669（深翠绿） | #34D399（亮翠绿） | 数据类型、变量、常量         |
| 强调色1（宇宙） | #F59E0B（琥珀） | #D97706（深琥珀） | #FBBF24（亮琥珀） | 宇宙、实例相关语法元素       |
| 强调色2（法则） | #EC4899（粉红） | #BE185D（深粉红） | #F472B6（亮粉红） | 法则、约束相关语法元素       |
| 警告色         | #F87171（红色） | #DC2626（深红） | #EF4444（亮红） | 错误、警告提示               |
| 边框/分割线    | #1E293B（深灰） | #E2E8F0（浅灰） | #374151（中灰） | 面板分割、代码块边框         |

### 2. 仓颉专属语法元素色彩映射
针对仓颉独特的语法元素，设计专属色彩映射，强化语义识别：
| 仓颉语法元素 | 关联色彩       | 深色模式       | 浅色模式       | 高对比度模式   |
|--------------|----------------|----------------|----------------|----------------|
| 法则定义（`law`） | 强调色2（法则） | #EC4899（粉红） | #BE185D（深粉红） | #F472B6（亮粉红） |
| 宇宙定义（`cosmos`） | 强调色1（宇宙） | #F59E0B（琥珀） | #D97706（深琥珀） | #FBBF24（亮琥珀） |
| 载体定义（`carrier`） | 辅助色1（逻辑） | #06B6D4（青蓝） | #0891B2（深青蓝） | #22D3EE（亮青蓝） |
| 本质定义（`essence`） | 辅助色2（数据） | #10B981（翠绿） | #059669（深翠绿） | #34D399（亮翠绿） |
| 归一法则（`unified_law`） | 主色+渐变 | #4F46E5（靛蓝） | #4338CA（深靛蓝） | #818CF8（亮靛蓝） |
| 约束条件（`constraint`） | 警告色（弱化） | #F8717180（半透红） | #DC262680（半透深红） | #EF4444（亮红） |
| 演化参数（`evolution_param`） | 主色（浅化） | #818CF8（浅靛蓝） | #6366F1（浅靛蓝） | #A5B4FC（超浅靛蓝） |

## 核心实现代码
### 1. Cargo.toml（扩展配置）
```toml
[package]
name = "cangjie-theme"
version = "0.1.0"
edition = "2021"
description = "Cangjie Lang 专属语法主题 for Zed Editor，融合「本源归一」「无界创世」设计理念"
authors = ["Cangjie Lang Team <team@cangjie-lang.org>"]
license = "MIT"
repository = "https://github.com/your-username/cangjie-theme"

[dependencies]
zed_extension_api = "0.100.0"  # 适配 Zed 最新 API 版本
serde = { version = "1.0", features = ["derive"] }
once_cell = "1.18.0"
```

### 2. src/theme.rs（主题元数据）
```rust
//! 仓颉语法主题元数据定义
use zed_extension_api::{Theme, ThemeMode, ThemePalette, ThemeSyntax};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// 仓颉主题元数据（统一管理三个模式的主题配置）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CangjieTheme {
    /// 深色模式主题
    dark: Theme,
    /// 浅色模式主题
    light: Theme,
    /// 高对比度模式主题
    high_contrast: Theme,
}

impl CangjieTheme {
    /// 加载并初始化所有模式主题
    pub fn load() -> Self {
        Self {
            dark: Self::load_theme("themes/cangjie-dark.toml"),
            light: Self::load_theme("themes/cangjie-light.toml"),
            high_contrast: Self::load_theme("themes/cangjie-high-contrast.toml"),
        }
    }

    /// 从 TOML 文件加载主题配置
    fn load_theme(path: &str) -> Theme {
        let theme_content = std::fs::read_to_string(path)
            .expect(&format!("Failed to read theme file: {}", path));
        toml::from_str(&theme_content)
            .expect(&format!("Failed to parse theme file: {}", path))
    }

    /// 根据主题模式获取对应的主题
    pub fn get_theme(&self, mode: ThemeMode) -> &Theme {
        match mode {
            ThemeMode::Dark => &self.dark,
            ThemeMode::Light => &self.light,
            ThemeMode::HighContrast => &self.high_contrast,
        }
    }
}

/// 扩展 Zed Theme 结构体，添加仓颉专属语法高亮配置
impl Theme {
    /// 配置仓颉专属语法高亮（覆盖默认语法映射）
    pub fn configure_cangjie_syntax(&mut self) {
        // 确保 syntax 字段存在
        if self.syntax.is_none() {
            self.syntax = Some(ThemeSyntax::default());
        }
        let syntax = self.syntax.as_mut().unwrap();

        // 仓颉专属语法元素映射（需与 Tree-sitter 语法定义的 scope 对应）
        let cangjie_scopes = HashMap::from([
            // 核心关键字
            ("keyword.control.cangjie".to_string(), self.palette.as_ref().unwrap().primary.clone()),
            ("keyword.operator.logical.cangjie".to_string(), self.palette.as_ref().unwrap().secondary.clone()),
            ("keyword.operator.arithmetic.cangjie".to_string(), self.palette.as_ref().unwrap().secondary.clone()),
            
            // 仓颉专属语法元素
            ("entity.name.type.law.cangjie".to_string(), self.palette.as_ref().unwrap().accent2.clone()), // 法则类型
            ("entity.name.type.cosmos.cangjie".to_string(), self.palette.as_ref().unwrap().accent1.clone()), // 宇宙类型
            ("entity.name.type.carrier.cangjie".to_string(), self.palette.as_ref().unwrap().secondary.clone()), // 载体类型
            ("entity.name.type.essence.cangjie".to_string(), self.palette.as_ref().unwrap().tertiary.clone()), // 本质类型
            ("entity.name.function.unified-law.cangjie".to_string(), self.palette.as_ref().unwrap().primary.clone()), // 归一法则函数
            ("meta.constraint.cangjie".to_string(), self.palette.as_ref().unwrap().warning.clone()), // 约束条件
            ("variable.other.evolution-param.cangjie".to_string(), self.palette.as_ref().unwrap().primary.clone().lighten(0.2)), // 演化参数
            
            // 字符串、数字、注释
            ("string.quoted.double.cangjie".to_string(), self.palette.as_ref().unwrap().tertiary.clone()),
            ("constant.numeric.cangjie".to_string(), self.palette.as_ref().unwrap().tertiary.clone()),
            ("comment.line.double-slash.cangjie".to_string(), self.palette.as_ref().unwrap().comment.clone()),
            ("comment.block.cangjie".to_string(), self.palette.as_ref().unwrap().comment.clone()),
            
            // 函数、变量、结构体
            ("entity.name.function.cangjie".to_string(), self.palette.as_ref().unwrap().primary.clone().lighten(0.1)),
            ("variable.name.cangjie".to_string(), self.palette.as_ref().unwrap().foreground.clone()),
            ("entity.name.type.struct.cangjie".to_string(), self.palette.as_ref().unwrap().tertiary.clone().darken(0.1)),
        ]);

        // 合并仓颉专属语法映射到主题中
        if let Some(existing_scopes) = &mut syntax.scopes {
            existing_scopes.extend(cangjie_scopes);
        } else {
            syntax.scopes = Some(cangjie_scopes);
        }

        // 配置代码块高亮（选中、当前行、匹配括号等）
        syntax.selection = Some(self.palette.as_ref().unwrap().primary.clone().alpha(0.1));
        syntax.line_highlight = Some(self.palette.as_ref().unwrap().background.clone().lighten(0.05));
        syntax.matching_bracket = Some(self.palette.as_ref().unwrap().primary.clone().lighten(0.3));
        syntax.non_matching_bracket = Some(self.palette.as_ref().unwrap().warning.clone());
    }
}

/// 扩展 ThemePalette，添加色彩明暗调整方法（简化主题配置）
trait PaletteColorExt {
    fn lighten(&self, amount: f32) -> String;
    fn darken(&self, amount: f32) -> String;
    fn alpha(&self, alpha: f32) -> String;
}

impl PaletteColorExt for String {
    /// 提亮颜色（amount: 0.0-1.0）
    fn lighten(&self, amount: f32) -> String {
        self.adjust_rgb(amount)
    }

    /// 加深颜色（amount: 0.0-1.0）
    fn darken(&self, amount: f32) -> String {
        self.adjust_rgb(-amount)
    }

    /// 设置颜色透明度（alpha: 0.0-1.0）
    fn alpha(&self, alpha: f32) -> String {
        let rgba = self.rgb_to_rgba();
        format!("rgba({}, {}, {}, {:.2})", rgba.0, rgba.1, rgba.2, alpha)
    }

    /// 辅助方法：RGB 颜色调整
    fn adjust_rgb(&self, amount: f32) -> String {
        let (r, g, b) = self.rgb_to_rgba();
        let r = (r as f32 + amount * 255.0).clamp(0.0, 255.0) as u8;
        let g = (g as f32 + amount * 255.0).clamp(0.0, 255.0) as u8;
        let b = (b as f32 + amount * 255.0).clamp(0.0, 255.0) as u8;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// 辅助方法：十六进制颜色转 RGB
    fn rgb_to_rgba(&self) -> (u8, u8, u8) {
        let hex = self.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        (r, g, b)
    }
}
```

### 3. src/lib.rs（主题注册逻辑）
```rust
//! 仓颉语法主题扩展入口
use zed_extension_api::{self as zed, ThemeProvider, Result, ThemeMode, Theme as ZedTheme};
use once_cell::sync::Lazy;
use crate::theme::CangjieTheme;

/// 全局主题实例（加载所有模式配置）
static CANGJIE_THEME: Lazy<CangjieTheme> = Lazy::new(CangjieTheme::load);

/// 仓颉主题提供器
struct CangjieThemeProvider;

impl ThemeProvider for CangjieThemeProvider {
    /// 获取主题列表（返回三个模式的主题）
    fn themes(&self) -> Vec<&dyn zed::Theme> {
        vec![
            CANGJIE_THEME.get_theme(ThemeMode::Dark),
            CANGJIE_THEME.get_theme(ThemeMode::Light),
            CANGJIE_THEME.get_theme(ThemeMode::HighContrast),
        ]
    }

    /// 根据主题 ID 和模式获取主题
    fn theme(&self, theme_id: &str, mode: ThemeMode) -> Option<&dyn zed::Theme> {
        match theme_id {
            "cangjie-dark" => Some(CANGJIE_THEME.get_theme(ThemeMode::Dark)),
            "cangjie-light" => Some(CANGJIE_THEME.get_theme(ThemeMode::Light)),
            "cangjie-high-contrast" => Some(CANGJIE_THEME.get_theme(ThemeMode::HighContrast)),
            _ => None,
        }
    }
}

/// 扩展入口：注册仓颉主题
#[zed::extension]
fn activate(_: &zed::Workspace) -> Result<Box<dyn zed::Extension>> {
    // 初始化主题并配置仓颉专属语法高亮
    let mut dark_theme = CANGJIE_THEME.dark.clone();
    let mut light_theme = CANGJIE_THEME.light.clone();
    let mut high_contrast_theme = CANGJIE_THEME.high_contrast.clone();
    
    dark_theme.configure_cangjie_syntax();
    light_theme.configure_cangjie_syntax();
    high_contrast_theme.configure_cangjie_syntax();
    
    // 重新初始化全局主题（应用语法配置）
    *CANGJIE_THEME.lock().unwrap() = CangjieTheme {
        dark: dark_theme,
        light: light_theme,
        high_contrast: high_contrast_theme,
    };

    Ok(Box::new(CangjieThemeProvider))
}

/// 实现 Zed Theme trait（适配扩展 API）
impl zed::Theme for crate::theme::Theme {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn author(&self) -> &str {
        &self.author
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn palette(&self) -> Option<&zed::ThemePalette> {
        self.palette.as_ref().map(|palette| {
            &palette as &dyn zed::ThemePalette
        })
    }

    fn syntax(&self) -> Option<&zed::ThemeSyntax> {
        self.syntax.as_ref().map(|syntax| {
            &syntax as &dyn zed::ThemeSyntax
        })
    }

    fn ui(&self) -> Option<&zed::ThemeUi> {
        self.ui.as_ref().map(|ui| {
            &ui as &dyn zed::ThemeUi
        })
    }

    fn mode(&self) -> zed::ThemeMode {
        self.mode
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn license(&self) -> &str {
        &self.license
    }
}

/// 实现 Zed ThemePalette trait
impl zed::ThemePalette for crate::theme::ThemePalette {
    fn background(&self) -> &str {
        &self.background
    }

    fn foreground(&self) -> &str {
        &self.foreground
    }

    fn primary(&self) -> &str {
        &self.primary
    }

    fn secondary(&self) -> &str {
        &self.secondary
    }

    fn tertiary(&self) -> &str {
        &self.tertiary
    }

    fn accent1(&self) -> &str {
        &self.accent1
    }

    fn accent2(&self) -> &str {
        &self.accent2
    }

    fn success(&self) -> &str {
        &self.success
    }

    fn warning(&self) -> &str {
        &self.warning
    }

    fn error(&self) -> &str {
        &self.error
    }

    fn comment(&self) -> &str {
        &self.comment
    }
}

/// 实现 Zed ThemeSyntax trait
impl zed::ThemeSyntax for crate::theme::ThemeSyntax {
    fn scopes(&self) -> Option<&std::collections::HashMap<String, String>> {
        self.scopes.as_ref()
    }

    fn selection(&self) -> Option<&str> {
        self.selection.as_deref()
    }

    fn line_highlight(&self) -> Option<&str> {
        self.line_highlight.as_deref()
    }

    fn matching_bracket(&self) -> Option<&str> {
        self.matching_bracket.as_deref()
    }

    fn non_matching_bracket(&self) -> Option<&str> {
        self.non_matching_bracket.as_deref()
    }

    fn cursor(&self) -> Option<&str> {
        self.cursor.as_deref()
    }

    fn cursor_line(&self) -> Option<&str> {
        self.cursor_line.as_deref()
    }
}

/// 实现 Zed ThemeUi trait
impl zed::ThemeUi for crate::theme::ThemeUi {
    fn background(&self) -> &str {
        &self.background
    }

    fn foreground(&self) -> &str {
        &self.foreground
    }

    fn border(&self) -> &str {
        &self.border
    }

    fn hover(&self) -> &str {
        &self.hover
    }

    fn active(&self) -> &str {
        &self.active
    }

    fn disabled(&self) -> &str {
        &self.disabled
    }

    fn input_background(&self) -> &str {
        &self.input_background
    }

    fn input_foreground(&self) -> &str {
        &self.input_foreground
    }

    fn panel_background(&self) -> &str {
        &self.panel_background
    }

    fn panel_foreground(&self) -> &str {
        &self.panel_foreground
    }

    fn sidebar_background(&self) -> &str {
        &self.sidebar_background
    }

    fn sidebar_foreground(&self) -> &str {
        &self.sidebar_foreground
    }
}
```

## 主题配置文件（TOML）
### 1. themes/cangjie-dark.toml（深色模式）
```toml
id = "cangjie-dark"
name = "Cangjie Dark"
author = "Cangjie Lang Team"
description = "仓颉语言专属深色主题，以靛蓝为主色，适配无界创世理念"
mode = "Dark"
version = "0.1.0"
license = "MIT"

[palette]
background = "#0F172A"    # 深蓝灰背景
foreground = "#E2E8F0"    # 浅灰前景
primary = "#4F46E5"       # 靛蓝主色（核心语法）
secondary = "#06B6D4"     # 青蓝辅助色（逻辑）
tertiary = "#10B981"      # 翠绿辅助色（数据）
accent1 = "#F59E0B"       # 琥珀强调色（宇宙）
accent2 = "#EC4899"       # 粉红强调色（法则）
success = "#10B981"       # 成功色
warning = "#F8717180"     # 警告色（半透明）
error = "#F87171"         # 错误色
comment = "#94A3B8"       # 注释色

[syntax]
selection = "#4F46E520"   # 选中背景（主色半透明）
line_highlight = "#1E293B"# 当前行高亮
matching_bracket = "#818CF8" # 匹配括号颜色
non_matching_bracket = "#F87171" # 不匹配括号颜色
cursor = "#4F46E5"        # 光标颜色（主色）
cursor_line = "#1E293B80" # 光标行背景（半透明）

[ui]
background = "#0F172A"    # UI 背景
foreground = "#E2E8F0"    # UI 前景
border = "#1E293B"        # 边框颜色
hover = "#1E293B"         # 悬停背景
active = "#4F46E520"      # 激活状态背景
disabled = "#64748B"      # 禁用状态前景
input_background = "#1E293B" # 输入框背景
input_foreground = "#E2E8F0" # 输入框前景
panel_background = "#1E293B" # 面板背景
panel_foreground = "#E2E8F0" # 面板前景
sidebar_background = "#1E293B" # 侧边栏背景
sidebar_foreground = "#E2E8F0" # 侧边栏前景
```

### 2. themes/cangjie-light.toml（浅色模式）
```toml
id = "cangjie-light"
name = "Cangjie Light"
author = "Cangjie Lang Team"
description = "仓颉语言专属浅色主题，低饱和度色彩，降低视觉疲劳"
mode = "Light"
version = "0.1.0"
license = "MIT"

[palette]
background = "#FAFAFA"    # 近白背景
foreground = "#1E293B"    # 深灰前景
primary = "#4338CA"       # 深靛蓝主色
secondary = "#0891B2"     # 深青蓝辅助色
tertiary = "#059669"      # 深翠绿辅助色
accent1 = "#D97706"       # 深琥珀强调色
accent2 = "#BE185D"       # 深粉红强调色
success = "#059669"       # 成功色
warning = "#DC262680"     # 警告色（半透明）
error = "#DC2626"         # 错误色
comment = "#64748B"       # 注释色

[syntax]
selection = "#4338CA20"   # 选中背景（主色半透明）
line_highlight = "#F1F5F9"# 当前行高亮
matching_bracket = "#6366F1" # 匹配括号颜色
non_matching_bracket = "#DC2626" # 不匹配括号颜色
cursor = "#4338CA"        # 光标颜色（主色）
cursor_line = "#F1F5F980" # 光标行背景（半透明）

[ui]
background = "#FAFAFA"    # UI 背景
foreground = "#1E293B"    # UI 前景
border = "#E2E8F0"        # 边框颜色
hover = "#F1F5F9"         # 悬停背景
active = "#4338CA20"      # 激活状态背景
disabled = "#94A3B8"      # 禁用状态前景
input_background = "#F1F5F9" # 输入框背景
input_foreground = "#1E293B" # 输入框前景
panel_background = "#F1F5F9" # 面板背景
panel_foreground = "#1E293B" # 面板前景
sidebar_background = "#F1F5F9" # 侧边栏背景
sidebar_foreground = "#1E293B" # 侧边栏前景
```

### 3. themes/cangjie-high-contrast.toml（高对比度模式）
```toml
id = "cangjie-high-contrast"
name = "Cangjie High Contrast"
author = "Cangjie Lang Team"
description = "仓颉语言专属高对比度主题，适配视力障碍开发者"
mode = "HighContrast"
version = "0.1.0"
license = "MIT"

[palette]
background = "#000000"    # 纯黑背景
foreground = "#FFFFFF"    # 纯白前景
primary = "#818CF8"       # 亮靛蓝主色
secondary = "#22D3EE"     # 亮青蓝辅助色
tertiary = "#34D399"      # 亮翠绿辅助色
accent1 = "#FBBF24"       # 亮琥珀强调色
accent2 = "#F472B6"       # 亮粉红强调色
success = "#34D399"       # 成功色
warning = "#EF4444"       # 警告色（不透明）
error = "#EF4444"         # 错误色
comment = "#94A3B8"       # 注释色

[syntax]
selection = "#818CF830"   # 选中背景（主色半透明）
line_highlight = "#1E293B"# 当前行高亮
matching_bracket = "#A5B4FC" # 匹配括号颜色
non_matching_bracket = "#EF4444" # 不匹配括号颜色
cursor = "#818CF8"        # 光标颜色（主色）
cursor_line = "#1E293B"   # 光标行背景（不透明）

[ui]
background = "#000000"    # UI 背景
foreground = "#FFFFFF"    # UI 前景
border = "#374151"        # 边框颜色
hover = "#1E293B"         # 悬停背景
active = "#818CF830"      # 激活状态背景
disabled = "#64748B"      # 禁用状态前景
input_background = "#1E293B" # 输入框背景
input_foreground = "#FFFFFF" # 输入框前景
panel_background = "#1E293B" # 面板背景
panel_foreground = "#FFFFFF" # 面板前景
sidebar_background = "#1E293B" # 侧边栏背景
sidebar_foreground = "#FFFFFF" # 侧边栏前景
```

## 安装与使用
### 1. 本地开发安装
1. 克隆本仓库：`git clone https://github.com/your-username/cangjie-theme.git`
2. 进入目录：`cd cangjie-theme`
3. 构建扩展：`cargo build --release`
4. 在 Zed 中启用：
   - 打开 Zed → 偏好设置（Settings）→ 扩展（Extensions）
   - 点击「Install from Path」，选择 `target/release` 目录下的 `cangjie_theme.zedextension` 文件
   - 切换主题：偏好设置 → 外观（Appearance）→ 语法主题（Syntax Theme）→ 选择对应模式的 Cangjie 主题

### 2. 发布到 Zed 扩展市场
1. 遵循 Zed 扩展发布规范：https://zed.dev/docs/extensions/publishing
2. 打包扩展：`zed extension package`
3. 提交到 Zed 扩展市场：`zed extension publish`

## 语法适配说明
### 1. Tree-sitter 语法 scope 匹配
主题的语法高亮依赖仓颉语言的 Tree-sitter 语法定义，需确保以下 `scope` 已在 Tree-sitter 中定义：
- `keyword.control.cangjie`：控制关键字（`if`、`else`、`for` 等）
- `entity.name.type.law.cangjie`：法则类型定义（`law` 关键字关联的类型）
- `entity.name.type.cosmos.cangjie`：宇宙类型定义（`cosmos` 关键字关联的类型）
- `meta.constraint.cangjie`：约束条件（`constraint` 关键字关联的代码块）
- `variable.other.evolution-param.cangjie`：演化参数（`evolution_param` 关键字关联的变量）

### 2. 自定义语法扩展
若仓颉语言新增语法元素，可通过以下步骤扩展主题：
1. 在 Tree-sitter 语法定义中添加对应的 `scope`；
2. 在 `Theme::configure_cangjie_syntax` 方法中添加 `scope` 与色彩的映射；
3. （可选）在 TOML 配置文件中新增专属色彩变量。

## 兼容性说明
- 支持 Zed 版本：v0.100.0+（与 `zed_extension_api` 版本匹配）
- 适配系统：macOS、Linux、Windows（Zed 支持的所有平台）
- 语法兼容性：需配合仓颉语言的 Tree-sitter 语法插件使用
- 可访问性：符合 WCAG 2.1 AA 标准，支持屏幕阅读器

## 许可证
本主题采用 MIT 许可证（与 Zed 一致），允许自由使用、修改和分发，详见 [LICENSE](LICENSE) 文件。

---

**主题版本**：v0.1.0  
**发布日期**：2025-11-09  
**适配语言**：仓颉编程语言（Cangjie Lang）  
**支持编辑器**：Zed v0.100.0+  
**官方仓库**：https://github.com/your-username/cangjie-theme  
**反馈渠道**：https://github.com/your-username/cangjie-theme/issues