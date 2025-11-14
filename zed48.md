# 附录：扩展配置参考与开发指南
## 十四、扩展配置详情
扩展支持通过 Zed 配置文件自定义功能行为，所有配置项均提供默认值，无需手动配置即可正常使用。

### 1. 核心配置（`settings.json`）
```json
{
  "cangjie": {
    // 基础配置
    "stdx_version": "0.3.0", // 默认使用的 stdx 版本（自动适配宇宙文件配置）
    "cosmos_default_path": "${workspace}/cosmos", // 宇宙文件默认存储目录
    "law_file_pattern": "*.cosmic.law", // 法则文件匹配模式

    // AI 功能配置
    "ai": {
      "enabled": true, // 全局启用/禁用 AI 功能
      "code_completion": {
        "enabled": true, // 启用 AI 代码补全
        "confidence_threshold": 0.6, // AI 补全置信度阈值（0.0-1.0）
        "cache_ttl": 300 // 补全结果缓存过期时间（秒）
      },
      "law_conflict_check": {
        "enabled": true, // 启用 AI 法则冲突检测
        "risk_threshold": 0.3 // 冲突风险警告阈值（0.0-1.0）
      }
    },

    // 调试配置
    "debug": {
      "multi_cosmos": {
        "max_sessions": 4, // 最大并行调试会话数（默认 4）
        "session_cleanup_ttl": 30, // 会话停止后清理时间（秒）
        "lazy_load": {
          "evolution_history_limit": 100, // 懒加载演化历史条数
          "law_validation_history_limit": 50 // 懒加载法则验证历史条数
        }
      },
      "breakpoint": {
        "enable_magic_method_breakpoints": true, // 启用魔术方法断点
        "enable_law_breakpoints": true // 启用法则执行断点
      }
    },

    // 性能优化配置
    "performance": {
      "incremental_parsing": true, // 启用文档增量解析
      "hot_symbol_caching": true, // 启用热点符号缓存
      "hot_symbol_cache_size": 1000, // 热点符号缓存最大条数
      "document_cache_ttl": 300, // 文档解析结果缓存过期时间（秒）
      "lsp_response_compression": true // 启用 LSP 响应压缩
    },

    // CLI 工具链配置
    "cli": {
      "path": "", // Cangjie CLI 路径（留空自动检测）
      "build_optimize": true, // 构建宇宙文件时启用优化
      "validate_strict": true // 验证宇宙文件时启用严格模式
    }
  }
}
```

### 2. 配置项说明
| 配置路径 | 类型 | 说明 | 默认值 |
|----------|------|------|--------|
| `cangjie.stdx_version` | string | 全局默认 stdx 版本，宇宙文件未指定时使用 | "0.3.0" |
| `cangjie.ai.enabled` | boolean | 全局开关 AI 所有功能 | true |
| `cangjie.ai.code_completion.confidence_threshold` | number | AI 补全结果置信度阈值，低于该值的结果不显示 | 0.6 |
| `cangjie.debug.multi_cosmos.max_sessions` | number | 限制同时运行的多宇宙调试会话数，避免资源占用过高 | 4 |
| `cangjie.performance.hot_symbol_cache_size` | number | 热点符号缓存最大条数，超出后按 LRU 淘汰 | 1000 |
| `cangjie.cli.path` | string | Cangjie CLI 可执行文件路径，留空时自动从 PATH 或环境变量检测 | "" |

