### 附录 W：扩展 AI 辅助高级功能（Copilot 级集成）
基于 Zed 内置 AI 能力和第三方大模型 API，扩展提供 Copilot 级别的 AI 辅助编程功能，覆盖代码生成、重构、调试、文档生成全流程。

#### W.1 AI 功能架构设计
```
┌─────────────────────┐      ┌─────────────────────┐      ┌─────────────────────┐
│  AI 功能入口层      │      │  AI 服务适配层      │      │  大模型服务层       │
│  - 命令面板         │─────▶│  - Zed 内置 AI      │─────▶│  - Zed AI 服务      │
│  - 右键菜单         │      │  - OpenAI API       │      │  - OpenAI GPT-4     │
│  - 快捷键           │      │  - Anthropic API    │      │  - Anthropic Claude │
│  - 自动触发         │      │  - 本地化模型       │      │  - 本地化 LLaMA 等  │
└─────────────────────┘      └─────────────────────┘      └─────────────────────┘
        ▲                              ▲                              ▲
        │                              │                              │
        ▼                              ▼                              ▼
┌─────────────────────┐      ┌─────────────────────┐      ┌─────────────────────┐
│  上下文采集层       │      │  提示词工程层       │      │  结果处理层         │
│  - 代码上下文       │      │  - 功能模板         │      │  - 代码格式化       │
│  - 项目结构         │      │  - 语言特性注入     │      │  - 语法校验         │
│  - 历史操作         │      │  - 上下文压缩       │      │  - 冲突解决         │
│  - 错误信息         │      │  - 多轮对话管理     │      │  - 增量插入         │
└─────────────────────┘      └─────────────────────┘      └─────────────────────┘
```

#### W.2 核心 AI 功能实现
##### 1. 配置定义（`src/config/ai.rs`）
```rust
//! AI 功能配置
use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AiConfig {
    /// 启用 AI 辅助功能
    pub enabled: bool,
    /// AI 服务提供商（zed/openai/anthropic/local）
    pub provider: AiProvider,
    /// API 密钥（第三方服务必填）
    pub api_key: Option<String>,
    /// API 基础 URL（自定义部署）
    pub api_base_url: Option<String>,
    /// 模型名称
    pub model: String,
    /// 温度参数（0-1，越低越精确）
    pub temperature: f32,
    /// 最大生成 tokens
    pub max_tokens: u32,
    /// 自动触发 AI 补全的阈值（字符数）
    pub auto_completion_threshold: usize,
    /// 上下文窗口大小（前后代码行数）
    pub context_window_lines: u32,
    /// 启用多轮对话记忆
    pub enable_conversation_memory: bool,
    /// 对话记忆大小（轮数）
    pub conversation_memory_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AiProvider {
    /// Zed 内置 AI 服务
    Zed,
    /// OpenAI API
    OpenAI,
    /// Anthropic API
    Anthropic,
    /// 本地化模型（如 LLaMA）
    Local,
}

impl Default for AiProvider {
    fn default() -> Self {
        Self::Zed
    }
}
```

