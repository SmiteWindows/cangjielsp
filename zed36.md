# Cangjie Icon Themes for Zed: 仓颉语言专属图标主题

基于 [Zed 图标主题扩展规范](https://zed.dev/docs/extensions/icon-themes)，为 **仓颉编程语言（Cangjie Lang）** 设计一套专属图标主题，覆盖文件类型、语法元素、项目资源等核心场景，兼顾视觉一致性与仓颉语言的「本质赋能」「无界创世」设计理念，适配 Zed 编辑器的深色/浅色模式，提供清晰的视觉辨识度。

## 主题核心设计理念
1. **本源归一**：以「仓颉」字形演化的几何线条为基础，图标风格简洁统一，呼应「归一法则」的核心思想；
2. **无界创世**：融入「宇宙法则」「载体适配」的视觉符号（如量子比特、维度线条、意识波纹），体现语言的无界特性；
3. **高辨识度**：采用对比鲜明的色彩体系（主色：靛蓝 #4F46E5，辅助色：青蓝 #06B6D4，强调色：琥珀 #F59E0B），适配代码编辑场景的视觉聚焦需求；
4. **自适应兼容**：支持 Zed 深色/浅色模式自动切换，图标边缘优化为抗锯齿设计，确保不同分辨率下清晰显示。

## 主题结构（遵循 Zed 图标主题规范）
```
cangjie-icon-theme/
├── Cargo.toml                # 扩展配置（依赖、元信息）
├── LICENSE                   # 许可证（MIT，与 Zed 一致）
├── README.md                 # 主题说明文档
├── icons/                    # 图标资源目录（SVG 格式，支持矢量缩放）
│   ├── dark/                 # 深色模式图标
│   │   ├── file-types/       # 文件类型图标（仓颉源文件、配置文件等）
│   │   ├── syntax/           # 语法元素图标（函数、结构体、法则等）
│   │   ├── project/          # 项目资源图标（文件夹、宇宙实例、载体等）
│   │   └── ui/               # UI 图标（搜索、运行、调试等）
│   └── light/                # 浅色模式图标（结构与 dark 一致）
└── src/
    ├── lib.rs                # 图标主题注册逻辑
    ├── icon_map.rs           # 图标映射配置（关联文件类型/语法元素与图标）
    └── theme.rs              # 主题元数据定义（名称、作者、支持模式等）
```

## 核心实现代码
### 1. Cargo.toml（扩展配置）
```toml
[package]
name = "cangjie-icon-theme"
version = "0.1.0"
edition = "2021"
description = "Cangjie Lang 专属图标主题 for Zed Editor"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
repository = "https://github.com/your-username/cangjie-icon-theme"

[dependencies]
zed_extension_api = "0.100.0"  # 适配 Zed 最新 API 版本
serde = { version = "1.0", features = ["derive"] }
once_cell = "1.18.0"
```

### 2. src/theme.rs（主题元数据）
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
            version: "0.1.0".to_string(),
            license: "MIT".to_string(),
        }
    }
}
```

### 3. src/icon_map.rs（图标映射配置）
```rust
//! 图标映射：关联文件类型/语法元素与图标路径
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
    
    // UI 图标（覆盖 Zed 默认 UI 图标）
    RunCosmos,               // 运行宇宙实例
    DebugCosmos,             // 调试宇宙实例
    AnalyzeLaw,              // 分析法则一致性
    MigrateCarrier,          // 跨载体迁移
    ObserveEvolution,        // 观测宇宙演化
}

