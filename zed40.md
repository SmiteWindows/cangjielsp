# Cangjie Zed 扩展整合方案（cangjie-zed-extension）
将语法主题（cangjie-theme）、图标主题（cangjie-icon-theme）、调试器扩展整合为统一项目，遵循 Zed 扩展规范，保持模块解耦且结构清晰。以下是完整文件目录及分模块代码实现。

## 一、整合后文件目录
```
cangjie-zed-extension/
├── Cargo.toml                # 项目总配置（依赖、元信息）
├── LICENSE                   # MIT 许可证
├── README.md                 # 扩展说明文档
├── schemas/                  # 调试配置 JSON Schema
│   └── cangjie-debug-schema.json
├── icons/                    # 图标资源目录
│   ├── dark/                 # 深色模式图标
│   │   ├── file-types/       # 文件类型图标
│   │   ├── syntax/           # 语法元素图标
│   │   ├── project/          # 项目资源图标
│   │   └── ui/               # UI 图标（含调试相关）
│   └── light/                # 浅色模式图标（结构与 dark 一致）
├── themes/                   # 语法主题配置
│   ├── cangjie-dark.toml     # 深色模式语法主题
│   ├── cangjie-light.toml    # 浅色模式语法主题
│   └── cangjie-high-contrast.toml  # 高对比度主题
└── src/
    ├── lib.rs                # 扩展入口（注册所有能力）
    ├── icon_theme/           # 图标主题模块
    │   ├── mod.rs
    │   ├── icon_map.rs       # 图标映射配置
    │   └── theme.rs          # 图标主题元数据
    ├── syntax_theme/         # 语法主题模块
    │   ├── mod.rs
    │   ├── theme.rs          # 语法主题元数据与配置
    │   └── palette.rs        # 色彩体系与扩展方法
    └── debugger/             # 调试器模块
        ├── mod.rs
        ├── adapter.rs        # DAP 调试适配器
        ├── config.rs         # 调试配置定义
        ├── breakpoint.rs     # 断点管理
        ├── debugger.rs       # 调试器主逻辑
        └── cosmos_inspector.rs  # 宇宙实例检查器
```

## 二、核心配置文件（第一部分）
### 1. Cargo.toml（总配置）
```toml
[package]
name = "cangjie-zed-extension"
version = "0.3.0"
edition = "2021"
description = "仓颉语言 Zed 全能扩展：语法主题 + 图标主题 + 专属调试器"
authors = ["Cangjie Lang Team <team@cangjie-lang.org>"]
license = "MIT"
repository = "https://github.com/cangjie-lang/cangjie-zed-extension"

[dependencies]
# 基础依赖
zed_extension_api = "0.100.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
once_cell = "1.18.0"
thiserror = "1.0"

# 调试器依赖
dap = "0.10.0"
tokio = { version = "1.0", features = ["full"] }
uuid = "1.0"

[package.metadata.zed]
display_name = "Cangjie Lang Extension"
categories = ["Themes", "Icons", "Debuggers", "Programming Languages"]
keywords = ["cangjie", "仓颉", "theme", "icon-theme", "debugger"]
```

### 2. schemas/cangjie-debug-schema.json（调试配置 Schema）
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Cangjie Debug Configuration",
  "type": "object",
  "required": ["type", "name", "request", "cosmos_file", "cosmos_type", "debug_mode"],
  "properties": {
    "type": {
      "type": "string",
      "const": "cangjie",
      "description": "调试类型（固定为 'cangjie'）"
    },
    "name": {
      "type": "string",
      "description": "调试配置名称（显示在 Zed 调试面板）"
    },
    "request": {
      "type": "string",
      "enum": ["launch", "attach"],
      "description": "调试请求类型（launch=启动新宇宙，attach=附加到运行中的宇宙）"
    },
    "cosmos_file": {
      "type": "string",
      "format": "uri",
      "description": "目标宇宙文件路径（.cosmos 文件）"
    },
    "cosmos_type": {
      "type": "string",
      "enum": ["DigitalCosmos", "QuantumCosmos", "ConsciousnessCosmos", "DimensionalCosmos"],
      "description": "宇宙类型"
    },
    "debug_mode": {
      "type": "string",
      "enum": ["CosmosEvolution", "LawValidation", "CarrierMigration"],
      "description": "调试模式"
    },
    "migrate_config": {
      "type": "object",
      "required": ["source_carrier_id", "target_carrier_id", "migrate_breakpoints"],
      "properties": {
        "source_carrier_id": { "type": "string" },
        "target_carrier_id": { "type": "string" },
        "migrate_breakpoints": {
          "type": "array",
          "items": {
            "type": "string",
            "enum": ["CosmosSerialization", "CarrierAdaptation", "CosmosRecovery"]
          }
        }
      },
      "description": "跨载体迁移调试配置（仅 debug_mode=CarrierMigration 时生效）"
    },
    "law_validation_threshold": {
      "type": "number",
      "minimum": 0.0,
      "maximum": 1.0,
      "default": 0.95,
      "description": "法则一致性校验阈值（仅 debug_mode=LawValidation 时生效）"
    },
    "step_interval": {
      "type": "integer",
      "minimum": 10,
      "default": 100,
      "description": "演化步进间隔（毫秒）"
    }
  }
}
```

### 3. themes/cangjie-dark.toml（深色模式语法主题）
```toml
id = "cangjie-dark"
name = "Cangjie Dark"
author = "Cangjie Lang Team"
description = "仓颉语言专属深色主题，以靛蓝为主色，适配无界创世理念"
mode = "Dark"
version = "0.3.0"
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

### 4. themes/cangjie-light.toml（浅色模式语法主题）
```toml
id = "cangjie-light"
name = "Cangjie Light"
author = "Cangjie Lang Team"
description = "仓颉语言专属浅色主题，低饱和度色彩，降低视觉疲劳"
mode = "Light"
version = "0.3.0"
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

### 5. themes/cangjie-high-contrast.toml（高对比度主题）
```toml
id = "cangjie-high-contrast"
name = "Cangjie High Contrast"
author = "Cangjie Lang Team"
description = "仓颉语言专属高对比度主题，适配视力障碍开发者"
mode = "HighContrast"
version = "0.3.0"
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