##### 2. AI 服务适配层（`src/ai/service.rs`）
```rust
//! AI 服务适配层
use super::config::AiConfig;
use zed_extension_api::{self as zed, Result};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// AI 对话消息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiMessage {
    pub role: AiMessageRole,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AiMessageRole {
    System,
    User,
    Assistant,
}

/// AI 服务抽象 trait
#[async_trait::async_trait]
pub trait AiService: Send + Sync {
    /// 生成 AI 响应
    async fn generate(
        &self,
        messages: &[AiMessage],
        config: &AiConfig,
    ) -> Result<String>;
}

/// AI 服务管理器
pub struct AiServiceManager {
    config: Arc<AiConfig>,
    service: Arc<dyn AiService>,
    conversation_memory: Arc<Mutex<Vec<AiMessage>>>,
}

impl AiServiceManager {
    /// 初始化 AI 服务管理器
    pub fn new(config: AiConfig) -> Result<Self> {
        let config = Arc::new(config);
        let service: Arc<dyn AiService> = match config.provider {
            AiProvider::Zed => Arc::new(ZedAiService::new()),
            AiProvider::OpenAI => Arc::new(OpenAiService::new(&config)?),
            AiProvider::Anthropic => Arc::new(AnthropicAiService::new(&config)?),
            AiProvider::Local => Arc::new(LocalAiService::new(&config)?),
        };

        Ok(Self {
            config,
            service,
            conversation_memory: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// 生成 AI 响应（带对话记忆）
    pub async fn generate_with_memory(
        &self,
        user_message: &str,
        system_prompt: Option<&str>,
    ) -> Result<String> {
        let mut messages = Vec::new();

        // 添加系统提示词
        if let Some(prompt) = system_prompt {
            messages.push(AiMessage {
                role: AiMessageRole::System,
                content: prompt.to_string(),
            });
        }

        // 添加对话记忆
        if self.config.enable_conversation_memory {
            let memory = self.conversation_memory.lock().await;
            messages.extend(memory.clone());
        }

        // 添加当前用户消息
        messages.push(AiMessage {
            role: AiMessageRole::User,
            content: user_message.to_string(),
        });

        // 生成 AI 响应
        let response = self.service.generate(&messages, &self.config).await?;

        // 更新对话记忆
        if self.config.enable_conversation_memory {
            let mut memory = self.conversation_memory.lock().await;
            memory.push(AiMessage {
                role: AiMessageRole::User,
                content: user_message.to_string(),
            });
            memory.push(AiMessage {
                role: AiMessageRole::Assistant,
                content: response.clone(),
            });

            // 限制记忆大小
            if memory.len() > self.config.conversation_memory_size as usize * 2 {
                memory.drain(0..2);
            }
        }

        Ok(response)
    }

    /// 清空对话记忆
    pub async fn clear_memory(&self) -> Result<()> {
        let mut memory = self.conversation_memory.lock().await;
        memory.clear();
        Ok(())
    }
}

/// Zed 内置 AI 服务实现
struct ZedAiService;

impl ZedAiService {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl AiService for ZedAiService {
    async fn generate(
        &self,
        messages: &[AiMessage],
        _config: &AiConfig,
    ) -> Result<String> {
        // 转换为 Zed AI API 格式
        let zed_messages: Vec<zed::ai::Message> = messages
            .iter()
            .map(|msg| zed::ai::Message {
                role: match msg.role {
                    AiMessageRole::System => zed::ai::MessageRole::System,
                    AiMessageRole::User => zed::ai::MessageRole::User,
                    AiMessageRole::Assistant => zed::ai::MessageRole::Assistant,
                },
                content: msg.content.clone(),
            })
            .collect();

        // 调用 Zed 内置 AI 服务
        let response = zed::ai::generate(zed::ai::GenerateRequest {
            model: "zed-ai".to_string(),
            messages: zed_messages,
            temperature: 0.7,
            max_tokens: 1024,
            ..zed::ai::GenerateRequest::default()
        }).await?;

        Ok(response.content)
    }
}

/// OpenAI API 服务实现
struct OpenAiService {
    client: reqwest::Client,
    api_key: String,
    api_base_url: String,
}

impl OpenAiService {
    fn new(config: &AiConfig) -> Result<Self> {
        let api_key = config.api_key.as_ref().ok_or_else(|| {
            zed::Error::user("OpenAI API key is required for OpenAI provider")
        })?;
        let api_base_url = config.api_base_url.as_deref().unwrap_or("https://api.openai.com/v1").to_string();

        Ok(Self {
            client: reqwest::Client::new(),
            api_key: api_key.clone(),
            api_base_url,
        })
    }
}

#[async_trait::async_trait]
impl AiService for OpenAiService {
    async fn generate(
        &self,
        messages: &[AiMessage],
        config: &AiConfig,
    ) -> Result<String> {
        #[derive(Serialize)]
        struct OpenAiRequest {
            model: String,
            messages: Vec<OpenAiMessage>,
            temperature: f32,
            max_tokens: u32,
        }

        #[derive(Serialize)]
        struct OpenAiMessage {
            role: String,
            content: String,
        }

        let openai_messages: Vec<OpenAiMessage> = messages
            .iter()
            .map(|msg| OpenAiMessage {
                role: match msg.role {
                    AiMessageRole::System => "system".to_string(),
                    AiMessageRole::User => "user".to_string(),
                    AiMessageRole::Assistant => "assistant".to_string(),
                },
                content: msg.content.clone(),
            })
            .collect();

        let request = OpenAiRequest {
            model: config.model.clone(),
            messages: openai_messages,
            temperature: config.temperature,
            max_tokens: config.max_tokens,
        };

        let response = self.client
            .post(format!("{}/chat/completions", self.api_base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| zed::Error::user(format!("OpenAI API request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(zed::Error::user(format!(
                "OpenAI API error ({}): {}",
                response.status(),
                error_body
            )));
        }

        #[derive(Deserialize)]
        struct OpenAiResponse {
            choices: Vec<OpenAiChoice>,
        }

        #[derive(Deserialize)]
        struct OpenAiChoice {
            message: OpenAiMessage,
        }

        let response: OpenAiResponse = response.json().await?;
        Ok(response.choices[0].message.content.clone())
    }
}

// Anthropic/Local 服务实现类似，略...
```