impl From<CangjieIconId> for IconId {
    fn from(id: CangjieIconId) -> Self {
        IconId::new("cangjie-icon-theme", &id.to_string())
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
            
            // UI 图标
            Self::RunCosmos => "ui/run-cosmos",
            Self::DebugCosmos => "ui/debug-cosmos",
            Self::AnalyzeLaw => "ui/analyze-law",
            Self::MigrateCarrier => "ui/migrate-carrier",
            Self::ObserveEvolution => "ui/observe-evolution",
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
    // 假设仓颉语法的 Tree-sitter 定义了以下 SyntaxKind（需根据实际调整）
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

/// UI 命令 -> 图标映射（覆盖 Zed 运行/调试等命令的图标）
pub static UI_COMMAND_ICON_MAP: Lazy<HashMap<&str, IconId>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    map.insert("run_cosmos", CangjieIconId::RunCosmos.into());
    map.insert("debug_cosmos", CangjieIconId::DebugCosmos.into());
    map.insert("analyze_law_consistency", CangjieIconId::AnalyzeLaw.into());
    map.insert("migrate_cosmos_carrier", CangjieIconId::MigrateCarrier.into());
    map.insert("observe_cosmos_evolution", CangjieIconId::ObserveEvolution.into());
    
    map
});
```

### 4. src/lib.rs（主题注册逻辑）
```rust
//! 仓颉图标主题扩展入口
use zed_extension_api::{self as zed, IconTheme, IconThemeProvider, Result, ThemeMode};
use once_cell::sync::Lazy;
use crate::{theme::CangjieIconTheme, icon_map::*};

/// 全局主题实例
static CANGJIE_THEME: Lazy<CangjieIconTheme> = Lazy::new(CangjieIconTheme::default);

/// 仓颉图标主题提供器
struct CangjieIconThemeProvider;

impl IconThemeProvider for CangjieIconThemeProvider {
    /// 获取主题元数据
    fn theme(&self) -> &dyn zed::IconTheme {
        &*CANGJIE_THEME
    }

    /// 获取文件类型对应的图标
    fn file_type_icon(&self, file_type: &zed::FileType) -> Option<zed::IconId> {
        FILE_TYPE_ICON_MAP.get(file_type).cloned()
    }

    /// 获取语法元素对应的图标
    fn syntax_icon(&self, syntax_kind: &zed::SyntaxKind) -> Option<zed::IconId> {
        SYNTAX_ICON_MAP.get(syntax_kind).cloned()
    }

    /// 获取文件夹对应的图标（根据名称匹配）
    fn folder_icon(&self, folder_name: &str) -> Option<zed::IconId> {
        FOLDER_ICON_MAP.get(folder_name.to_lowercase().as_str()).cloned()
            .or(Some(CangjieIconId::CangjieFolder.into())) // 默认文件夹图标
    }

    /// 获取 UI 命令对应的图标
    fn ui_icon(&self, command_id: &str) -> Option<zed::IconId> {
        UI_COMMAND_ICON_MAP.get(command_id).cloned()
    }

    /// 获取图标资源路径（根据主题模式选择 dark/light 文件夹）
    fn icon_path(&self, icon_id: &zed::IconId, mode: zed::ThemeMode) -> Option<String> {
        // 确保图标 ID 属于当前主题
        if icon_id.namespace() != "cangjie-icon-theme" {
            return None;
        }

        // 拼接图标路径（SVG 格式，支持矢量缩放）
        let mode_folder = match mode {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
        };
        Some(format!("icons/{}/{}.svg", mode_folder, icon_id.name()))
    }
}

/// 扩展入口：注册图标主题
#[zed::extension]
fn activate(_: &zed::Workspace) -> Result<Box<dyn zed::Extension>> {
    Ok(Box::new(CangjieIconThemeProvider))
}

/// 实现 Zed 的 IconTheme trait 以适配扩展 API
impl zed::IconTheme for CangjieIconTheme {
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

