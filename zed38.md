# 仓颉语言 Zed 调试器扩展（Cangjie Debugger Extension）
基于 [Zed 调试器扩展规范](https://zed.dev/docs/extensions/debugger-extensions)，为仓颉编程语言新增专属调试能力，聚焦其「宇宙实例演化」「法则一致性校验」「跨载体迁移调试」等核心场景，复用原有扩展架构，仅新增/修改调试相关模块。以下是 **差异部分**（完整扩展需结合前文图标主题、语法主题模块）。

## 核心差异：新增调试器模块结构
在原有扩展目录基础上新增调试相关目录，整体结构差异如下：
```
cangjie-extension/
├── src/
│   ├── debugger/                # 新增：调试器核心模块
│   │   ├── mod.rs               # 调试器模块入口
│   │   ├── adapter.rs           # 调试适配器（实现 DAP 协议）
│   │   ├── config.rs            # 调试配置定义（宇宙实例、法则校验等）
│   │   ├── breakpoint.rs        # 断点管理（支持法则断点、演化断点）
│   │   ├── debugger.rs          # 调试器主逻辑（启动/暂停/步进/观测）
│   │   └── cosmos_inspector.rs  # 宇宙实例检查器（查看宇宙状态、法则参数）
│   ├── lib.rs                   # 修改：注册调试器扩展
│   └── theme.rs / icon_map.rs   # 复用：原有主题/图标模块
├── Cargo.toml                   # 修改：新增调试相关依赖
└── schemas/                     # 新增：调试配置 JSON Schema（供 Zed 自动补全）
    └── cangjie-debug-schema.json
```

## 1. Cargo.toml 依赖差异（新增调试相关依赖）
```toml
[package]
name = "cangjie-extension"
version = "0.2.0"  # 版本升级：从 0.1.0 → 0.2.0
edition = "2021"
description = "仓颉语言 Zed 扩展（含语法主题、图标主题、专属调试器）"

[dependencies]
# 原有依赖（语法/图标主题）
zed_extension_api = "0.100.0"
serde = { version = "1.0", features = ["derive"] }
once_cell = "1.18.0"

# 新增：调试器依赖（适配 Zed DAP 协议、宇宙实例调试）
dap = "0.10.0"                  # DAP（Debug Adapter Protocol）协议实现
tokio = { version = "1.0", features = ["full"] }  # 异步运行时（调试器后台任务）
thiserror = "1.0"               # 错误处理
serde_json = "1.0"              # 序列化宇宙状态、调试配置
uuid = "1.0"                    # 调试会话 ID 生成
```

## 2. 新增：调试器核心代码（src/debugger/）
### 2.1 src/debugger/mod.rs
```rust
//! 仓颉语言调试器模块（适配 Zed 调试器扩展规范）
pub mod adapter;
pub mod config;
pub mod breakpoint;
pub mod debugger;
pub mod cosmos_inspector;

pub use adapter::CangjieDebugAdapter;
pub use config::CangjieDebugConfig;
pub use debugger::CangjieDebugger;
```

### 2.2 src/debugger/config.rs（调试配置定义）
```rust
//! 仓颉调试配置（支持宇宙实例运行、法则一致性校验、跨载体迁移调试）
use serde::{Serialize, Deserialize};
use zed_extension_api::Url;
use crate::cosmic::cosmos::实例化::CosmosType;

/// 仓颉调试配置（在 Zed launch.json 中配置）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CangjieDebugConfig {
    /// 调试类型（固定为 "cangjie"）
    pub type_: String,
    /// 调试名称（显示在 Zed 调试面板）
    pub name: String,
    /// 调试请求类型（launch = 启动宇宙实例调试，attach = 附加到运行中的宇宙）
    pub request: DebugRequestType,
    /// 目标宇宙文件路径（.cosmos 文件）
    pub cosmos_file: Url,
    /// 宇宙类型（数字/量子/意识载体等）
    pub cosmos_type: CosmosType,
    /// 调试模式（普通演化/法则校验/跨载体迁移）
    pub debug_mode: CangjieDebugMode,
    /// 跨载体迁移调试配置（仅 debug_mode = Migrate 时生效）
    pub migrate_config: Option<MigrateDebugConfig>,
    /// 法则一致性校验阈值（仅 debug_mode = LawValidation 时生效）
    pub law_validation_threshold: Option<f32>,
    /// 演化步进间隔（毫秒，默认 100ms）
    pub step_interval: Option<u64>,
}

/// 调试请求类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DebugRequestType {
    Launch,
    Attach,
}

/// 仓颉专属调试模式
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CangjieDebugMode {
    /// 普通演化调试（单步跟踪宇宙演化过程）
    CosmosEvolution,
    /// 法则一致性校验（断点触发于法则冲突时）
    LawValidation,
    /// 跨载体迁移调试（跟踪宇宙从源载体到目标载体的迁移过程）
    CarrierMigration,
}

/// 跨载体迁移调试配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MigrateDebugConfig {
    /// 源载体 ID
    pub source_carrier_id: String,
    /// 目标载体 ID
    pub target_carrier_id: String,
    /// 迁移断点（迁移阶段：序列化/适配/恢复）
    pub migrate_breakpoints: Vec<MigrateStage>,
}

/// 跨载体迁移阶段
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MigrateStage {
    CosmosSerialization,  // 宇宙状态序列化
    CarrierAdaptation,    // 载体差异适配
    CosmosRecovery,       // 宇宙状态恢复
}

impl Default for CangjieDebugConfig {
    fn default() -> Self {
        Self {
            type_: "cangjie".to_string(),
            name: "Launch Cosmos".to_string(),
            request: DebugRequestType::Launch,
            cosmos_file: Url::from_file_path("src/main.cosmos").unwrap(),
            cosmos_type: CosmosType::DigitalCosmos,
            debug_mode: CangjieDebugMode::CosmosEvolution,
            migrate_config: None,
            law_validation_threshold: Some(0.95),
            step_interval: Some(100),
        }
    }
}
```

### 2.3 src/debugger/breakpoint.rs（仓颉专属断点）
```rust
//! 仓颉断点管理（支持普通代码断点、法则断点、演化断点）
use serde::{Serialize, Deserialize};
use zed_extension_api::{Url, Position};
use crate::cosmic::unification::归一法则::LawType;

/// 仓颉断点类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CangjieBreakpoint {
    /// 普通代码断点（行号断点）
    CodeBreakpoint {
        source: Url,
        line: u32,
        column: Option<u32>,
        enabled: bool,
    },
    /// 法则断点（触发于特定法则执行时）
    LawBreakpoint {
        law_id: String,
        law_type: LawType,
        enabled: bool,
        // 条件：法则参数阈值（如 "gravitational_constant < 6.67e-11"）
        condition: Option<String>,
    },
    /// 演化断点（触发于宇宙演化到特定阶段）
    EvolutionBreakpoint {
        stage: String,  // 如 "expansion"（膨胀阶段）、"law_adjustment"（法则调整阶段）
        enabled: bool,
        // 条件：演化时间（如 "evolution_time > 1000"）
        condition: Option<String>,
    },
}

impl CangjieBreakpoint {
    pub fn is_enabled(&self) -> bool {
        match self {
            Self::CodeBreakpoint { enabled, .. } => *enabled,
            Self::LawBreakpoint { enabled, .. } => *enabled,
            Self::EvolutionBreakpoint { enabled, .. } => *enabled,
        }
    }

    /// 检查断点是否触发（结合宇宙状态、法则执行情况）
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

/// 宇宙状态提供器（供断点检查触发条件）
pub trait CosmosStateProvider {
    fn current_source(&self) -> Url;
    fn current_position(&self) -> Position;
    fn current_evolution_stage(&self) -> String;
    fn eval_condition(&self, condition: &str) -> bool;
}
```

### 2.4 src/debugger/adapter.rs（DAP 调试适配器）
```rust
//! 仓颉调试适配器（实现 DAP 协议，对接 Zed 调试面板）
use dap::prelude::*;
use tokio::sync::mpsc;
use zed_extension_api::Result;
use crate::debugger::{CangjieDebugger, CangjieDebugConfig, CangjieBreakpoint};

/// 仓颉调试适配器（实现 Zed DebugAdapter trait）
pub struct CangjieDebugAdapter {
    debugger: CangjieDebugger,
    sender: mpsc::Sender<DebugEvent>,
    receiver: mpsc::Receiver<DebugEvent>,
}

impl CangjieDebugAdapter {
    pub fn new(config: CangjieDebugConfig) -> Result<Self> {
        let (sender, receiver) = mpsc::channel(100);
        let debugger = CangjieDebugger::new(config, sender.clone())?;
        Ok(Self {
            debugger,
            sender,
            receiver,
        })
    }
}

/// 实现 Zed 的 DebugAdapter trait（核心：对接 Zed 调试面板与仓颉调试器）
impl zed_extension_api::DebugAdapter for CangjieDebugAdapter {
    /// 初始化调试会话
    fn initialize(&mut self, _args: zed_extension_api::InitializeArgs) -> Result<zed_extension_api::InitializeResult> {
        Ok(zed_extension_api::InitializeResult {
            supports_configuration_done_request: true,
            supports_set_breakpoints_request: true,
            supports_step_in_request: true,
            supports_step_over_request: true,
            supports_step_out_request: true,
            supports_continue_request: true,
            supports_pause_request: true,
            supports_disconnect_request: true,
            supports_inspect_variables_request: true,
            // 仓颉专属：支持法则参数查看、宇宙状态检查
            supports_custom_request: Some(vec![
                "cangjie/inspectCosmos".to_string(),
                "cangjie/validateLaw".to_string(),
            ]),
            ..Default::default()
        })
    }

    /// 配置调试（加载宇宙文件、初始化断点）
    fn configuration_done(&mut self) -> Result<()> {
        self.debugger.start()?;
        Ok(())
    }

    /// 设置断点（转换 Zed 断点为仓颉专属断点）
    fn set_breakpoints(&mut self, args: zed_extension_api::SetBreakpointsArgs) -> Result<zed_extension_api::SetBreakpointsResult> {
        let cangjie_breakpoints: Vec<CangjieBreakpoint> = args.breakpoints
            .into_iter()
            .map(|bp| CangjieBreakpoint::CodeBreakpoint {
                source: args.source.clone(),
                line: bp.line,
                column: bp.column,
                enabled: bp.enabled,
            })
            .collect();

        self.debugger.set_breakpoints(cangjie_breakpoints)?;

        // 返回断点设置结果（告知 Zed 断点是否生效）
        Ok(zed_extension_api::SetBreakpointsResult {
            breakpoints: self.debugger.get_breakpoints()
                .iter()
                .map(|bp| zed_extension_api::Breakpoint {
                    id: Some(bp.id()),
                    line: bp.line(),
                    column: bp.column(),
                    enabled: bp.is_enabled(),
                    verified: true,
                    ..Default::default()
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
    fn inspect_variables(&mut self, _args: zed_extension_api::InspectVariablesArgs) -> Result<zed_extension_api::InspectVariablesResult> {
        let cosmos_state = self.debugger.inspect_cosmos()?;
        Ok(zed_extension_api::InspectVariablesResult {
            variables: vec![
                // 宇宙基础信息
                zed_extension_api::Variable {
                    name: "cosmos_id".to_string(),
                    value: cosmos_state.id,
                    type_: "String".to_string(),
                    ..Default::default()
                },
                zed_extension_api::Variable {
                    name: "evolution_time".to_string(),
                    value: format!("{}s", cosmos_state.evolution_time),
                    type_: "f64".to_string(),
                    ..Default::default()
                },
                // 法则参数（示例：引力常量）
                zed_extension_api::Variable {
                    name: "gravitational_constant".to_string(),
                    value: format!("{}", cosmos_state.physics_params.gravitational_constant),
                    type_: "f64".to_string(),
                    ..Default::default()
                },
            ],
        })
    }

    /// 仓颉专属自定义请求（如检查宇宙状态、校验法则一致性）
    fn custom_request(&mut self, request: &zed_extension_api::CustomDebugRequest) -> Result<serde_json::Value> {
        match request.command.as_str() {
            "cangjie/inspectCosmos" => {
                let cosmos_state = self.debugger.inspect_cosmos()?;
                Ok(serde_json::to_value(cosmos_state)?)
            }
            "cangjie/validateLaw" => {
                let law_id = request.args.get("law_id").unwrap().as_str().unwrap();
                let validation_result = self.debugger.validate_law(law_id)?;
                Ok(serde_json::to_value(validation_result)?)
            }
            _ => Err(zed_extension_api::Error::user(format!(
                "不支持的自定义调试请求：{}", request.command
            ))),
        }
    }

    /// 接收调试事件（如断点触发、宇宙演化完成）
    fn next_event(&mut self) -> Result<Option<zed_extension_api::DebugEvent>> {
        match self.receiver.try_recv() {
            Ok(event) => Ok(Some(event.into())),
            Err(mpsc::error::TryRecvError::Empty) => Ok(None),
            Err(mpsc::error::TryRecvError::Disconnected) => Ok(None),
        }
    }
}

/// 调试事件（仓颉调试器 → Zed 调试面板）
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
}

/// 转换为 Zed 调试事件
impl From<DebugEvent> for zed_extension_api::DebugEvent {
    fn from(event: DebugEvent) -> Self {
        match event {
            DebugEvent::BreakpointHit { breakpoint_id } => zed_extension_api::DebugEvent::BreakpointHit {
                breakpoint_id: Some(breakpoint_id),
            },
            DebugEvent::Terminated => zed_extension_api::DebugEvent::Terminated,
            DebugEvent::CosmosEvolutionCompleted => zed_extension_api::DebugEvent::Custom {
                command: "cangjie/cosmosEvolutionCompleted".to_string(),
                args: serde_json::Value::Null,
            },
            DebugEvent::LawConflictWarning { law_id, message } => zed_extension_api::DebugEvent::Custom {
                command: "cangjie/lawConflictWarning".to_string(),
                args: serde_json::json!({ "law_id": law_id, "message": message }),
            },
        }
    }
}
```

### 2.5 src/debugger/debugger.rs（调试器主逻辑）
```rust
//! 仓颉调试器主逻辑（管理调试生命周期、宇宙演化控制）
use tokio::sync::mpsc;
use zed_extension_api::Result;
use crate::cosmic::cosmos::实例化::{CosmosInstantiationManager, CosmosInstance};
use crate::cosmic::unification::归一法则::UnifiedLawManager;
use super::{CangjieDebugConfig, CangjieBreakpoint, CosmosStateProvider, DebugEvent};

pub struct CangjieDebugger {
    config: CangjieDebugConfig,
    cosmos_instance: Option<CosmosInstance>,
    breakpoints: Vec<CangjieBreakpoint>,
    is_paused: bool,
    sender: mpsc::Sender<DebugEvent>,
    cosmos_manager: CosmosInstantiationManager,
    law_manager: UnifiedLawManager,
}

impl CangjieDebugger {
    /// 初始化调试器
    pub fn new(config: CangjieDebugConfig, sender: mpsc::Sender<DebugEvent>) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            cosmos_instance: None,
            breakpoints: Vec::new(),
            is_paused: false,
            sender,
            cosmos_manager: CosmosInstantiationManager::new()?,
            law_manager: UnifiedLawManager::new()?,
        })
    }

    /// 启动调试（加载宇宙实例、开始演化）
    pub fn start(&mut self) -> Result<()> {
        // 加载宇宙文件
        let cosmos_file = self.config.cosmos_file.to_file_path().unwrap();
        let cosmos_meta = self.cosmos_manager.load_cosmos_meta(&cosmos_file)?;
        
        // 实例化宇宙（基于调试配置）
        let cosmos_instance = self.cosmos_manager.instantiate_cosmos(
            &cosmos_meta,
            self.config.cosmos_type.clone(),
            self.config.step_interval.unwrap_or(100),
        )?;
        
        self.cosmos_instance = Some(cosmos_instance);
        
        // 启动宇宙演化（异步任务）
        self.spawn_evolution_task();
        
        Ok(())
    }

    /// 设置断点
    pub fn set_breakpoints(&mut self, breakpoints: Vec<CangjieBreakpoint>) -> Result<()> {
        self.breakpoints = breakpoints;
        Ok(())
    }

    /// 获取当前断点列表
    pub fn get_breakpoints(&self) -> &[CangjieBreakpoint] {
        &self.breakpoints
    }

    /// 单步执行（演化一次步进）
    pub fn step_over(&mut self) -> Result<()> {
        if let Some(cosmos) = &mut self.cosmos_instance {
            cosmos.step_evolution()?;
            self.check_breakpoints()?;
        }
        Ok(())
    }

    /// 继续执行（恢复宇宙演化）
    pub fn continue_(&mut self) -> Result<()> {
        self.is_paused = false;
        self.spawn_evolution_task();
        Ok(())
    }

    /// 暂停执行
    pub fn pause(&mut self) -> Result<()> {
        self.is_paused = true;
        if let Some(cosmos) = &mut self.cosmos_instance {
            cosmos.pause_evolution()?;
        }
        Ok(())
    }

    /// 停止调试（终止宇宙实例）
    pub fn stop(&mut self) -> Result<()> {
        if let Some(cosmos) = &mut self.cosmos_instance {
            cosmos.terminate_evolution()?;
        }
        self.sender.try_send(DebugEvent::Terminated)?;
        Ok(())
    }

    /// 检查断点是否触发
    fn check_breakpoints(&mut self) -> Result<()> {
        let cosmos = self.cosmos_instance.as_ref().unwrap();
        let current_law = cosmos.current_executing_law();

        for breakpoint in &self.breakpoints {
            if breakpoint.should_trigger(cosmos, current_law) {
                self.is_paused = true;
                self.sender.try_send(DebugEvent::BreakpointHit {
                    breakpoint_id: breakpoint.id(),
                })?;
                break;
            }
        }

        // 法则一致性校验（仅 LawValidation 模式）
        if self.config.debug_mode == CangjieDebugMode::LawValidation {
            self.validate_laws()?;
        }

        Ok(())
    }

    /// 校验法则一致性（触发警告事件）
    fn validate_laws(&mut self) -> Result<()> {
        let cosmos = self.cosmos_instance.as_ref().unwrap();
        let threshold = self.config.law_validation_threshold.unwrap_or(0.95);

        for law in cosmos.get_all_laws() {
            let consistency = self.law_manager.validate_law_consistency(law)?;
            if consistency < threshold {
                self.sender.try_send(DebugEvent::LawConflictWarning {
                    law_id: law.id.clone(),
                    message: format!("法则一致性低于阈值（当前：{:.2}，阈值：{:.2}）", consistency, threshold),
                })?;
            }
        }

        Ok(())
    }

    /// 检查宇宙状态
    pub fn inspect_cosmos(&self) -> Result<CosmosInspectState> {
        let cosmos = self.cosmos_instance.as_ref().unwrap();
        Ok(CosmosInspectState {
            id: cosmos.id().to_string(),
            evolution_time: cosmos.evolution_time(),
            physics_params: cosmos.physics_params().clone(),
            evolution_stage: cosmos.current_evolution_stage(),
            carrier_id: cosmos.carrier_id().to_string(),
        })
    }

    /// 异步演化任务（后台运行，触发断点时暂停）
    fn spawn_evolution_task(&mut self) {
        let mut cosmos = self.cosmos_instance.take().unwrap();
        let sender = self.sender.clone();
        let breakpoints = self.breakpoints.clone();
        let step_interval = self.config.step_interval.unwrap_or(100);

        tokio::spawn(async move {
            while !cosmos.is_evolution_completed() && !self.is_paused {
                // 演化步进
                cosmos.step_evolution().unwrap();

                // 检查断点
                let current_law = cosmos.current_executing_law();
                let mut breakpoint_triggered = false;
                for bp in &breakpoints {
                    if bp.should_trigger(&cosmos, current_law) {
                        sender.send(DebugEvent::BreakpointHit {
                            breakpoint_id: bp.id(),
                        }).await.unwrap();
                        breakpoint_triggered = true;
                        break;
                    }
                }

                if breakpoint_triggered {
                    break;
                }

                // 等待步进间隔
                tokio::time::sleep(tokio::time::Duration::from_millis(step_interval)).await;
            }

            // 演化完成
            if cosmos.is_evolution_completed() {
                sender.send(DebugEvent::CosmosEvolutionCompleted).await.unwrap();
            }

            // 归还宇宙实例
            self.cosmos_instance = Some(cosmos);
        });
    }
}

/// 宇宙检查状态（供调试面板展示）
#[derive(Debug, Serialize, Deserialize)]
pub struct CosmosInspectState {
    pub id: String,
    pub evolution_time: f64,
    pub physics_params: HashMap<String, f64>,
    pub evolution_stage: String,
    pub carrier_id: String,
}
```

## 3. src/lib.rs 扩展注册差异（新增调试器注册）
```rust
//! 仓颉语言 Zed 扩展（v0.2.0：新增调试器）
use zed_extension_api::{self as zed, Result, Extension, ThemeProvider, IconThemeProvider, DebugAdapterProvider};
use once_cell::sync::Lazy;
use crate::{
    theme::CangjieTheme,
    icon_map::CangjieIconThemeProvider,
    debugger::{CangjieDebugAdapter, CangjieDebugConfig},
};

// 复用：原有主题、图标实例
static CANGJIE_THEME: Lazy<CangjieTheme> = Lazy::new(CangjieTheme::load);
static CANGJIE_ICON_PROVIDER: Lazy<CangjieIconThemeProvider> = Lazy::new(CangjieIconThemeProvider::default);

/// 扩展主结构体（新增 DebugAdapterProvider 实现）
struct CangjieExtension;

impl Extension for CangjieExtension {}

// 复用：原有主题提供器实现
impl ThemeProvider for CangjieExtension {
    fn themes(&self) -> Vec<&dyn zed::Theme> {
        vec![
            CANGJIE_THEME.get_theme(zed::ThemeMode::Dark),
            CANGJIE_THEME.get_theme(zed::ThemeMode::Light),
            CANGJIE_THEME.get_theme(zed::ThemeMode::HighContrast),
        ]
    }

    fn theme(&self, theme_id: &str, mode: zed::ThemeMode) -> Option<&dyn zed::Theme> {
        match theme_id {
            "cangjie-dark" => Some(CANGJIE_THEME.get_theme(zed::ThemeMode::Dark)),
            "cangjie-light" => Some(CANGJIE_THEME.get_theme(zed::ThemeMode::Light)),
            "cangjie-high-contrast" => Some(CANGJIE_THEME.get_theme(zed::ThemeMode::HighContrast)),
            _ => None,
        }
    }
}

// 复用：原有图标主题提供器实现
impl IconThemeProvider for CangjieExtension {
    fn theme(&self) -> &dyn zed::IconTheme {
        &*CANGJIE_ICON_PROVIDER
    }

    fn file_type_icon(&self, file_type: &zed::FileType) -> Option<zed::IconId> {
        CANGJIE_ICON_PROVIDER.file_type_icon(file_type)
    }

    // ... 复用其他图标映射方法 ...
}

// 新增：调试适配器提供器实现（核心差异）
impl DebugAdapterProvider for CangjieExtension {
    /// 支持的调试类型（与调试配置中的 "type_" 字段匹配）
    fn debug_types(&self) -> Vec<&str> {
        vec!["cangjie"]
    }

    /// 创建调试适配器（基于用户配置）
    fn create_debug_adapter(
        &self,
        config: serde_json::Value,
    ) -> Result<Box<dyn zed::DebugAdapter>> {
        // 解析用户配置（launch.json）
        let cangjie_config: CangjieDebugConfig = serde_json::from_value(config)?;
        let adapter = CangjieDebugAdapter::new(cangjie_config)?;
        Ok(Box::new(adapter))
    }

    /// 提供调试配置 JSON Schema（供 Zed 自动补全）
    fn debug_config_schema(&self, _debug_type: &str) -> Option<serde_json::Value> {
        let schema = std::fs::read_to_string("schemas/cangjie-debug-schema.json")?;
        Some(serde_json::from_str(&schema)?)
    }
}

/// 扩展入口（注册新增的 DebugAdapterProvider）
#[zed::extension]
fn activate(_: &zed::Workspace) -> Result<Box<dyn zed::Extension>> {
    // 初始化主题（复用原有逻辑）
    let mut dark_theme = CANGJIE_THEME.dark.clone();
    let mut light_theme = CANGJIE_THEME.light.clone();
    let mut high_contrast_theme = CANGJIE_THEME.high_contrast.clone();
    dark_theme.configure_cangjie_syntax();
    light_theme.configure_cangjie_syntax();
    high_contrast_theme.configure_cangjie_syntax();

    *CANGJIE_THEME.lock().unwrap() = CangjieTheme {
        dark: dark_theme,
        light: light_theme,
        high_contrast: high_contrast_theme,
    };

    // 返回扩展实例（新增 DebugAdapterProvider 能力）
    Ok(Box::new(CangjieExtension))
}
```

## 4. 新增：调试配置 JSON Schema（schemas/cangjie-debug-schema.json）
供 Zed 调试配置面板提供自动补全和校验：
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

## 5. 原有模块兼容说明
- **语法主题**：无需修改，调试过程中语法高亮正常生效；
- **图标主题**：新增调试相关 UI 图标（如「启动宇宙调试」「检查宇宙状态」），需在 `icon_map.rs` 中添加对应映射（示例如下）：

### 图标主题兼容差异（src/icon_map.rs）
```rust
// 新增：调试相关 UI 图标 ID
pub enum CangjieIconId {
    // ... 原有图标 ID ...
    // 调试相关 UI 图标
    DebugCosmos,            // 调试宇宙实例
    InspectCosmos,          // 检查宇宙状态
    LawValidation,          // 法则一致性校验
    MigrateDebug,           // 跨载体迁移调试
}

impl ToString for CangjieIconId {
    fn to_string(&self) -> String {
        match self {
            // ... 原有映射 ...
            // 新增调试图标路径
            Self::DebugCosmos => "ui/debug-cosmos",
            Self::InspectCosmos => "ui/inspect-cosmos",
            Self::LawValidation => "ui/law-validation",
            Self::MigrateDebug => "ui/migrate-debug",
        }
    }
}

// 新增：调试命令 -> 图标映射
pub static DEBUG_COMMAND_ICON_MAP: Lazy<HashMap<&str, IconId>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("debug_cosmos", CangjieIconId::DebugCosmos.into());
    map.insert("inspect_cosmos", CangjieIconId::InspectCosmos.into());
    map.insert("law_validation", CangjieIconId::LawValidation.into());
    map.insert("migrate_debug", CangjieIconId::MigrateDebug.into());
    map
});
```

## 扩展使用说明（新增调试功能）
1. 在 Zed 中打开仓颉项目，创建 `launch.json` 调试配置：
   ```json
   {
     "version": "0.2.0",
     "configurations": [
       {
         "type": "cangjie",
         "name": "Launch Digital Cosmos",
         "request": "launch",
         "cosmos_file": "${workspaceFolder}/src/main.cosmos",
         "cosmos_type": "DigitalCosmos",
         "debug_mode": "CosmosEvolution",
         "step_interval": 200
       }
     ]
   }
   ```
2. 启动调试：点击 Zed 调试面板 → 选择配置 → 点击「启动调试」；
3. 调试操作：
   - 单步执行：跟踪宇宙演化每一步；
   - 法则断点：在特定法则定义处设置断点，触发于法则执行时；
   - 宇宙检查：在调试面板查看宇宙演化时间、物理参数、载体信息；
   - 法则校验：自动检测法则一致性，触发警告时暂停调试。

## 版本升级说明
- 版本号：v0.1.0 → v0.2.0
- 新增功能：仓颉专属调试器（宇宙演化调试、法则一致性校验、跨载体迁移调试）；
- 兼容说明：完全兼容原有语法主题、图标主题功能，无破坏性变更。