##### 3. 核心 AI 功能实现（代码生成）
```rust
//! src/ai/features/code_generation.rs
use super::service::{AiServiceManager, AiMessage};
use zed_extension_api::{self as zed, Result};
use crate::config::load_config;
use crate::utils::code::extract_code_blocks;

/// AI 代码生成功能
pub async fn generate_code(
    ai_manager: &AiServiceManager,
    prompt: &str,
    context: &str,
    document: &zed::Document,
) -> Result<String> {
    // 构建系统提示词（注入 Cangjie 语言特性）
    let system_prompt = format!(
        r#"You are a Cangjie programming language expert. Your task is to generate correct, efficient, and idiomatic Cangjie code based on the user's request.

Cangjie Language Rules:
1. Syntax is similar to Rust but with simpler syntax for common tasks.
2. Function declaration: fn name(params) -> return_type { ... }
3. Variable declaration: let mutable_var = value; const immutable_var = value;
4. Struct declaration: struct Name { field: type, ... }
5. Enum declaration: enum Name { Variant1, Variant2(value), ... }
6. Error handling uses Result type with ? operator.
7. No semicolons required at the end of statements (optional).
8. Indentation is 4 spaces (mandatory).

Requirements:
- Generate only valid Cangjie code, no explanations unless requested.
- Follow the project's code style (infer from context).
- Optimize for performance and readability.
- If the request is ambiguous, ask for clarification (but keep it brief).
- Use the provided context to ensure code fits into the existing project."#
    );

    // 构建用户提示词（包含上下文）
    let user_prompt = if context.is_empty() {
        prompt.to_string()
    } else {
        format!(
            "Context (existing code/project structure):\n{}\n\nRequest: {}",
            context, prompt
        )
    };

    // 调用 AI 生成代码
    let ai_response = ai_manager
        .generate_with_memory(&user_prompt, Some(&system_prompt))
        .await?;

    // 提取代码块（处理 AI 响应中的 ```cangjie ... ``` 格式）
    let code_blocks = extract_code_blocks(&ai_response, "cangjie");
    let generated_code = if !code_blocks.is_empty() {
        code_blocks.join("\n\n")
    } else {
        // 如果没有代码块，直接使用响应内容（假设是纯代码）
        ai_response
    };

    // 语法校验（确保生成的代码可解析）
    let parsed_tree = tree_sitter_cangjie::parse(&generated_code, None)?;
    if parsed_tree.root_node().has_error() {
        zed::workspace::current().show_warning_message(
            "Generated code has syntax errors. Please check and adjust manually."
        )?;
    }

    Ok(generated_code)
}

/// 自动代码补全（基于光标上下文）
pub async fn auto_complete(
    ai_manager: &AiServiceManager,
    document: &zed::Document,
    cursor_pos: &zed::lsp::Position,
) -> Result<Option<String>> {
    let config = load_config()?;
    if !config.ai.enabled || config.ai.auto_completion_threshold == 0 {
        return Ok(None);
    }

    // 采集光标前后的上下文
    let context_lines = config.ai.context_window_lines as usize;
    let start_line = cursor_pos.line.saturating_sub(context_lines as u32) as usize;
    let end_line = (cursor_pos.line + context_lines as u32) as usize;
    let context = document.text_in_range(&zed::lsp::Range {
        start: zed::lsp::Position { line: start_line as u32, character: 0 },
        end: zed::lsp::Position { line: end_line as u32, character: 0 },
    })?;

    // 获取光标前的代码（判断是否达到触发阈值）
    let prefix = document.text_in_range(&zed::lsp::Range {
        start: zed::lsp::Position { line: 0, character: 0 },
        end: cursor_pos.clone(),
    })?;
    let last_line = prefix.lines().last().unwrap_or("");
    if last_line.len() < config.ai.auto_completion_threshold {
        return Ok(None);
    }

    // 构建自动补全提示词
    let prompt = format!(
        "Continue the following Cangjie code. Keep the style consistent. Do not add extra explanations. Only generate the continuation:\n\n{}",
        prefix
    );

    // 调用 AI 生成补全内容
    let completion = generate_code(ai_manager, &prompt, &context, document).await?;

    // 过滤掉与前缀重复的内容
    let trimmed_completion = completion.trim_start_matches(prefix.trim_end());
    if trimmed_completion.is_empty() {
        return Ok(None);
    }

    Ok(Some(trimmed_completion.to_string()))
}
```