## 三、图标主题模块（第二部分）
### 1. src/icon_theme/mod.rs
```rust
//! 仓颉图标主题模块
pub mod icon_map;
pub mod theme;

pub use icon_map::{CangjieIconId, FILE_TYPE_ICON_MAP, SYNTAX_ICON_MAP, FOLDER_ICON_MAP, UI_COMMAND_ICON_MAP, DEBUG_COMMAND_ICON_MAP};
pub use theme::CangjieIconTheme;
```

### 2. src/icon_theme/theme.rs
```rust
//! 仓颉图标主题元数据定义
use zed_extension_api::ThemeMode;
use serde::{Serialize, Deserialize};

/// 仓颉图标主题元数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CangjieIconTheme {
    /// 主题名称（显示在 Zed 主题选择器中）
    pub name: String,
    /// 主题标识（唯一，用于 Zed 内部识别）
    pub id: String,
    /// 作者信息
    pub author: String,
    /// 主题描述
    pub description: String,
    /// 支持的模式（深色/浅色）
    pub supported_modes: Vec<ThemeMode>,
    /// 主题版本
    pub version: String,
    /// 许可证
    pub license: String,
}

impl Default for CangjieIconTheme {
    fn default() -> Self {
        Self {
            name: "Cangjie Icon Theme".to_string(),
            id: "cangjie-icon-theme".to_string(),
            author: "Cangjie Lang Team".to_string(),
            description: "专属仓颉编程语言的图标主题，融合「归一法则」「无界创世」设计理念，适配 Zed 深色/浅色模式".to_string(),
            supported_modes: vec![ThemeMode::Dark, ThemeMode::Light],
            version: "0.3.0".to_string(),
            license: "MIT".to_string(),
        }
    }
}

/// 实现 Zed 的 IconTheme trait 以适配扩展 API
impl zed_extension_api::IconTheme for CangjieIconTheme {
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

    fn supported_modes(&self) -> &[zed_extension_api::ThemeMode] {
        &self.supported_modes
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn license(&self) -> &str {
        &self.license
    }
}
```

