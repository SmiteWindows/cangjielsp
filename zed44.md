# 补充：LSP 集成基础配置与多人协作调试支持
以下是扩展的进阶功能补充，包括仓颉语言 LSP 集成基础配置和 Zed 多人协作调试支持，进一步提升开发体验，贴合 Zed 核心特性。

## 二十三、仓颉 LSP 集成基础配置（src/lsp/ 目录）
Zed 支持通过 LSP（Language Server Protocol）提供代码补全、跳转定义等核心编辑能力，以下是 LSP 集成的基础实现：

### 1. LSP 客户端封装（src/lsp/client.rs）
```rust
//! 仓颉 LSP 客户端封装
use lsp_client::{Client, ClientConfig, LanguageClient};
use lsp_types::{
    request::Initialize, InitializeParams, InitializeResult, ServerCapabilities,
    TextDocumentSyncKind, CompletionOptions, DefinitionOptions,
};
use std::path::PathBuf;
use tokio::sync::Mutex;
use once_cell::sync::OnceCell;

// 全局 LSP 客户端实例（线程安全）
static LSP_CLIENT: OnceCell<Mutex<Client>> = OnceCell::new();

/// 初始化 LSP 客户端
pub async fn init_lsp_client() -> Result<(), Box<dyn std::error::Error>> {
    // 配置 LSP 客户端
    let config = ClientConfig {
        server_command: vec![
            "cangjie-lsp".to_string(), // 仓颉 LSP 服务端可执行文件（需单独实现）
            "--stdio".to_string(),
        ],
        root_uri: None, // 后续从工作区自动获取
        ..Default::default()
    };

    // 启动 LSP 客户端
    let client = Client::new(config).await?;
    LSP_CLIENT.set(Mutex::new(client)).unwrap();

    // 发送初始化请求
    let init_params = InitializeParams {
        capabilities: Default::default(),
        root_uri: None,
        ..Default::default()
    };

    let mut client = LSP_CLIENT.get().unwrap().lock().await;
    let init_result: InitializeResult = client.send_request::<Initialize>(init_params).await?;
    println!("LSP 初始化成功：{:?}", init_result.capabilities);

    Ok(())
}

/// 获取 LSP 客户端实例
pub async fn get_lsp_client() -> Option<MutexGuard<'static, Client>> {
    LSP_CLIENT.get().map(|client| client.lock().await)
}

/// 配置 LSP 工作区根目录
pub async fn set_lsp_root(root_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = get_lsp_client().await.ok_or("LSP 客户端未初始化")?;
    client.set_root_uri(root_path)?;
    Ok(())
}

/// LSP 服务端能力配置（供服务端参考）
pub fn default_server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncKind::Incremental.into()),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
            ..Default::default()
        }),
        definition_provider: Some(DefinitionOptions::default().into()),
        hover_provider: Some(true.into()),
        ..Default::default()
    }
}
```