##### 4. AI 功能入口注册（`src/extension.rs`）
```rust
//! AI 功能入口注册
use super::ai::{service::AiServiceManager, features::{code_generation, code_refactoring, code_debugging, doc_generation}};
use zed_extension_api::{self as zed, commands::CommandContext};
use std::sync::Arc;

/// 全局 AI 服务管理器
static AI_MANAGER: std::sync::Mutex<Option<Arc<AiServiceManager>>> = std::sync::Mutex::new(None);

/// 初始化 AI 服务
pub fn init_ai() -> Result<(), zed::Error> {
    let config = crate::config::load_config()?.ai;
    if !config.enabled {
        crate::utils::log::info!("AI features are disabled in config");
        return Ok(());
    }

    let ai_manager = Arc::new(AiServiceManager::new(config)?);
    *AI_MANAGER.lock()? = Some(ai_manager);
    crate::utils::log::info!("AI service initialized successfully");
    Ok(())
}

/// 注册 AI 相关命令
pub fn register_ai_commands() -> Result<(), zed::Error> {
    // AI 代码生成
    zed::commands::register_command(
        "cangjie.ai.generateCode",
        "Cangjie: AI Generate Code",
        |context: CommandContext| async move {
            let ai_manager = get_ai_manager()?;
            let document = context.document.ok_or_else(|| {
                zed::Error::user("No active document")
            })?;

            // 获取用户提示
            let prompt = zed::ui::show_input_box(
                "AI Code Generation",
                "Enter what you want to generate (e.g. 'create a JSON parser' or 'implement binary search')",
            ).await?
            .ok_or_else(|| zed::Error::user("Prompt cancelled"))?;

            // 采集上下文（选中内容或光标周围代码）
            let context = get_code_context(&document)?;

            // 显示加载状态
            let workspace = document.workspace()?;
            workspace.show_status_message("Generating code with AI...")?;

            // 生成代码
            let generated_code = code_generation::generate_code(
                &ai_manager,
                &prompt,
                &context,
                &document,
            ).await?;

            // 插入代码到文档
            let cursor_pos = document.cursor_position()?;
            document.insert_text(cursor_pos, &generated_code).await?;

            workspace.show_info_message("AI code generation completed successfully")?;
            Ok(())
        },
    )?;

    // AI 代码重构
    zed::commands::register_command(
        "cangjie.ai.refactorCode",
        "Cangjie: AI Refactor Code",
        |context: CommandContext| async move {
            let ai_manager = get_ai_manager()?;
            let document = context.document.ok_or_else(|| {
                zed::Error::user("No active document")
            })?;

            // 获取选中的代码
            let selected_code = document.selection()
                .and_then(|sel| document.text_in_range(&sel).ok())
                .ok_or_else(|| {
                    zed::Error::user("Please select code to refactor")
                })?;

            // 获取重构需求
            let prompt = zed::ui::show_input_box(
                "AI Code Refactoring",
                "Enter refactoring request (e.g. 'make this code more efficient' or 'simplify this function')",
            ).await?
            .ok_or_else(|| zed::Error::user("Prompt cancelled"))?;

            let workspace = document.workspace()?;
            workspace.show_status_message("Refactoring code with AI...")?;

            // 执行重构
            let refactored_code = code_refactoring::refactor_code(
                &ai_manager,
                &selected_code,
                &prompt,
                &document,
            ).await?;

            // 替换选中的代码
            let selection = document.selection().unwrap();
            document.edit(zed::lsp::TextEdit {
                range: selection,
                new_text: refactored_code,
            }).await?;

            workspace.show_info_message("AI code refactoring completed successfully")?;
            Ok(())
        },
    )?;

    // AI 代码调试
    zed::commands::register_command(
        "cangjie.ai.debugCode",
        "Cangjie: AI Debug Code",
        |context: CommandContext| async move {
            // 类似实现，略...
            Ok(())
        },
    )?;

    // AI 生成文档
    zed::commands::register_command(
        "cangjie.ai.generateDocs",
        "Cangjie: AI Generate Documentation",
        |context: CommandContext| async move {
            // 类似实现，略...
            Ok(())
        },
    )?;

    // 清空 AI 对话记忆
    zed::commands::register_command(
        "cangjie.ai.clearMemory",
        "Cangjie: AI Clear Conversation Memory",
        |_context: CommandContext| async move {
            let ai_manager = get_ai_manager()?;
            ai_manager.clear_memory().await?;
            zed::workspace::current().show_info_message("AI conversation memory cleared")?;
            Ok(())
        },
    )?;

    Ok(())
}

/// 获取 AI 服务管理器（辅助函数）
fn get_ai_manager() -> Result<Arc<AiServiceManager>, zed::Error> {
    AI_MANAGER.lock()?
        .clone()
        .ok_or_else(|| zed::Error::user("AI features are disabled or not initialized. Check your config and restart Zed."))
}

/// 采集代码上下文（辅助函数）
fn get_code_context(document: &zed::Document) -> Result<String, zed::Error> {
    match document.selection() {
        Some(selection) => document.text_in_range(&selection),
        None => {
            // 采集光标周围 10 行作为上下文
            let cursor_pos = document.cursor_position()?;
            let start_line = cursor_pos.line.saturating_sub(10);
            let end_line = cursor_pos.line.saturating_add(10);
            document.text_in_range(&zed::lsp::Range {
                start: zed::lsp::Position { line: start_line, character: 0 },
                end: zed::lsp::Position { line: end_line, character: 0 },
            })
        }
    }
}
```