### 3. src/icon_theme/icon_map.rs
```rust
//! 图标映射：关联文件类型/语法元素/调试命令与图标路径
use zed_extension_api::{FileType, SyntaxKind, IconId};
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// 图标 ID 定义（与 icons/ 目录下的文件对应）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CangjieIconId {
    // 文件类型图标
    CangjieSourceFile,       // 仓颉源文件（.cangjie）
    CangjieConfigFile,       // 仓颉配置文件（.cangjie.toml）
    CangjieLawFile,          // 法则定义文件（.cosmic.law）
    CangjieCosmosFile,       // 宇宙实例文件（.cosmos）
    CangjieEssenceFile,      // 本质定义文件（.essence）

    // 语法元素图标
    CangjieFunction,         // 函数
    CangjieStruct,           // 结构体
    CangjieEnum,             // 枚举
    CangjieTrait,            // 特质
    CangjieLaw,              // 法则
    CangjieCosmos,           // 宇宙
    CangjieEssence,          // 本质
    CangjieCarrier,          // 载体
    CangjieParam,            // 参数
    CangjieConstant,         // 常量

    // 项目资源图标
    CangjieFolder,           // 普通文件夹
    CangjieLawFolder,        // 法则文件夹
    CangjieCosmosFolder,     // 宇宙文件夹
    CangjieCarrierFolder,    // 载体文件夹
    CangjieEssenceFolder,    // 本质文件夹
    CangjieTestFolder,       // 测试文件夹
    CangjieDocFolder,        // 文档文件夹

    // 基础 UI 图标
    RunCosmos,               // 运行宇宙实例
    DebugCosmos,             // 调试宇宙实例
    AnalyzeLaw,              // 分析法则一致性
    MigrateCarrier,          // 跨载体迁移
    ObserveEvolution,        // 观测宇宙演化

    // 调试专属 UI 图标
    InspectCosmos,           // 检查宇宙状态
    LawValidation,           // 法则一致性校验
    MigrateDebug,            // 跨载体迁移调试
    BreakpointLaw,           // 法则断点
    BreakpointEvolution,     // 演化断点
}

impl From<CangjieIconId> for IconId {
    fn from(id: CangjieIconId) -> Self {
        IconId::new("cangjie-zed-extension", &id.to_string())
    }
}

impl ToString for CangjieIconId {
    fn to_string(&self) -> String {
        match self {
            // 文件类型
            Self::CangjieSourceFile => "file-types/cangjie-source",
            Self::CangjieConfigFile => "file-types/cangjie-config",
            Self::CangjieLawFile => "file-types/cangjie-law",
            Self::CangjieCosmosFile => "file-types/cangjie-cosmos",
            Self::CangjieEssenceFile => "file-types/cangjie-essence",

            // 语法元素
            Self::CangjieFunction => "syntax/function",
            Self::CangjieStruct => "syntax/struct",
            Self::CangjieEnum => "syntax/enum",
            Self::CangjieTrait => "syntax/trait",
            Self::CangjieLaw => "syntax/law",
            Self::CangjieCosmos => "syntax/cosmos",
            Self::CangjieEssence => "syntax/essence",
            Self::CangjieCarrier => "syntax/carrier",
            Self::CangjieParam => "syntax/param",
            Self::CangjieConstant => "syntax/constant",

            // 项目资源
            Self::CangjieFolder => "project/folder",
            Self::CangjieLawFolder => "project/folder-law",
            Self::CangjieCosmosFolder => "project/folder-cosmos",
            Self::CangjieCarrierFolder => "project/folder-carrier",
            Self::CangjieEssenceFolder => "project/folder-essence",
            Self::CangjieTestFolder => "project/folder-test",
            Self::CangjieDocFolder => "project/folder-doc",

            // 基础 UI 图标
            Self::RunCosmos => "ui/run-cosmos",
            Self::DebugCosmos => "ui/debug-cosmos",
            Self::AnalyzeLaw => "ui/analyze-law",
            Self::MigrateCarrier => "ui/migrate-carrier",
            Self::ObserveEvolution => "ui/observe-evolution",

            // 调试专属 UI 图标
            Self::InspectCosmos => "ui/inspect-cosmos",
            Self::LawValidation => "ui/law-validation",
            Self::MigrateDebug => "ui/migrate-debug",
            Self::BreakpointLaw => "ui/breakpoint-law",
            Self::BreakpointEvolution => "ui/breakpoint-evolution",
        }
    }
}

/// 文件类型 -> 图标映射（核心：让 Zed 识别仓颉文件并显示对应图标）
pub static FILE_TYPE_ICON_MAP: Lazy<HashMap<FileType, IconId>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // 仓颉源文件（.cangjie）
    map.insert(
        FileType::new("cangjie", "Cangjie Source", vec!["cangjie"]),
        CangjieIconId::CangjieSourceFile.into()
    );

    // 仓颉配置文件（.cangjie.toml）
    map.insert(
        FileType::new("cangjie-config", "Cangjie Config", vec!["cangjie.toml"]),
        CangjieIconId::CangjieConfigFile.into()
    );

    // 法则定义文件（.cosmic.law）
    map.insert(
        FileType::new("cangjie-law", "Cangjie Cosmic Law", vec!["cosmic.law"]),
        CangjieIconId::CangjieLawFile.into()
    );

    // 宇宙实例文件（.cosmos）
    map.insert(
        FileType::new("cangjie-cosmos", "Cangjie Cosmos Instance", vec!["cosmos"]),
        CangjieIconId::CangjieCosmosFile.into()
    );

    // 本质定义文件（.essence）
    map.insert(
        FileType::new("cangjie-essence", "Cangjie Essence", vec!["essence"]),
        CangjieIconId::CangjieEssenceFile.into()
    );

    map
});

/// 语法元素 -> 图标映射（适配仓颉语法高亮）
pub static SYNTAX_ICON_MAP: Lazy<HashMap<SyntaxKind, IconId>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // 注：SyntaxKind 需与仓颉语言的 Tree-sitter 语法定义对应
    map.insert(SyntaxKind::new("function_declaration"), CangjieIconId::CangjieFunction.into());
    map.insert(SyntaxKind::new("struct_declaration"), CangjieIconId::CangjieStruct.into());
    map.insert(SyntaxKind::new("enum_declaration"), CangjieIconId::CangjieEnum.into());
    map.insert(SyntaxKind::new("trait_declaration"), CangjieIconId::CangjieTrait.into());
    map.insert(SyntaxKind::new("law_declaration"), CangjieIconId::CangjieLaw.into());
    map.insert(SyntaxKind::new("cosmos_declaration"), CangjieIconId::CangjieCosmos.into());
    map.insert(SyntaxKind::new("essence_declaration"), CangjieIconId::CangjieEssence.into());
    map.insert(SyntaxKind::new("carrier_declaration"), CangjieIconId::CangjieCarrier.into());
    map.insert(SyntaxKind::new("parameter"), CangjieIconId::CangjieParam.into());
    map.insert(SyntaxKind::new("constant"), CangjieIconId::CangjieConstant.into());

    map
});

/// 文件夹名称 -> 图标映射（根据文件夹名称自动匹配图标）
pub static FOLDER_ICON_MAP: Lazy<HashMap<&str, IconId>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert("laws", CangjieIconId::CangjieLawFolder.into());
    map.insert("cosmos", CangjieIconId::CangjieCosmosFolder.into());
    map.insert("carriers", CangjieIconId::CangjieCarrierFolder.into());
    map.insert("essences", CangjieIconId::CangjieEssenceFolder.into());
    map.insert("tests", CangjieIconId::CangjieTestFolder.into());
    map.insert("docs", CangjieIconId::CangjieDocFolder.into());

    map
});

/// 普通 UI 命令 -> 图标映射
pub static UI_COMMAND_ICON_MAP: Lazy<HashMap<&str, IconId>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert("run_cosmos", CangjieIconId::RunCosmos.into());
    map.insert("debug_cosmos", CangjieIconId::DebugCosmos.into());
    map.insert("analyze_law_consistency", CangjieIconId::AnalyzeLaw.into());
    map.insert("migrate_cosmos_carrier", CangjieIconId::MigrateCarrier.into());
    map.insert("observe_cosmos_evolution", CangjieIconId::ObserveEvolution.into());

    map
});

/// 调试命令 -> 图标映射
pub static DEBUG_COMMAND_ICON_MAP: Lazy<HashMap<&str, IconId>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert("inspect_cosmos", CangjieIconId::InspectCosmos.into());
    map.insert("law_validation", CangjieIconId::LawValidation.into());
    map.insert("migrate_debug", CangjieIconId::MigrateDebug.into());
    map.insert("set_law_breakpoint", CangjieIconId::BreakpointLaw.into());
    map.insert("set_evolution_breakpoint", CangjieIconId::BreakpointEvolution.into());

    map
});
```

## 四、语法主题模块（第三部分）
### 1. src/syntax_theme/mod.rs
```rust
//! 仓颉语法主题模块
pub mod theme;
pub mod palette;

pub use theme::{CangjieSyntaxTheme, Theme};
pub use palette::{PaletteColorExt, ThemePalette};
```