### 3. 快捷键配置（`keybindings.json`）
扩展支持自定义快捷键，以下是默认快捷键（可覆盖）：
```json
[
  // 调试相关
  {
    "binding": "ctrl+shift+c",
    "command": "cangjie.debug.createMultiCosmosSession",
    "description": "创建多宇宙调试会话"
  },
  {
    "binding": "ctrl+shift+s",
    "command": "cangjie.debug.switchMultiCosmosSession",
    "description": "切换多宇宙调试会话"
  },
  {
    "binding": "ctrl+shift+z",
    "command": "cangjie.debug.compareSelectedCosmos",
    "description": "对比选中的宇宙会话"
  },

  // AI 功能相关
  {
    "binding": "ctrl+shift+a",
    "command": "cangjie.ai.generateMagicMethod",
    "description": "AI 生成魔术方法"
  },
  {
    "binding": "ctrl+shift+r",
    "command": "cangjie.ai.refactorCode",
    "description": "AI 重构选中代码"
  },

  // CLI 工具链相关
  {
    "binding": "ctrl+shift+b",
    "command": "cangjie.cli.buildCosmos",
    "description": "构建宇宙文件"
  },
  {
    "binding": "ctrl+shift+v",
    "command": "cangjie.cli.validateCosmos",
    "description": "验证宇宙文件"
  }
]
```

## 十五、扩展开发指南（贡献代码）
### 1. 开发环境准备
#### 前置依赖
- Rust 1.75+（推荐使用 `rustup` 安装）
- Node.js 18+（用于 Zed 扩展开发工具链）
- Zed 0.211+（开发和测试环境）
- Cangjie 生态工具链（`cangjie_runtime` 0.5.0+、`cangjie_cli` 0.5.0+）

#### 环境配置
1. 克隆扩展仓库：
   ```bash
   git clone https://gitcode.com/Cangjie/cangjie-zed-extension.git
   cd cangjie-zed-extension
   ```
2. 安装依赖：
   ```bash
   # 安装 Rust 依赖
   cargo install --path .

   # 安装 Node.js 依赖（用于 Zed 扩展打包）
   npm install
   ```
3. 配置开发环境变量：
   ```bash
   # Linux/macOS
   export CANGJIE_DEV_MODE=true
   export CANGJIE_AI_API_KEY="your-dev-api-key"
   export CANGJIE_CLI_PATH="path/to/dev/cangjie-cli"

   # Windows
   set CANGJIE_DEV_MODE=true
   set CANGJIE_AI_API_KEY="your-dev-api-key"
   set CANGJIE_CLI_PATH="path/to/dev/cangjie-cli.exe"
   ```

### 2. 项目结构说明
```
cangjie-zed-extension/
├── src/
│   ├── ai/                # AI 辅助功能模块
│   ├── cli/               # Cangjie CLI 工具链集成模块
│   ├── commands/          # 命令处理器（AI、CLI、调试等）
│   ├── debugger/          # 调试核心模块（单宇宙、多宇宙）
│   ├── lsp/               # LSP 协议实现（补全、跳转、诊断等）
│   ├── optimization/      # 性能优化模块
│   ├── syntax/            # 仓颉语法解析、高亮模块
│   ├── ui/                # 自定义 UI 组件（可视化面板等）
│   ├── utils/             # 工具函数（配置、日志、路径等）
│   └── lib.rs             # 扩展入口文件
├── assets/                # 静态资源（图标、语法高亮配置等）
├── Cargo.toml             # Rust 项目配置
├── package.json           # Node.js 项目配置（Zed 扩展打包）
└── README.md              # 项目说明文档
```

