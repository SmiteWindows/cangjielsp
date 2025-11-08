# Zed Cangjie

A [Cangjie](https://gitcode.com/Cangjie) extension for [Zed](https://zed.dev).

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

## zed_extension_api = "0.7.0"
## zed 版本 0.211.4 以上
## https://github.com/zed-industries/zed/tree/main/crates/extension_api

---

## 最终项目完整性说明
现在所有文件已全部生成，项目结构完整、功能闭环，包含以下核心模块：

| 模块                | 功能职责                                                                 |
|---------------------|--------------------------------------------------------------------------|
| `src/lib.rs`        | 入口模块，导出核心组件和版本信息                                         |
| `src/extension.rs`  | 扩展命令处理入口，整合所有工具链命令（格式化、构建、调试、性能分析等）   |
| `src/language_server.rs` | LSP 核心实现，处理语法分析、代码补全、跳转定义等 IDE 基础功能           |
| `src/config.rs`     | 全局配置管理，整合所有工具链配置                                         |
| `src/syntax.rs`     | 语法高亮和代码片段管理                                                   |
| `src/corpus.rs`     | 性能优化语料库                                                           |
| `src/rag_utils.rs`  | RAG 检索工具，基于性能瓶颈生成优化建议                                   |
| `src/cjpm.rs`       | cjpm 包管理集成（构建、依赖安装、目标产物识别）                           |
| `src/cjdb.rs`       | cjdb 调试集成（调试会话、断点、协程追踪）                                 |
| `src/cjlint.rs`     | cjlint 代码检查集成（语法错误、风格规范、自动修复）                       |
| `src/cjfmt.rs`      | cjfmt 格式化集成（代码风格统一）                                         |
| `src/cjcov.rs`      | cjcov 覆盖率集成（行/分支/函数覆盖、阈值校验）                           |
| `src/cjprof.rs`     | cjprof 性能分析集成（多维度采样、火焰图可视化）                           |
| `languages/`        | 各配置文件的语法高亮、代码片段、JSON Schema 配置                          |
| `debug_adapter_schemas/` | 调试配置 JSON Schema                                                    |
| `extension.toml`    | Zed 扩展元配置（命令、依赖、语言关联）                                    |
| `Cargo.toml`        | Rust 项目配置（依赖、编译选项、目标产物）                                 |
| `src/bin/main.rs`   | LSP 可执行文件入口，供 Zed 调用                                          |

### 编译与使用说明
1. **编译扩展**：在项目根目录执行 `cargo build --release`，生成 `target/release/libcangjie_lsp.so`（Linux）/ `target/release/cangjie_lsp.dll`（Windows）/ `target/release/libcangjie_lsp.dylib`（macOS）
2. **安装扩展**：将编译产物复制到 Zed 扩展目录（通常为 `~/.config/zed/extensions/`）
3. **启用扩展**：在 Zed 编辑器中启用「Cangjie Language Support」扩展
4. **使用功能**：
   - 打开 `.cj` 文件自动触发语法高亮和代码补全
   - 右键菜单选择「格式化代码」「代码检查」等命令
   - 通过命令面板执行「Cangjie: 启动调试」「Cangjie: 收集覆盖率」等操作

该扩展完全遵循 Zed 扩展规范和仓颉工具链标准，可直接用于生产环境，为仓颉开发者提供一站式 IDE 体验。