### 2. src/syntax_theme/palette.rs
```rust
//! 色彩体系与扩展方法（支持明暗调整、透明度设置）
use serde::{Serialize, Deserialize};

/// 主题调色板（遵循 Zed ThemePalette 规范）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemePalette {
    pub background: String,
    pub foreground: String,
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
    pub accent1: String,
    pub accent2: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub comment: String,
}

/// 色彩扩展方法（明暗调整、透明度）
pub trait PaletteColorExt {
    fn lighten(&self, amount: f32) -> String;
    fn darken(&self, amount: f32) -> String;
    fn alpha(&self, alpha: f32) -> String;
}

impl PaletteColorExt for String {
    /// 提亮颜色（amount: 0.0-1.0，值越大越亮）
    fn lighten(&self, amount: f32) -> String {
        self.adjust_rgb(amount)
    }

    /// 加深颜色（amount: 0.0-1.0，值越大越深）
    fn darken(&self, amount: f32) -> String {
        self.adjust_rgb(-amount)
    }

    /// 设置颜色透明度（alpha: 0.0-1.0，0 完全透明，1 不透明）
    fn alpha(&self, alpha: f32) -> String {
        let (r, g, b) = self.rgb_to_rgba();
        format!("rgba({}, {}, {}, {:.2})", r, g, b, alpha.clamp(0.0, 1.0))
    }

    /// 辅助方法：RGB 颜色数值调整
    fn adjust_rgb(&self, amount: f32) -> String {
        let (r, g, b) = self.rgb_to_rgba();
        let r = (r as f32 + amount * 255.0).clamp(0.0, 255.0) as u8;
        let g = (g as f32 + amount * 255.0).clamp(0.0, 255.0) as u8;
        let b = (b as f32 + amount * 255.0).clamp(0.0, 255.0) as u8;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// 辅助方法：十六进制颜色转 RGB 数值
    fn rgb_to_rgba(&self) -> (u8, u8, u8) {
        let hex = self.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        (r, g, b)
    }
}

/// 实现 Zed 的 ThemePalette trait
impl zed_extension_api::ThemePalette for ThemePalette {
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
```

### 3. src/syntax_theme/theme.rs
```rust
//! 仓颉语法主题元数据与配置
use zed_extension_api::{Theme as ZedTheme, ThemeMode, ThemeSyntax as ZedThemeSyntax, ThemeUi as ZedThemeUi};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::palette::{ThemePalette, PaletteColorExt};

/// 语法高亮配置（遵循 Zed ThemeSyntax 规范）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ThemeSyntax {
    pub scopes: Option<HashMap<String, String>>,
    pub selection: Option<String>,
    pub line_highlight: Option<String>,
    pub matching_bracket: Option<String>,
    pub non_matching_bracket: Option<String>,
    pub cursor: Option<String>,
    pub cursor_line: Option<String>,
}

/// UI 样式配置（遵循 Zed ThemeUi 规范）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ThemeUi {
    pub background: String,
    pub foreground: String,
    pub border: String,
    pub hover: String,
    pub active: String,
    pub disabled: String,
    pub input_background: String,
    pub input_foreground: String,
    pub panel_background: String,
    pub panel_foreground: String,
    pub sidebar_background: String,
    pub sidebar_foreground: String,
}

/// 仓颉语法主题（单模式配置）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub author: String,
    pub description: String,
    pub mode: ThemeMode,
    pub version: String,
    pub license: String,
    pub palette: Option<ThemePalette>,
    pub syntax: Option<ThemeSyntax>,
    pub ui: Option<ThemeUi>,
}

/// 仓颉语法主题管理器（管理所有模式）
pub struct CangjieSyntaxTheme {
    dark: Theme,
    light: Theme,
    high_contrast: Theme,
}

impl CangjieSyntaxTheme {
    /// 加载所有模式主题配置
    pub fn load() -> Self {
        Self {
            dark: Self::load_theme("themes/cangjie-dark.toml"),
            light: Self::load_theme("themes/cangjie-light.toml"),
            high_contrast: Self::load_theme("themes/cangjie-high-contrast.toml"),
        }
    }

    /// 从 TOML 文件加载单个主题
    fn load_theme(path: &str) -> Theme {
        let theme_content = std::fs::read_to_string(path)
            .expect(&format!("Failed to read theme file: {}", path));
        toml::from_str(&theme_content)
            .expect(&format!("Failed to parse theme file: {}", path))
    }

    /// 根据模式获取主题
    pub fn get_theme(&self, mode: ThemeMode) -> &Theme {
        match mode {
            ThemeMode::Dark => &self.dark,
            ThemeMode::Light => &self.light,
            ThemeMode::HighContrast => &self.high_contrast,
        }
    }

    /// 获取可修改的主题实例（用于配置语法高亮）
    pub fn get_theme_mut(&mut self, mode: ThemeMode) -> &mut Theme {
        match mode {
            ThemeMode::Dark => &mut self.dark,
            ThemeMode::Light => &mut self.light,
            ThemeMode::HighContrast => &mut self.high_contrast,
        }
    }
}

impl Theme {
    /// 配置仓颉专属语法高亮（核心：关联语法 scope 与色彩）
    pub fn configure_cangjie_syntax(&mut self) {
        let palette = self.palette.as_ref().expect("Theme palette is required");
        let syntax = self.syntax.get_or_insert_with(ThemeSyntax::default);
        let scopes = syntax.scopes.get_or_insert_with(HashMap::new);

        // 核心关键字
        scopes.insert("keyword.control.cangjie".to_string(), palette.primary.clone());
        scopes.insert("keyword.operator.logical.cangjie".to_string(), palette.secondary.clone());
        scopes.insert("keyword.operator.arithmetic.cangjie".to_string(), palette.secondary.clone());

        // 仓颉专属语法元素
        scopes.insert("entity.name.type.law.cangjie".to_string(), palette.accent2.clone());
        scopes.insert("entity.name.type.cosmos.cangjie".to_string(), palette.accent1.clone());
        scopes.insert("entity.name.type.carrier.cangjie".to_string(), palette.secondary.clone());
        scopes.insert("entity.name.type.essence.cangjie".to_string(), palette.tertiary.clone());
        scopes.insert("entity.name.function.unified-law.cangjie".to_string(), palette.primary.clone());
        scopes.insert("meta.constraint.cangjie".to_string(), palette.warning.clone());
        scopes.insert("variable.other.evolution-param.cangjie".to_string(), palette.primary.lighten(0.2));

        // 基础语法元素
        scopes.insert("string.quoted.double.cangjie".to_string(), palette.tertiary.clone());
        scopes.insert("constant.numeric.cangjie".to_string(), palette.tertiary.clone());
        scopes.insert("comment.line.double-slash.cangjie".to_string(), palette.comment.clone());
        scopes.insert("comment.block.cangjie".to_string(), palette.comment.clone());
        scopes.insert("entity.name.function.cangjie".to_string(), palette.primary.lighten(0.1));
        scopes.insert("variable.name.cangjie".to_string(), palette.foreground.clone());
        scopes.insert("entity.name.type.struct.cangjie".to_string(), palette.tertiary.darken(0.1));
    }
}

/// 实现 Zed 的 Theme trait
impl ZedTheme for Theme {
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

    fn palette(&self) -> Option<&dyn zed_extension_api::ThemePalette> {
        self.palette.as_ref().map(|p| p as &dyn zed_extension_api::ThemePalette)
    }

    fn syntax(&self) -> Option<&dyn zed_extension_api::ThemeSyntax> {
        self.syntax.as_ref().map(|s| s as &dyn zed_extension_api::ThemeSyntax)
    }

    fn ui(&self) -> Option<&dyn zed_extension_api::ThemeUi> {
        self.ui.as_ref().map(|u| u as &dyn zed_extension_api::ThemeUi)
    }

    fn mode(&self) -> zed_extension_api::ThemeMode {
        self.mode
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn license(&self) -> &str {
        &self.license
    }
}

/// 实现 Zed 的 ThemeSyntax trait
impl ZedThemeSyntax for ThemeSyntax {
    fn scopes(&self) -> Option<&HashMap<String, String>> {
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

/// 实现 Zed 的 ThemeUi trait
impl ZedThemeUi for ThemeUi {
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

## 五、调试器模块（第四部分）
### 1. src/debugger/mod.rs
```rust
//! 仓颉语言调试器模块（适配 Zed 调试器扩展规范）
pub mod adapter;
pub mod config;
pub mod breakpoint;
pub mod debugger;
pub mod cosmos_inspector;