### 2. LSP 集成到扩展（src/lib.rs 补充）
```rust
// 导入 LSP 模块
mod lsp;
use lsp::{init_lsp_client, set_lsp_root};

// 实现 Zed LSP 提供器 trait
impl zed_extension_api::LanguageServerProvider for CangjieZedExtension {
    /// 支持的语言 ID（与仓颉文件关联）
    fn language_ids(&self) -> Vec<&str> {
        vec!["cangjie", "cangjie-cosmos", "cangjie-law"]
    }

    /// 启动 LSP 服务
    async fn start_server(
        &self,
        workspace: &zed::Workspace,
    ) -> Result<Box<dyn zed::LanguageServer>, zed::Error> {
        // 初始化 LSP 客户端
        init_lsp_client().await.map_err(|e| {
            zed::Error::user(format!("LSP 客户端初始化失败：{}", e))
        })?;

        // 设置工作区根目录
        let root_path = workspace.root().path().to_path_buf();
        set_lsp_root(root_path).await.map_err(|e| {
            zed::Error::user(format!("设置 LSP 工作区失败：{}", e))
        })?;

        // 返回 Zed 兼容的 LSP 服务包装器
        Ok(Box::new(CangjieLanguageServer))
    }
}

/// 仓颉 LSP 服务包装器（适配 Zed LSP 接口）
struct CangjieLanguageServer;

impl zed::LanguageServer for CangjieLanguageServer {
    /// 发送 LSP 请求（示例：补全请求）
    async fn send_request(
        &mut self,
        request: zed::lsp::Request,
    ) -> Result<zed::lsp::Response, zed::Error> {
        let mut client = lsp::get_lsp_client().await.ok_or_else(|| {
            zed::Error::user("LSP 客户端未初始化")
        })?;

        // 转发请求到 LSP 服务端
        let response = client.send_raw_request(request.method, request.params).await.map_err(|e| {
            zed::Error::user(format!("LSP 请求失败：{}", e))
        })?;

        Ok(zed::lsp::Response {
            id: request.id,
            result: response.result,
            error: response.error,
        })
    }

    /// 发送 LSP 通知
    async fn send_notification(
        &mut self,
        notification: zed::lsp::Notification,
    ) -> Result<(), zed::Error> {
        let mut client = lsp::get_lsp_client().await.ok_or_else(|| {
            zed::Error::user("LSP 客户端未初始化")
        })?;

        client.send_raw_notification(notification.method, notification.params).await.map_err(|e| {
            zed::Error::user(format!("LSP 通知失败：{}", e))
        })?;

        Ok(())
    }
}

// 在 activate 函数中添加 LSP 初始化日志
#[zed::extension]
fn activate(workspace: &zed::Workspace) -> Result<Box<dyn Extension>> {
    init_logger();
    info!("Cangjie Zed Extension v0.3.0 activated");
    
    // 预初始化 LSP（可选，提升首次使用体验）
    tokio::spawn(async move {
        if let Err(e) = init_lsp_client().await {
            warn!("LSP 预初始化失败：{}", e);
        }
    });

    Ok(Box::new(CangjieZedExtension))
}
```

### 3. LSP 依赖配置（Cargo.toml 补充）
```toml
# LSP 相关依赖
[dependencies]
lsp-client = "0.10.0"
lsp-types = "0.94.0"
tokio = { version = "1.0", features = ["full"] }
log = "0.4"
env_logger = "0.10"

# 仅开发环境需要的 LSP 模拟依赖
[dev-dependencies]
lsp-server = "0.7.0"
```

## 二十四、Zed 多人协作调试支持
Zed 核心特性之一是多人协作，以下是调试器的协作能力扩展：