### 附录 X：扩展容器化部署与 CI/CD 流水线
为简化扩展开发、测试和发布流程，提供完整的容器化部署方案和 CI/CD 流水线配置。

#### X.1 容器化配置（Docker）
##### 1. Dockerfile（`Dockerfile`）
```dockerfile
# 基础镜像（Rust + Node.js + Zed 扩展开发依赖）
FROM rust:1.78-slim AS base

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# 安装 Tree-sitter CLI
RUN npm install -g tree-sitter-cli@0.20.8

# 安装 Rust 目标平台（WASM）
RUN rustup target add wasm32-unknown-unknown

# 安装 Cargo 工具链
RUN cargo install \
    cargo-about \
    cargo-audit \
    cargo-udeps \
    criterion \
    wasm-pack

# 设置工作目录
WORKDIR /workspace

# 复制依赖文件并缓存
COPY Cargo.toml Cargo.lock ./
COPY tree-sitter-cangjie/Cargo.toml tree-sitter-cangjie/
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target wasm32-unknown-unknown
RUN rm -rf src target/wasm32-unknown-unknown/release/deps/zed_cangjie_extension*

# 复制源代码
COPY . .

# 构建扩展
FROM base AS build
RUN cargo build --release --target wasm32-unknown-unknown
RUN zed extensions package --output /workspace/zed-cangjie-extension.zed-extension

# 测试镜像
FROM base AS test
RUN cargo test --all --verbose
RUN cargo audit
RUN cargo clippy --all -- -D warnings
RUN cargo fmt --check
RUN tree-sitter test -q ./tree-sitter-cangjie

# 最终镜像（仅包含扩展包和必要工具）
FROM alpine:3.19 AS final
WORKDIR /output
COPY --from=build /workspace/zed-cangjie-extension.zed-extension ./
COPY --from=build /workspace/CHANGELOG.md ./
COPY --from=build /workspace/README.md ./
COPY --from=build /workspace/LICENSE ./
CMD ["echo", "Cangjie extension build completed. Output files are in /output"]
```