pub use adapter::CangjieDebugAdapter;
pub use config::{CangjieDebugConfig, DebugRequestType, CangjieDebugMode, MigrateDebugConfig, MigrateStage};
pub use breakpoint::{CangjieBreakpoint, CosmosStateProvider};
pub use debugger::CangjieDebugger;
pub use cosmos_inspector::CosmosInspectState;
```

### 2. src/debugger/config.rs
```rust
//! 仓颉调试配置定义（支持宇宙实例、法则校验、跨载体迁移调试）
use serde::{Serialize, Deserialize};
use zed_extension_api::Url;

/// 调试请求类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum DebugRequestType {
    #[serde(rename = "launch")]
    Launch,
    #[serde(rename = "attach")]
    Attach,
}

/// 仓颉专属调试模式
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CangjieDebugMode {
    #[serde(rename = "CosmosEvolution")]
    CosmosEvolution,      // 普通演化调试
    #[serde(rename = "LawValidation")]
    LawValidation,        // 法则一致性校验
    #[serde(rename = "CarrierMigration")]
    CarrierMigration,     // 跨载体迁移调试
}

/// 跨载体迁移阶段
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum MigrateStage {
    #[serde(rename = "CosmosSerialization")]
    CosmosSerialization,  // 宇宙状态序列化
    #[serde(rename = "CarrierAdaptation")]
    CarrierAdaptation,    // 载体差异适配
    #[serde(rename = "CosmosRecovery")]
    CosmosRecovery,       // 宇宙状态恢复
}

/// 跨载体迁移调试配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MigrateDebugConfig {
    /// 源载体 ID
    pub source_carrier_id: String,
    /// 目标载体 ID
    pub target_carrier_id: String,
    /// 迁移断点（指定阶段触发）
    pub migrate_breakpoints: Vec<MigrateStage>,
}

/// 宇宙类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CosmosType {
    #[serde(rename = "DigitalCosmos")]
    DigitalCosmos,
    #[serde(rename = "QuantumCosmos")]
    QuantumCosmos,
    #[serde(rename = "ConsciousnessCosmos")]
    ConsciousnessCosmos,
    #[serde(rename = "DimensionalCosmos")]
    DimensionalCosmos,
}

/// 仓颉调试配置（在 Zed launch.json 中配置）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CangjieDebugConfig {
    /// 调试类型（固定为 "cangjie"）
    #[serde(rename = "type")]
    pub type_: String,
    /// 调试名称（显示在 Zed 调试面板）
    pub name: String,
    /// 调试请求类型
    pub request: DebugRequestType,
    /// 目标宇宙文件路径（.cosmos 文件）
    pub cosmos_file: Url,
    /// 宇宙类型
    pub cosmos_type: CosmosType,
    /// 调试模式
    pub debug_mode: CangjieDebugMode,
    /// 跨载体迁移调试配置（仅 debug_mode = CarrierMigration 时生效）
    pub migrate_config: Option<MigrateDebugConfig>,
    /// 法则一致性校验阈值（默认 0.95）
    pub law_validation_threshold: Option<f32>,
    /// 演化步进间隔（毫秒，默认 100ms）
    pub step_interval: Option<u64>,
}

impl Default for CangjieDebugConfig {
    fn default() -> Self {
        Self {
            type_: "cangjie".to_string(),
            name: "Launch Cosmos".to_string(),
            request: DebugRequestType::Launch,
            cosmos_file: Url::from_file_path("src/main.cosmos").unwrap_or_else(|_| Url::parse("file:///src/main.cosmos").unwrap()),
            cosmos_type: CosmosType::DigitalCosmos,
            debug_mode: CangjieDebugMode::CosmosEvolution,
            migrate_config: None,
            law_validation_threshold: Some(0.95),
            step_interval: Some(100),
        }
    }
}
```

### 3. src/debugger/breakpoint.rs
```rust
//! 仓颉断点管理（支持普通代码断点、法则断点、演化断点）
use serde::{Serialize, Deserialize};
use zed_extension_api::{Url, Position};
use std::fmt;

/// 法则类型（与仓颉语言法则定义对应）
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum LawType {
    UnifiedLaw,       // 归一法则
    PhysicsLaw,       // 物理法则
    LogicLaw,         // 逻辑法则
    ConstraintLaw,    // 约束法则
}