### 1. 协作调试状态同步（src/debugger/collab.rs）
```rust
//! 多人协作调试状态同步
use super::*;
use zed_extension_api::collab::{CollabState, CollabStateProvider};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

/// 协作调试状态（序列化后同步给所有协作成员）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollabDebugState {
    /// 当前调试会话 ID
    session_id: String,
    /// 宇宙实例状态
    cosmos_state: CosmosInspectState,
    /// 当前断点列表
    breakpoints: Vec<CangjieBreakpoint>,
    /// 调试模式
    debug_mode: CangjieDebugMode,
    /// 是否暂停
    is_paused: bool,
    /// 当前操作人
    current_operator: String,
}

impl CollabState for CollabDebugState {
    /// 状态类型标识
    fn type_id() -> &'static str {
        "cangjie-collab-debug"
    }
}

/// 协作调试状态提供器
pub struct CollabDebugStateProvider {
    debugger: Arc<Mutex<CangjieDebugger>>,
}

impl CollabDebugStateProvider {
    pub fn new(debugger: Arc<Mutex<CangjieDebugger>>) -> Self {
        Self { debugger }
    }
}

impl CollabStateProvider for CollabDebugStateProvider {
    /// 获取当前状态（用于同步给新加入的协作成员）
    async fn get_state(&self) -> Result<Box<dyn CollabState>, zed::Error> {
        let debugger = self.debugger.lock().await;
        let cosmos_state = debugger.inspect_cosmos()?;
        let breakpoints = debugger.get_breakpoints().into_iter().cloned().collect();

        Ok(Box::new(CollabDebugState {
            session_id: uuid::Uuid::new_v4().to_string(),
            cosmos_state,
            breakpoints,
            debug_mode: debugger.config.debug_mode.clone(),
            is_paused: debugger.is_paused,
            current_operator: zed::collab::current_user_id().await?,
        }))
    }

    /// 接收远程状态更新（同步其他成员的操作）
    async fn update_state(&self, state: Box<dyn CollabState>) -> Result<(), zed::Error> {
        let collab_state = state.as_any().downcast_ref::<CollabDebugState>()
            .ok_or_else(|| zed::Error::user("无效的协作调试状态"))?;

        let mut debugger = self.debugger.lock().await;

        // 同步断点列表
        debugger.set_breakpoints(collab_state.breakpoints.clone())?;
        // 同步调试模式
        debugger.config.debug_mode = collab_state.debug_mode.clone();
        // 同步暂停状态
        debugger.is_paused = collab_state.is_paused;

        info!(
            "协作调试状态更新：操作人={}, 状态={:?}",
            collab_state.current_operator,
            collab_state.cosmos_state.evolution_status
        );

        Ok(())
    }

    /// 发送本地状态更新（将本地操作同步给其他成员）
    async fn send_update(&self, operator: &str) -> Result<(), zed::Error> {
        let debugger = self.debugger.lock().await;
        let cosmos_state = debugger.inspect_cosmos()?;
        let breakpoints = debugger.get_breakpoints().into_iter().cloned().collect();

        let state = CollabDebugState {
            session_id: uuid::Uuid::new_v4().to_string(),
            cosmos_state,
            breakpoints,
            debug_mode: debugger.config.debug_mode.clone(),
            is_paused: debugger.is_paused,
            current_operator: operator.to_string(),
        };

        // 通过 Zed 协作 API 发送状态更新
        zed::collab::broadcast_state(Box::new(state)).await?;
        Ok(())
    }
}

/// 协作调试操作封装（单步、继续、暂停等）
impl CangjieDebugger {
    /// 协作模式下的单步执行
    pub async fn collab_step_over(&mut self, operator: &str) -> Result<()> {
        self.step_over()?;
        // 发送状态更新给其他协作成员
        let collab_state = CollabDebugState {
            session_id: uuid::Uuid::new_v4().to_string(),
            cosmos_state: self.inspect_cosmos()?,
            breakpoints: self.get_breakpoints().into_iter().cloned().collect(),
            debug_mode: self.config.debug_mode.clone(),
            is_paused: self.is_paused,
            current_operator: operator.to_string(),
        };
        zed::collab::broadcast_state(Box::new(collab_state)).await?;
        Ok(())
    }

    /// 协作模式下添加断点
    pub async fn collab_add_breakpoint(&mut self, breakpoint: CangjieBreakpoint, operator: &str) -> Result<()> {
        self.add_breakpoint(breakpoint)?;
        // 发送状态更新
        self.broadcast_collab_state(operator).await?;
        Ok(())
    }

    /// 广播当前状态
    async fn broadcast_collab_state(&self, operator: &str) -> Result<()> {
        let state = CollabDebugState {
            session_id: uuid::Uuid::new_v4().to_string(),
            cosmos_state: self.inspect_cosmos()?,
            breakpoints: self.get_breakpoints().into_iter().cloned().collect(),
            debug_mode: self.config.debug_mode.clone(),
            is_paused: self.is_paused,
            current_operator: operator.to_string(),
        };
        zed::collab::broadcast_state(Box::new(state)).await?;
        Ok(())
    }
}
```