    fn supported_modes(&self) -> &[zed::ThemeMode] {
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

## 图标资源设计规范（icons/ 目录）
所有图标采用 **SVG 矢量格式**（确保缩放不失真），遵循以下设计规范：

### 1. 尺寸规范
- 基础尺寸：24x24px（Zed 推荐默认尺寸）
- 内边距：2px（避免图标边缘被裁剪）
- 线条粗细：1.5px（深色模式）/ 2px（浅色模式，提升辨识度）

### 2. 核心图标设计示例
#### （1）仓颉源文件图标（`icons/dark/file-types/cangjie-source.svg`）
```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#4F46E5" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
  <!-- 基础文件形状 -->
  <path d="M4 22h16a2 2 0 0 0 2-2V6l-6-6H6a2 2 0 0 0-2 2v16z"/>
  <!-- 仓颉「仓」字简化线条 -->
  <path d="M8 12h8M8 16h4M8 8h8"/>
  <!-- 本源归一符号（中心圆点） -->
  <circle cx="12" cy="12" r="1.5" fill="#4F46E5"/>
</svg>
```

#### （2）法则文件图标（`icons/dark/file-types/cangjie-law.svg`）
```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#06B6D4" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
  <!-- 基础文件形状 -->
  <path d="M4 22h16a2 2 0 0 0 2-2V6l-6-6H6a2 2 0 0 0-2 2v16z"/>
  <!-- 法则符号（循环箭头+公式符号） -->
  <path d="M10 8l4 4l-4 4M14 8v8"/>
  <path d="M8 12h2M14 12h2"/>
</svg>
```

#### （3）宇宙实例图标（`icons/dark/syntax/cosmos.svg`）
```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#F59E0B" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
  <!-- 宇宙球体 -->
  <circle cx="12" cy="12" r="8"/>
  <!-- 维度线条（3D 交叉） -->
  <path d="M12 4l0 16M4 12l16 0M7 7l10 10M17 7l-10 10"/>
  <!-- 演化波纹 -->
  <circle cx="12" cy="12" r="4" stroke-dasharray="4 2"/>
</svg>
```

### 3. 色彩规范
| 图标类型       | 深色模式颜色 | 浅色模式颜色 |
|----------------|--------------|--------------|
| 仓颉源文件     | #4F46E5（靛蓝） | #4338CA（深靛蓝） |
| 配置文件       | #6366F1（浅靛蓝） | #3730A3（靛蓝） |
| 法则相关       | #06B6D4（青蓝） | #0891B2（深青蓝） |
| 宇宙相关       | #F59E0B（琥珀） | #D97706（深琥珀） |
| 本质相关       | #10B981（翠绿） | #059669（深翠绿） |
| 载体相关       | #EC4899（粉红） | #BE185D（深粉红） |
| 语法元素       | #64748B（深灰） | #475569（黑灰） |
| 文件夹         | #94A3B8（中灰） | #64748B（深灰） |

## 安装与使用
### 1. 本地开发安装
1. 克隆本仓库：`git clone https://github.com/your-username/cangjie-icon-theme.git`
2. 进入目录：`cd cangjie-icon-theme`
3. 构建扩展：`cargo build --release`
4. 在 Zed 中启用：
   - 打开 Zed → 偏好设置（Settings）→ 扩展（Extensions）
   - 点击「Install from Path」，选择 `target/release` 目录下的 `cangjie_icon_theme.zedextension` 文件
   - 启用主题：偏好设置 → 外观（Appearance）→ 图标主题（Icon Theme）→ 选择「Cangjie Icon Theme」

### 2. 发布到 Zed 扩展市场
1. 遵循 Zed 扩展发布规范：https://zed.dev/docs/extensions/publishing
2. 打包扩展：`zed extension package`
3. 提交到 Zed 扩展市场：`zed extension publish`

## 扩展与定制
1. **新增图标**：在 `icons/dark/` 和 `icons/light/` 目录下添加 SVG 图标，更新 `CangjieIconId` 枚举和对应映射；
2. **修改颜色**：调整 SVG 中的 `stroke` 属性值，或新增主题变体（如「Cangjie Icon Theme - High Contrast」）；
3. **适配新文件类型**：在 `FILE_TYPE_ICON_MAP` 中添加新的 `FileType` 与图标映射。

## 兼容性说明
- 支持 Zed 版本：v0.100.0+（与 `zed_extension_api` 版本匹配）
- 适配系统：macOS、Linux、Windows（Zed 支持的所有平台）
- 语法兼容性：需配合仓颉语言的 Tree-sitter 语法插件使用（确保 `SyntaxKind` 映射正确）

## 许可证
本图标主题采用 MIT 许可证（与 Zed 一致），允许自由使用、修改和分发，详见 [LICENSE](LICENSE) 文件。

---

**主题版本**：v0.1.0  
**发布日期**：2025-11-09  
**适配语言**：仓颉编程语言（Cangjie Lang）  
**支持编辑器**：Zed v0.100.0+  
**官方仓库**：https://github.com/your-username/cangjie-icon-theme  
**反馈渠道**：https://github.com/your-username/cangjie-icon-theme/issues