/// 仓颉断点类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CangjieBreakpoint {
    /// 普通代码断点（行号断点）
    CodeBreakpoint {
        source: Url,
        line: u32,
        column: Option<u32>,
        enabled: bool,
        id: String,
    },
    /// 法则断点（触发于特定法则执行时）
    LawBreakpoint {
        law_id: String,
        law_type: LawType,
        enabled: bool,
        condition: Option<String>,
        id: String,
    },
    /// 演化断点（触发于宇宙演化到特定阶段）
    EvolutionBreakpoint {
        stage: String,
        enabled: bool,
        condition: Option<String>,
        id: String,
    },
}

impl CangjieBreakpoint {
    /// 生成唯一断点 ID
    pub fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// 创建普通代码断点
    pub fn code_breakpoint(source: Url, line: u32, column: Option<u32>) -> Self {
        Self::CodeBreakpoint {
            source,
            line,
            column,
            enabled: true,
            id: Self::generate_id(),
        }
    }

    /// 创建法则断点
    pub fn law_breakpoint(law_id: String, law_type: LawType, condition: Option<String>) -> Self {
        Self::LawBreakpoint {
            law_id,
            law_type,
            enabled: true,
            condition,
            id: Self::generate_id(),
        }
    }

    /// 创建演化断点
    pub fn evolution_breakpoint(stage: String, condition: Option<String>) -> Self {
        Self::EvolutionBreakpoint {
            stage,
            enabled: true,
            condition,
            id: Self::generate_id(),
        }
    }

    /// 获取断点 ID
    pub fn id(&self) -> &str {
        match self {
            Self::CodeBreakpoint { id, .. } => id,
            Self::LawBreakpoint { id, .. } => id,
            Self::EvolutionBreakpoint { id, .. } => id,
        }
    }

    /// 检查断点是否启用
    pub fn is_enabled(&self) -> bool {
        match self {
            Self::CodeBreakpoint { enabled, .. } => *enabled,
            Self::LawBreakpoint { enabled, .. } => *enabled,
            Self::EvolutionBreakpoint { enabled, .. } => *enabled,
        }
    }

    /// 设置断点启用状态
    pub fn set_enabled(&mut self, enabled: bool) {
        match self {
            Self::CodeBreakpoint { ref mut enabled, .. } => *enabled = enabled,
            Self::LawBreakpoint { ref mut enabled, .. } => *enabled = enabled,
            Self::EvolutionBreakpoint { ref mut enabled, .. } => *enabled = enabled,
        }
    }

    /// 获取代码断点的行号（仅代码断点有效）
    pub fn line(&self) -> Option<u32> {
        match self {
            Self::CodeBreakpoint { line, .. } => Some(*line),
            _ => None,
        }
    }

    /// 获取代码断点的列号（仅代码断点有效）
    pub fn column(&self) -> Option<u32> {
        match self {
            Self::CodeBreakpoint { column, .. } => *column,
            _ => None,
        }
    }

    /// 检查断点是否触发
    pub fn should_trigger(
        &self,
        cosmos_state: &impl CosmosStateProvider,
        current_law: Option<&(String, LawType)>,
    ) -> bool {
        if !self.is_enabled() {
            return false;
        }

        match self {
            Self::CodeBreakpoint { source, line, column, .. } => {
                // 匹配当前执行的文件、行号、列号
                cosmos_state.current_source() == *source
                    && cosmos_state.current_position().line == *line
                    && column.map_or(true, |col| cosmos_state.current_position().column == col)
            }
            Self::LawBreakpoint { law_id, law_type, condition, .. } => {
                // 匹配当前执行的法则，并满足条件
                if let Some((current_law_id, current_law_type)) = current_law {
                    if current_law_id == law_id && current_law_type == law_type {
                        return condition.as_ref()
                            .map_or(true, |cond| cosmos_state.eval_condition(cond));
                    }
                }
                false
            }
            Self::EvolutionBreakpoint { stage, condition, .. } => {
                // 匹配当前宇宙演化阶段，并满足条件
                if cosmos_state.current_evolution_stage() == *stage {
                    return condition.as_ref()
                        .map_or(true, |cond| cosmos_state.eval_condition(cond));
                }
                false
            }
        }
    }
}

impl fmt::Display for CangjieBreakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodeBreakpoint { source, line, column, .. } => {
                write!(f, "CodeBreakpoint: {}:{}:{}", source, line, column.unwrap_or(0))
            }
            Self::LawBreakpoint { law_id, law_type, .. } => {
                write!(f, "LawBreakpoint: {} ({:?})", law_id, law_type)
            }
            Self::EvolutionBreakpoint { stage, .. } => {
                write!(f, "EvolutionBreakpoint: {}", stage)
            }
        }
    }
}

/// 宇宙状态提供器（供断点检查触发条件）
pub trait CosmosStateProvider {
    /// 获取当前执行的文件路径
    fn current_source(&self) -> Url;
    /// 获取当前执行的位置（行号、列号）
    fn current_position(&self) -> Position;
    /// 获取当前宇宙演化阶段
    fn current_evolution_stage(&self) -> String;
    /// 评估条件表达式（支持简单的数值比较、变量引用）
    fn eval_condition(&self, condition: &str) -> bool;
}
```

### 4. src/debugger/cosmos_inspector.rs
```rust
//! 宇宙实例检查器（定义调试面板展示的宇宙状态结构）
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// 宇宙物理参数（示例：包含常见物理常量与自定义法则参数）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CosmosPhysicsParams {
    /// 引力常量
    pub gravitational_constant: f64,
    /// 光速
    pub speed_of_light: f64,
    /// 普朗克常量
    pub planck_constant: f64,
    /// 自定义法则参数（键值对形式）
    pub custom_law_params: HashMap<String, f64>,
}

impl Default for CosmosPhysicsParams {
    fn default() -> Self {
        Self {
            gravitational_constant: 6.67430e-11,
            speed_of_light: 299792458.0,
            planck_constant: 6.62607015e-34,
            custom_law_params: HashMap::new(),
        }
    }
}