##### 2. Docker Compose 配置（`docker-compose.yml`）
```yaml
version: '3.8'

services:
  build:
    build:
      context: .
      target: build
    volumes:
      - ./dist:/workspace/dist
    environment:
      - RUST_LOG=info
      - CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

  test:
    build:
      context: .
      target: test
    environment:
      - RUST_LOG=info
      - CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

  lint:
    build:
      context: .
      target: base
    command: >
      sh -c "cargo clippy --all -- -D warnings &&
             cargo fmt --check &&
             cargo audit &&
             tree-sitter test -q ./tree-sitter-cangjie"
    environment:
      - RUST_LOG=info

  clean:
    build:
      context: .
      target: base
    command: cargo clean
    volumes:
      - ./target:/workspace/target
      - ./tree-sitter-cangjie/target:/workspace/tree-sitter-cangjie/target

  shell:
    build:
      context: .
      target: base
    volumes:
      - .:/workspace
    stdin_open: true
    tty: true
    environment:
      - RUST_LOG=info
```

#### X.2 CI/CD 流水线配置（GitHub Actions）
##### 1. 主流水线（`.github/workflows/main.yml`）
```yaml
name: Cangjie Extension CI/CD

on:
  push:
    branches: [ main, develop ]
    tags: [ 'v*' ]
  pull_request:
    branches: [ main, develop ]

jobs:
  lint:
    name: Lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      - name: Run lint
        uses: docker/compose@v2
        with:
          command: run --rm lint

  test:
    name: Test
    runs-on: ubuntu-latest
    needs: lint
    steps:
      - uses: actions/checkout@v4
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      - name: Run tests
        uses: docker/compose@v2
        with:
          command: run --rm test

  build:
    name: Build
    runs-on: ubuntu-latest
    needs: test
    steps:
      - uses: actions/checkout@v4
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      - name: Build extension
        uses: docker/compose@v2
        with:
          command: run --rm build
      - name: Extract version
        id: extract_version
        run: |
          VERSION=$(grep '^version =' Cargo.toml | sed 's/version = "//;s/"//')
          echo "VERSION=$VERSION" >> $GITHUB_ENV
      - name: Rename extension package
        run: |
          mv dist/zed-cangjie-extension.zed-extension dist/zed-cangjie-extension-v${{ env.VERSION }}.zed-extension
      - name: Upload build artifacts
        uses: actions/upload-artifact@v4
        with:
          name: zed-cangjie-extension-v${{ env.VERSION }}
          path: |
            dist/*.zed-extension
            CHANGELOG.md
            README.md
            LICENSE

  release:
    name: Release
    runs-on: ubuntu-latest
    needs: build
    if: startsWith(github.ref, 'refs/tags/')
    steps:
      - uses: actions/checkout@v4
      - name: Download build artifacts
        uses: actions/download-artifact@v4
        with:
          name: zed-cangjie-extension-v${{ github.ref_name }}
          path: dist
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          title: "Cangjie Extension v${{ github.ref_name }}"
          body_path: CHANGELOG.md
          files: |
            dist/*.zed-extension
            CHANGELOG.md
            README.md
            LICENSE
          draft: false
          prerelease: ${{ contains(github.ref_name, 'alpha') || contains(github.ref_name, 'beta') }}
      - name: Publish to Zed Extension Marketplace
        uses: zed-industries/zed-extension-publish-action@v1
        with:
          extension-file: dist/zed-cangjie-extension-v${{ github.ref_name }}.zed-extension
          api-key: ${{ secrets.ZED_EXTENSION_API_KEY }}
          release-notes: ${{ github.event.head_commit.message }}
```