### 3. 核心模块开发规范
#### （1）LSP 模块开发
- 所有 LSP 协议实现需遵循 [LSP 3.17 规范](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
- 新增 LSP 功能需在 `src/lsp/mod.rs` 中注册对应的处理器
- 性能敏感操作（如解析、搜索）需集成性能优化模块的缓存机制

#### （2）调试模块开发
- 单宇宙调试功能扩展需实现 `CangjieDebugger` trait
- 多宇宙调试功能需通过 `MultiCosmosDebugManager` 管理会话
- 调试数据加载需优先使用懒加载机制，避免阻塞 UI

#### （3）AI 模块开发
- 新增 AI 功能需在 `AiFeature` 枚举中添加对应的类型
- AI 请求需通过 `Cangjie AI SDK` 发送，避免直接调用 HTTP 接口
- AI 结果需添加缓存机制，避免重复请求

### 4. 测试流程
#### （1）本地测试
1. 在 Zed 中打开扩展项目
2. 运行 `cargo build` 构建扩展
3. 打开 Zed 命令面板，输入「Extensions: Load Local Extension」
4. 选择项目根目录下的 `target/debug` 文件夹，加载开发版扩展
5. 新建仓颉项目（包含 `.cosmic.law` 和 `cosmos.toml`），测试功能

#### （2）单元测试
```bash
# 运行所有单元测试
cargo test

# 运行指定模块测试（如 AI 模块）
cargo test ai::tests -- --nocapture
```

#### （3）集成测试
1. 构建扩展包：
   ```bash
   npm run package
   ```
2. 生成的扩展包位于 `target/package` 目录
3. 在 Zed 中安装扩展包，测试生产环境下的功能稳定性

### 5. 贡献代码流程
1. Fork 仓库到个人账号
2. 创建特性分支（如 `feature/ai-code-refactor`）
3. 提交代码（遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范）
4. 运行测试确保无报错
5. 提交 Pull Request 到主仓库
6. 等待审核通过后合并

### 6. 开发资源
- Zed 扩展开发文档：[Zed Extension API](https://zed.dev/docs/extensions)
- 仓颉生态文档：[Cangjie Developer Docs](https://developer.cangjie-lang.org/)
- LSP 协议规范：[Language Server Protocol Specification](https://microsoft.github.io/language-server-protocol/)
- Rust 开发指南：[The Rust Programming Language](https://doc.rust-lang.org/book/)

## 十六、扩展版本历史
### v1.0.0（2025-11-10）
- 初始版本，适配 Zed 0.211+
- 核心功能：语法高亮、LSP 补全/跳转/诊断、单宇宙调试
- 生态集成：Cangjie CLI、stdx 标准库、CangjieMagic
- 性能优化：增量解析、热点缓存、调试数据懒加载

### v1.1.0（规划中）
- 新增 AI 辅助开发功能（代码补全、重构、法则冲突检测）
- 新增多宇宙并行调试功能
- 增强可视化面板（宇宙对比、3D 演化视图）
- 优化 CLI 工具链集成（支持更多命令、自定义配置）

### v1.2.0（规划中）
- 支持 CangjieMagic 2.0 新特性
- 集成仓颉测试框架
- 增强协作开发功能（多用户共享调试会话）
- 全平台性能优化（Windows/macOS/Linux 深度适配）

## 十七、版权与许可
本扩展基于 [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) 开源许可协议分发：
```
Copyright 2025 Cangjie Lang Team

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

### 第三方依赖许可
- Rust 依赖：遵循各自的开源许可（详见 `Cargo.toml`）
- Zed Extension API：[MIT License](https://opensource.org/licenses/MIT)
- Cangjie AI SDK：[Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0)

## 总结
本扩展是仓颉生态与 Zed 编辑器深度融合的产物，旨在为开发者提供高效、稳定、全功能的仓颉开发体验。从基础的语法支持到高级的 AI 辅助开发，从单宇宙调试到多宇宙并行实验，从本地开发到生态工具链联动，扩展覆盖了仓颉开发的全流程。

无论是个人开发者进行小规模实验，还是企业团队开展大规模项目开发，本扩展都能提供适配的功能和性能优化。我们欢迎所有开发者参与到扩展的开发和优化中，共同推动仓颉生态的发展！

如需进一步了解扩展功能或遇到问题，可通过以下渠道获取支持：
- 官方文档：[Cangjie Zed Extension Docs](https://docs.cangjie-lang.org/extensions/zed)
- GitHub 仓库：[cangjie-zed-extension](https://gitcode.com/Cangjie/cangjie-zed-extension)
- 开发者社区：[Cangjie Developer Forum](https://forum.cangjie-lang.org/)
- 联系邮箱：extensions@cangjie-lang.org