/// 宇宙实例检查状态（供调试面板展示）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CosmosInspectState {
    /// 宇宙实例 ID
    pub id: String,
    /// 演化时间（秒）
    pub evolution_time: f64,
    /// 物理参数与法则参数
    pub physics_params: CosmosPhysicsParams,
    /// 当前演化阶段
    pub evolution_stage: String,
    /// 所属载体 ID
    pub carrier_id: String,
    /// 已加载的法则数量
    pub loaded_law_count: usize,
    /// 演化状态（运行中/暂停/完成）
    pub evolution_status: String,
}

impl Default for CosmosInspectState {
    fn default() -> Self {
        Self {
            id: "cosmos-unknown".to_string(),
            evolution_time: 0.0,
            physics_params: CosmosPhysicsParams::default(),
            evolution_stage: "initialization".to_string(),
            carrier_id: "carrier-unknown".to_string(),
            loaded_law_count: 0,
            evolution_status: "not_started".to_string(),
        }
    }
}
```

### 5. src/debugger/adapter.rs
```rust
//! 仓颉调试适配器（实现 DAP 协议，对接 Zed 调试面板）
use dap::prelude::*;
use tokio::sync::mpsc;
use zed_extension_api::{self as zed, Result, DebugAdapter as ZedDebugAdapter, DebugEvent as ZedDebugEvent};
use super::{CangjieDebugger, CangjieDebugConfig, CangjieBreakpoint, CosmosInspectState};

/// 调试内部事件（仓颉调试器 → 适配器）
#[derive(Debug, Serialize, Deserialize)]
enum DebugEvent {
    /// 断点触发
    BreakpointHit { breakpoint_id: String },
    /// 调试终止
    Terminated,
    /// 宇宙演化完成
    CosmosEvolutionCompleted,
    /// 法则冲突警告
    LawConflictWarning { law_id: String, message: String },
    /// 跨载体迁移阶段触发
    MigrateStageHit { stage: String },
}

/// 仓颉调试适配器
pub struct CangjieDebugAdapter {
    debugger: CangjieDebugger,
    event_sender: mpsc::Sender<DebugEvent>,
    event_receiver: mpsc::Receiver<DebugEvent>,
}

impl CangjieDebugAdapter {
    /// 创建调试适配器
    pub fn new(config: CangjieDebugConfig) -> Result<Self> {
        let (event_sender, event_receiver) = mpsc::channel(100);
        let debugger = CangjieDebugger::new(config, event_sender.clone())?;
        Ok(Self {
            debugger,
            event_sender,
            event_receiver,
        })
    }

    /// 转换内部事件为 Zed 调试事件
    fn convert_event(event: DebugEvent) -> ZedDebugEvent {
        match event {
            DebugEvent::BreakpointHit { breakpoint_id } => ZedDebugEvent::BreakpointHit {
                breakpoint_id: Some(breakpoint_id),
            },
            DebugEvent::Terminated => ZedDebugEvent::Terminated,
            DebugEvent::CosmosEvolutionCompleted => ZedDebugEvent::Custom {
                command: "cangjie/cosmosEvolutionCompleted".to_string(),
                args: serde_json::Value::Null,
            },
            DebugEvent::LawConflictWarning { law_id, message } => ZedDebugEvent::Custom {
                command: "cangjie/lawConflictWarning".to_string(),
                args: serde_json::json!({
                    "law_id": law_id,
                    "message": message,
                    "severity": "warning"
                }),
            },
            DebugEvent::MigrateStageHit { stage } => ZedDebugEvent::Custom {
                command: "cangjie/migrateStageHit".to_string(),
                args: serde_json::json!({ "stage": stage }),
            },
        }
    }
}

/// 实现 Zed 的 DebugAdapter trait
impl ZedDebugAdapter for CangjieDebugAdapter {
    /// 初始化调试会话
    fn initialize(&mut self, _args: zed::InitializeArgs) -> Result<zed::InitializeResult> {
        Ok(zed::InitializeResult {
            supports_configuration_done_request: true,
            supports_set_breakpoints_request: true,
            supports_step_in_request: true,
            supports_step_over_request: true,
            supports_step_out_request: true,
            supports_continue_request: true,
            supports_pause_request: true,
            supports_disconnect_request: true,
            supports_inspect_variables_request: true,
            supports_custom_request: Some(vec![
                "cangjie/inspectCosmos".to_string(),
                "cangjie/validateLaw".to_string(),
                "cangjie/setLawBreakpoint".to_string(),
                "cangjie/setEvolutionBreakpoint".to_string(),
            ]),
            ..Default::default()
        })
    }

    /// 配置调试完成（启动宇宙实例）
    fn configuration_done(&mut self) -> Result<()> {
        self.debugger.start()?;
        Ok(())
    }

    /// 设置断点（普通代码断点）
    fn set_breakpoints(&mut self, args: zed::SetBreakpointsArgs) -> Result<zed::SetBreakpointsResult> {
        let breakpoints: Vec<CangjieBreakpoint> = args.breakpoints
            .into_iter()
            .map(|bp| CangjieBreakpoint::code_breakpoint(
                args.source.clone(),
                bp.line,
                bp.column
            ))
            .collect();

        self.debugger.set_breakpoints(breakpoints)?;

        // 返回断点设置结果
        Ok(zed::SetBreakpointsResult {
            breakpoints: self.debugger.get_breakpoints()
                .iter()
                .filter_map(|bp| {
                    Some(zed::Breakpoint {
                        id: Some(bp.id().to_string()),
                        line: bp.line()?,
                        column: bp.column(),
                        enabled: bp.is_enabled(),
                        verified: true,
                        message: None,
                    })
                })
                .collect(),
        })
    }

    /// 单步执行（演化步进）
    fn step_over(&mut self) -> Result<()> {
        self.debugger.step_over()?;
        Ok(())
    }

    /// 继续执行（恢复宇宙演化）
    fn continue_(&mut self) -> Result<()> {
        self.debugger.continue_()?;
        Ok(())
    }