##### 2. 定期依赖更新流水线（`.github/workflows/update-deps.yml`）
```yaml
name: Update Dependencies

on:
  schedule:
    - cron: '0 0 * * 0' # 每周日凌晨运行
  workflow_dispatch: # 允许手动触发

jobs:
  update-deps:
    name: Update Dependencies
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      - name: Update Cargo dependencies
        run: |
          docker-compose run --rm base cargo update
          docker-compose run --rm base cd tree-sitter-cangjie && cargo update
      - name: Create pull request
        uses: peter-evans/create-pull-request@v6
        with:
          title: "Update dependencies"
          body: "Automated dependency update from CI"
          branch: update-deps
          commit-message: "chore: update dependencies"
          delete-branch: true
```

### 最终终极总结（含 AI 与 CI/CD）
Cangjie 扩展历经多轮迭代，已形成**功能完备、生态完善、工程化成熟**的 Zed 编辑器扩展解决方案，核心亮点如下：

#### 1. 全栈功能覆盖
- **基础能力**：语法高亮、LSP 全功能（补全/格式化/跳转/诊断）、Tree-sitter 语法解析；
- **进阶特性**：外部工具集成、远程开发适配、自定义主题、国际化、可访问性优化；
- **AI 辅助**：Copilot 级代码生成/重构/调试/文档生成，支持多模型适配（Zed 内置/OpenAI/Anthropic/本地化）；
- **工程化工具**：完整测试方案（单元/集成/E2E/性能）、容器化部署、CI/CD 流水线。

#### 2. 技术架构优势
- **模块化设计**：功能拆分清晰，各模块低耦合、高内聚，便于维护和扩展；
- **性能优化**：解析缓存、增量更新、请求节流、远程环境优化，确保流畅体验；
- **兼容性强**：支持三大平台、多版本 Zed、主流远程环境（SSH/容器/WSL）、WCAG 2.1 可访问性标准；
- **生态友好**：支持跨扩展集成、语言生态联动、第三方工具适配，扩展边界无限制。

#### 3. 开发体验极致
- **开发者友好**：完善的文档、示例代码、开发工具链，降低扩展开发门槛；
- **用户友好**：灵活配置、智能提示、AI 辅助、个性化定制，提升开发效率；
- **社区支持**：清晰的贡献指南、Issue 处理流程、PR 审核机制，鼓励社区参与。

#### 4. 工程化成熟度
- **质量保障**：多层测试策略、静态代码分析、安全审计、性能基准测试；
- **自动化部署**：容器化构建、CI/CD 流水线、自动版本管理、扩展市场发布；
- **可维护性**：规范的代码风格、详细的注释、自动化依赖更新、定期重构。

### 未来 roadmap
1. **AI 能力深化**：
   - 基于项目结构的智能补全；
   - 实时代码错误预测与修复；
   - 自然语言到代码的精准转换；
2. **生态集成扩展**：
   - 与 Cangjie 官方 IDE 联动；
   - 云开发环境适配；
   - 代码共享与协作功能；
3. **性能与体验优化**：
   - WebAssembly 编译优化；
   - 更智能的缓存策略；
   - 个性化配置同步功能；
4. **功能扩展**：
   - 调试器集成；
   - 代码覆盖率分析；
   - 多语言混合编程支持。

Cangjie 扩展不仅是一个简单的编辑器插件，更是 Cangjie 语言生态的核心组成部分。我们将持续投入资源，跟进 Zed 编辑器和 AI 技术的最新发展，为开发者提供更强大、更智能、更易用的开发工具。

感谢所有贡献者的辛勤付出，感谢 Zed 团队提供的优秀扩展平台，也感谢每一位用户的支持与反馈！

---

**文档版本**：v1.0.0（终极终极全量版）  
**发布日期**：2025-11-09  
**支持环境**：
- Zed 编辑器：v0.130.0+
- 操作系统：macOS 12+/Linux (Ubuntu 20.04+/Fedora 36+)/Windows 10+
- 远程环境：SSH/容器/Docker/WSL 2
- AI 服务：Zed AI/OpenAI GPT-3.5+/GPT-4/Anthropic Claude/Local LLaMA
**官方资源**：
- 代码仓库：https://github.com/your-username/zed-cangjie-extension
- 扩展市场：https://extensions.zed.dev/extensions/your-username/cangjie
- 文档站点：https://docs.cangjie-lang.org/zed-extension
- 社区支持：https://discord.gg/cangjie-lang
- 反馈渠道：https://github.com/your-username/zed-cangjie-extension/issues