### 2. 协作调试集成到扩展（src/lib.rs 补充）
```rust
// 导入协作调试模块
use debugger::collab::{CollabDebugState, CollabDebugStateProvider};

// 实现 Zed 协作状态提供器 trait
impl zed_extension_api::CollabStateProvider for CangjieZedExtension {
    fn state_types(&self) -> Vec<&str> {
        vec![CollabDebugState::type_id()]
    }

    async fn create_state_provider(
        &self,
        state_type: &str,
    ) -> Result<Box<dyn zed::CollabStateProvider>, zed::Error> {
        if state_type != CollabDebugState::type_id() {
            return Err(zed::Error::user(format!("不支持的协作状态类型：{}", state_type)));
        }

        // 创建调试器实例（Arc 共享）
        let config = CangjieDebugConfig::default();
        let (event_sender, _) = tokio::sync::mpsc::channel(10);
        let debugger = Arc::new(tokio::sync::Mutex::new(
            CangjieDebugger::new(config, event_sender)?
        ));

        // 返回协作状态提供器
        Ok(Box::new(CollabDebugStateProvider::new(debugger)))
    }
}

// 调试适配器添加协作支持（src/debugger/adapter.rs 补充）
impl CangjieDebugAdapter {
    /// 协作模式下处理调试请求
    async fn handle_collab_request(
        &mut self,
        request: zed::debug::Request,
        operator: &str,
    ) -> Result<zed::debug::Response, zed::Error> {
        match request.method.as_str() {
            "next" => {
                // 单步执行
                self.debugger.lock().await.collab_step_over(operator).await?;
                Ok(zed::debug::Response::success(request.id, serde_json::Value::Null))
            }
            "continue" => {
                // 继续执行
                self.debugger.lock().await.continue_()?;
                self.debugger.lock().await.broadcast_collab_state(operator).await?;
                Ok(zed::debug::Response::success(request.id, serde_json::Value::Null))
            }
            "pause" => {
                // 暂停执行
                self.debugger.lock().await.pause()?;
                self.debugger.lock().await.broadcast_collab_state(operator).await?;
                Ok(zed::debug::Response::success(request.id, serde_json::Value::Null))
            }
            "setBreakpoints" => {
                // 设置断点
                let params: zed::debug::SetBreakpointsParams = serde_json::from_value(request.params)?;
                let breakpoint = CangjieBreakpoint::from_zed_breakpoint(params.breakpoints[0].clone())?;
                self.debugger.lock().await.collab_add_breakpoint(breakpoint, operator).await?;
                Ok(zed::debug::Response::success(request.id, serde_json::Value::Null))
            }
            _ => {
                // 其他请求转发到普通处理逻辑
                self.handle_request(request).await
            }
        }
    }
}
```

### 3. 协作调试使用说明（补充到 README.md）
```markdown
## 多人协作调试
Zed 扩展支持多人协作调试，多个开发者可实时同步调试状态：

### 启用协作调试
1. 发起者创建 Zed 协作会话（快捷键：Ctrl+Shift+K → 选择「创建协作会话」）
2. 邀请其他成员加入会话（通过链接或二维码）
3. 发起者启动调试（F5），所有成员将自动同步调试状态

### 协作调试特性
- 实时同步：断点列表、宇宙演化状态、调试模式自动同步给所有成员
- 操作权限：任何成员可执行单步、继续、暂停操作，操作结果实时同步
- 状态显示：调试面板显示当前操作人，便于协作沟通

### 协作调试限制
- 仅支持同一宇宙实例的协作调试（多人共享一个宇宙演化进程）
- 跨载体迁移调试时，所有成员需访问同一载体资源
```

## 二十五、扩展兼容性与迁移指南
### 1. 兼容性说明（补充到 README.md）
```markdown
## 兼容性说明
### 支持环境
| 环境 | 要求 |
|------|------|
| Zed 版本 | ≥ v0.130.0（需支持协作调试和 LSP 扩展） |
| Rust 版本 | ≥ 1.75.0 |
| 操作系统 | macOS（arm64/x86_64）、Linux（x86_64）、Windows（x86_64） |
| 仓颉 LSP 版本 | ≥ v0.1.0（需单独安装，见下文） |

### 依赖安装
#### 仓颉 LSP 服务端（必需，用于代码补全/跳转）
```bash
# 克隆 LSP 服务端仓库（假设单独维护）
git clone https://github.com/cangjie-lang/cangjie-lsp.git
cd cangjie-lsp
cargo install --path .
```

### 不兼容变更
- v0.3.0：调试配置参数 `migrate_breakpoints` 重命名为 `migrate_breakpoints`（原 `migrate_breakpoints` 废弃）
- v0.2.0：移除对 Zed v0.120.0 及以下版本的支持
```

### 2. 版本迁移指南（补充到 CHANGELOG.md）
```markdown
## [0.3.0] - 2025-XX-XX
### Added
- 多人协作调试支持，实时同步调试状态
- LSP 集成，支持代码补全、跳转定义、悬停提示
- 协作调试状态显示，标识当前操作人

### Changed
- 调试配置参数 `migrate_breakpoints` 重命名为 `migrate_breakpoints`（保持一致）
- 优化图标加载性能，使用 Zed 图标缓存机制
- 扩展元数据完善，适配 Zed 扩展市场展示

### Deprecated
- 废弃 `cargo-about` 手动配置，改用自动许可证检查
- 移除对 Zed v0.120.0 及以下版本的支持

### Migration Guide
#### 调试配置迁移
旧配置（v0.2.0）：
```json
{
  "migrate_breakpoints": ["CosmosSerialization"]
}
```
新配置（v0.3.0）：
```json
{
  "migrate_breakpoints": ["CosmosSerialization"]
}
```
（注：仅名称统一，参数值不变）

#### Zed 版本迁移
若使用 Zed v0.120.0 及以下版本，需升级 Zed 至 v0.130.0+，或使用扩展 v0.2.0 版本：
```bash
# 切换到 v0.2.0 版本
git checkout v0.2.0
cargo build --release
```
```