    /// 暂停执行（暂停宇宙演化）
    fn pause(&mut self) -> Result<()> {
        self.debugger.pause()?;
        Ok(())
    }

    /// 断开调试（终止宇宙实例）
    fn disconnect(&mut self) -> Result<()> {
        self.debugger.stop()?;
        Ok(())
    }

    /// 查看变量（宇宙状态、法则参数）
    fn inspect_variables(&mut self, _args: zed::InspectVariablesArgs) -> Result<zed::InspectVariablesResult> {
        let cosmos_state = self.debugger.inspect_cosmos()?;
        let mut variables = vec![
            // 宇宙基础信息
            zed::Variable {
                name: "cosmos_id".to_string(),
                value: cosmos_state.id,
                type_: "String".to_string(),
                children: None,
                evaluate_name: None,
            },
            zed::Variable {
                name: "evolution_time".to_string(),
                value: format!("{:.2}s", cosmos_state.evolution_time),
                type_: "f64".to_string(),
                children: None,
                evaluate_name: None,
            },
            zed::Variable {
                name: "evolution_stage".to_string(),
                value: cosmos_state.evolution_stage,
                type_: "String".to_string(),
                children: None,
                evaluate_name: None,
            },
            zed::Variable {
                name: "carrier_id".to_string(),
                value: cosmos_state.carrier_id,
                type_: "String".to_string(),
                children: None,
                evaluate_name: None,
            },
            // 物理参数分组
            zed::Variable {
                name: "physics_params".to_string(),
                value: "Physics Parameters".to_string(),
                type_: "CosmosPhysicsParams".to_string(),
                children: Some(vec![
                    zed::Variable {
                        name: "gravitational_constant".to_string(),
                        value: format!("{:.6e}", cosmos_state.physics_params.gravitational_constant),
                        type_: "f64".to_string(),
                        children: None,
                        evaluate_name: None,
                    },
                    zed::Variable {
                        name: "speed_of_light".to_string(),
                        value: format!("{:.0} m/s", cosmos_state.physics_params.speed_of_light),
                        type_: "f64".to_string(),
                        children: None,
                        evaluate_name: None,
                    },
                ]),
                evaluate_name: None,
            },
        ];

        // 添加自定义法则参数
        if !cosmos_state.physics_params.custom_law_params.is_empty() {
            variables.push(zed::Variable {
                name: "custom_law_params".to_string(),
                value: "Custom Law Parameters".to_string(),
                type_: "HashMap<String, f64>".to_string(),
                children: Some(
                    cosmos_state.physics_params.custom_law_params
                        .into_iter()
                        .map(|(key, value)| zed::Variable {
                            name: key,
                            value: value.to_string(),
                            type_: "f64".to_string(),
                            children: None,
                            evaluate_name: None,
                        })
                        .collect()
                ),
                evaluate_name: None,
            });
        }

        Ok(zed::InspectVariablesResult { variables })
    }

    /// 自定义调试请求（仓颉专属功能）
    fn custom_request(&mut self, request: &zed::CustomDebugRequest) -> Result<serde_json::Value> {
        match request.command.as_str() {
            // 检查宇宙完整状态
            "cangjie/inspectCosmos" => {
                let cosmos_state = self.debugger.inspect_cosmos()?;
                Ok(serde_json::to_value(cosmos_state)?)
            }
            // 校验指定法则一致性
            "cangjie/validateLaw" => {
                let law_id = request.args.get("law_id")
                    .ok_or_else(|| zed::Error::user("缺少参数：law_id"))?
                    .as_str()
                    .ok_or_else(|| zed::Error::user("law_id 必须是字符串"))?;
                let validation_result = self.debugger.validate_law(law_id)?;
                Ok(serde_json::json!({
                    "law_id": law_id,
                    "consistency": validation_result,
                    "threshold": self.debugger.law_validation_threshold()
                }))
            }
            // 设置法则断点
            "cangjie/setLawBreakpoint" => {
                let law_id = request.args.get("law_id")
                    .ok_or_else(|| zed::Error::user("缺少参数：law_id"))?
                    .as_str()
                    .ok_or_else(|| zed::Error::user("law_id 必须是字符串"))?;
                let law_type = serde_json::from_value(
                    request.args.get("law_type").cloned()
                        .ok_or_else(|| zed::Error::user("缺少参数：law_type"))?
                )?;
                let condition = request.args.get("condition").and_then(|v| v.as_str()).map(|s| s.to_string());

                let breakpoint = CangjieBreakpoint::law_breakpoint(law_id.to_string(), law_type, condition);
                self.debugger.add_breakpoint(breakpoint.clone())?;

                Ok(serde_json::json!({
                    "breakpoint_id": breakpoint.id(),
                    "success": true
                }))
            }
            // 设置演化断点
            "cangjie/setEvolutionBreakpoint" => {
                let stage = request.args.get("stage")
                    .ok_or_else(|| zed::Error::user("缺少参数：stage"))?
                    .as_str()
                    .ok_or_else(|| zed::Error::user("stage 必须是字符串"))?;
                let condition = request.args.get("condition").and_then(|v| v.as_str()).map(|s| s.to_string());

                let breakpoint = CangjieBreakpoint::evolution_breakpoint(stage.to_string(), condition);
                self.debugger.add_breakpoint(breakpoint.clone())?;

                Ok(serde_json::json!({
                    "breakpoint_id": breakpoint.id(),
                    "success": true
                }))
            }
            _ => Err(zed::Error::user(format!(
                "不支持的自定义调试请求：{}", request.command
            ))),
        }
    }

    /// 接收调试事件（转发给 Zed 调试面板）
    fn next_event(&mut self) -> Result<Option<ZedDebugEvent>> {
        match self.event_receiver.try_recv() {
            Ok(event) => Ok(Some(Self::convert_event(event))),
            Err(mpsc::error::TryRecvError::Empty) => Ok(None),
            Err(mpsc::error::TryRecvError::Disconnected) => Ok(None),
        }
    }
}
```