## 二十六、项目最终总结与展望
### 1. 项目总结
`cangjie-zed-extension` 已发展为**功能完整、工程化成熟、生态兼容**的仓颉语言 Zed 扩展，核心亮点：
- 全栈能力：语法主题、图标主题、调试器、LSP 集成、协作调试全覆盖
- 生态适配：深度贴合 Zed 核心特性（协作、LSP、调试面板）
- 工程化完善：测试覆盖、文档齐全、打包发布流程标准化
- 社区友好：贡献指南清晰，支持多人协作开发

### 2. 未来展望
#### 短期规划（v0.4.0）
- 完善 LSP 功能：支持代码重构、错误诊断、格式化
- 优化协作调试：添加操作权限控制（如仅发起者可修改断点）
- 扩展主题定制：支持用户自定义色彩和图标

#### 中期规划（v0.5.0）
- 多宇宙并行调试：支持同时调试多个宇宙实例
- 法则冲突自动修复：基于 LSP 提供修复建议
- 集成仓颉包管理器：支持依赖自动补全和跳转

#### 长期规划（v1.0.0）
- 全功能 LSP 支持：覆盖仓颉语言所有语法特性
- 跨编辑器兼容：提炼核心逻辑为独立库，支持 VS Code 等其他编辑器
- 云协作调试：支持通过云端载体进行跨地域协作调试

## 二十七、最终项目目录（完整版）
整合所有进阶功能后，最终项目目录如下：
```
cangjie-zed-extension/
├── Cargo.toml                # 项目配置（含 LSP/协作依赖）
├── Cargo.lock                # 依赖锁定文件
├── LICENSE                   # MIT 许可证
├── README.md                 # 扩展说明文档（含协作/LSP 说明）
├── CHANGELOG.md              # 更新日志（含迁移指南）
├── CONTRIBUTING.md           # 贡献指南
├── schemas/                  # 调试配置 JSON Schema
│   └── cangjie-debug-schema.json
├── icons/                    # 图标资源目录
│   ├── dark/
│   └── light/
├── themes/                   # 语法主题配置
│   ├── cangjie-dark.toml
│   ├── cangjie-light.toml
│   └── cangjie-high-contrast.toml
├── src/                      # 源代码目录
│   ├── lib.rs                # 扩展入口（集成 LSP/协作）
│   ├── icon_theme/           # 图标主题模块
│   ├── syntax_theme/         # 语法主题模块
│   ├── debugger/             # 调试器模块
│   │   ├── collab.rs         # 协作调试支持
│   │   ├── adapter.rs        # 调试适配器（协作扩展）
│   │   └── debugger_test.rs  # 调试器测试
│   ├── lsp/                  # LSP 集成模块
│   │   ├── client.rs         # LSP 客户端封装
│   │   └── lsp_test.rs       # LSP 测试（可选）
│   └── tests/                # 集成测试
├── examples/                 # 模拟数据
│   ├── test.cosmos
│   ├── law-physics-001.cosmic.law
│   └── launch.json
├── script/                   # 脚本目录
│   └── licenses/
│       └── zed-licenses.toml
├── assets/                   # 扩展市场截图
├── build.sh                  # 构建脚本
├── test.sh                   # 测试脚本
├── build.bat                 # Windows 构建脚本
└── package.sh                # 扩展打包脚本
```

## 二十八、结语
`cangjie-zed-extension` 从基础的主题/图标扩展，逐步演进为支持 LSP、多人协作调试的全功能开发工具，完全贴合仓颉语言「宇宙演化」「跨载体迁移」的核心特性，同时深度适配 Zed 编辑器的协作和性能优势。

项目具备良好的可扩展性和维护性，可作为仓颉语言生态的核心开发工具，助力开发者提升开发效率、简化协作流程。后续将持续跟进 Zed 编辑器和仓颉语言的版本更新，不断完善功能、优化体验。

如需进一步扩展或反馈问题，欢迎通过 GitHub Issues 或 Discord 社区联系我们！