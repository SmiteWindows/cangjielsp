# 优化后的 Cangjie Zed 扩展项目（遵循 Zed 官方规范）
基于 Zed 官方文档（`docs/src`）的扩展开发规范、LSP 集成标准、用户体验最佳实践，对项目进行全面优化，以下是完整的文件目录和独立文件内容。

## 项目文件目录
```
cangjie-zed-extension/
├── Cargo.toml                # Rust 项目配置（依赖、版本、目标平台）
├── Cargo.lock                # 依赖锁定文件（自动生成）
├── package.json              # Zed 扩展配置（遵循官方 manifest 规范）
├── language-configuration.json  # 语言基础配置（注释、括号、折叠等）
├── README.md                 # 官方风格的安装使用文档
├── LICENSE                   # MIT 许可证
├── build.sh                  # 跨平台编译脚本（优化兼容性）
├── .github/
│   └── workflows/
│       └── ci.yml            # 官方风格 CI 配置（适配 Zed 扩展测试）
├── .gitignore                # Git 忽略规则（优化构建产物过滤）
├── examples/
│   ├── hello.cj              # 简单示例代码
│   ├── user.cj               # 结构体/方法示例
│   └── custom-rules.json     # 自定义规则示例
├── src/
│   ├── lib.rs                # 扩展入口（遵循 Zed Extension API 规范）
│   ├── config.rs             # 配置管理（序列化/默认值/校验）
│   ├── lsp/
│   │   ├── mod.rs            # LSP 模块导出
│   │   ├── server.rs         # LSP 服务实现（适配 Zed LSP 协议）
│   │   ├── diagnostics.rs    # 诊断功能（语法/风格检查）
│   │   ├── completion.rs     # 代码补全（片段+符号）
│   │   ├── definition.rs     # 跳转定义（文档内+跨文件）
│   │   ├── hover.rs          # 悬停提示（文档注释解析）
│   │   └── formatting.rs     # 代码格式化（遵循官方格式化协议）
│   ├── syntax/
│   │   ├── mod.rs            # 语法模块导出
│   │   ├── highlights.rs     # 语法高亮（适配 tree-sitter 节点）
│   │   ├── snippets.rs       # 代码片段（符合 Zed 片段规范）
│   │   └── tree_sitter_utils.rs  # tree-sitter 工具函数（官方兼容版）
│   ├── lint/
│   │   ├── mod.rs            # 检查模块导出
│   │   ├── rules.rs          # 内置检查规则（命名/语法/风格）
│   │   └── custom.rs         # 自定义规则解析（JSON 规范）
│   └── utils/
│       ├── mod.rs            # 工具模块导出
│       ├── file.rs           # 文件操作（安全读取/路径处理）
│       ├── log.rs            # 日志工具（适配 Zed 日志系统）
│       └── error.rs          # 错误处理（统一错误类型）
└── target/                   # 构建产物（自动生成）
```

## 各文件独立内容

### 1. Cargo.toml
```toml
[package]
name = "cangjie-zed-extension"
version = "0.6.0"
edition = "2021"
description = "Cangjie 语言 Zed 扩展：语法高亮、补全、格式化、LSP 全功能支持"
license = "MIT"
authors = ["Your Name <your-email@example.com>"]
repository = "https://github.com/your-username/cangjie-zed-extension"
keywords = ["zed-extension", "cangjie", "language-server", "tree-sitter"]
categories = ["Development Tools", "Text Editors"]

[lib]
name = "cangjie_zed_extension"
crate-type = ["cdylib"]  # Zed 扩展要求的动态库类型

[dependencies]
# Zed 扩展核心依赖（严格遵循官方版本规范）
zed_extension_api = "0.130.0"
tree-sitter = "0.20.10"
tree-sitter-cangjie = "0.1.0"  # 假设的 Cangjie tree-sitter 语法包

# 功能依赖
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
regex = "1.10.0"
log = "0.4"
thiserror = "1.0"
walkdir = "2.4"
once_cell = "1.19.0"
anyhow = "1.0"

[dev-dependencies]
zed_extension_api = { version = "0.130.0", features = ["test-support"] }
tempfile = "3.8"
rstest = "0.18.2"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = "debuginfo"  # 减小发布产物体积

[profile.dev]
opt-level = 0
debug = true
```

### 2. package.json（遵循 Zed 扩展 Manifest 规范）
```json
{
  "name": "cangjie-zed-extension",
  "displayName": "Cangjie Language Support",
  "description": "Full-featured support for Cangjie language: syntax highlighting, auto-completion, formatting, LSP integration, and more.",
  "version": "0.6.0",
  "engines": {
    "zed": ">=0.130.0"  # 匹配 Zed 最低支持版本
  },
  "categories": [
    "Languages",
    "Formatters",
    "Linters",
    "Code Intelligence"
  ],
  "keywords": [
    "cangjie",
    "仓颉",
    "language-server",
    "zed-extension",
    "tree-sitter"
  ],
  "author": {
    "name": "Your Name",
    "email": "your-email@example.com",
    "url": "https://your-website.com"
  },
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/your-username/cangjie-zed-extension.git"
  },
  "bugs": {
    "url": "https://github.com/your-username/cangjie-zed-extension/issues"
  },
  "homepage": "https://github.com/your-username/cangjie-zed-extension#readme",
  "contributes": {
    "languages": [
      {
        "id": "cangjie",
        "aliases": ["Cangjie", "仓颉", "cj"],
        "extensions": [".cj"],
        "configuration": "./language-configuration.json",
        "icon": {
          "light": "./icons/cangjie-light.svg",
          "dark": "./icons/cangjie-dark.svg"
        }
      }
    ],
    "grammars": [
      {
        "language": "cangjie",
        "scopeName": "source.cangjie",
        "path": "./syntaxes/cangjie.tmLanguage.json"
      }
    ],
    "snippets": [
      {
        "language": "cangjie",
        "path": "./snippets/cangjie.json"
      }
    ],
    "configuration": {
      "title": "Cangjie Language Configuration",
      "properties": {
        "cangjie.lsp": {
          "type": "object",
          "default": {
            "timeout_ms": 5000,
            "trace": "off"
          },
          "description": "LSP server configuration",
          "properties": {
            "timeout_ms": {
              "type": "integer",
              "default": 5000,
              "minimum": 1000,
              "maximum": 30000,
              "description": "LSP request timeout in milliseconds"
            },
            "trace": {
              "type": "string",
              "default": "off",
              "enum": ["off", "messages", "verbose"],
              "description": "LSP trace level for debugging"
            }
          }
        },
        "cangjie.formatting": {
          "type": "object",
          "default": {
            "indent_style": "space",
            "indent_size": 4,
            "tab_width": 4,
            "line_ending": "lf",
            "max_line_length": 120,
            "function_brace_style": "same_line",
            "struct_brace_style": "same_line",
            "trailing_comma": "always",
            "space_around_operators": true,
            "space_inside_brackets": false,
            "auto_fix": true
          },
          "description": "Code formatting configuration",
          "properties": {
            "indent_style": {
              "type": "string",
              "enum": ["space", "tab"],
              "description": "Indentation style"
            },
            "indent_size": {
              "type": "integer",
              "minimum": 1,
              "maximum": 16,
              "description": "Number of spaces per indent (when using space indentation)"
            },
            "tab_width": {
              "type": "integer",
              "minimum": 1,
              "maximum": 16,
              "description": "Width of a tab character (when using tab indentation)"
            },
            "line_ending": {
              "type": "string",
              "enum": ["lf", "crlf"],
              "description": "Line ending style (LF for Unix, CRLF for Windows)"
            },
            "max_line_length": {
              "type": "integer",
              "minimum": 80,
              "maximum": 200,
              "description": "Maximum line length before wrapping"
            },
            "function_brace_style": {
              "type": "string",
              "enum": ["same_line", "next_line"],
              "description": "Brace placement for functions"
            },
            "struct_brace_style": {
              "type": "string",
              "enum": ["same_line", "next_line"],
              "description": "Brace placement for structs"
            },
            "trailing_comma": {
              "type": "string",
              "enum": ["always", "never", "multiline"],
              "description": "Trailing comma policy for arrays/structs"
            },
            "space_around_operators": {
              "type": "boolean",
              "description": "Add spaces around operators (+, =, ==, etc.)"
            },
            "space_inside_brackets": {
              "type": "boolean",
              "description": "Add spaces inside brackets ([], {}, ())"
            },
            "auto_fix": {
              "type": "boolean",
              "description": "Auto-fix minor syntax issues during formatting (e.g., missing semicolons)"
            }
          }
        },
        "cangjie.linting": {
          "type": "object",
          "default": {
            "enabled": true,
            "severity_level": "warning",
            "enable_syntax_checks": true,
            "enable_style_checks": true,
            "ignore_rules": [],
            "custom_rules_path": null
          },
          "description": "Code linting configuration",
          "properties": {
            "enabled": {
              "type": "boolean",
              "description": "Enable/disable linting"
            },
            "severity_level": {
              "type": "string",
              "enum": ["error", "warning", "information", "hint"],
              "description": "Minimum severity level to display"
            },
            "enable_syntax_checks": {
              "type": "boolean",
              "description": "Enable syntax error checking"
            },
            "enable_style_checks": {
              "type": "boolean",
              "description": "Enable code style checking (naming, formatting, etc.)"
            },
            "ignore_rules": {
              "type": "array",
              "items": {
                "type": "string"
              },
              "description": "List of rule IDs to ignore (e.g., [\"UNUSED_VARIABLE\", \"LINE_TOO_LONG\"])"
            },
            "custom_rules_path": {
              "type": ["string", "null"],
              "description": "Path to custom lint rules JSON file"
            }
          }
        },
        "cangjie.completion": {
          "type": "object",
          "default": {
            "enabled": true,
            "trigger_on_typing": true,
            "include_snippets": true,
            "include_workspace_symbols": true,
            "snippet_expansion": "tab"
          },
          "description": "Auto-completion configuration",
          "properties": {
            "enabled": {
              "type": "boolean",
              "description": "Enable/disable auto-completion"
            },
            "trigger_on_typing": {
              "type": "boolean",
              "description": "Trigger completion automatically as you type"
            },
            "include_snippets": {
              "type": "boolean",
              "description": "Include code snippets in completion results"
            },
            "include_workspace_symbols": {
              "type": "boolean",
              "description": "Include symbols from other files in the workspace"
            },
            "snippet_expansion": {
              "type": "string",
              "enum": ["tab", "enter"],
              "description": "Key to expand snippets (Tab or Enter)"
            }
          }
        }
      }
    }
  },
  "main": "./target/release/libcangjie_zed_extension.so",
  "activationEvents": [
    "onLanguage:cangjie",
    "onStartupFinished"
  ],
  "scripts": {
    "build": "cargo build --release",
    "dev": "cargo build",
    "test": "cargo test",
    "package": "zed extension package",
    "lint": "cargo clippy",
    "format": "cargo fmt"
  },
  "devDependencies": {
    "zed-cli": "^0.130.0"
  }
}
```

### 3. language-configuration.json（官方规范版）
```json
{
  "comments": {
    "lineComment": "//",
    "blockComment": ["/*", "*/"]
  },
  "brackets": [
    ["{", "}"],
    ["(", ")"],
    ["[", "]"]
  ],
  "autoClosingPairs": [
    {"open": "{", "close": "}"},
    {"open": "(", "close": ")"},
    {"open": "[", "close": "]"},
    {"open": "\"", "close": "\"", "notIn": ["string"]},
    {"open": "'", "close": "'", "notIn": ["string", "comment"]},
    {"open": "/*", "close": "*/", "notIn": ["string", "comment"]}
  ],
  "surroundingPairs": [
    {"open": "{", "close": "}"},
    {"open": "(", "close": ")"},
    {"open": "[", "close": "]"},
    {"open": "\"", "close": "\""},
    {"open": "'", "close": "'"},
    {"open": "/*", "close": "*/"}
  ],
  "folding": {
    "markers": {
      "start": "^\\s*//\\s*#region",
      "end": "^\\s*//\\s*#endregion"
    },
    "enable": true
  },
  "wordPattern": "\\w+|[^\u0000-\u007F\u2013-\u2014\u2026\u2018-\u2019\u201C-\u201D]+|[^\\s\\w\\[\\]\\{\\}\\(\\)\\;\\,\\.\\<\\>\\?\\!\\@\\#\\$\\%\\^\\&\\*\\-\\+\\=\\/\\|\\:\\\"\\'\\`\\~]++",
  "indentationRules": {
    "increaseIndentPattern": "^\\s*(fn|struct|enum|interface|impl|if|else|for|while|do|try|catch)\\s*[\\{\\(\\[]?",
    "decreaseIndentPattern": "^\\s*[\\}\\)\\]]\\s*;?\\s*$|^\\s*else\\s*if|^\\s*else"
  }
}
```

### 4. README.md（Zed 官方风格）
```markdown
# Cangjie Language Support for Zed

[![CI](https://github.com/your-username/cangjie-zed-extension/actions/workflows/ci.yml/badge.svg)](https://github.com/your-username/cangjie-zed-extension/actions/workflows/ci.yml)
[![Version](https://img.shields.io/badge/version-0.6.0-blue)](https://github.com/your-username/cangjie-zed-extension/releases)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

Full-featured extension for the Cangjie programming language, built to integrate seamlessly with Zed's native capabilities.

## Features

- ✅ **Syntax Highlighting**: Precise highlighting for keywords, types, constants, and comments (powered by Tree-sitter)
- ✅ **Intelligent Completion**: Auto-complete for symbols, snippets, and standard library functions
- ✅ **Code Formatting**: Configurable formatting with support for indentation, brace style, and more
- ✅ **Real-time Linting**: Syntax error detection and style checking (with custom rules support)
- ✅ **Go to Definition**: Jump to symbol definitions within files and across the workspace
- ✅ **Hover Information**: Detailed tooltips for functions, structs, and constants
- ✅ **Document Symbols**: Outline view for easy navigation of file structure
- ✅ **Auto-closing Pairs**: Automatic closing of brackets, quotes, and comments
- ✅ **Code Folding**: Fold functions, structs, and comment blocks

## Prerequisites

- Zed Editor: v0.130.0 or later (check in `Zed > About Zed`)
- Rust Toolchain (for building from source): v1.70.0 or later (install via [rustup](https://www.rust-lang.org/tools/install))

## Installation

### Option 1: Install from Zed Extension Marketplace (Recommended)

1. Open Zed
2. Navigate to the Extensions panel (`Cmd/Ctrl + Shift + X`)
3. Search for "Cangjie Language Support"
4. Click "Install"

### Option 2: Install Local Extension Package

1. Download the latest `.zed` package from the [Releases](https://github.com/your-username/cangjie-zed-extension/releases) page
2. Open Zed > Extensions > "Install Local Extension"
3. Select the downloaded `.zed` package
4. Restart Zed to activate the extension

### Option 3: Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/cangjie-zed-extension.git
   cd cangjie-zed-extension
   ```

2. Build the extension:
   ```bash
   # Build in release mode (recommended for performance)
   ./build.sh

   # Or build in development mode (for debugging)
   ./build.sh dev
   ```

3. Install the extension:
   - Open Zed > Extensions > "Install Local Extension"
   - Select the build output from `target/release/` (e.g., `libcangjie_zed_extension.so` on Linux)

## Getting Started

1. Create a new file with the `.cj` extension (e.g., `main.cj`)
2. Start writing Cangjie code (try typing `fn` and pressing `Tab` to expand a function snippet)
3. Use the features below to enhance your workflow

### Example Code

```cangjie
/**
 * A simple Cangjie program demonstrating core syntax
 */
fn main() -> Void {
    // Define a struct
    struct Person {
        name: String;
        age: Int;
    }

    // Create an instance
    let alice = Person {
        name: "Alice",
        age: 25,
    };

    // Call a function
    println(greet(alice.name));
}

/// Greet a user by name
/// @param name - The user's name
/// @return Greeting string
fn greet(name: String) -> String {
    return "Hello, " + name + "!";
}
```

## Usage Guide

### Syntax Highlighting

The extension provides syntax highlighting for all core Cangjie constructs:
- Keywords: `fn`, `let`, `const`, `struct`, `enum`, `interface`, `if`, `for`, etc.
- Types: `Int`, `String`, `Float`, `Bool`, and custom struct/enum names
- Constants: Uppercase snake_case names (e.g., `MAX_RETRY_COUNT`)
- Comments: Line comments (`//`), block comments (`/* */`), and doc comments (`/** */`)

### Code Completion

- **Trigger Completion**: Type naturally (completion triggers automatically) or press `Cmd/Ctrl + Space`
- **Expand Snippets**: Type a snippet trigger (e.g., `fn`, `struct`, `if`) and press `Tab` or `Enter` (configurable)
- **Workspace Symbols**: Completion includes symbols from all `.cj` files in your workspace

### Formatting

- **Format Document**: Use the shortcut `Cmd/Ctrl + Shift + I` or right-click > "Format Document"
- **Configure Formatting**: Customize rules in Zed's settings (see [Configuration](#configuration))
- **Auto-fix**: Minor syntax issues (e.g., missing semicolons) are fixed automatically during formatting

### Linting

- **Real-time Feedback**: Errors (red squiggles), warnings (yellow squiggles), and hints (blue squiggles) appear as you type
- **View Details**: Hover over squiggles to see issue descriptions and fixes
- **Custom Rules**: Use a JSON file to define your own lint rules (see [Custom Rules](#custom-rules))

### Go to Definition

- **Jump to Definition**: Hold `Cmd/Ctrl` and click a symbol, or select the symbol and press `F12`
- **Cross-file Jumps**: Works for symbols defined in other `.cj` files in your workspace

### Hover Information

Hover over any symbol to see:
- Functions: Signature (parameters, return type) and doc comments
- Structs: Fields and their types
- Constants: Value and type
- Standard Library Symbols: Official documentation

### Document Symbols

- Open the Outline panel (`Cmd/Ctrl + Shift + O`) to see a structured view of your file
- Click any symbol to jump to its location
- Symbols are grouped by type (functions, structs, enums, etc.)

## Configuration

Customize the extension by modifying Zed's settings (`Cmd/Ctrl + ,`). Here's an example configuration:

```json
{
  "cangjie": {
    "formatting": {
      "indent_style": "space",
      "indent_size": 2,
      "max_line_length": 100,
      "function_brace_style": "next_line",
      "trailing_comma": "multiline"
    },
    "linting": {
      "severity_level": "error",
      "ignore_rules": ["UNUSED_VARIABLE"],
      "custom_rules_path": "/path/to/custom-rules.json"
    },
    "completion": {
      "snippet_expansion": "enter",
      "include_workspace_symbols": true
    }
  }
}
```

See `package.json` for the full list of configurable options.

## Custom Rules

Create a JSON file to define custom lint rules. Example `custom-rules.json`:

```json
{
  "description": "Custom Cangjie lint rules",
  "regex_rules": [
    {
      "name": "NO_DEBUG_PRINT",
      "pattern": "debug_print\\(",
      "message": "Use `log_info` instead of `debug_print` in production code",
      "severity": "error",
      "ignore": false
    }
  ],
  "node_rules": [
    {
      "name": "NO_GLOBAL_VARS",
      "node_kind": "variable_declaration",
      "message": "Avoid global variables - use module-level constants instead",
      "severity": "warning"
    }
  ]
}
```

Specify the path to this file in your Zed settings (see [Configuration](#configuration)).

## Troubleshooting

### Extension Not Activating

- Ensure Zed is v0.130.0 or later
- Verify the extension is enabled in the Extensions panel
- Check the Zed logs (`Zed > Help > Show Log File`) for errors

### Syntax Highlighting Not Working

- Ensure your file has the `.cj` extension
- Restart Zed to reload the Tree-sitter grammar
- Check that the extension is activated (see above)

### Completion Not Triggering

- Ensure `cangjie.completion.enabled` is `true` in settings
- Verify you're editing a `.cj` file
- Press `Cmd/Ctrl + Space` to trigger completion manually

### Formatting Not Working

- Fix any syntax errors (red squiggles) in your code
- Check that `cangjie.formatting` settings are valid (no invalid values)
- Try using the right-click menu instead of the shortcut (to rule out shortcut conflicts)

## Development

### Setup

1. Clone the repository (see [Build from Source](#option-3-build-from-source))
2. Install dependencies:
   ```bash
   cargo install --path . --force
   ```

3. Run tests:
   ```bash
   cargo test
   ```

### Debugging

1. Build the extension in debug mode:
   ```bash
   ./build.sh dev
   ```

2. Open the project in Zed
3. Use Zed's extension debugging tools (`Zed > Extensions > Debug Extensions`)
4. Set `cangjie.lsp.trace` to `verbose` in settings to see LSP logs

### Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -m "Add your feature"`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a Pull Request

Please ensure your code passes linting (`cargo clippy`) and tests (`cargo test`).

## License

This extension is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgements

- Built with [Zed Extension API](https://zed.dev/docs/extensions)
- Powered by [Tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- Inspired by Zed's official language extensions
```

### 5. LICENSE
```
MIT License

Copyright (c) 2024 Your Name

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### 6. build.sh（跨平台优化版）
```bash
#!/bin/bash
set -euo pipefail

# Cangjie Zed Extension Build Script
# Compatible with Linux/macOS/Windows (WSL)
# Follows Zed extension build best practices

# --------------------------
# Configuration
# --------------------------
EXT_NAME="cangjie_zed_extension"
CARGO_CMD="cargo"
ZED_CLI="zed"

# --------------------------
# Dependency Checks
# --------------------------
check_dependency() {
    if ! command -v "$1" &> /dev/null; then
        echo "Error: $1 is not installed. Please install it first."
        exit 1
    fi
}

check_dependency "$CARGO_CMD"

# --------------------------
# Build Mode
# --------------------------
BUILD_MODE="release"
if [ "${1:-}" = "dev" ]; then
    BUILD_MODE="debug"
    echo "=== Building in DEVELOPMENT mode ==="
else
    echo "=== Building in RELEASE mode ==="
fi

# --------------------------
# Platform Detection
# --------------------------
PLATFORM="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$PLATFORM" in
    linux*)
        TARGET_EXT="so"
        ;;
    darwin*)
        TARGET_EXT="dylib"
        ;;
    msys*|mingw*|cygwin*)
        TARGET_EXT="dll"
        PLATFORM="windows"
        ;;
    *)
        echo "Warning: Unknown platform $PLATFORM. Building with default settings."
        TARGET_EXT="so"
        ;;
esac

# --------------------------
# Build Command
# --------------------------
echo "Building for $PLATFORM-$ARCH ($BUILD_MODE)..."
"$CARGO_CMD" build --$BUILD_MODE

# --------------------------
# Copy Artifact
# --------------------------
ARTIFACT_PATH="target/$BUILD_MODE/lib${EXT_NAME}.$TARGET_EXT"
if [ "$PLATFORM" = "windows" ]; then
    ARTIFACT_PATH="target/$BUILD_MODE/${EXT_NAME}.dll"
fi

if [ -f "$ARTIFACT_PATH" ]; then
    cp "$ARTIFACT_PATH" "./${EXT_NAME}-${PLATFORM}.$TARGET_EXT"
    echo "✅ Built artifact copied to: ./${EXT_NAME}-${PLATFORM}.$TARGET_EXT"
else
    echo "Error: Build artifact not found at $ARTIFACT_PATH"
    exit 1
fi

# --------------------------
# Package Extension (if Zed CLI is available)
# --------------------------
if command -v "$ZED_CLI" &> /dev/null; then
    echo "=== Packaging extension for Zed ==="
    "$ZED_CLI" extension package --output "${EXT_NAME}-${PLATFORM}.zed"
    echo "✅ Extension package created: ./${EXT_NAME}-${PLATFORM}.zed"
else
    echo "ℹ️ Zed CLI not found - skipping extension packaging. Install Zed to package."
fi

# --------------------------
# Post-build Instructions
# --------------------------
echo -e "\n=== Build Complete ==="
echo "Next steps:"
echo "1. Open Zed → Extensions → Install Local Extension"
echo "2. Select the built artifact (./${EXT_NAME}-${PLATFORM}.$TARGET_EXT) or .zed package"
echo "3. Create a .cj file to test the extension"
```

### 7. .github/workflows/ci.yml（Zed 官方风格 CI）
```yaml
name: Cangjie Zed Extension CI

on:
  push:
    branches: [ main ]
    paths:
      - '**.rs'
      - 'Cargo.toml'
      - 'Cargo.lock'
      - 'package.json'
      - '.github/workflows/ci.yml'
  pull_request:
    branches: [ main ]
    paths:
      - '**.rs'
      - 'Cargo.toml'
      - 'Cargo.lock'
      - 'package.json'
      - '.github/workflows/ci.yml'
  release:
    types: [ published ]

jobs:
  lint:
    name: Lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: clippy, rustfmt
          override: true
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      - name: Run rustfmt
        run: cargo fmt --check

  test:
    name: Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - name: Run tests
        run: cargo test --all-features --verbose

  build:
    name: Build
    needs: [lint, test]
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - name: Build release
        run: cargo build --release
      - name: Rename artifact
        id: rename
        run: |
          if [[ "${{ matrix.os }}" == "ubuntu-latest" ]]; then
            mv target/release/libcangjie_zed_extension.so cangjie_zed_extension-linux.so
            echo "artifact_path=cangjie_zed_extension-linux.so" >> $GITHUB_ENV
          elif [[ "${{ matrix.os }}" == "macos-latest" ]]; then
            mv target/release/libcangjie_zed_extension.dylib cangjie_zed_extension-macos.dylib
            echo "artifact_path=cangjie_zed_extension-macos.dylib" >> $GITHUB_ENV
          elif [[ "${{ matrix.os }}" == "windows-latest" ]]; then
            mv target/release/cangjie_zed_extension.dll cangjie_zed_extension-windows.dll
            echo "artifact_path=cangjie_zed_extension-windows.dll" >> $GITHUB_ENV
          fi
      - name: Upload build artifact
        uses: actions/upload-artifact@v4
        with:
          name: cangjie_zed_extension-${{ matrix.os }}
          path: ${{ env.artifact_path }}

  package:
    name: Package Extension
    needs: build
    runs-on: ubuntu-latest
    if: github.event_name == 'release'
    steps:
      - uses: actions/checkout@v4
      - name: Download all build artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts
      - name: Install Zed CLI
        run: |
          curl -fsSL https://zed.dev/install.sh | sh
          echo "$HOME/.zed/bin" >> $GITHUB_PATH
      - name: Package for Linux
        run: |
          cp artifacts/cangjie_zed_extension-ubuntu-latest/cangjie_zed_extension-linux.so target/release/libcangjie_zed_extension.so
          zed extension package --output cangjie-zed-extension-linux.zed
      - name: Package for macOS
        run: |
          cp artifacts/cangjie_zed_extension-macos-latest/cangjie_zed_extension-macos.dylib target/release/libcangjie_zed_extension.dylib
          zed extension package --output cangjie-zed-extension-macos.zed
      - name: Package for Windows
        run: |
          cp artifacts/cangjie_zed_extension-windows-latest/cangjie_zed_extension-windows.dll target/release/cangjie_zed_extension.dll
          zed extension package --output cangjie-zed-extension-windows.zed
      - name: Upload packages to release
        uses: softprops/action-gh-release@v2
        with:
          files: |
            cangjie-zed-extension-linux.zed
            cangjie-zed-extension-macos.zed
            cangjie-zed-extension-windows.zed
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### 8. .gitignore
```
# Rust build artifacts
/target/
Cargo.lock
*.rlib
*.dll
*.so
*.dylib
*.lib
*.exp
*.pdb
*.obj

# Zed extension packages
*.zed
*.zed.zip

# Log files
*.log
logs/

# Configuration files
.env
.env.local
custom-rules.json
settings.json

# Editor files
.idea/
.vscode/
*.swp
*.swo
.DS_Store
Thumbs.db

# Test artifacts
test_output/
tmp/
temp/

# Build script outputs
cangjie_zed_extension-*.so
cangjie_zed_extension-*.dylib
cangjie_zed_extension-*.dll

# Generated syntax files
syntaxes/cangjie.tmLanguage.json
snippets/cangjie.json
```

### 9. examples/hello.cj
```cangjie
/**
 * Hello World Example for Cangjie Language
 * Demonstrates:
 * - Function declaration
 * - Print statement
 * - String concatenation
 */
fn main() -> Void {
    // Print a simple message
    println("Hello, Cangjie!");

    // Call a helper function
    let name = "Zed User";
    println(greet(name));
}

/// Greet a user by name
/// @param name - The user's name (String)
/// @return Formatted greeting string
fn greet(name: String) -> String {
    return "Welcome, " + name + "! Let's code with Cangjie.";
}
```

### 10. examples/user.cj
```cangjie
/**
 * Struct and Method Example
 * Demonstrates:
 * - Struct definition
 * - Struct methods
 * - Interface implementation
 * - Enums and pattern matching
 */

// Define an enum for user roles
enum UserRole {
    Admin,
    Editor,
    Viewer,
}

// Define a struct for User
struct User {
    id: Int;
    username: String;
    email: String;
    role: UserRole;
    is_active: Bool;
}

// Define an interface for serializable types
interface Serializable {
    serialize(): String;
}

// Implement methods for User
impl User {
    /// Create a new User instance
    method new(id: Int, username: String, email: String, role: UserRole) -> User {
        return User {
            id: id,
            username: username,
            email: email,
            role: role,
            is_active: true,
        };
    }

    /// Deactivate the user
    method deactivate() -> Void {
        self.is_active = false;
    }

    /// Get the user's role as a string
    method get_role_name() -> String {
        match self.role {
            UserRole::Admin => return "Admin",
            UserRole::Editor => return "Editor",
            UserRole::Viewer => return "Viewer",
        }
    }
}

// Implement Serializable interface for User
impl User: Serializable {
    method serialize() -> String {
        return "{
            \"id\": " + self.id + ",
            \"username\": \"" + self.username + "\",
            \"email\": \"" + self.email + "\",
            \"role\": \"" + self.get_role_name() + "\",
            \"is_active\": " + (self.is_active ? "true" : "false") + "
        }";
    }
}

// Main function to test the code
fn main() -> Void {
    // Create a new user
    let admin = User::new(
        1,
        "admin_user",
        "admin@example.com",
        UserRole::Admin,
    );

    // Print user info
    println("User ID: " + admin.id);
    println("Role: " + admin.get_role_name());
    println("Serialized User: " + admin.serialize());

    // Deactivate the user
    admin.deactivate();
    println("User active status: " + (admin.is_active ? "true" : "false"));
}
```

### 11. examples/custom-rules.json
```json
{
  "description": "Example custom lint rules for Cangjie language",
  "regex_rules": [
    {
      "name": "NO_DEBUG_PRINT",
      "pattern": "debug_print\\(",
      "message": "Debug print statements are not allowed in production code. Use `log_info` instead.",
      "severity": "error",
      "ignore": false
    },
    {
      "name": "HARDCODED_CREDENTIALS",
      "pattern": "(password|pwd|secret|token)\\s*=\\s*[\"'][^\"']*[\"']",
      "message": "Hardcoded credentials detected. Use environment variables or a secure config system.",
      "severity": "error"
    },
    {
      "name": "LONG_LINE",
      "pattern": "^.{140,}$",
      "message": "Line exceeds 140 characters. Consider splitting into multiple lines for readability.",
      "severity": "warning"
    },
    {
      "name": "TODO_COMMENT",
      "pattern": "//\\s*TODO",
      "message": "TODO comment detected. Resolve the task or create a ticket to track it.",
      "severity": "information"
    }
  ],
  "node_rules": [
    {
      "name": "GLOBAL_VARIABLE",
      "node_kind": "variable_declaration",
      "message": "Global variables can lead to unexpected side effects. Use module-level constants or pass as function parameters.",
      "severity": "warning"
    },
    {
      "name": "EMPTY_STRUCT",
      "node_kind": "struct_declaration",
      "message": "Empty structs have no practical purpose. Add fields or remove the struct.",
      "severity": "warning"
    },
    {
      "name": "ANY_TYPE_USAGE",
      "node_kind": "type_identifier",
      "message": "Using `Any` type sacrifices type safety. Specify a concrete type instead.",
      "severity": "error"
    },
    {
      "name": "OVERLY_LONG_FUNCTION",
      "node_kind": "function_declaration",
      "message": "Function may be overly complex. Consider splitting into smaller, single-responsibility functions.",
      "severity": "information"
    }
  ]
}
```

### 12. src/lib.rs（扩展入口，遵循 Zed API 规范）
```rust
//! Cangjie Language Support Extension for Zed
//!
//! Provides full-featured support for the Cangjie programming language,
//! including syntax highlighting, completion, formatting, LSP integration, and more.

#![warn(missing_docs, clippy::missing_docs_in_private_items)]
#![doc(html_root_url = "https://docs.rs/cangjie-zed-extension/0.6.0")]

use zed_extension_api::{self as zed, Result, ExtensionContext, LanguageServerId};

pub mod config;
pub mod lsp;
pub mod syntax;
pub mod lint;
pub mod utils;

/// Extension activation function (called by Zed when the extension is activated)
#[zed::extension]
fn activate(context: ExtensionContext) -> Result<()> {
    // Initialize logging
    utils::log::init(context.log_level("cangjie.log_level")?)?;

    // Register language server
    let server_id = LanguageServerId::new("cangjie-lsp");
    context.register_language_server(server_id.clone(), lsp::server::CangjieLanguageServer::new)?;

    // Register language configuration
    context.register_language(zed::LanguageConfiguration {
        id: "cangjie".to_string(),
        server_id: Some(server_id),
        ..Default::default()
    })?;

    // Register syntax highlights (Tree-sitter based)
    syntax::highlights::register(&context)?;

    // Register code snippets
    syntax::snippets::register(&context)?;

    log::info!("Cangjie Zed Extension activated successfully");
    Ok(())
}

/// Extension deactivation function (called by Zed when the extension is disabled/uninstalled)
#[zed::extension_deactivate]
fn deactivate() -> Result<()> {
    log::info!("Cangjie Zed Extension deactivated");
    Ok(())
}
```

### 13. src/config.rs（配置管理）
```rust
//! Configuration management for the Cangjie Zed Extension
//!
//! Handles serialization, deserialization, default values, and validation of extension settings.
//! Aligns with the configuration schema defined in package.json.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use zed_extension_api::Result;

/// Top-level configuration for the Cangjie extension
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CangjieConfig {
    /// LSP server configuration
    pub lsp: LspConfig,

    /// Code formatting configuration
    pub formatting: FormattingConfig,

    /// Code linting configuration
    pub linting: LintingConfig,

    /// Auto-completion configuration
    pub completion: CompletionConfig,
}

/// LSP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LspConfig {
    /// LSP request timeout in milliseconds
    pub timeout_ms: u32,

    /// LSP trace level for debugging
    pub trace: LspTraceLevel,
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 5000,
            trace: LspTraceLevel::Off,
        }
    }
}

/// LSP trace level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LspTraceLevel {
    /// No tracing
    Off,

    /// Trace only messages (requests/responses)
    Messages,

    /// Verbose tracing (includes payloads)
    Verbose,
}

/// Code formatting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormattingConfig {
    /// Indentation style (space or tab)
    pub indent_style: IndentStyle,

    /// Number of spaces per indent (when using space indentation)
    pub indent_size: u8,

    /// Width of a tab character (when using tab indentation)
    pub tab_width: u8,

    /// Line ending style (LF or CRLF)
    pub line_ending: LineEnding,

    /// Maximum line length before wrapping
    pub max_line_length: u16,

    /// Brace placement for functions
    pub function_brace_style: BraceStyle,

    /// Brace placement for structs
    pub struct_brace_style: BraceStyle,

    /// Trailing comma policy for arrays/structs
    pub trailing_comma: TrailingCommaPolicy,

    /// Add spaces around operators (+, =, ==, etc.)
    pub space_around_operators: bool,

    /// Add spaces inside brackets ([], {}, ())
    pub space_inside_brackets: bool,

    /// Auto-fix minor syntax issues during formatting
    pub auto_fix: bool,
}

impl Default for FormattingConfig {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::Space,
            indent_size: 4,
            tab_width: 4,
            line_ending: LineEnding::Lf,
            max_line_length: 120,
            function_brace_style: BraceStyle::SameLine,
            struct_brace_style: BraceStyle::SameLine,
            trailing_comma: TrailingCommaPolicy::Always,
            space_around_operators: true,
            space_inside_brackets: false,
            auto_fix: true,
        }
    }
}

/// Indentation style
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IndentStyle {
    /// Use spaces for indentation
    Space,

    /// Use tabs for indentation
    Tab,
}

/// Line ending style
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LineEnding {
    /// Line Feed (Unix-style)
    Lf,

    /// Carriage Return + Line Feed (Windows-style)
    Crlf,
}

/// Brace placement style
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BraceStyle {
    /// Brace on the same line as the declaration
    SameLine,

    /// Brace on the next line after the declaration
    NextLine,
}

/// Trailing comma policy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TrailingCommaPolicy {
    /// Always add a trailing comma
    Always,

    /// Never add a trailing comma
    Never,

    /// Add a trailing comma only for multi-line collections
    Multiline,
}

/// Code linting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LintingConfig {
    /// Enable/disable linting
    pub enabled: bool,

    /// Minimum severity level to display
    pub severity_level: SeverityLevel,

    /// Enable syntax error checking
    pub enable_syntax_checks: bool,

    /// Enable code style checking
    pub enable_style_checks: bool,

    /// List of rule IDs to ignore
    pub ignore_rules: Vec<String>,

    /// Path to custom lint rules JSON file
    pub custom_rules_path: Option<PathBuf>,
}

impl Default for LintingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            severity_level: SeverityLevel::Warning,
            enable_syntax_checks: true,
            enable_style_checks: true,
            ignore_rules: Vec::new(),
            custom_rules_path: None,
        }
    }
}

/// Severity level for linting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SeverityLevel {
    /// Hint (lowest priority)
    Hint,

    /// Information
    Information,

    /// Warning
    Warning,

    /// Error (highest priority)
    Error,
}

/// Auto-completion configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionConfig {
    /// Enable/disable auto-completion
    pub enabled: bool,

    /// Trigger completion automatically as you type
    pub trigger_on_typing: bool,

    /// Include code snippets in completion results
    pub include_snippets: bool,

    /// Include symbols from other files in the workspace
    pub include_workspace_symbols: bool,

    /// Key to expand snippets (Tab or Enter)
    pub snippet_expansion: SnippetExpansionKey,
}

/// Snippet expansion key
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SnippetExpansionKey {
    /// Expand snippets with Tab key
    Tab,

    /// Expand snippets with Enter key
    Enter,
}

impl Default for SnippetExpansionKey {
    fn default() -> Self {
        Self::Tab
    }
}

/// Configuration validation errors
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Invalid indent size (must be between 1 and 16)
    #[error("Invalid indent size: {0} (must be between 1 and 16)")]
    InvalidIndentSize(u8),

    /// Invalid tab width (must be between 1 and 16)
    #[error("Invalid tab width: {0} (must be between 1 and 16)")]
    InvalidTabWidth(u8),

    /// Invalid max line length (must be between 80 and 200)
    #[error("Invalid max line length: {0} (must be between 80 and 200)")]
    InvalidMaxLineLength(u16),

    /// Invalid timeout (must be between 1000 and 30000)
    #[error("Invalid LSP timeout: {0} (must be between 1000 and 30000)")]
    InvalidTimeout(u32),

    /// Custom rules file not found
    #[error("Custom rules file not found: {0}")]
    CustomRulesFileNotFound(PathBuf),

    /// Custom rules file is not a valid JSON
    #[error("Custom rules file is not valid JSON: {0}")]
    InvalidCustomRulesJson(serde_json::Error),
}

impl CangjieConfig {
    /// Validate the configuration for invalid values
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate LSP config
        if self.lsp.timeout_ms < 1000 || self.lsp.timeout_ms > 30000 {
            return Err(ConfigError::InvalidTimeout(self.lsp.timeout_ms));
        }

        // Validate formatting config
        if self.formatting.indent_size < 1 || self.formatting.indent_size > 16 {
            return Err(ConfigError::InvalidIndentSize(self.formatting.indent_size));
        }

        if self.formatting.tab_width < 1 || self.formatting.tab_width > 16 {
            return Err(ConfigError::InvalidTabWidth(self.formatting.tab_width));
        }

        if self.formatting.max_line_length < 80 || self.formatting.max_line_length > 200 {
            return Err(ConfigError::InvalidMaxLineLength(
                self.formatting.max_line_length,
            ));
        }

        // Validate custom rules path (if provided)
        if let Some(ref path) = self.linting.custom_rules_path {
            if !path.exists() {
                return Err(ConfigError::CustomRulesFileNotFound(path.clone()));
            }
        }

        Ok(())
    }

    /// Load custom rules from the configured path (if any)
    pub fn load_custom_rules(&self) -> Result<Option<lint::custom::CustomRules>, ConfigError> {
        let Some(path) = &self.linting.custom_rules_path else {
            return Ok(None);
        };

        let content = std::fs::read_to_string(path)
            .map_err(|_| ConfigError::CustomRulesFileNotFound(path.clone()))?;

        let rules = serde_json::from_str(&content)
            .map_err(ConfigError::InvalidCustomRulesJson)?;

        Ok(Some(rules))
    }
}

/// Load the Cangjie configuration from Zed's settings
pub fn load_config(context: &zed_extension_api::ExtensionContext) -> Result<CangjieConfig> {
    let config = context.config::<CangjieConfig>("cangjie")?;
    config.validate()?;
    Ok(config)
}
```

### 14. src/lsp/mod.rs
```rust
//! LSP (Language Server Protocol) integration for Cangjie
//!
//! Implements the LSP protocol to provide code intelligence features:
//! - Diagnostics (syntax errors, style issues)
//! - Completion (symbols, snippets)
//! - Go to Definition
//! - Hover information
//! - Formatting
//! - Document Symbols

pub mod server;
pub mod diagnostics;
pub mod completion;
pub mod definition;
pub mod hover;
pub mod formatting;

// Re-export core types for easier access
pub use server::CangjieLanguageServer;
pub use diagnostics::DiagnosticProvider;
pub use completion::CompletionProvider;
pub use definition::DefinitionProvider;
pub use hover::HoverProvider;
pub use formatting::FormattingProvider;
```

### 15. src/lsp/server.rs
```rust
//! Cangjie Language Server implementation
//!
//! Implements the Zed LanguageServer trait to handle LSP requests from Zed.
//! Coordinates between different providers (diagnostics, completion, etc.)

use super::{
    diagnostics::DiagnosticProvider,
    completion::CompletionProvider,
    definition::DefinitionProvider,
    hover::HoverProvider,
    formatting::FormattingProvider,
};
use crate::{config::CangjieConfig, utils::log::Logger};
use std::sync::Arc;
use zed_extension_api::{
    self as zed, LanguageServer, LanguageServerHost, Result, Document, Workspace,
    lsp::{InitializeParams, InitializeResult, ServerCapabilities}
};

/// Cangjie Language Server
///
/// Coordinates all LSP features and manages shared state between providers.
#[derive(Debug, Clone)]
pub struct CangjieLanguageServer {
    config: Arc<CangjieConfig>,
    logger: Arc<Logger>,
    diagnostic_provider: DiagnosticProvider,
    completion_provider: CompletionProvider,
    definition_provider: DefinitionProvider,
    hover_provider: HoverProvider,
    formatting_provider: FormattingProvider,
}

impl CangjieLanguageServer {
    /// Create a new Cangjie Language Server instance
    pub fn new() -> Self {
        let logger = Arc::new(Logger::new("cangjie-lsp"));
        let config = Arc::new(CangjieConfig::default());

        Self {
            config: config.clone(),
            logger: logger.clone(),
            diagnostic_provider: DiagnosticProvider::new(config.clone(), logger.clone()),
            completion_provider: CompletionProvider::new(config.clone(), logger.clone()),
            definition_provider: DefinitionProvider::new(config.clone(), logger.clone()),
            hover_provider: HoverProvider::new(config.clone(), logger.clone()),
            formatting_provider: FormattingProvider::new(config.clone(), logger.clone()),
        }
    }

    /// Update the server configuration (called when Zed settings change)
    pub fn update_config(&mut self, new_config: CangjieConfig) {
        let new_config = Arc::new(new_config);
        self.config = new_config.clone();

        // Update providers with new config
        self.diagnostic_provider.update_config(new_config.clone());
        self.completion_provider.update_config(new_config.clone());
        self.definition_provider.update_config(new_config.clone());
        self.hover_provider.update_config(new_config.clone());
        self.formatting_provider.update_config(new_config);

        self.logger.info("Language server configuration updated");
    }
}

#[async_trait::async_trait]
impl LanguageServer for CangjieLanguageServer {
    /// Initialize the language server
    async fn initialize(
        &mut self,
        _host: LanguageServerHost,
        _params: InitializeParams,
    ) -> Result<InitializeResult> {
        self.logger.info("Cangjie Language Server initialized");

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // Enable diagnostics
                text_document_sync: Some(zed::lsp::TextDocumentSyncCapability::Kind(
                    zed::lsp::TextDocumentSyncKind::Incremental,
                )),
                // Enable completion
                completion_provider: Some(zed::lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string(), "(".to_string()]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                // Enable formatting
                document_formatting_provider: Some(zed::lsp::DocumentFormattingOptions::default()),
                // Enable go to definition
                definition_provider: Some(zed::lsp::OneOf::Left(true)),
                // Enable hover
                hover_provider: Some(zed::lsp::OneOf::Left(true)),
                // Enable document symbols
                document_symbol_provider: Some(zed::lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    /// Shutdown the language server
    async fn shutdown(&mut self) -> Result<()> {
        self.logger.info("Cangjie Language Server shutting down");
        Ok(())
    }

    /// Handle text document changes (trigger diagnostics)
    async fn did_change_text_document(
        &mut self,
        host: LanguageServerHost,
        params: zed::lsp::DidChangeTextDocumentParams,
    ) -> Result<()> {
        if !self.config.linting.enabled {
            return Ok(());
        }

        let document = host.document(&params.text_document.uri)?;
        let diagnostics = self.diagnostic_provider.provide_diagnostics(&document)?;

        host.publish_diagnostics(params.text_document.uri, diagnostics, None)?;
        Ok(())
    }

    /// Provide code completion
    async fn completion(
        &mut self,
        _host: LanguageServerHost,
        params: zed::lsp::CompletionParams,
    ) -> Result<Option<zed::lsp::CompletionResponse>> {
        if !self.config.completion.enabled {
            return Ok(None);
        }

        let document = params.text_document_position.text_document.document()?;
        let position = params.text_document_position.position;

        self.completion_provider.provide_completion(&document, position)
    }

    /// Provide go to definition
    async fn goto_definition(
        &mut self,
        host: LanguageServerHost,
        params: zed::lsp::GotoDefinitionParams,
    ) -> Result<Option<zed::lsp::GotoDefinitionResponse>> {
        let document = params.text_document_position.text_document.document()?;
        let position = params.text_document_position.position;
        let workspace = host.workspace()?;

        self.definition_provider.provide_definition(&document, &workspace, position)
    }

    /// Provide hover information
    async fn hover(
        &mut self,
        _host: LanguageServerHost,
        params: zed::lsp::HoverParams,
    ) -> Result<Option<zed::lsp::Hover>> {
        let document = params.text_document_position.text_document.document()?;
        let position = params.text_document_position.position;

        self.hover_provider.provide_hover(&document, position)
    }

    /// Format a document
    async fn format_document(
        &mut self,
        _host: LanguageServerHost,
        params: zed::lsp::DocumentFormattingParams,
    ) -> Result<Option<Vec<zed::lsp::TextEdit>>> {
        let document = params.text_document.document()?;

        self.formatting_provider.format_document(&document)
    }

    /// Provide document symbols (outline view)
    async fn document_symbol(
        &mut self,
        _host: LanguageServerHost,
        params: zed::lsp::DocumentSymbolParams,
    ) -> Result<Option<zed::lsp::DocumentSymbolResponse>> {
        let document = params.text_document.document()?;

        self.definition_provider.provide_document_symbols(&document)
    }
}
```

### 16. src/lsp/diagnostics.rs
```rust
//! Diagnostics provider for Cangjie language
//!
//! Provides real-time syntax error checking and style linting.
//! Integrates with built-in rules and custom user rules.

use super::super::{
    config::{CangjieConfig, SeverityLevel},
    lint::{self, rules::BuiltInRules, custom::CustomRules},
    syntax::tree_sitter_utils::{self, parse_document},
    utils::log::Logger,
};
use std::sync::Arc;
use zed_extension_api::{self as zed, Document, Result, lsp::{Diagnostic, DiagnosticSeverity, Range}};

/// Diagnostics provider for Cangjie language
#[derive(Debug, Clone)]
pub struct DiagnosticProvider {
    config: Arc<CangjieConfig>,
    logger: Arc<Logger>,
    built_in_rules: BuiltInRules,
    custom_rules: Option<CustomRules>,
}

impl DiagnosticProvider {
    /// Create a new DiagnosticProvider instance
    pub fn new(config: Arc<CangjieConfig>, logger: Arc<Logger>) -> Self {
        let custom_rules = config.load_custom_rules().unwrap_or_default();

        Self {
            config,
            logger,
            built_in_rules: BuiltInRules::new(),
            custom_rules,
        }
    }

    /// Update the provider's configuration (called when settings change)
    pub fn update_config(&mut self, new_config: Arc<CangjieConfig>) {
        self.config = new_config;
        self.custom_rules = self.config.load_custom_rules().unwrap_or_default();
        self.logger.info("Diagnostic provider configuration updated");
    }

    /// Provide diagnostics for a document
    pub fn provide_diagnostics(&self, document: &Document) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        // Skip if linting is disabled
        if !self.config.linting.enabled {
            return Ok(diagnostics);
        }

        let content = document.text();
        let tree = parse_document(document)?;
        let min_severity = &self.config.linting.severity_level;

        // 1. Syntax error diagnostics (from Tree-sitter)
        if self.config.linting.enable_syntax_checks {
            self.add_syntax_diagnostics(&mut diagnostics, &tree, content, min_severity);
        }

        // 2. Built-in style diagnostics
        if self.config.linting.enable_style_checks {
            let style_diagnostics = self.built_in_rules.check(
                document,
                &tree,
                &self.config.linting.ignore_rules,
                min_severity,
            )?;
            diagnostics.extend(style_diagnostics);
        }

        // 3. Custom rules diagnostics
        if let Some(ref custom_rules) = self.custom_rules {
            let custom_diagnostics = custom_rules.check(
                document,
                &tree,
                &self.config.linting.ignore_rules,
                min_severity,
            )?;
            diagnostics.extend(custom_diagnostics);
        }

        self.logger.debug(&format!("Found {} diagnostics for document", diagnostics.len()));
        Ok(diagnostics)
    }

    /// Add syntax errors from Tree-sitter parse tree
    fn add_syntax_diagnostics(
        &self,
        diagnostics: &mut Vec<Diagnostic>,
        tree: &tree_sitter::Tree,
        content: &str,
        min_severity: &SeverityLevel,
    ) {
        // Skip if error severity is below minimum
        if *min_severity > SeverityLevel::Error {
            return;
        }

        // Traverse the parse tree for error nodes
        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        loop {
            let node = cursor.node();
            if node.kind() == "ERROR" {
                let range = tree_sitter_utils::node_to_range(&node);
                let message = self.get_error_message(&node, content);

                diagnostics.push(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: Some(zed::lsp::DiagnosticCode::String("SYNTAX_ERROR".to_string())),
                    code_description: None,
                    message,
                    source: Some("cangjie-lsp".to_string()),
                    related_information: None,
                    tags: None,
                    data: None,
                    documentation: Some(zed::lsp::Documentation::MarkupContent(
                        zed::lsp::MarkupContent {
                            kind: zed::lsp::MarkupKind::Markdown,
                            value: "Syntax error in Cangjie code. Check for missing brackets, semicolons, or invalid syntax.".to_string(),
                        },
                    )),
                });
            }

            if cursor.goto_first_child() {
                continue;
            }
            if cursor.goto_next_sibling() {
                continue;
            }
            loop {
                if !cursor.goto_parent() {
                    return;
                }
                if cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    /// Get a human-readable error message for a Tree-sitter error node
    fn get_error_message(&self, error_node: &tree_sitter::Node, content: &str) -> String {
        let error_range = error_node.range();
        let start_byte = error_range.start_byte;
        let end_byte = error_range.end_byte.min(content.len());

        // Extract the problematic text
        let problematic_text = &content[start_byte..end_byte];
        let trimmed_text = problematic_text.trim().escape_default();

        if trimmed_text.is_empty() {
            "Unexpected end of input or empty error node".to_string()
        } else {
            format!("Syntax error near '{}'", trimmed_text)
        }
    }
}
```

### 17. src/lsp/completion.rs（完整版本）
```rust
//! Completion provider for Cangjie language
//!
//! Provides intelligent code completion for:
//! - Built-in keywords and types
//! - Document symbols (functions, structs, variables, constants)
//! - Workspace symbols (cross-file completion)
//! - Code snippets
//! - Standard library functions and types

use super::super::{
    config::{CangjieConfig, CompletionConfig},
    syntax::{
        snippets::CANGJIE_SNIPPETS,
        tree_sitter_utils::{self, parse_document, get_node_at_position},
    },
    utils::{log::Logger, file::find_workspace_files},
};
use std::sync::Arc;
use tree_sitter::Node;
use zed_extension_api::{
    self as zed, Document, Workspace, Result,
    lsp::{
        CompletionItem, CompletionItemKind, CompletionResponse, InsertTextFormat,
        Position, Range, TextDocumentPositionParams,
    },
};

/// Completion provider for Cangjie language
#[derive(Debug, Clone)]
pub struct CompletionProvider {
    config: Arc<CangjieConfig>,
    logger: Arc<Logger>,
    built_in_keywords: Vec<CompletionItem>,
    built_in_types: Vec<CompletionItem>,
    standard_library_symbols: Vec<CompletionItem>,
}

impl CompletionProvider {
    /// Create a new CompletionProvider instance
    pub fn new(config: Arc<CangjieConfig>, logger: Arc<Logger>) -> Self {
        Self {
            config,
            logger,
            built_in_keywords: Self::initialize_keywords(),
            built_in_types: Self::initialize_types(),
            standard_library_symbols: Self::initialize_standard_library(),
        }
    }

    /// Update the provider's configuration (called when settings change)
    pub fn update_config(&mut self, new_config: Arc<CangjieConfig>) {
        self.config = new_config;
        self.logger.info("Completion provider configuration updated");
    }

    /// Provide completion items for the given document and position
    pub fn provide_completion(
        &self,
        document: &Document,
        position: Position,
    ) -> Result<Option<CompletionResponse>> {
        let completion_config = &self.config.completion;
        let mut completion_items = Vec::new();

        // Parse document to get context
        let tree = parse_document(document)?;
        let content = document.text();
        let position_byte = document.offset_at_position(position)?;
        let current_node = get_node_at_position(&tree, position_byte);

        // 1. Add built-in keywords (context-aware)
        let keyword_items = self.get_keyword_completions(&current_node);
        completion_items.extend(keyword_items);

        // 2. Add built-in types
        completion_items.extend(self.built_in_types.clone());

        // 3. Add standard library symbols
        completion_items.extend(self.standard_library_symbols.clone());

        // 4. Add document symbols (functions, structs, variables, constants)
        let document_symbols = self.get_document_symbol_completions(document, &tree)?;
        completion_items.extend(document_symbols);

        // 5. Add workspace symbols (if enabled)
        if completion_config.include_workspace_symbols {
            let workspace = document.workspace()?;
            let workspace_symbols = self.get_workspace_symbol_completions(&workspace)?;
            completion_items.extend(workspace_symbols);
        }

        // 6. Add code snippets (if enabled)
        if completion_config.include_snippets {
            let snippet_items = self.get_snippet_completions(&current_node);
            completion_items.extend(snippet_items);
        }

        // Filter duplicate items (by label)
        let mut unique_items = Vec::new();
        let mut seen_labels = std::collections::HashSet::new();
        for item in completion_items {
            if !seen_labels.contains(&item.label) {
                seen_labels.insert(item.label.clone());
                unique_items.push(item);
            }
        }

        self.logger.debug(&format!(
            "Provided {} completion items for position {:?}",
            unique_items.len(),
            position
        ));

        Ok(Some(CompletionResponse::Array(unique_items)))
    }

    /// Initialize built-in keywords with completion metadata
    fn initialize_keywords() -> Vec<CompletionItem> {
        let keywords = [
            ("fn", "Function declaration"),
            ("let", "Mutable variable declaration"),
            ("const", "Constant declaration"),
            ("struct", "Struct declaration"),
            ("enum", "Enum declaration"),
            ("interface", "Interface declaration"),
            ("impl", "Interface implementation"),
            ("method", "Struct/enum method declaration"),
            ("if", "Conditional statement"),
            ("else", "Else clause"),
            ("for", "For loop"),
            ("while", "While loop"),
            ("do", "Do-while loop"),
            ("return", "Return statement"),
            ("match", "Pattern matching"),
            ("try", "Error handling try block"),
            ("catch", "Error handling catch block"),
            ("break", "Break loop/switch"),
            ("continue", "Continue loop"),
            ("import", "Import module"),
            ("export", "Export symbol"),
        ];

        keywords
            .iter()
            .map(|(label, detail)| CompletionItem {
                label: label.to_string(),
                kind: Some(CompletionItemKind::Keyword),
                detail: Some(detail.to_string()),
                documentation: None,
                insert_text: Some(label.to_string()),
                insert_text_format: Some(InsertTextFormat::PlainText),
                range: None,
                ..Default::default()
            })
            .collect()
    }

    /// Initialize built-in types with completion metadata
    fn initialize_types() -> Vec<CompletionItem> {
        let types = [
            ("Void", "Empty type (no return value)"),
            ("Int", "32-bit integer type"),
            ("Int64", "64-bit integer type"),
            ("Float", "32-bit floating-point type"),
            ("Float64", "64-bit floating-point type"),
            ("Bool", "Boolean type (true/false)"),
            ("String", "UTF-8 string type"),
            ("Char", "Unicode character type"),
            ("Array<T>", "Generic array type"),
            ("Vec<T>", "Dynamic vector type"),
            ("Map<K, V>", "Key-value map type"),
            ("Option<T>", "Optional type (Some/None)"),
            ("Result<T, E>", "Result type (Ok/Err)"),
            ("Any", "Dynamic type (use with caution)"),
        ];

        types
            .iter()
            .map(|(label, detail)| CompletionItem {
                label: label.to_string(),
                kind: Some(CompletionItemKind::TypeParameter),
                detail: Some(detail.to_string()),
                documentation: None,
                insert_text: Some(label.to_string()),
                insert_text_format: Some(InsertTextFormat::PlainText),
                range: None,
                ..Default::default()
            })
            .collect()
    }

    /// Initialize standard library symbols with completion metadata
    fn initialize_standard_library() -> Vec<CompletionItem> {
        let std_symbols = [
            (
                "println",
                "Print string to console",
                "fn println(message: String) -> Void",
                CompletionItemKind::Function,
            ),
            (
                "print",
                "Print string to console without newline",
                "fn print(message: String) -> Void",
                CompletionItemKind::Function,
            ),
            (
                "log_info",
                "Log informational message",
                "fn log_info(message: String) -> Void",
                CompletionItemKind::Function,
            ),
            (
                "log_warn",
                "Log warning message",
                "fn log_warn(message: String) -> Void",
                CompletionItemKind::Function,
            ),
            (
                "log_error",
                "Log error message",
                "fn log_error(message: String) -> Void",
                CompletionItemKind::Function,
            ),
            (
                "sleep",
                "Pause execution for specified milliseconds",
                "fn sleep(ms: Int) -> Void",
                CompletionItemKind::Function,
            ),
            (
                "read_file",
                "Read file content as string",
                "fn read_file(path: String) -> Result<String, Error>",
                CompletionItemKind::Function,
            ),
            (
                "write_file",
                "Write string to file",
                "fn write_file(path: String, content: String) -> Result<Void, Error>",
                CompletionItemKind::Function,
            ),
            (
                "Error",
                "Standard error type",
                "struct Error { message: String, code: Int }",
                CompletionItemKind::Struct,
            ),
            (
                "Math",
                "Mathematical utility functions",
                "module Math { ... }",
                CompletionItemKind::Module,
            ),
            (
                "StringUtils",
                "String utility functions",
                "module StringUtils { ... }",
                CompletionItemKind::Module,
            ),
        ];

        std_symbols
            .iter()
            .map(|(label, detail, doc, kind)| CompletionItem {
                label: label.to_string(),
                kind: Some(*kind),
                detail: Some(detail.to_string()),
                documentation: Some(zed::lsp::Documentation::MarkupContent(
                    zed::lsp::MarkupContent {
                        kind: zed::lsp::MarkupKind::Markdown,
                        value: doc.to_string(),
                    },
                )),
                insert_text: Some(label.to_string()),
                insert_text_format: Some(InsertTextFormat::PlainText),
                range: None,
                ..Default::default()
            })
            .collect()
    }

    /// Get context-aware keyword completions based on the current node
    fn get_keyword_completions(&self, current_node: &Node) -> Vec<CompletionItem> {
        let node_kind = current_node.kind();
        let parent_kind = current_node.parent().map(|p| p.kind()).unwrap_or("");

        // Filter keywords based on current context
        self.built_in_keywords
            .clone()
            .into_iter()
            .filter(|item| {
                match item.label.as_str() {
                    // Don't suggest 'fn' inside a function body
                    "fn" => !node_kind.starts_with("function") && !parent_kind.starts_with("function"),
                    // Don't suggest 'struct' inside a struct body
                    "struct" => !node_kind.starts_with("struct") && !parent_kind.starts_with("struct"),
                    // Don't suggest 'return' outside function bodies
                    "return" => parent_kind.starts_with("function"),
                    // Suggest all other keywords unconditionally
                    _ => true,
                }
            })
            .collect()
    }

    /// Get document symbol completions (functions, structs, variables, constants)
    fn get_document_symbol_completions(
        &self,
        document: &Document,
        tree: &tree_sitter::Tree,
    ) -> Result<Vec<CompletionItem>> {
        let mut symbols = Vec::new();
        let content = document.text();

        // Traverse parse tree to find symbols
        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.traverse_tree_for_symbols(&mut cursor, content, &mut symbols)?;

        Ok(symbols)
    }

    /// Traverse Tree-sitter parse tree to collect document symbols
    fn traverse_tree_for_symbols(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        symbols: &mut Vec<CompletionItem>,
    ) -> Result<()> {
        let node = cursor.node();
        let node_kind = node.kind();

        // Collect symbols based on node type
        match node_kind {
            "function_declaration" => self.collect_function_symbol(node, content, symbols)?,
            "struct_declaration" => self.collect_struct_symbol(node, content, symbols)?,
            "enum_declaration" => self.collect_enum_symbol(node, content, symbols)?,
            "interface_declaration" => self.collect_interface_symbol(node, content, symbols)?,
            "variable_declaration" => self.collect_variable_symbol(node, content, symbols)?,
            "constant_declaration" => self.collect_constant_symbol(node, content, symbols)?,
            _ => {}
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.traverse_tree_for_symbols(cursor, content, symbols)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.traverse_tree_for_symbols(cursor, content, symbols)?;
        }

        Ok(())
    }

    /// Collect function symbol from function_declaration node
    fn collect_function_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<CompletionItem>,
    ) -> Result<()> {
        // Extract function name
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Function node missing name field"))?;
        let name = tree_sitter_utils::node_text(&name_node, content)?;

        // Extract function signature (simplified)
        let params_node = node.child_by_field_name("parameters").unwrap_or_else(Node::new_null);
        let return_type_node = node.child_by_field_name("return_type").unwrap_or_else(Node::new_null);

        let params_text = tree_sitter_utils::node_text(&params_node, content)?;
        let return_type_text = tree_sitter_utils::node_text(&return_type_node, content)?;
        let signature = format!("fn {}({}) -> {}", name, params_text, return_type_text);

        symbols.push(CompletionItem {
            label: name.clone(),
            kind: Some(CompletionItemKind::Function),
            detail: Some("Function".to_string()),
            documentation: Some(zed::lsp::Documentation::MarkupContent(
                zed::lsp::MarkupContent {
                    kind: zed::lsp::MarkupKind::Markdown,
                    value: signature,
                },
            )),
            insert_text: Some(name),
            insert_text_format: Some(InsertTextFormat::PlainText),
            range: None,
            ..Default::default()
        });

        Ok(())
    }

    /// Collect struct symbol from struct_declaration node
    fn collect_struct_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<CompletionItem>,
    ) -> Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Struct node missing name field"))?;
        let name = tree_sitter_utils::node_text(&name_node, content)?;

        symbols.push(CompletionItem {
            label: name.clone(),
            kind: Some(CompletionItemKind::Struct),
            detail: Some("Struct".to_string()),
            documentation: Some(zed::lsp::Documentation::MarkupContent(
                zed::lsp::MarkupContent {
                    kind: zed::lsp::MarkupKind::Markdown,
                    value: format!("struct {}", name),
                },
            )),
            insert_text: Some(name),
            insert_text_format: Some(InsertTextFormat::PlainText),
            range: None,
            ..Default::default()
        });

        Ok(())
    }

    /// Collect enum symbol from enum_declaration node
    fn collect_enum_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<CompletionItem>,
    ) -> Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Enum node missing name field"))?;
        let name = tree_sitter_utils::node_text(&name_node, content)?;

        symbols.push(CompletionItem {
            label: name.clone(),
            kind: Some(CompletionItemKind::Enum),
            detail: Some("Enum".to_string()),
            documentation: Some(zed::lsp::Documentation::MarkupContent(
                zed::lsp::MarkupContent {
                    kind: zed::lsp::MarkupKind::Markdown,
                    value: format!("enum {}", name),
                },
            )),
            insert_text: Some(name),
            insert_text_format: Some(InsertTextFormat::PlainText),
            range: None,
            ..Default::default()
        });

        Ok(())
    }

    /// Collect interface symbol from interface_declaration node
    fn collect_interface_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<CompletionItem>,
    ) -> Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Interface node missing name field"))?;
        let name = tree_sitter_utils::node_text(&name_node, content)?;

        symbols.push(CompletionItem {
            label: name.clone(),
            kind: Some(CompletionItemKind::Interface),
            detail: Some("Interface".to_string()),
            documentation: Some(zed::lsp::Documentation::MarkupContent(
                zed::lsp::MarkupContent {
                    kind: zed::lsp::MarkupKind::Markdown,
                    value: format!("interface {}", name),
                },
            )),
            insert_text: Some(name),
            insert_text_format: Some(InsertTextFormat::PlainText),
            range: None,
            ..Default::default()
        });

        Ok(())
    }

    /// Collect variable symbol from variable_declaration node
    fn collect_variable_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<CompletionItem>,
    ) -> Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Variable node missing name field"))?;
        let name = tree_sitter_utils::node_text(&name_node, content)?;

        let type_node = node.child_by_field_name("type").unwrap_or_else(Node::new_null);
        let type_text = tree_sitter_utils::node_text(&type_node, content)?;
        let detail = if !type_text.is_empty() {
            format!("Variable ({})", type_text)
        } else {
            "Variable".to_string()
        };

        symbols.push(CompletionItem {
            label: name.clone(),
            kind: Some(CompletionItemKind::Variable),
            detail: Some(detail),
            documentation: None,
            insert_text: Some(name),
            insert_text_format: Some(InsertTextFormat::PlainText),
            range: None,
            ..Default::default()
        });

        Ok(())
    }

    /// Collect constant symbol from constant_declaration node
    fn collect_constant_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<CompletionItem>,
    ) -> Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Constant node missing name field"))?;
        let name = tree_sitter_utils::node_text(&name_node, content)?;

        let value_node = node.child_by_field_name("value").unwrap_or_else(Node::new_null);
        let value_text = tree_sitter_utils::node_text(&value_node, content)?;
        let detail = format!("Constant = {}", value_text);

        symbols.push(CompletionItem {
            label: name.clone(),
            kind: Some(CompletionItemKind::Constant),
            detail: Some(detail),
            documentation: None,
            insert_text: Some(name),
            insert_text_format: Some(InsertTextFormat::PlainText),
            range: None,
            ..Default::default()
        });

        Ok(())
    }

    /// Get workspace symbol completions (cross-file)
    fn get_workspace_symbol_completions(&self, workspace: &Workspace) -> Result<Vec<CompletionItem>> {
        let mut workspace_symbols = Vec::new();

        // Find all .cj files in the workspace
        let cj_files = find_workspace_files(workspace, "*.cj")?;

        for file_path in cj_files {
            // Skip current document (already added in document symbols)
            if let Some(current_doc_path) = workspace.active_document().map(|d| d.path()) {
                if file_path == current_doc_path {
                    continue;
                }
            }

            // Read file content and parse
            let content = std::fs::read_to_string(&file_path)?;
            let document = zed::Document::new(file_path.clone(), content.clone())?;
            let tree = parse_document(&document)?;

            // Collect symbols from the file
            let mut file_symbols = Vec::new();
            let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
            self.traverse_tree_for_symbols(&mut cursor, &content, &mut file_symbols)?;

            // Add file path to symbol detail
            let file_name = file_path
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("unknown.cj");

            let file_symbols = file_symbols
                .into_iter()
                .map(|mut item| {
                    item.detail = Some(format!("{} ({}", item.detail.unwrap_or_default(), file_name));
                    item
                })
                .collect::<Vec<_>>();

            workspace_symbols.extend(file_symbols);
        }

        Ok(workspace_symbols)
    }

    /// Get snippet completions (context-aware)
    fn get_snippet_completions(&self, current_node: &Node) -> Vec<CompletionItem> {
        let node_kind = current_node.kind();
        let parent_kind = current_node.parent().map(|p| p.kind()).unwrap_or("");

        // Filter snippets based on current context
        CANGJIE_SNIPPETS
            .iter()
            .filter(|snippet| {
                // Skip snippets that shouldn't be used in current context
                match snippet.trigger {
                    "fn" => !node_kind.starts_with("function") && !parent_kind.starts_with("function"),
                    "struct" => !node_kind.starts_with("struct") && !parent_kind.starts_with("struct"),
                    "enum" => !node_kind.starts_with("enum") && !parent_kind.starts_with("enum"),
                    "if" => !node_kind.starts_with("if") && !parent_kind.starts_with("if"),
                    "for" => !node_kind.starts_with("for") && !parent_kind.starts_with("for"),
                    _ => true,
                }
            })
            .map(|snippet| CompletionItem {
                label: format!("{} - {}", snippet.trigger, snippet.description),
                kind: Some(CompletionItemKind::Snippet),
                detail: Some("Code Snippet".to_string()),
                documentation: Some(zed::lsp::Documentation::MarkupContent(
                    zed::lsp::MarkupContent {
                        kind: zed::lsp::MarkupKind::Markdown,
                        value: format!("```cangjie\n{}\n```", snippet.body),
                    },
                )),
                insert_text: Some(snippet.body.clone()),
                insert_text_format: Some(InsertTextFormat::Snippet),
                range: None,
                ..Default::default()
            })
            .collect()
    }
}
```

### 18. src/lsp/definition.rs
```rust
//! Go to Definition provider for Cangjie language
//!
//! Provides support for jumping to symbol definitions:
//! - Within the current document
//! - Across files in the workspace
//! - Also provides document symbols for outline view

use super::super::{
    syntax::tree_sitter_utils::{self, parse_document, get_node_at_position, NodeText},
    utils::{log::Logger, file::find_workspace_files},
};
use std::sync::Arc;
use tree_sitter::Node;
use zed_extension_api::{
    self as zed, Document, Workspace, Result,
    lsp::{
        GotoDefinitionParams, GotoDefinitionResponse, Location, LocationLink,
        DocumentSymbolParams, DocumentSymbolResponse, SymbolKind, SymbolInformation,
        Range, TextDocumentIdentifier,
    },
};

/// Definition provider for Cangjie language
#[derive(Debug, Clone)]
pub struct DefinitionProvider {
    config: Arc<super::super::config::CangjieConfig>,
    logger: Arc<Logger>,
}

impl DefinitionProvider {
    /// Create a new DefinitionProvider instance
    pub fn new(config: Arc<super::super::config::CangjieConfig>, logger: Arc<Logger>) -> Self {
        Self { config, logger }
    }

    /// Update the provider's configuration (called when settings change)
    pub fn update_config(&mut self, new_config: Arc<super::super::config::CangjieConfig>) {
        self.config = new_config;
        self.logger.info("Definition provider configuration updated");
    }

    /// Provide go to definition results
    pub fn provide_definition(
        &self,
        document: &Document,
        workspace: &Workspace,
        position: zed::lsp::Position,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let content = document.text();
        let position_byte = document.offset_at_position(position)?;

        // Parse current document
        let tree = parse_document(document)?;
        let current_node = get_node_at_position(&tree, position_byte);

        // Find the symbol node under the cursor (identifier)
        let symbol_node = self.find_symbol_node(&current_node)?;
        if symbol_node.is_null() {
            self.logger.debug("No symbol found at cursor position");
            return Ok(None);
        }

        let symbol_name = symbol_node.text(content)?;
        self.logger.debug(&format!("Looking for definition of '{}'", symbol_name));

        // 1. Check current document for definition
        let mut definitions = self.find_definitions_in_document(document, &tree, &symbol_name)?;

        // 2. If no definitions found, check other workspace files
        if definitions.is_empty() {
            let workspace_definitions = self.find_definitions_in_workspace(workspace, &symbol_name)?;
            definitions.extend(workspace_definitions);
        }

        if definitions.is_empty() {
            self.logger.debug(&format!("No definition found for '{}'", symbol_name));
            return Ok(None);
        }

        // Convert to LSP response format
        let response = if definitions.len() == 1 {
            GotoDefinitionResponse::Scalar(definitions.into_iter().next().unwrap())
        } else {
            GotoDefinitionResponse::Array(definitions)
        };

        Ok(Some(response))
    }

    /// Provide document symbols for outline view
    pub fn provide_document_symbols(
        &self,
        document: &Document,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let content = document.text();
        let tree = parse_document(document)?;

        let mut symbols = Vec::new();
        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.traverse_tree_for_symbols(&mut cursor, content, &mut symbols)?;

        Ok(Some(DocumentSymbolResponse::Array(symbols)))
    }

    /// Find the symbol node (identifier) under the cursor
    fn find_symbol_node(&self, current_node: &Node) -> Result<Node> {
        let mut node = current_node.clone();

        // Traverse up the tree to find an identifier node
        while !node.is_null() {
            if node.kind() == "identifier" {
                return Ok(node);
            }
            node = node.parent().unwrap_or_else(Node::new_null);
        }

        Ok(Node::new_null())
    }

    /// Find definitions of a symbol in the current document
    fn find_definitions_in_document(
        &self,
        document: &Document,
        tree: &tree_sitter::Tree,
        symbol_name: &str,
    ) -> Result<Vec<LocationLink>> {
        let mut definitions = Vec::new();
        let content = document.text();

        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.traverse_tree_for_definitions(
            &mut cursor,
            content,
            symbol_name,
            document.uri(),
            &mut definitions,
        )?;

        Ok(definitions)
    }

    /// Find definitions of a symbol in the workspace (other files)
    fn find_definitions_in_workspace(
        &self,
        workspace: &Workspace,
        symbol_name: &str,
    ) -> Result<Vec<LocationLink>> {
        let mut definitions = Vec::new();

        // Find all .cj files in the workspace
        let cj_files = find_workspace_files(workspace, "*.cj")?;

        for file_path in cj_files {
            // Read file content and parse
            let content = std::fs::read_to_string(&file_path)?;
            let document = zed::Document::new(file_path.clone(), content.clone())?;
            let tree = parse_document(&document)?;

            // Look for definitions in this file
            let mut file_definitions = Vec::new();
            let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
            self.traverse_tree_for_definitions(
                &mut cursor,
                &content,
                symbol_name,
                document.uri(),
                &mut file_definitions,
            )?;

            definitions.extend(file_definitions);
        }

        Ok(definitions)
    }

    /// Traverse Tree-sitter parse tree to find symbol definitions
    fn traverse_tree_for_definitions(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        symbol_name: &str,
        document_uri: String,
        definitions: &mut Vec<LocationLink>,
    ) -> Result<()> {
        let node = cursor.node();
        let node_kind = node.kind();

        // Check if current node is a definition of the target symbol
        match node_kind {
            "function_declaration" | "struct_declaration" | "enum_declaration" |
            "interface_declaration" | "variable_declaration" | "constant_declaration" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = name_node.text(content)?;
                    if name == symbol_name {
                        let definition_range = tree_sitter_utils::node_to_range(&node);
                        let selection_range = tree_sitter_utils::node_to_range(&name_node);

                        definitions.push(LocationLink {
                            origin_selection_range: None,
                            target_uri: document_uri.clone(),
                            target_range: definition_range,
                            target_selection_range: selection_range,
                        });
                    }
                }
            }
            _ => {}
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.traverse_tree_for_definitions(cursor, content, symbol_name, document_uri.clone(), definitions)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.traverse_tree_for_definitions(cursor, content, symbol_name, document_uri.clone(), definitions)?;
        }

        Ok(())
    }

    /// Traverse Tree-sitter parse tree to collect document symbols
    fn traverse_tree_for_symbols(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        symbols: &mut Vec<SymbolInformation>,
    ) -> Result<()> {
        let node = cursor.node();
        let node_kind = node.kind();

        // Collect symbols based on node type
        match node_kind {
            "function_declaration" => self.collect_function_symbol(node, content, symbols)?,
            "struct_declaration" => self.collect_struct_symbol(node, content, symbols)?,
            "enum_declaration" => self.collect_enum_symbol(node, content, symbols)?,
            "interface_declaration" => self.collect_interface_symbol(node, content, symbols)?,
            "constant_declaration" => self.collect_constant_symbol(node, content, symbols)?,
            _ => {}
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.traverse_tree_for_symbols(cursor, content, symbols)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.traverse_tree_for_symbols(cursor, content, symbols)?;
        }

        Ok(())
    }

    /// Collect function symbol for document symbols
    fn collect_function_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<SymbolInformation>,
    ) -> Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Function node missing name field"))?;
        let name = name_node.text(content)?;
        let range = tree_sitter_utils::node_to_range(&node);
        let selection_range = tree_sitter_utils::node_to_range(&name_node);

        symbols.push(SymbolInformation {
            name: name.clone(),
            kind: SymbolKind::Function,
            tags: None,
            deprecated: None,
            location: Location {
                uri: content.to_string(), // Will be replaced by Zed with actual document URI
                range,
            },
            container_name: None,
            documentation: None,
            selection_range: Some(selection_range),
        });

        Ok(())
    }

    /// Collect struct symbol for document symbols
    fn collect_struct_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<SymbolInformation>,
    ) -> Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Struct node missing name field"))?;
        let name = name_node.text(content)?;
        let range = tree_sitter_utils::node_to_range(&node);
        let selection_range = tree_sitter_utils::node_to_range(&name_node);

        symbols.push(SymbolInformation {
            name: name.clone(),
            kind: SymbolKind::Struct,
            tags: None,
            deprecated: None,
            location: Location {
                uri: content.to_string(),
                range,
            },
            container_name: None,
            documentation: None,
            selection_range: Some(selection_range),
        });

        Ok(())
    }

    /// Collect enum symbol for document symbols
    fn collect_enum_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<SymbolInformation>,
    ) -> Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Enum node missing name field"))?;
        let name = name_node.text(content)?;
        let range = tree_sitter_utils::node_to_range(&node);
        let selection_range = tree_sitter_utils::node_to_range(&name_node);

        symbols.push(SymbolInformation {
            name: name.clone(),
            kind: SymbolKind::Enum,
            tags: None,
            deprecated: None,
            location: Location {
                uri: content.to_string(),
                range,
            },
            container_name: None,
            documentation: None,
            selection_range: Some(selection_range),
        });

        Ok(())
    }

    /// Collect interface symbol for document symbols
    fn collect_interface_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<SymbolInformation>,
    ) -> Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Interface node missing name field"))?;
        let name = name_node.text(content)?;
        let range = tree_sitter_utils::node_to_range(&node);
        let selection_range = tree_sitter_utils::node_to_range(&name_node);

        symbols.push(SymbolInformation {
            name: name.clone(),
            kind: SymbolKind::Interface,
            tags: None,
            deprecated: None,
            location: Location {
                uri: content.to_string(),
                range,
            },
            container_name: None,
            documentation: None,
            selection_range: Some(selection_range),
        });

        Ok(())
    }

    /// Collect constant symbol for document symbols
    fn collect_constant_symbol(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<SymbolInformation>,
    ) -> Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .ok_or_else(|| zed::Error::internal("Constant node missing name field"))?;
        let name = name_node.text(content)?;
        let range = tree_sitter_utils::node_to_range(&node);
        let selection_range = tree_sitter_utils::node_to_range(&name_node);

        symbols.push(SymbolInformation {
            name: name.clone(),
            kind: SymbolKind::Constant,
            tags: None,
            deprecated: None,
            location: Location {
                uri: content.to_string(),
                range,
            },
            container_name: None,
            documentation: None,
            selection_range: Some(selection_range),
        });

        Ok(())
    }
}

/// Trait to get text from a Tree-sitter node
trait NodeText {
    fn text(&self, content: &str) -> Result<String>;
}

impl NodeText for Node {
    fn text(&self, content: &str) -> Result<String> {
        let start_byte = self.start_byte();
        let end_byte = self.end_byte();
        let text = &content[start_byte..end_byte];
        Ok(text.to_string())
    }
}
```

### 19. src/lsp/hover.rs
```rust
//! Hover provider for Cangjie language
//!
//! Provides detailed hover information for:
//! - Functions (signature, parameters, return type, doc comments)
//! - Structs (fields, methods, doc comments)
//! - Enums (variants, doc comments)
//! - Constants (value, type)
//! - Variables (type, value)
//! - Standard library symbols (documentation)

use super::super::{
    syntax::tree_sitter_utils::{self, parse_document, get_node_at_position, NodeText},
    utils::log::Logger,
};
use std::sync::Arc;
use tree_sitter::Node;
use zed_extension_api::{
    self as zed, Document, Result,
    lsp::{Hover, HoverParams, MarkupContent, MarkupKind, Range},
};

/// Hover provider for Cangjie language
#[derive(Debug, Clone)]
pub struct HoverProvider {
    config: Arc<super::super::config::CangjieConfig>,
    logger: Arc<Logger>,
}

impl HoverProvider {
    /// Create a new HoverProvider instance
    pub fn new(config: Arc<super::super::config::CangjieConfig>, logger: Arc<Logger>) -> Self {
        Self { config, logger }
    }

    /// Update the provider's configuration (called when settings change)
    pub fn update_config(&mut self, new_config: Arc<super::super::config::CangjieConfig>) {
        self.config = new_config;
        self.logger.info("Hover provider configuration updated");
    }

    /// Provide hover information for the given document and position
    pub fn provide_hover(
        &self,
        document: &Document,
        position: zed::lsp::Position,
    ) -> Result<Option<Hover>> {
        let content = document.text();
        let position_byte = document.offset_at_position(position)?;

        // Parse document
        let tree = parse_document(document)?;
        let current_node = get_node_at_position(&tree, position_byte);

        // Find the symbol node under the cursor
        let (symbol_node, symbol_kind) = self.find_symbol_node(&current_node)?;
        if symbol_node.is_null() {
            self.logger.debug("No symbol found for hover");
            return Ok(None);
        }

        let symbol_name = symbol_node.text(content)?;
        self.logger.debug(&format!("Providing hover for '{}' ({})", symbol_name, symbol_kind));

        // Generate hover content based on symbol type
        let hover_content = match symbol_kind {
            "function" => self.generate_function_hover(&symbol_node, content)?,
            "struct" => self.generate_struct_hover(&symbol_node, content)?,
            "enum" => self.generate_enum_hover(&symbol_node, content)?,
            "interface" => self.generate_interface_hover(&symbol_node, content)?,
            "constant" => self.generate_constant_hover(&symbol_node, content)?,
            "variable" => self.generate_variable_hover(&symbol_node, content)?,
            "type" => self.generate_type_hover(&symbol_name)?,
            _ => return Ok(None),
        };

        let hover_range = tree_sitter_utils::node_to_range(&symbol_node);

        Ok(Some(Hover {
            contents: hover_content,
            range: Some(hover_range),
        }))
    }

    /// Find the symbol node and its kind under the cursor
    fn find_symbol_node(&self, current_node: &Node) -> Result<(Node, &'static str)> {
        let mut node = current_node.clone();

        // Traverse up the tree to find a symbol node
        while !node.is_null() {
            let node_kind = node.kind();

            // Check if current node is a symbol identifier
            if node_kind == "identifier" {
                let parent_node = node.parent().unwrap_or_else(Node::new_null);
                let parent_kind = parent_node.kind();

                // Determine the symbol type based on parent node
                let symbol_kind = match parent_kind {
                    "function_declaration" => "function",
                    "struct_declaration" => "struct",
                    "enum_declaration" => "enum",
                    "interface_declaration" => "interface",
                    "constant_declaration" => "constant",
                    "variable_declaration" => "variable",
                    "type_identifier" => "type",
                    "field_declaration" => "field",
                    "method_declaration" => "method",
                    _ => {
                        // Check if parent is a type reference
                        if parent_kind == "type" || parent_kind == "return_type" {
                            "type"
                        } else {
                            node = parent_node;
                            continue;
                        }
                    }
                };

                return Ok((node, symbol_kind));
            }

            node = node.parent().unwrap_or_else(Node::new_null);
        }

        Ok((Node::new_null(), ""))
    }

    /// Generate hover content for a function
    fn generate_function_hover(&self, name_node: &Node, content: &str) -> Result<MarkupContent> {
        let function_node = name_node
            .parent()
            .ok_or_else(|| zed::Error::internal("Function name node missing parent"))?;

        // Extract function components
        let name = name_node.text(content)?;
        let params_node = function_node.child_by_field_name("parameters").unwrap_or_else(Node::new_null);
        let return_type_node = function_node.child_by_field_name("return_type").unwrap_or_else(Node::new_null);

        let params_text = params_node.text(content)?;
        let return_type_text = return_type_node.text(content)?;
        let signature = format!("fn {}({}) -> {}", name, params_text, return_type_text);

        // Extract doc comment (if any)
        let doc_comment = self.extract_doc_comment(&function_node, content)?;

        // Build hover content
        let mut content = format!("```cangjie\n{}\n```", signature);
        if !doc_comment.is_empty() {
            content.push_str(&format!("\n\n{}", doc_comment));
        }

        Ok(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        })
    }

    /// Generate hover content for a struct
    fn generate_struct_hover(&self, name_node: &Node, content: &str) -> Result<MarkupContent> {
        let struct_node = name_node
            .parent()
            .ok_or_else(|| zed::Error::internal("Struct name node missing parent"))?;

        let name = name_node.text(content)?;

        // Extract fields
        let mut fields = Vec::new();
        let fields_node = struct_node.child_by_field_name("fields").unwrap_or_else(Node::new_null);
        for field_node in fields_node.children_by_field_name("field") {
            let field_name_node = field_node.child_by_field_name("name").unwrap_or_else(Node::new_null);
            let field_type_node = field_node.child_by_field_name("type").unwrap_or_else(Node::new_null);

            let field_name = field_name_node.text(content)?;
            let field_type = field_type_node.text(content)?;
            fields.push(format!("{}: {}", field_name, field_type));
        }

        // Extract doc comment
        let doc_comment = self.extract_doc_comment(&struct_node, content)?;

        // Build hover content
        let mut content = format!("```cangjie\nstruct {}\n```", name);
        if !fields.is_empty() {
            content.push_str(&format!("\n\n**Fields:**\n{}", fields.join("\n")));
        }
        if !doc_comment.is_empty() {
            content.push_str(&format!("\n\n{}", doc_comment));
        }

        Ok(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        })
    }

    /// Generate hover content for an enum
    fn generate_enum_hover(&self, name_node: &Node, content: &str) -> Result<MarkupContent> {
        let enum_node = name_node
            .parent()
            .ok_or_else(|| zed::Error::internal("Enum name node missing parent"))?;

        let name = name_node.text(content)?;

        // Extract variants
        let mut variants = Vec::new();
        let variants_node = enum_node.child_by_field_name("variants").unwrap_or_else(Node::new_null);
        for variant_node in variants_node.children_by_field_name("variant") {
            let variant_name = variant_node.text(content)?;
            variants.push(variant_name);
        }

        // Extract doc comment
        let doc_comment = self.extract_doc_comment(&enum_node, content)?;

        // Build hover content
        let mut content = format!("```cangjie\nenum {}\n```", name);
        if !variants.is_empty() {
            content.push_str(&format!("\n\n**Variants:**\n{}", variants.join("\n")));
        }
        if !doc_comment.is_empty() {
            content.push_str(&format!("\n\n{}", doc_comment));
        }

        Ok(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        })
    }

    /// Generate hover content for an interface
    fn generate_interface_hover(&self, name_node: &Node, content: &str) -> Result<MarkupContent> {
        let interface_node = name_node
            .parent()
            .ok_or_else(|| zed::Error::internal("Interface name node missing parent"))?;

        let name = name_node.text(content)?;

        // Extract methods
        let mut methods = Vec::new();
        let methods_node = interface_node.child_by_field_name("methods").unwrap_or_else(Node::new_null);
        for method_node in methods_node.children_by_field_name("method") {
            let method_name_node = method_node.child_by_field_name("name").unwrap_or_else(Node::new_null);
            let params_node = method_node.child_by_field_name("parameters").unwrap_or_else(Node::new_null);
            let return_type_node = method_node.child_by_field_name("return_type").unwrap_or_else(Node::new_null);

            let method_name = method_name_node.text(content)?;
            let params_text = params_node.text(content)?;
            let return_type_text = return_type_node.text(content)?;

            methods.push(format!("{}({}) -> {}", method_name, params_text, return_type_text));
        }

        // Extract doc comment
        let doc_comment = self.extract_doc_comment(&interface_node, content)?;

        // Build hover content
        let mut content = format!("```cangjie\ninterface {}\n```", name);
        if !methods.is_empty() {
            content.push_str(&format!("\n\n**Methods:**\n{}", methods.join("\n")));
        }
        if !doc_comment.is_empty() {
            content.push_str(&format!("\n\n{}", doc_comment));
        }

        Ok(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        })
    }

    /// Generate hover content for a constant
    fn generate_constant_hover(&self, name_node: &Node, content: &str) -> Result<MarkupContent> {
        let constant_node = name_node
            .parent()
            .ok_or_else(|| zed::Error::internal("Constant name node missing parent"))?;

        let name = name_node.text(content)?;
        let value_node = constant_node.child_by_field_name("value").unwrap_or_else(Node::new_null);
        let value_text = value_node.text(content)?;

        // Extract doc comment
        let doc_comment = self.extract_doc_comment(&constant_node, content)?;

        // Build hover content
        let mut content = format!("```cangjie\nconst {} = {}\n```", name, value_text);
        if !doc_comment.is_empty() {
            content.push_str(&format!("\n\n{}", doc_comment));
        }

        Ok(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        })
    }

    /// Generate hover content for a variable
    fn generate_variable_hover(&self, name_node: &Node, content: &str) -> Result<MarkupContent> {
        let variable_node = name_node
            .parent()
            .ok_or_else(|| zed::Error::internal("Variable name node missing parent"))?;

        let name = name_node.text(content)?;
        let type_node = variable_node.child_by_field_name("type").unwrap_or_else(Node::new_null);
        let type_text = type_node.text(content)?;

        // Build hover content
        let content = if !type_text.is_empty() {
            format!("```cangjie\nlet {}: {}\n```", name, type_text)
        } else {
            format!("```cangjie\nlet {}\n```", name)
        };

        Ok(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        })
    }

    /// Generate hover content for a built-in type
    fn generate_type_hover(&self, type_name: &str) -> Result<MarkupContent> {
        let type_documentation = match type_name {
            "Void" => "Empty type, used for functions with no return value.",
            "Int" => "32-bit signed integer type (range: -2³¹ to 2³¹-1).",
            "Int64" => "64-bit signed integer type (range: -2⁶³ to 2⁶³-1).",
            "Float" => "32-bit floating-point type (single precision).",
            "Float64" => "64-bit floating-point type (double precision).",
            "Bool" => "Boolean type with two possible values: `true` or `false`.",
            "String" => "UTF-8 encoded string type (immutable by default).",
            "Char" => "Unicode character type (UTF-8 scalar value).",
            "Array<T>" => "Fixed-size generic array type (length known at compile time).",
            "Vec<T>" => "Dynamic vector type (resizable, length known at runtime).",
            "Map<K, V>" => "Generic key-value map type (hash-based implementation).",
            "Option<T>" => "Optional type representing either `Some(value)` or `None`.",
            "Result<T, E>" => "Result type representing either `Ok(value)` or `Err(error)`.",
            "Any" => "Dynamic type that can hold any value (sacrifices type safety).",
            _ => "Built-in Cangjie type.",
        };

        let content = format!(
            "### {}\n\n{}",
            type_name, type_documentation
        );

        Ok(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        })
    }

    /// Extract doc comment (/** ... */) for a node
    fn extract_doc_comment(&self, node: &Node, content: &str) -> Result<String> {
        // Get the previous sibling (potential doc comment)
        let prev_sibling = node.prev_sibling().unwrap_or_else(Node::new_null);
        if prev_sibling.kind() == "comment" {
            let comment_text = prev_sibling.text(content)?;
            // Check if it's a doc comment (starts with /**)
            if comment_text.starts_with("/**") {
                // Clean up the comment (remove /**, */, and leading asterisks)
                let cleaned = comment_text
                    .replace("/**", "")
                    .replace("*/", "")
                    .lines()
                    .map(|line| line.trim_start_matches(|c: char| c.is_whitespace() || c == '*'))
                    .collect::<Vec<_>>()
                    .join("\n")
                    .trim()
                    .to_string();

                return Ok(cleaned);
            }
        }

        Ok(String::new())
    }
}

/// Trait to get text from a Tree-sitter node
trait NodeText {
    fn text(&self, content: &str) -> Result<String>;
}

impl NodeText for Node {
    fn text(&self, content: &str) -> Result<String> {
        let start_byte = self.start_byte();
        let end_byte = self.end_byte();
        let text = &content[start_byte..end_byte];
        Ok(text.to_string())
    }
}
```
### 20. src/lsp/formatting.rs（完整版本）
```rust
//! Formatting provider for Cangjie language
//!
//! Implements configurable code formatting based on the extension's formatting configuration.
//! Supports:
//! - Indentation (spaces/tabs, custom size)
//! - Brace placement (same line/next line)
//! - Line length enforcement
//! - Trailing commas
//! - Space around operators
//! - Auto-fix of minor syntax issues

use super::super::{
    config::{CangjieConfig, FormattingConfig, IndentStyle, BraceStyle, TrailingCommaPolicy, LineEnding},
    syntax::tree_sitter_utils::{self, parse_document, NodeText},
    utils::log::Logger,
};
use std::sync::Arc;
use tree_sitter::Node;
use zed_extension_api::{self as zed, Document, Result, lsp::{DocumentFormattingParams, TextEdit, Range}};

/// Formatting provider for Cangjie language
#[derive(Debug, Clone)]
pub struct FormattingProvider {
    config: Arc<CangjieConfig>,
    logger: Arc<Logger>,
}

impl FormattingProvider {
    /// Create a new FormattingProvider instance
    pub fn new(config: Arc<CangjieConfig>, logger: Arc<Logger>) -> Self {
        Self { config, logger }
    }

    /// Update the provider's configuration (called when settings change)
    pub fn update_config(&mut self, new_config: Arc<CangjieConfig>) {
        self.config = new_config;
        self.logger.info("Formatting provider configuration updated");
    }

    /// Format a full document
    pub fn format_document(&self, document: &Document) -> Result<Option<Vec<TextEdit>>> {
        let config = &self.config.formatting;
        let content = document.text();

        // Parse document to get structure
        let tree = parse_document(document)?;
        if tree.root_node().has_error() {
            self.logger.warn("Cannot format document with syntax errors");
            return Ok(None);
        }

        // Generate formatted text
        let formatted_text = self.format_node(
            tree.root_node(),
            content,
            config,
            0, // Initial indent level
            false, // Not inside a block
        )?;

        // If formatted text is the same as original, return no edits
        if formatted_text == content {
            self.logger.debug("Document is already formatted");
            return Ok(None);
        }

        // Create a single text edit to replace the entire document
        let full_range = Range {
            start: zed::lsp::Position { line: 0, character: 0 },
            end: document.position_at_offset(content.len())?,
        };

        let edit = TextEdit {
            range: full_range,
            new_text: formatted_text,
        };

        self.logger.debug("Document formatted successfully");
        Ok(Some(vec![edit]))
    }

    /// Recursively format a Tree-sitter node and its children
    fn format_node(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
        inside_block: bool,
    ) -> Result<String> {
        let node_kind = node.kind();
        let mut formatted = String::new();

        // Add indentation if needed
        if self.should_indent_node(&node, inside_block) {
            formatted.push_str(&self.get_indent(indent_level, config));
        }

        // Process node based on its kind
        match node_kind {
            "function_declaration" => {
                formatted.push_str(&self.format_function_declaration(node, content, config, indent_level)?);
            }
            "struct_declaration" => {
                formatted.push_str(&self.format_struct_declaration(node, content, config, indent_level)?);
            }
            "enum_declaration" => {
                formatted.push_str(&self.format_enum_declaration(node, content, config, indent_level)?);
            }
            "interface_declaration" => {
                formatted.push_str(&self.format_interface_declaration(node, content, config, indent_level)?);
            }
            "block" => {
                formatted.push_str(&self.format_block(node, content, config, indent_level)?);
            }
            "if_statement" => {
                formatted.push_str(&self.format_if_statement(node, content, config, indent_level)?);
            }
            "for_statement" => {
                formatted.push_str(&self.format_for_statement(node, content, config, indent_level)?);
            }
            "while_statement" => {
                formatted.push_str(&self.format_while_statement(node, content, config, indent_level)?);
            }
            "return_statement" => {
                formatted.push_str(&self.format_return_statement(node, content, config, indent_level)?);
            }
            "variable_declaration" | "constant_declaration" => {
                formatted.push_str(&self.format_variable_declaration(node, content, config)?);
                formatted.push_str(&self.get_line_ending(config));
            }
            "field_declaration" => {
                formatted.push_str(&self.format_field_declaration(node, content, config)?);
                formatted.push_str(&self.get_line_ending(config));
            }
            "enum_variant" => {
                formatted.push_str(&self.format_enum_variant(node, content, config)?);
            }
            "comment" => {
                formatted.push_str(&self.format_comment(node, content, config, indent_level)?);
                formatted.push_str(&self.get_line_ending(config));
            }
            "string_literal" | "number_literal" | "boolean_literal" => {
                // Literals are formatted as-is
                formatted.push_str(&node.text(content)?);
            }
            "identifier" | "type_identifier" => {
                // Identifiers are formatted as-is
                formatted.push_str(&node.text(content)?);
            }
            "binary_expression" => {
                formatted.push_str(&self.format_binary_expression(node, content, config)?);
            }
            "call_expression" => {
                formatted.push_str(&self.format_call_expression(node, content, config)?);
            }
            "parenthesized_expression" => {
                formatted.push_str(&self.format_parenthesized_expression(node, content, config)?);
            }
            "brace_expression" => {
                formatted.push_str(&self.format_brace_expression(node, content, config, indent_level)?);
            }
            "semicolon" => {
                formatted.push(';');
            }
            "comma" => {
                formatted.push(',');
                if config.trailing_comma != TrailingCommaPolicy::Never {
                    formatted.push(' ');
                }
            }
            "colon" => {
                formatted.push(':');
                formatted.push(' ');
            }
            "arrow" => {
                formatted.push(' ');
                formatted.push_str("->");
                formatted.push(' ');
            }
            _ => {
                // For unknown node types, format children recursively
                let mut children = node.children();
                while let Some(child) = children.next() {
                    formatted.push_str(&self.format_node(
                        child,
                        content,
                        config,
                        indent_level,
                        inside_block || node_kind == "block",
                    )?);
                }
            }
        }

        Ok(formatted)
    }

    /// Determine if a node should be indented
    fn should_indent_node(&self, node: &Node, inside_block: bool) -> bool {
        let node_kind = node.kind();

        // Nodes that are always indented inside blocks
        if inside_block {
            match node_kind {
                "function_declaration" | "struct_declaration" | "enum_declaration" |
                "interface_declaration" | "variable_declaration" | "constant_declaration" |
                "if_statement" | "for_statement" | "while_statement" | "return_statement" |
                "comment" | "field_declaration" | "enum_variant" => return true,
                _ => {}
            }
        }

        // Specific node types that need indentation regardless
        match node_kind {
            "field_declaration" | "enum_variant" | "method_declaration" => return true,
            _ => false,
        }
    }

    /// Get indentation string for a given level
    fn get_indent(&self, level: usize, config: &FormattingConfig) -> String {
        match config.indent_style {
            IndentStyle::Space => " ".repeat((config.indent_size as usize) * level),
            IndentStyle::Tab => "\t".repeat(level),
        }
    }

    /// Get line ending string based on config
    fn get_line_ending(&self, config: &FormattingConfig) -> String {
        match config.line_ending {
            LineEnding::Lf => "\n".to_string(),
            LineEnding::Crlf => "\r\n".to_string(),
        }
    }

    /// Format a function declaration
    fn format_function_declaration(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract function components
        let fn_keyword = node.child_by_field_name("fn_keyword").unwrap().text(content)?;
        let name = node.child_by_field_name("name").unwrap().text(content)?;
        let params = node.child_by_field_name("parameters").unwrap();
        let return_type = node.child_by_field_name("return_type").unwrap_or_else(Node::new_null);
        let body = node.child_by_field_name("body").unwrap_or_else(Node::new_null);

        // Format function header
        formatted.push_str(&format!("{} {}", fn_keyword, name));
        formatted.push_str(&self.format_node(params, content, config, indent_level, false)?);

        if !return_type.is_null() {
            formatted.push_str(&self.format_node(return_type, content, config, indent_level, false)?);
        }

        // Format function body
        if !body.is_null() {
            match config.function_brace_style {
                BraceStyle::SameLine => {
                    formatted.push(' ');
                    formatted.push_str(&self.format_node(body, content, config, indent_level, true)?);
                }
                BraceStyle::NextLine => {
                    formatted.push_str(&self.get_line_ending(config));
                    formatted.push_str(&self.format_node(body, content, config, indent_level, true)?);
                }
            }
        } else {
            // No body (abstract function)
            formatted.push(';');
            formatted.push_str(&self.get_line_ending(config));
        }

        Ok(formatted)
    }

    /// Format a struct declaration
    fn format_struct_declaration(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract struct components
        let struct_keyword = node.child_by_field_name("struct_keyword").unwrap().text(content)?;
        let name = node.child_by_field_name("name").unwrap().text(content)?;
        let fields = node.child_by_field_name("fields").unwrap_or_else(Node::new_null);

        // Format struct header
        formatted.push_str(&format!("{} {}", struct_keyword, name));

        // Format fields
        if !fields.is_null() {
            match config.struct_brace_style {
                BraceStyle::SameLine => {
                    formatted.push(' ');
                }
                BraceStyle::NextLine => {
                    formatted.push_str(&self.get_line_ending(config));
                    formatted.push_str(&self.get_indent(indent_level, config));
                }
            }

            formatted.push('{');
            formatted.push_str(&self.get_line_ending(config));

            // Format each field
            let field_nodes = fields.children_by_field_name("field");
            let field_count = field_nodes.count();
            let mut fields_iter = fields.children_by_field_name("field");

            for (i, field_node) in fields_iter.enumerate() {
                formatted.push_str(&self.format_node(
                    field_node,
                    content,
                    config,
                    indent_level + 1,
                    true,
                )?);

                // Add trailing comma based on config
                if config.trailing_comma == TrailingCommaPolicy::Always
                    || (config.trailing_comma == TrailingCommaPolicy::Multiline && field_count > 1)
                {
                    formatted.push(',');
                }
            }

            // Close struct
            formatted.push_str(&self.get_indent(indent_level, config));
            formatted.push('}');
        }

        formatted.push_str(&self.get_line_ending(config));
        Ok(formatted)
    }

    /// Format an enum declaration
    fn format_enum_declaration(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract enum components
        let enum_keyword = node.child_by_field_name("enum_keyword").unwrap().text(content)?;
        let name = node.child_by_field_name("name").unwrap().text(content)?;
        let variants = node.child_by_field_name("variants").unwrap_or_else(Node::new_null);

        // Format enum header
        formatted.push_str(&format!("{} {}", enum_keyword, name));
        formatted.push(' ');
        formatted.push('{');
        formatted.push_str(&self.get_line_ending(config));

        // Format each variant
        let variant_nodes = variants.children_by_field_name("variant");
        let variant_count = variant_nodes.count();
        let mut variants_iter = variants.children_by_field_name("variant");

        for (i, variant_node) in variants_iter.enumerate() {
            formatted.push_str(&self.format_node(
                variant_node,
                content,
                config,
                indent_level + 1,
                true,
            )?);

            // Add trailing comma based on config
            if config.trailing_comma == TrailingCommaPolicy::Always
                || (config.trailing_comma == TrailingCommaPolicy::Multiline && variant_count > 1)
            {
                formatted.push(',');
            }
            formatted.push_str(&self.get_line_ending(config));
        }

        // Close enum
        formatted.push_str(&self.get_indent(indent_level, config));
        formatted.push('}');
        formatted.push_str(&self.get_line_ending(config));

        Ok(formatted)
    }

    /// Format an interface declaration
    fn format_interface_declaration(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract interface components
        let interface_keyword = node.child_by_field_name("interface_keyword").unwrap().text(content)?;
        let name = node.child_by_field_name("name").unwrap().text(content)?;
        let methods = node.child_by_field_name("methods").unwrap_or_else(Node::new_null);

        // Format interface header
        formatted.push_str(&format!("{} {}", interface_keyword, name));
        formatted.push(' ');
        formatted.push('{');
        formatted.push_str(&self.get_line_ending(config));

        // Format each method
        for method_node in methods.children_by_field_name("method") {
            formatted.push_str(&self.format_node(
                method_node,
                content,
                config,
                indent_level + 1,
                true,
            )?);
            formatted.push_str(&self.get_line_ending(config));
        }

        // Close interface
        formatted.push_str(&self.get_indent(indent_level, config));
        formatted.push('}');
        formatted.push_str(&self.get_line_ending(config));

        Ok(formatted)
    }

    /// Format a block (code inside {})
    fn format_block(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Open block
        formatted.push('{');
        formatted.push_str(&self.get_line_ending(config));

        // Format block children
        let mut children = node.children();
        while let Some(child) = children.next() {
            if child.kind() == "}" {
                continue; // Handled separately
            }

            formatted.push_str(&self.format_node(
                child,
                content,
                config,
                indent_level + 1,
                true,
            )?);
        }

        // Close block
        formatted.push_str(&self.get_indent(indent_level, config));
        formatted.push('}');

        Ok(formatted)
    }

    /// Format an if statement
    fn format_if_statement(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract if statement components
        let if_keyword = node.child_by_field_name("if_keyword").unwrap().text(content)?;
        let condition = node.child_by_field_name("condition").unwrap();
        let consequence = node.child_by_field_name("consequence").unwrap();
        let alternative = node.child_by_field_name("alternative").unwrap_or_else(Node::new_null);

        // Format if header
        formatted.push_str(&format!("{} ", if_keyword));
        formatted.push_str(&self.format_node(condition, content, config, indent_level, false)?);
        formatted.push(' ');

        // Format consequence (body)
        formatted.push_str(&self.format_node(consequence, content, config, indent_level, true)?);

        // Format alternative (else/else if)
        if !alternative.is_null() {
            formatted.push_str(&self.get_line_ending(config));
            formatted.push_str(&self.get_indent(indent_level, config));
            formatted.push_str(&self.format_node(alternative, content, config, indent_level, false)?);
        }

        formatted.push_str(&self.get_line_ending(config));
        Ok(formatted)
    }

    /// Format a for statement
    fn format_for_statement(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract for statement components
        let for_keyword = node.child_by_field_name("for_keyword").unwrap().text(content)?;
        let condition = node.child_by_field_name("condition").unwrap();
        let body = node.child_by_field_name("body").unwrap();

        // Format for header
        formatted.push_str(&format!("{} ", for_keyword));
        formatted.push_str(&self.format_node(condition, content, config, indent_level, false)?);
        formatted.push(' ');

        // Format body
        formatted.push_str(&self.format_node(body, content, config, indent_level, true)?);
        formatted.push_str(&self.get_line_ending(config));

        Ok(formatted)
    }

    /// Format a while statement
    fn format_while_statement(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract while statement components
        let while_keyword = node.child_by_field_name("while_keyword").unwrap().text(content)?;
        let condition = node.child_by_field_name("condition").unwrap();
        let body = node.child_by_field_name("body").unwrap();

        // Format while header
        formatted.push_str(&format!("{} ", while_keyword));
        formatted.push_str(&self.format_node(condition, content, config, indent_level, false)?);
        formatted.push(' ');

        // Format body
        formatted.push_str(&self.format_node(body, content, config, indent_level, true)?);
        formatted.push_str(&self.get_line_ending(config));

        Ok(formatted)
    }

    /// Format a return statement
    fn format_return_statement(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract return statement components
        let return_keyword = node.child_by_field_name("return_keyword").unwrap().text(content)?;
        let value = node.child_by_field_name("value").unwrap_or_else(Node::new_null);

        // Format return keyword
        formatted.push_str(&return_keyword);

        // Format return value (if any)
        if !value.is_null() {
            formatted.push(' ');
            formatted.push_str(&self.format_node(value, content, config, indent_level, false)?);
        }

        // Add semicolon (auto-fix if missing)
        formatted.push(';');
        formatted.push_str(&self.get_line_ending(config));

        Ok(formatted)
    }

    /// Format a variable or constant declaration
    fn format_variable_declaration(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract declaration components
        let keyword = node.child_by_field_name("keyword").unwrap().text(content)?;
        let name = node.child_by_field_name("name").unwrap().text(content)?;
        let type_annotation = node.child_by_field_name("type").unwrap_or_else(Node::new_null);
        let initializer = node.child_by_field_name("value").unwrap_or_else(Node::new_null);

        // Format keyword and name
        formatted.push_str(&format!("{} {}", keyword, name));

        // Format type annotation (if any)
        if !type_annotation.is_null() {
            formatted.push_str(": ");
            formatted.push_str(&self.format_node(type_annotation, content, config, 0, false)?);
        }

        // Format initializer (if any)
        if !initializer.is_null() {
            formatted.push_str(" = ");
            formatted.push_str(&self.format_node(initializer, content, config, 0, false)?);
        }

        Ok(formatted)
    }

    /// Format a struct field declaration
    fn format_field_declaration(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract field components
        let name = node.child_by_field_name("name").unwrap().text(content)?;
        let type_annotation = node.child_by_field_name("type").unwrap();

        // Format field
        formatted.push_str(&format!("{}: ", name));
        formatted.push_str(&self.format_node(type_annotation, content, config, 0, false)?);

        Ok(formatted)
    }

    /// Format an enum variant
    fn format_enum_variant(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
    ) -> Result<String> {
        let name = node.child_by_field_name("name").unwrap().text(content)?;
        Ok(name)
    }

    /// Format a comment
    fn format_comment(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let comment_text = node.text(content)?;

        // For line comments, ensure proper indentation
        if comment_text.starts_with("//") {
            let indent = self.get_indent(indent_level, config);
            let trimmed = comment_text.trim_start();
            Ok(format!("{}{}", indent, trimmed))
        } else {
            // Block comments are formatted as-is (preserve structure)
            Ok(comment_text)
        }
    }

    /// Format a binary expression (e.g., a + b, x == y)
    fn format_binary_expression(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract expression components
        let left = node.child_by_field_name("left").unwrap();
        let operator = node.child_by_field_name("operator").unwrap().text(content)?;
        let right = node.child_by_field_name("right").unwrap();

        // Format left operand
        formatted.push_str(&self.format_node(left, content, config, 0, false)?);

        // Format operator with spaces (if enabled)
        if config.space_around_operators {
            formatted.push_str(&format!(" {} ", operator));
        } else {
            formatted.push_str(&operator);
        }

        // Format right operand
        formatted.push_str(&self.format_node(right, content, config, 0, false)?);

        Ok(formatted)
    }

    /// Format a call expression (e.g., func(a, b))
    fn format_call_expression(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract call components
        let function = node.child_by_field_name("function").unwrap();
        let arguments = node.child_by_field_name("arguments").unwrap();

        // Format function name
        formatted.push_str(&self.format_node(function, content, config, 0, false)?);
        formatted.push('(');

        // Format arguments
        let arg_nodes = arguments.children().filter(|n| n.kind() != "(" && n.kind() != ")");
        let arg_count = arg_nodes.count();
        let mut args_iter = arguments.children().filter(|n| n.kind() != "(" && n.kind() != ")");

        for (i, arg_node) in args_iter.enumerate() {
            formatted.push_str(&self.format_node(arg_node, content, config, 0, false)?);

            // Add comma and space between arguments
            if i < arg_count - 1 {
                formatted.push_str(", ");
            }
        }

        formatted.push(')');
        Ok(formatted)
    }

    /// Format a parenthesized expression (e.g., (a + b))
    fn format_parenthesized_expression(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract expression components
        let left_paren = node.child_by_field_name("left_paren").unwrap().text(content)?;
        let expression = node.child_by_field_name("expression").unwrap();
        let right_paren = node.child_by_field_name("right_paren").unwrap().text(content)?;

        // Format with optional spaces inside parentheses
        formatted.push_str(&left_paren);
        if config.space_inside_brackets {
            formatted.push(' ');
        }
        formatted.push_str(&self.format_node(expression, content, config, 0, false)?);
        if config.space_inside_brackets {
            formatted.push(' ');
        }
        formatted.push_str(&right_paren);

        Ok(formatted)
    }

    /// Format a brace expression (e.g., { key: value })
    fn format_brace_expression(
        &self,
        node: Node,
        content: &str,
        config: &FormattingConfig,
        indent_level: usize,
    ) -> Result<String> {
        let mut formatted = String::new();

        // Extract expression components
        let left_brace = node.child_by_field_name("left_brace").unwrap().text(content)?;
        let right_brace = node.child_by_field_name("right_brace").unwrap().text(content)?;
        let properties = node.children().filter(|n| n.kind() != "{" && n.kind() != "}");

        // Format left brace
        formatted.push_str(&left_brace);

        // Format properties
        let prop_count = properties.count();
        let mut props_iter = node.children().filter(|n| n.kind() != "{" && n.kind() != "}");

        if prop_count > 0 {
            formatted.push_str(&self.get_line_ending(config));

            for (i, prop_node) in props_iter.enumerate() {
                formatted.push_str(&self.get_indent(indent_level + 1, config));
                formatted.push_str(&self.format_node(prop_node, content, config, indent_level + 1, true)?);

                // Add trailing comma based on config
                if config.trailing_comma == TrailingCommaPolicy::Always
                    || (config.trailing_comma == TrailingCommaPolicy::Multiline && prop_count > 1)
                {
                    formatted.push(',');
                }

                formatted.push_str(&self.get_line_ending(config));
            }

            formatted.push_str(&self.get_indent(indent_level, config));
        }

        // Format right brace
        formatted.push_str(&right_brace);

        Ok(formatted)
    }
}

/// Trait to get text from a Tree-sitter node
trait NodeText {
    fn text(&self, content: &str) -> Result<String>;
}

impl NodeText for Node {
    fn text(&self, content: &str) -> Result<String> {
        let start_byte = self.start_byte();
        let end_byte = self.end_byte();
        let text = &content[start_byte..end_byte];
        Ok(text.to_string())
    }
}
```

### 21. src/syntax/mod.rs
```rust
//! Syntax handling for Cangjie language
//!
//! Provides syntax-related features:
//! - Tree-sitter integration for parsing
//! - Syntax highlighting
//! - Code snippets
//! - Utility functions for working with Tree-sitter nodes

pub mod highlights;
pub mod snippets;
pub mod tree_sitter_utils;

// Re-export core types for easier access
pub use highlights::register as register_highlights;
pub use snippets::register as register_snippets;
pub use tree_sitter_utils::{parse_document, get_node_at_position, node_to_range, NodeText};
```

### 22. src/syntax/highlights.rs
```rust
//! Syntax highlighting for Cangjie language
//!
//! Implements Tree-sitter-based syntax highlighting by mapping
//! Tree-sitter nodes to Zed's syntax scopes.

use zed_extension_api::{self as zed, ExtensionContext, Result, SyntaxHighlightRegistry};
use tree_sitter::Query;

/// Register syntax highlighting for Cangjie language
pub fn register(context: &ExtensionContext) -> Result<()> {
    let mut registry = SyntaxHighlightRegistry::new("cangjie")?;

    // Define highlight queries (Tree-sitter query syntax)
    let queries = r#"
        ; Keywords
        (keyword) @keyword
        (fn_keyword) @keyword.function
        (struct_keyword) @keyword.struct
        (enum_keyword) @keyword.enum
        (interface_keyword) @keyword.interface
        (impl_keyword) @keyword.impl
        (method_keyword) @keyword.method
        (let_keyword) @keyword.let
        (const_keyword) @keyword.constant
        (if_keyword) @keyword.conditional
        (else_keyword) @keyword.conditional
        (for_keyword) @keyword.repeat
        (while_keyword) @keyword.repeat
        (do_keyword) @keyword.repeat
        (return_keyword) @keyword.return
        (match_keyword) @keyword.control
        (try_keyword) @keyword.control
        (catch_keyword) @keyword.control
        (break_keyword) @keyword.control
        (continue_keyword) @keyword.control
        (import_keyword) @keyword.import
        (export_keyword) @keyword.export

        ; Types
        (type_identifier) @type
        (primitive_type) @type.builtin
        (generic_type) @type.generic

        ; Identifiers
        (identifier) @variable
        (function_declaration (identifier) @function)
        (struct_declaration (identifier) @type.struct)
        (enum_declaration (identifier) @type.enum)
        (interface_declaration (identifier) @type.interface)
        (constant_declaration (identifier) @constant)
        (field_declaration (identifier) @variable.other.member)
        (enum_variant (identifier) @constant.enum)

        ; Literals
        (string_literal) @string
        (character_literal) @string.char
        (number_literal) @constant.numeric
        (boolean_literal) @constant.builtin.boolean
        (null_literal) @constant.builtin.null

        ; Operators
        (operator) @operator
        (assignment_operator) @operator.assignment
        (comparison_operator) @operator.comparison
        (arithmetic_operator) @operator.arithmetic
        (logical_operator) @operator.logical
        (bitwise_operator) @operator.bitwise

        ; Punctuation
        (punctuation) @punctuation
        (brace) @punctuation.bracket
        (bracket) @punctuation.bracket
        (parenthesis) @punctuation.bracket
        (comma) @punctuation.separator
        (colon) @punctuation.separator
        (semicolon) @punctuation.terminator
        (arrow) @punctuation.arrow

        ; Comments
        (comment) @comment
        (doc_comment) @comment.documentation

        ; Parameters
        (parameter_declaration (identifier) @variable.parameter)

        ; Attributes
        (attribute) @attribute
    "#;

    // Compile and register the highlight query
    let query = Query::new(tree_sitter_cangjie::language(), queries)
        .map_err(|err| zed::Error::internal(format!("Failed to compile highlight query: {}", err)))?;

    registry.add_query(query)?;
    context.register_syntax_highlighting(registry)?;

    Ok(())
}
```

### 23. src/syntax/snippets.rs
```rust
//! Code snippets for Cangjie language
//!
//! Provides commonly used code snippets that follow Cangjie's syntax
//! and Zed's snippet format (supports tab stops and placeholders).

use zed_extension_api::{self as zed, ExtensionContext, Result, SnippetRegistry};

/// Cangjie code snippets (trigger -> (description, body))
pub const CANGJIE_SNIPPETS: &[CangjieSnippet] = &[
    // Function snippet
    CangjieSnippet {
        trigger: "fn",
        description: "Function declaration",
        body: "fn ${1:function_name}(${2:parameters}) -> ${3:Void} {\n    ${0:// Function body}\n}",
    },
    // Struct snippet
    CangjieSnippet {
        trigger: "struct",
        description: "Struct declaration",
        body: "struct ${1:StructName} {\n    ${0:// Fields}\n    ${2:field_name}: ${3:Type};\n}",
    },
    // Enum snippet
    CangjieSnippet {
        trigger: "enum",
        description: "Enum declaration",
        body: "enum ${1:EnumName} {\n    ${0:// Variants}\n    ${2:Variant1},\n    ${3:Variant2},\n}",
    },
    // Interface snippet
    CangjieSnippet {
        trigger: "interface",
        description: "Interface declaration",
        body: "interface ${1:InterfaceName} {\n    ${0:// Methods}\n    method ${2:method_name}(${3:parameters}) -> ${4:ReturnType};\n}",
    },
    // Impl snippet
    CangjieSnippet {
        trigger: "impl",
        description: "Interface implementation",
        body: "impl ${1:StructName}: ${2:InterfaceName} {\n    ${0:// Method implementations}\n    method ${3:method_name}(${4:parameters}) -> ${5:ReturnType} {\n        ${6:// Implementation}\n    }\n}",
    },
    // If statement snippet
    CangjieSnippet {
        trigger: "if",
        description: "If statement",
        body: "if (${1:condition}) {\n    ${0:// If body}\n}",
    },
    // If-else statement snippet
    CangjieSnippet {
        trigger: "ife",
        description: "If-else statement",
        body: "if (${1:condition}) {\n    ${0:// If body}\n} else {\n    ${2:// Else body}\n}",
    },
    // For loop snippet
    CangjieSnippet {
        trigger: "for",
        description: "For loop",
        body: "for (${1:let i = 0; i < ${2:count}; i++}) {\n    ${0:// Loop body}\n}",
    },
    // While loop snippet
    CangjieSnippet {
        trigger: "while",
        description: "While loop",
        body: "while (${1:condition}) {\n    ${0:// Loop body}\n}",
    },
    // Do-while loop snippet
    CangjieSnippet {
        trigger: "dowhile",
        description: "Do-while loop",
        body: "do {\n    ${0:// Loop body}\n} while (${1:condition});",
    },
    // Let variable snippet
    CangjieSnippet {
        trigger: "let",
        description: "Mutable variable declaration",
        body: "let ${1:variable_name}: ${2:Type} = ${3:value};",
    },
    // Const variable snippet
    CangjieSnippet {
        trigger: "const",
        description: "Constant declaration",
        body: "const ${1:CONSTANT_NAME}: ${2:Type} = ${3:value};",
    },
    // Println snippet
    CangjieSnippet {
        trigger: "println",
        description: "Print line to console",
        body: "println(\"${0:message}\");",
    },
    // Doc comment snippet
    CangjieSnippet {
        trigger: "/**",
        description: "Document comment",
        body: "/**\n * ${0:Documentation}\n */",
    },
    // Try-catch snippet
    CangjieSnippet {
        trigger: "try",
        description: "Try-catch error handling",
        body: "try {\n    ${0:// Code that may throw}\n} catch (${1:error}) {\n    ${2:// Error handling}\n}",
    },
];

/// Cangjie snippet structure
#[derive(Debug, Clone)]
pub struct CangjieSnippet {
    /// Trigger text to activate the snippet
    pub trigger: &'static str,
    /// Description of the snippet (shown in completion)
    pub description: &'static str,
    /// Snippet body (supports Zed's snippet syntax)
    pub body: &'static str,
}

/// Register code snippets for Cangjie language
pub fn register(context: &ExtensionContext) -> Result<()> {
    let mut registry = SnippetRegistry::new("cangjie")?;

    // Register all snippets
    for snippet in CANGJIE_SNIPPETS {
        registry.add_snippet(
            snippet.trigger,
            snippet.description,
            snippet.body,
        )?;
    }

    context.register_snippets(registry)?;
    Ok(())
}
```

### 24. src/syntax/tree_sitter_utils.rs
```rust
//! Tree-sitter utility functions for Cangjie language
//!
//! Provides helper functions for working with Tree-sitter parse trees,
//! including parsing documents, finding nodes, converting between
//! Tree-sitter ranges and LSP ranges, and extracting text from nodes.

use zed_extension_api::{self as zed, Document, Result, lsp::Range};
use tree_sitter::{Language, Parser, Tree, Node, Point};

/// Get the Tree-sitter language for Cangjie
fn cangjie_language() -> Language {
    tree_sitter_cangjie::language()
}

/// Parse a Zed document into a Tree-sitter parse tree
pub fn parse_document(document: &Document) -> Result<Tree> {
    let mut parser = Parser::new();
    parser.set_language(cangjie_language())
        .map_err(|err| zed::Error::internal(format!("Failed to set Tree-sitter language: {}", err)))?;

    let text = document.text();
    let tree = parser.parse(text, None)
        .ok_or_else(|| zed::Error::internal("Failed to parse document with Tree-sitter"))?;

    Ok(tree)
}

/// Get the Tree-sitter node at a given byte offset
pub fn get_node_at_position(tree: &Tree, byte_offset: usize) -> Node {
    let root = tree.root_node();
    root.descendant_for_byte_range(byte_offset, byte_offset)
        .unwrap_or(root)
}

/// Convert a Tree-sitter Point to an LSP Position
fn point_to_position(point: Point) -> zed::lsp::Position {
    zed::lsp::Position {
        line: point.row as u32,
        character: point.column as u32,
    }
}

/// Convert a Tree-sitter Node to an LSP Range
pub fn node_to_range(node: &Node) -> Range {
    Range {
        start: point_to_position(node.start_position()),
        end: point_to_position(node.end_position()),
    }
}

/// Trait to get text from a Tree-sitter node
pub trait NodeText {
    /// Extract text from the node using the original document content
    fn text(&self, content: &str) -> Result<String>;
}

impl NodeText for Node {
    fn text(&self, content: &str) -> Result<String> {
        let start_byte = self.start_byte();
        let end_byte = self.end_byte();

        // Ensure byte offsets are within bounds
        if start_byte > content.len() || end_byte > content.len() {
            return Err(zed::Error::internal(format!(
                "Node byte range out of bounds: {}..{} (content length: {})",
                start_byte, end_byte, content.len()
            )));
        }

        Ok(content[start_byte..end_byte].to_string())
    }
}

/// Find a child node by field name (recursive)
pub fn find_child_by_field_name(node: &Node, field_name: &str) -> Option<Node> {
    let child = node.child_by_field_name(field_name);
    if child.is_some() {
        return child;
    }

    // Recursively search children
    for child in node.children() {
        let found = find_child_by_field_name(&child, field_name);
        if found.is_some() {
            return found;
        }
    }

    None
}

/// Check if a node has any error children
pub fn node_has_errors(node: &Node) -> bool {
    if node.kind() == "ERROR" {
        return true;
    }

    // Recursively check children
    for child in node.children() {
        if node_has_errors(&child) {
            return true;
        }
    }

    false
}
```

### 25. src/lint/mod.rs
```rust
//! Code linting for Cangjie language
//!
//! Provides static code analysis with:
//! - Built-in rules (syntax, style, naming conventions)
//! - Custom rule support (JSON configuration)
//! - Severity levels (error, warning, information, hint)

pub mod rules;
pub mod custom;

// Re-export core types for easier access
pub use rules::BuiltInRules;
pub use custom::CustomRules;
pub use rules::LintRule;
pub use rules::LintDiagnostic;
```
### 26. src/lint/rules.rs（完整版本）
```rust
//! Built-in lint rules for Cangjie language
//!
//! Implements common lint rules for syntax, style, and naming conventions.
//! Each rule can be enabled/disabled via configuration.

use super::super::{
    config::SeverityLevel,
    syntax::tree_sitter_utils::{self, parse_document, NodeText},
};
use zed_extension_api::{self as zed, Document, Result, lsp::{Diagnostic, DiagnosticSeverity, Range}};
use tree_sitter::Node;
use std::collections::HashSet;

/// Type alias for lint diagnostic (extends LSP Diagnostic with rule ID)
#[derive(Debug, Clone)]
pub struct LintDiagnostic {
    /// LSP diagnostic
    pub diagnostic: Diagnostic,
    /// Rule ID
    pub rule_id: String,
}

/// Trait for lint rules
pub trait LintRule {
    /// Get the unique rule ID
    fn rule_id(&self) -> &str;

    /// Get the default severity level
    fn default_severity(&self) -> SeverityLevel;

    /// Get the rule description
    fn description(&self) -> &str;

    /// Get the rule documentation (optional)
    fn documentation(&self) -> Option<&str> {
        None
    }

    /// Check a document for violations of this rule
    fn check(&self, document: &Document, tree: &tree_sitter::Tree) -> Result<Vec<LintDiagnostic>>;
}

/// Collection of built-in lint rules
pub struct BuiltInRules {
    rules: Vec<Box<dyn LintRule>>,
}

impl BuiltInRules {
    /// Create a new collection of built-in rules
    pub fn new() -> Self {
        Self {
            rules: vec![
                Box::new(UnusedVariableRule),
                Box::new(UnusedConstantRule),
                Box::new(LineTooLongRule),
                Box::new(InvalidNamingConventionRule),
                Box::new(MissingSemicolonRule),
                Box::new(EmptyBlockRule),
                Box::new(UnreachableCodeRule),
                Box::new(DeprecatedSyntaxRule),
            ],
        }
    }

    /// Run all enabled rules on a document
    pub fn check(
        &self,
        document: &Document,
        tree: &tree_sitter::Tree,
        ignore_rules: &[String],
        min_severity: &SeverityLevel,
    ) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        let ignore_set: HashSet<&str> = ignore_rules.iter().map(|s| s.as_str()).collect();

        for rule in &self.rules {
            // Skip ignored rules
            if ignore_set.contains(rule.rule_id()) {
                continue;
            }

            // Skip rules with severity below minimum
            if rule.default_severity() < *min_severity {
                continue;
            }

            // Run rule checks
            let rule_diagnostics = rule.check(document, tree)?;
            for lint_diag in rule_diagnostics {
                diagnostics.push(lint_diag.diagnostic);
            }
        }

        Ok(diagnostics)
    }
}

// -----------------------------------------------------------------------------
// Rule Implementations
// -----------------------------------------------------------------------------

/// Rule: Unused variable
#[derive(Debug, Clone)]
struct UnusedVariableRule;

impl LintRule for UnusedVariableRule {
    fn rule_id(&self) -> &str {
        "UNUSED_VARIABLE"
    }

    fn default_severity(&self) -> SeverityLevel {
        SeverityLevel::Warning
    }

    fn description(&self) -> &str {
        "Unused variable declaration"
    }

    fn documentation(&self) -> Option<&str> {
        Some("Variables that are declared but not used can be removed to improve code clarity.")
    }

    fn check(&self, document: &Document, tree: &tree_sitter::Tree) -> Result<Vec<LintDiagnostic>> {
        let mut diagnostics = Vec::new();
        let content = document.text();

        // Collect all variable declarations
        let mut declared_variables = Vec::new();
        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.collect_variable_declarations(&mut cursor, content, &mut declared_variables)?;

        // Collect all variable usages
        let mut used_variables = HashSet::new();
        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.collect_variable_usages(&mut cursor, content, &mut used_variables)?;

        // Find unused variables
        for (var_name, var_node) in declared_variables {
            if !used_variables.contains(&var_name) {
                let range = tree_sitter_utils::node_to_range(&var_node);
                let mut diagnostic = Diagnostic {
                    range,
                    severity: Some(self.default_severity().into()),
                    code: Some(zed::lsp::DiagnosticCode::String(self.rule_id().to_string())),
                    code_description: None,
                    message: self.description().to_string(),
                    source: Some("cangjie-lint".to_string()),
                    related_information: None,
                    tags: Some(vec![zed::lsp::DiagnosticTag::Unnecessary]),
                    data: None,
                    documentation: self.documentation().map(|doc| {
                        zed::lsp::Documentation::MarkupContent(
                            zed::lsp::MarkupContent {
                                kind: zed::lsp::MarkupKind::Markdown,
                                value: doc.to_string(),
                            }
                        )
                    }),
                };

                diagnostics.push(LintDiagnostic {
                    diagnostic,
                    rule_id: self.rule_id().to_string(),
                });
            }
        }

        Ok(diagnostics)
    }
}

impl UnusedVariableRule {
    /// Collect all variable declarations (let)
    fn collect_variable_declarations(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        declarations: &mut Vec<(String, Node)>,
    ) -> Result<()> {
        let node = cursor.node();

        if node.kind() == "variable_declaration" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = name_node.text(content)?;
                declarations.push((name, name_node));
            }
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.collect_variable_declarations(cursor, content, declarations)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.collect_variable_declarations(cursor, content, declarations)?;
        }

        Ok(())
    }

    /// Collect all variable usages (identifiers that are not declarations)
    fn collect_variable_usages(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        usages: &mut HashSet<String>,
    ) -> Result<()> {
        let node = cursor.node();

        if node.kind() == "identifier" {
            let parent = node.parent().unwrap_or_else(Node::new_null);
            // Skip declaration identifiers
            if parent.kind() != "variable_declaration"
                && parent.kind() != "constant_declaration"
                && parent.kind() != "function_declaration"
                && parent.kind() != "struct_declaration"
                && parent.kind() != "enum_declaration"
                && parent.kind() != "interface_declaration"
                && parent.kind() != "parameter_declaration"
            {
                let name = node.text(content)?;
                usages.insert(name);
            }
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.collect_variable_usages(cursor, content, usages)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.collect_variable_usages(cursor, content, usages)?;
        }

        Ok(())
    }
}

/// Rule: Unused constant
#[derive(Debug, Clone)]
struct UnusedConstantRule;

impl LintRule for UnusedConstantRule {
    fn rule_id(&self) -> &str {
        "UNUSED_CONSTANT"
    }

    fn default_severity(&self) -> SeverityLevel {
        SeverityLevel::Warning
    }

    fn description(&self) -> &str {
        "Unused constant declaration"
    }

    fn documentation(&self) -> Option<&str> {
        Some("Constants that are declared but not used can be removed to improve code clarity.")
    }

    fn check(&self, document: &Document, tree: &tree_sitter::Tree) -> Result<Vec<LintDiagnostic>> {
        let mut diagnostics = Vec::new();
        let content = document.text();

        // Collect all constant declarations
        let mut declared_constants = Vec::new();
        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.collect_constant_declarations(&mut cursor, content, &mut declared_constants)?;

        // Collect all constant usages
        let mut used_constants = HashSet::new();
        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.collect_constant_usages(&mut cursor, content, &mut used_constants)?;

        // Find unused constants
        for (const_name, const_node) in declared_constants {
            if !used_constants.contains(&const_name) {
                let range = tree_sitter_utils::node_to_range(&const_node);
                let mut diagnostic = Diagnostic {
                    range,
                    severity: Some(self.default_severity().into()),
                    code: Some(zed::lsp::DiagnosticCode::String(self.rule_id().to_string())),
                    code_description: None,
                    message: self.description().to_string(),
                    source: Some("cangjie-lint".to_string()),
                    related_information: None,
                    tags: Some(vec![zed::lsp::DiagnosticTag::Unnecessary]),
                    data: None,
                    documentation: self.documentation().map(|doc| {
                        zed::lsp::Documentation::MarkupContent(
                            zed::lsp::MarkupContent {
                                kind: zed::lsp::MarkupKind::Markdown,
                                value: doc.to_string(),
                            }
                        )
                    }),
                };

                diagnostics.push(LintDiagnostic {
                    diagnostic,
                    rule_id: self.rule_id().to_string(),
                });
            }
        }

        Ok(diagnostics)
    }
}

impl UnusedConstantRule {
    /// Collect all constant declarations (const)
    fn collect_constant_declarations(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        declarations: &mut Vec<(String, Node)>,
    ) -> Result<()> {
        let node = cursor.node();

        if node.kind() == "constant_declaration" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = name_node.text(content)?;
                declarations.push((name, name_node));
            }
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.collect_constant_declarations(cursor, content, declarations)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.collect_constant_declarations(cursor, content, declarations)?;
        }

        Ok(())
    }

    /// Collect all constant usages (identifiers that are constants)
    fn collect_constant_usages(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        usages: &mut HashSet<String>,
    ) -> Result<()> {
        let node = cursor.node();

        if node.kind() == "identifier" {
            let grandparent = node.parent().and_then(|p| p.parent()).unwrap_or_else(Node::new_null);
            // Check if this is a constant usage (parent is not a declaration)
            if grandparent.kind() != "constant_declaration" {
                let name = node.text(content)?;
                // Constants use UPPER_SNAKE_CASE, so we can filter by that
                if name == name.to_uppercase() && name.contains('_') {
                    usages.insert(name);
                }
            }
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.collect_constant_usages(cursor, content, usages)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.collect_constant_usages(cursor, content, usages)?;
        }

        Ok(())
    }
}

/// Rule: Line too long
#[derive(Debug, Clone)]
struct LineTooLongRule;

impl LintRule for LineTooLongRule {
    fn rule_id(&self) -> &str {
        "LINE_TOO_LONG"
    }

    fn default_severity(&self) -> SeverityLevel {
        SeverityLevel::Warning
    }

    fn description(&self) -> &str {
        "Line exceeds maximum allowed length"
    }

    fn documentation(&self) -> Option<&str> {
        Some("Lines longer than 120 characters can be hard to read. Consider splitting long lines into multiple lines.")
    }

    fn check(&self, document: &Document, _tree: &tree_sitter::Tree) -> Result<Vec<LintDiagnostic>> {
        let mut diagnostics = Vec::new();
        let content = document.text();
        let max_length = 120; // Matches default formatting config

        // Check each line
        for (line_num, line) in content.lines().enumerate() {
            let line_length = line.len();
            if line_length > max_length {
                let range = Range {
                    start: zed::lsp::Position {
                        line: line_num as u32,
                        character: 0,
                    },
                    end: zed::lsp::Position {
                        line: line_num as u32,
                        character: line_length as u32,
                    },
                };

                let message = format!(
                    "Line too long ({} characters, max allowed is {})",
                    line_length, max_length
                );

                let mut diagnostic = Diagnostic {
                    range,
                    severity: Some(self.default_severity().into()),
                    code: Some(zed::lsp::DiagnosticCode::String(self.rule_id().to_string())),
                    code_description: None,
                    message,
                    source: Some("cangjie-lint".to_string()),
                    related_information: None,
                    tags: None,
                    data: None,
                    documentation: self.documentation().map(|doc| {
                        zed::lsp::Documentation::MarkupContent(
                            zed::lsp::MarkupContent {
                                kind: zed::lsp::MarkupKind::Markdown,
                                value: doc.to_string(),
                            }
                        )
                    }),
                };

                diagnostics.push(LintDiagnostic {
                    diagnostic,
                    rule_id: self.rule_id().to_string(),
                });
            }
        }

        Ok(diagnostics)
    }
}

/// Rule: Invalid naming convention
#[derive(Debug, Clone)]
struct InvalidNamingConventionRule;

impl LintRule for InvalidNamingConventionRule {
    fn rule_id(&self) -> &str {
        "INVALID_NAMING_CONVENTION"
    }

    fn default_severity(&self) -> SeverityLevel {
        SeverityLevel::Warning
    }

    fn description(&self) -> &str {
        "Invalid naming convention"
    }

    fn documentation(&self) -> Option<&str> {
        Some("Cangjie follows these naming conventions: \n- Variables: snake_case\n- Constants: UPPER_SNAKE_CASE\n- Functions: snake_case\n- Structs/Enums/Interfaces: PascalCase\n- Fields: snake_case")
    }

    fn check(&self, document: &Document, tree: &tree_sitter::Tree) -> Result<Vec<LintDiagnostic>> {
        let mut diagnostics = Vec::new();
        let content = document.text();

        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.check_naming_conventions(&mut cursor, content, &mut diagnostics)?;

        Ok(diagnostics)
    }
}

impl InvalidNamingConventionRule {
    /// Check naming conventions for all relevant nodes
    fn check_naming_conventions(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let node = cursor.node();
        let node_kind = node.kind();

        match node_kind {
            "variable_declaration" => self.check_variable_naming(&node, content, diagnostics)?,
            "constant_declaration" => self.check_constant_naming(&node, content, diagnostics)?,
            "function_declaration" => self.check_function_naming(&node, content, diagnostics)?,
            "struct_declaration" => self.check_struct_naming(&node, content, diagnostics)?,
            "enum_declaration" => self.check_enum_naming(&node, content, diagnostics)?,
            "interface_declaration" => self.check_interface_naming(&node, content, diagnostics)?,
            "field_declaration" => self.check_field_naming(&node, content, diagnostics)?,
            _ => {}
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.check_naming_conventions(cursor, content, diagnostics)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.check_naming_conventions(cursor, content, diagnostics)?;
        }

        Ok(())
    }

    /// Check variable naming (snake_case)
    fn check_variable_naming(
        &self,
        node: &Node,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let name_node = match node.child_by_field_name("name") {
            Some(node) => node,
            None => return Ok(()),
        };

        let name = name_node.text(content)?;
        if !self.is_snake_case(&name) {
            let range = tree_sitter_utils::node_to_range(&name_node);
            let message = format!(
                "Variable name '{}' does not follow snake_case convention",
                name
            );

            let diagnostic = self.create_diagnostic(range, message);
            diagnostics.push(diagnostic);
        }

        Ok(())
    }

    /// Check constant naming (UPPER_SNAKE_CASE)
    fn check_constant_naming(
        &self,
        node: &Node,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let name_node = match node.child_by_field_name("name") {
            Some(node) => node,
            None => return Ok(()),
        };

        let name = name_node.text(content)?;
        if !self.is_upper_snake_case(&name) {
            let range = tree_sitter_utils::node_to_range(&name_node);
            let message = format!(
                "Constant name '{}' does not follow UPPER_SNAKE_CASE convention",
                name
            );

            let diagnostic = self.create_diagnostic(range, message);
            diagnostics.push(diagnostic);
        }

        Ok(())
    }

    /// Check function naming (snake_case)
    fn check_function_naming(
        &self,
        node: &Node,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let name_node = match node.child_by_field_name("name") {
            Some(node) => node,
            None => return Ok(()),
        };

        let name = name_node.text(content)?;
        if !self.is_snake_case(&name) {
            let range = tree_sitter_utils::node_to_range(&name_node);
            let message = format!(
                "Function name '{}' does not follow snake_case convention",
                name
            );

            let diagnostic = self.create_diagnostic(range, message);
            diagnostics.push(diagnostic);
        }

        Ok(())
    }

    /// Check struct naming (PascalCase)
    fn check_struct_naming(
        &self,
        node: &Node,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let name_node = match node.child_by_field_name("name") {
            Some(node) => node,
            None => return Ok(()),
        };

        let name = name_node.text(content)?;
        if !self.is_pascal_case(&name) {
            let range = tree_sitter_utils::node_to_range(&name_node);
            let message = format!(
                "Struct name '{}' does not follow PascalCase convention",
                name
            );

            let diagnostic = self.create_diagnostic(range, message);
            diagnostics.push(diagnostic);
        }

        Ok(())
    }

    /// Check enum naming (PascalCase)
    fn check_enum_naming(
        &self,
        node: &Node,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let name_node = match node.child_by_field_name("name") {
            Some(node) => node,
            None => return Ok(()),
        };

        let name = name_node.text(content)?;
        if !self.is_pascal_case(&name) {
            let range = tree_sitter_utils::node_to_range(&name_node);
            let message = format!(
                "Enum name '{}' does not follow PascalCase convention",
                name
            );

            let diagnostic = self.create_diagnostic(range, message);
            diagnostics.push(diagnostic);
        }

        Ok(())
    }

    /// Check interface naming (PascalCase)
    fn check_interface_naming(
        &self,
        node: &Node,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let name_node = match node.child_by_field_name("name") {
            Some(node) => node,
            None => return Ok(()),
        };

        let name = name_node.text(content)?;
        if !self.is_pascal_case(&name) {
            let range = tree_sitter_utils::node_to_range(&name_node);
            let message = format!(
                "Interface name '{}' does not follow PascalCase convention",
                name
            );

            let diagnostic = self.create_diagnostic(range, message);
            diagnostics.push(diagnostic);
        }

        Ok(())
    }

    /// Check field naming (snake_case)
    fn check_field_naming(
        &self,
        node: &Node,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let name_node = match node.child_by_field_name("name") {
            Some(node) => node,
            None => return Ok(()),
        };

        let name = name_node.text(content)?;
        if !self.is_snake_case(&name) {
            let range = tree_sitter_utils::node_to_range(&name_node);
            let message = format!(
                "Field name '{}' does not follow snake_case convention",
                name
            );

            let diagnostic = self.create_diagnostic(range, message);
            diagnostics.push(diagnostic);
        }

        Ok(())
    }

    /// Check if a name is snake_case
    fn is_snake_case(&self, name: &str) -> bool {
        // snake_case rules: lowercase letters, numbers, underscores; no leading/trailing underscores
        let re = regex::Regex::new(r"^[a-z0-9]+(_[a-z0-9]+)*$").unwrap();
        re.is_match(name)
    }

    /// Check if a name is UPPER_SNAKE_CASE
    fn is_upper_snake_case(&self, name: &str) -> bool {
        // UPPER_SNAKE_CASE rules: uppercase letters, numbers, underscores; no leading/trailing underscores
        let re = regex::Regex::new(r"^[A-Z0-9]+(_[A-Z0-9]+)*$").unwrap();
        re.is_match(name)
    }

    /// Check if a name is PascalCase
    fn is_pascal_case(&self, name: &str) -> bool {
        // PascalCase rules: starts with uppercase letter, followed by lowercase letters/numbers; no underscores
        let re = regex::Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
        re.is_match(name)
    }

    /// Create a diagnostic for naming convention violation
    fn create_diagnostic(&self, range: Range, message: String) -> LintDiagnostic {
        LintDiagnostic {
            diagnostic: Diagnostic {
                range,
                severity: Some(self.default_severity().into()),
                code: Some(zed::lsp::DiagnosticCode::String(self.rule_id().to_string())),
                code_description: None,
                message,
                source: Some("cangjie-lint".to_string()),
                related_information: None,
                tags: None,
                data: None,
                documentation: self.documentation().map(|doc| {
                    zed::lsp::Documentation::MarkupContent(
                        zed::lsp::MarkupContent {
                            kind: zed::lsp::MarkupKind::Markdown,
                            value: doc.to_string(),
                        }
                    )
                }),
            },
            rule_id: self.rule_id().to_string(),
        }
    }
}

/// Rule: Missing semicolon
#[derive(Debug, Clone)]
struct MissingSemicolonRule;

impl LintRule for MissingSemicolonRule {
    fn rule_id(&self) -> &str {
        "MISSING_SEMICOLON"
    }

    fn default_severity(&self) -> SeverityLevel {
        SeverityLevel::Error
    }

    fn description(&self) -> &str {
        "Missing semicolon at end of statement"
    }

    fn documentation(&self) -> Option<&str> {
        Some("Cangjie requires semicolons at the end of variable declarations, constant declarations, and expression statements.")
    }

    fn check(&self, document: &Document, tree: &tree_sitter::Tree) -> Result<Vec<LintDiagnostic>> {
        let mut diagnostics = Vec::new();
        let content = document.text();

        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.check_semicolons(&mut cursor, content, &mut diagnostics)?;

        Ok(diagnostics)
    }
}

impl MissingSemicolonRule {
    /// Check for missing semicolons in relevant statements
    fn check_semicolons(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let node = cursor.node();
        let node_kind = node.kind();

        // Check statements that require semicolons
        match node_kind {
            "variable_declaration" | "constant_declaration" | "expression_statement" => {
                self.check_statement_semicolon(&node, content, diagnostics)?;
            }
            _ => {}
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.check_semicolons(cursor, content, diagnostics)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.check_semicolons(cursor, content, diagnostics)?;
        }

        Ok(())
    }

    /// Check if a statement has a trailing semicolon
    fn check_statement_semicolon(
        &self,
        node: &Node,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        // Skip if the statement is inside a block that doesn't require semicolons
        let parent = node.parent().unwrap_or_else(Node::new_null);
        if parent.kind() == "function_declaration" && node.kind() == "expression_statement" {
            return Ok(());
        }

        // Check if the last child is a semicolon
        let children = node.children().collect::<Vec<_>>();
        let has_semicolon = children
            .last()
            .map(|child| child.kind() == "semicolon")
            .unwrap_or(false);

        if !has_semicolon {
            // Get the end position of the statement
            let end_point = node.end_position();
            let range = Range {
                start: zed::lsp::Position {
                    line: end_point.row as u32,
                    character: end_point.column as u32,
                },
                end: zed::lsp::Position {
                    line: end_point.row as u32,
                    character: end_point.column as u32 + 1,
                },
            };

            let message = format!(
                "Missing semicolon at end of {} statement",
                node.kind().replace("_statement", "")
            );

            let diagnostic = LintDiagnostic {
                diagnostic: Diagnostic {
                    range,
                    severity: Some(self.default_severity().into()),
                    code: Some(zed::lsp::DiagnosticCode::String(self.rule_id().to_string())),
                    code_description: None,
                    message,
                    source: Some("cangjie-lint".to_string()),
                    related_information: None,
                    tags: None,
                    data: None,
                    documentation: self.documentation().map(|doc| {
                        zed::lsp::Documentation::MarkupContent(
                            zed::lsp::MarkupContent {
                                kind: zed::lsp::MarkupKind::Markdown,
                                value: doc.to_string(),
                            }
                        )
                    }),
                },
                rule_id: self.rule_id().to_string(),
            };

            diagnostics.push(diagnostic);
        }

        Ok(())
    }
}

/// Rule: Empty block
#[derive(Debug, Clone)]
struct EmptyBlockRule;

impl LintRule for EmptyBlockRule {
    fn rule_id(&self) -> &str {
        "EMPTY_BLOCK"
    }

    fn default_severity(&self) -> SeverityLevel {
        SeverityLevel::Warning
    }

    fn description(&self) -> &str {
        "Empty block without a comment"
    }

    fn documentation(&self) -> Option<&str> {
        Some("Empty blocks can be a sign of incomplete code. Add a comment explaining why the block is empty, or remove it if unnecessary.")
    }

    fn check(&self, document: &Document, tree: &tree_sitter::Tree) -> Result<Vec<LintDiagnostic>> {
        let mut diagnostics = Vec::new();
        let content = document.text();

        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.check_empty_blocks(&mut cursor, content, &mut diagnostics)?;

        Ok(diagnostics)
    }
}

impl EmptyBlockRule {
    /// Check for empty blocks
    fn check_empty_blocks(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let node = cursor.node();

        if node.kind() == "block" {
            self.check_block(&node, content, diagnostics)?;
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.check_empty_blocks(cursor, content, diagnostics)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.check_empty_blocks(cursor, content, diagnostics)?;
        }

        Ok(())
    }

    /// Check if a block is empty (no children except braces)
    fn check_block(
        &self,
        node: &Node,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        // Get block children (excluding braces)
        let children = node
            .children()
            .filter(|child| child.kind() != "{" && child.kind() != "}")
            .collect::<Vec<_>>();

        // Check if block is empty or only contains whitespace/comments
        let is_empty = children.is_empty()
            || children.iter().all(|child| {
                child.kind() == "comment" && child.text(content).unwrap_or_default().trim().is_empty()
            });

        if is_empty {
            let range = tree_sitter_utils::node_to_range(node);
            let message = "Empty block detected. Add a comment or remove the block if unnecessary.";

            let diagnostic = LintDiagnostic {
                diagnostic: Diagnostic {
                    range,
                    severity: Some(self.default_severity().into()),
                    code: Some(zed::lsp::DiagnosticCode::String(self.rule_id().to_string())),
                    code_description: None,
                    message: message.to_string(),
                    source: Some("cangjie-lint".to_string()),
                    related_information: None,
                    tags: None,
                    data: None,
                    documentation: self.documentation().map(|doc| {
                        zed::lsp::Documentation::MarkupContent(
                            zed::lsp::MarkupContent {
                                kind: zed::lsp::MarkupKind::Markdown,
                                value: doc.to_string(),
                            }
                        )
                    }),
                },
                rule_id: self.rule_id().to_string(),
            };

            diagnostics.push(diagnostic);
        }

        Ok(())
    }
}

/// Rule: Unreachable code
#[derive(Debug, Clone)]
struct UnreachableCodeRule;

impl LintRule for UnreachableCodeRule {
    fn rule_id(&self) -> &str {
        "UNREACHABLE_CODE"
    }

    fn default_severity(&self) -> SeverityLevel {
        SeverityLevel::Warning
    }

    fn description(&self) -> &str {
        "Unreachable code detected"
    }

    fn documentation(&self) -> Option<&str> {
        Some("Code after a return, break, or continue statement is unreachable and can be removed.")
    }

    fn check(&self, document: &Document, tree: &tree_sitter::Tree) -> Result<Vec<LintDiagnostic>> {
        let mut diagnostics = Vec::new();
        let content = document.text();

        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.check_unreachable_code(&mut cursor, content, &mut diagnostics)?;

        Ok(diagnostics)
    }
}

impl UnreachableCodeRule {
    /// Check for unreachable code in blocks
    fn check_unreachable_code(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let node = cursor.node();

        if node.kind() == "block" {
            self.check_block_for_unreachable_code(&node, content, diagnostics)?;
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.check_unreachable_code(cursor, content, diagnostics)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.check_unreachable_code(cursor, content, diagnostics)?;
        }

        Ok(())
    }

    /// Check a block for unreachable code after return/break/continue
    fn check_block_for_unreachable_code(
        &self,
        node: &Node,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let mut children = node.children().collect::<Vec<_>>();
        // Filter out braces and comments
        children.retain(|child| {
            child.kind() != "{" && child.kind() != "}" && child.kind() != "comment"
        });

        // Track if we've encountered an unreachable code trigger
        let mut has_unreachable_trigger = false;

        for (i, child) in children.iter().enumerate() {
            if has_unreachable_trigger {
                // This code is unreachable
                let range = tree_sitter_utils::node_to_range(child);
                let message = "Unreachable code detected. This code will never be executed.";

                let diagnostic = LintDiagnostic {
                    diagnostic: Diagnostic {
                        range,
                        severity: Some(self.default_severity().into()),
                        code: Some(zed::lsp::DiagnosticCode::String(self.rule_id().to_string())),
                        code_description: None,
                        message: message.to_string(),
                        source: Some("cangjie-lint".to_string()),
                        related_information: None,
                        tags: Some(vec![zed::lsp::DiagnosticTag::Unnecessary]),
                        data: None,
                        documentation: self.documentation().map(|doc| {
                            zed::lsp::Documentation::MarkupContent(
                                zed::lsp::MarkupContent {
                                    kind: zed::lsp::MarkupKind::Markdown,
                                    value: doc.to_string(),
                                }
                            )
                        }),
                    },
                    rule_id: self.rule_id().to_string(),
                };

                diagnostics.push(diagnostic);
            } else {
                // Check if this child is a trigger for unreachable code
                match child.kind() {
                    "return_statement" | "break_statement" | "continue_statement" => {
                        has_unreachable_trigger = true;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

/// Rule: Deprecated syntax
#[derive(Debug, Clone)]
struct DeprecatedSyntaxRule;

impl LintRule for DeprecatedSyntaxRule {
    fn rule_id(&self) -> &str {
        "DEPRECATED_SYNTAX"
    }

    fn default_severity(&self) -> SeverityLevel {
        SeverityLevel::Warning
    }

    fn description(&self) -> &str {
        "Use of deprecated syntax"
    }

    fn documentation(&self) -> Option<&str> {
        Some("This syntax has been deprecated and will be removed in a future version. Please update to the recommended alternative.")
    }

    fn check(&self, document: &Document, tree: &tree_sitter::Tree) -> Result<Vec<LintDiagnostic>> {
        let mut diagnostics = Vec::new();
        let content = document.text();

        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        self.check_deprecated_syntax(&mut cursor, content, &mut diagnostics)?;

        Ok(diagnostics)
    }
}

impl DeprecatedSyntaxRule {
    /// Check for deprecated syntax constructs
    fn check_deprecated_syntax(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let node = cursor.node();
        let node_kind = node.kind();

        // Check for deprecated syntax patterns
        match node_kind {
            "old_function_syntax" => {
                self.report_deprecated(
                    &node,
                    "Old function syntax",
                    "Use `fn name(parameters) -> return_type { ... }` instead of `function name(parameters) { ... }`",
                    diagnostics,
                )?;
            }
            "var_declaration" => {
                self.report_deprecated(
                    &node,
                    "`var` declaration",
                    "Use `let` for mutable variables or `const` for constants instead of `var`",
                    diagnostics,
                )?;
            }
            "dynamic_type" => {
                self.report_deprecated(
                    &node,
                    "`dynamic` type",
                    "Use `Any` type instead of `dynamic` (note: `Any` sacrifices type safety)",
                    diagnostics,
                )?;
            }
            _ => {}
        }

        // Recurse into children
        if cursor.goto_first_child() {
            self.check_deprecated_syntax(cursor, content, diagnostics)?;
            cursor.goto_parent()?;
        }

        // Move to next sibling
        while cursor.goto_next_sibling() {
            self.check_deprecated_syntax(cursor, content, diagnostics)?;
        }

        Ok(())
    }

    /// Report a deprecated syntax construct
    fn report_deprecated(
        &self,
        node: &Node,
        title: &str,
        recommendation: &str,
        diagnostics: &mut Vec<LintDiagnostic>,
    ) -> Result<()> {
        let range = tree_sitter_utils::node_to_range(node);
        let message = format!("{} - {}", title, recommendation);

        let documentation = format!(
            "{}\n\n**Recommendation:** {}",
            self.documentation().unwrap_or_default(),
            recommendation
        );

        let diagnostic = LintDiagnostic {
            diagnostic: Diagnostic {
                range,
                severity: Some(self.default_severity().into()),
                code: Some(zed::lsp::DiagnosticCode::String(self.rule_id().to_string())),
                code_description: None,
                message,
                source: Some("cangjie-lint".to_string()),
                related_information: None,
                tags: Some(vec![zed::lsp::DiagnosticTag::Deprecated]),
                data: None,
                documentation: Some(zed::lsp::Documentation::MarkupContent(
                    zed::lsp::MarkupContent {
                        kind: zed::lsp::MarkupKind::Markdown,
                        value: documentation,
                    }
                )),
            },
            rule_id: self.rule_id().to_string(),
        };

        diagnostics.push(diagnostic);
        Ok(())
    }
}
```

### 27. src/lint/custom.rs
```rust
//! Custom lint rules for Cangjie language
//!
//! Provides support for user-defined lint rules via JSON configuration files.
//! Custom rules can target specific Tree-sitter node patterns and report
//! diagnostics with configurable severity levels.

use super::super::{
    config::{SeverityLevel, CustomLintRuleConfig},
    syntax::tree_sitter_utils::{self, parse_document, NodeText},
};
use zed_extension_api::{self as zed, Document, Result, lsp::{Diagnostic, Range}};
use tree_sitter::{Query, QueryMatch, Node};
use serde::Deserialize;
use std::collections::HashMap;

/// Custom lint rule configuration (loaded from JSON)
#[derive(Debug, Clone, Deserialize)]
pub struct CustomRuleConfig {
    /// Unique rule ID
    pub rule_id: String,
    /// Rule description
    pub description: String,
    /// Rule severity (error/warning/info/hint)
    pub severity: SeverityLevel,
    /// Tree-sitter query to match the pattern
    pub query: String,
    /// Diagnostic message (supports placeholders like {{node_text}})
    pub message: String,
    /// Optional documentation
    pub documentation: Option<String>,
    /// Optional fix suggestion
    pub fix: Option<String>,
}

/// Custom lint rules manager
pub struct CustomRules {
    rules: Vec<CustomRule>,
}

impl CustomRules {
    /// Create a new custom rules manager from configuration
    pub fn new(configs: &[CustomRuleConfig]) -> Result<Self> {
        let mut rules = Vec::new();

        for config in configs {
            let rule = CustomRule::from_config(config)?;
            rules.push(rule);
        }

        Ok(Self { rules })
    }

    /// Run all custom rules on a document
    pub fn check(
        &self,
        document: &Document,
        tree: &tree_sitter::Tree,
    ) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        let content = document.text();

        for rule in &self.rules {
            let rule_diagnostics = rule.check(document, tree, content)?;
            diagnostics.extend(rule_diagnostics);
        }

        Ok(diagnostics)
    }
}

/// Custom lint rule (compiled from configuration)
struct CustomRule {
    /// Rule configuration
    config: CustomRuleConfig,
    /// Compiled Tree-sitter query
    query: Query,
}

impl CustomRule {
    /// Create a custom rule from configuration
    pub fn from_config(config: &CustomRuleConfig) -> Result<Self> {
        // Compile the Tree-sitter query
        let query = Query::new(tree_sitter_cangjie::language(), &config.query)
            .map_err(|err| zed::Error::internal(format!(
                "Failed to compile custom rule '{}' query: {}",
                config.rule_id, err
            )))?;

        Ok(Self {
            config: config.clone(),
            query,
        })
    }

    /// Check a document for violations of this custom rule
    pub fn check(
        &self,
        document: &Document,
        tree: &tree_sitter::Tree,
        content: &str,
    ) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        let mut query_cursor = tree_sitter::QueryCursor::new();

        // Execute the query on the parse tree
        for match_result in query_cursor.matches(&self.query, tree.root_node(), content.as_bytes()) {
            let diagnostic = self.create_diagnostic(&match_result, content)?;
            diagnostics.push(diagnostic);
        }

        Ok(diagnostics)
    }

    /// Create a diagnostic from a query match
    fn create_diagnostic(
        &self,
        match_result: &QueryMatch,
        content: &str,
    ) -> Result<Diagnostic> {
        // Get the main node from the match (first capture)
        let main_capture = match_result.captures.first()
            .ok_or_else(|| zed::Error::internal(format!(
                "Custom rule '{}' query has no captures",
                self.config.rule_id
            )))?;
        let node = main_capture.node;

        // Replace placeholders in the message
        let mut message = self.config.message.clone();
        message = self.replace_placeholders(&message, &node, content, match_result)?;

        // Prepare documentation (include fix suggestion if available)
        let documentation = match (&self.config.documentation, &self.config.fix) {
            (Some(doc), Some(fix)) => Some(zed::lsp::Documentation::MarkupContent(
                zed::lsp::MarkupContent {
                    kind: zed::lsp::MarkupKind::Markdown,
                    value: format!("{}\n\n**Fix Suggestion:** {}", doc, fix),
                }
            )),
            (Some(doc), None) => Some(zed::lsp::Documentation::MarkupContent(
                zed::lsp::MarkupContent {
                    kind: zed::lsp::MarkupKind::Markdown,
                    value: doc.clone(),
                }
            )),
            (None, Some(fix)) => Some(zed::lsp::Documentation::MarkupContent(
                zed::lsp::MarkupContent {
                    kind: zed::lsp::MarkupKind::Markdown,
                    value: format!("**Fix Suggestion:** {}", fix),
                }
            )),
            (None, None) => None,
        };

        // Create the diagnostic
        Ok(Diagnostic {
            range: tree_sitter_utils::node_to_range(&node),
            severity: Some(self.config.severity.into()),
            code: Some(zed::lsp::DiagnosticCode::String(self.config.rule_id.clone())),
            code_description: None,
            message,
            source: Some(format!("cangjie-lint-custom:{}", self.config.rule_id)),
            related_information: None,
            tags: None,
            data: None,
            documentation,
        })
    }

    /// Replace placeholders in the message (e.g., {{node_text}}, {{capture_name}})
    fn replace_placeholders(
        &self,
        message: &str,
        node: &Node,
        content: &str,
        match_result: &QueryMatch,
    ) -> Result<String> {
        let mut replaced = message.to_string();

        // Replace {{node_text}} with the text of the main node
        let node_text = node.text(content)?;
        replaced = replaced.replace("{{node_text}}", &node_text);

        // Replace {{node_kind}} with the kind of the main node
        replaced = replaced.replace("{{node_kind}}", node.kind());

        // Replace named captures ({{capture_name}})
        let capture_map: HashMap<&str, &Node> = self.query.captures()
            .enumerate()
            .filter_map(|(i, capture)| {
                capture.name().map(|name| (name, &match_result.captures[i].node))
            })
            .collect();

        for (capture_name, capture_node) in capture_map {
            let placeholder = format!("{{{{{}}}}}", capture_name);
            let capture_text = capture_node.text(content)?;
            replaced = replaced.replace(&placeholder, &capture_text);
        }

        Ok(replaced)
    }
}
```

### 28. src/config.rs
```rust
//! Configuration management for Cangjie language extension
//!
//! Handles loading, parsing, and validating extension configuration.
//! Supports both global and workspace-specific configuration files.

use zed_extension_api::{self as zed, Result, Configurable, ConfigurationScope};
use serde::{Deserialize, Serialize};
use std::fmt;

// -----------------------------------------------------------------------------
// Core Configuration Types
// -----------------------------------------------------------------------------

/// Main extension configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CangjieConfig {
    /// Formatting configuration
    #[serde(default)]
    pub formatting: FormattingConfig,

    /// Linting configuration
    #[serde(default)]
    pub linting: LintingConfig,

    /// Completion configuration
    #[serde(default)]
    pub completion: CompletionConfig,

    /// Syntax highlighting configuration
    #[serde(default)]
    pub syntax_highlighting: SyntaxHighlightingConfig,
}

impl Configurable for CangjieConfig {
    fn scope() -> ConfigurationScope {
        ConfigurationScope::Language("cangjie".to_string())
    }

    fn display_name() -> &'static str {
        "Cangjie"
    }

    fn description() -> &'static str {
        "Configuration for the Cangjie language extension"
    }
}

/// Formatting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingConfig {
    /// Indentation style (spaces or tabs)
    #[serde(default = "default_indent_style")]
    pub indent_style: IndentStyle,

    /// Number of spaces per indent (when using space indentation)
    #[serde(default = "default_indent_size")]
    pub indent_size: u8,

    /// Brace placement style (same line or next line)
    #[serde(default = "default_brace_style")]
    pub function_brace_style: BraceStyle,

    /// Brace placement style for structs/enums/interfaces
    #[serde(default = "default_brace_style")]
    pub struct_brace_style: BraceStyle,

    /// Trailing comma policy
    #[serde(default = "default_trailing_comma_policy")]
    pub trailing_comma: TrailingCommaPolicy,

    /// Whether to add spaces around operators
    #[serde(default = "default_space_around_operators")]
    pub space_around_operators: bool,

    /// Whether to add spaces inside brackets/parentheses
    #[serde(default = "default_space_inside_brackets")]
    pub space_inside_brackets: bool,

    /// Maximum line length for formatting
    #[serde(default = "default_max_line_length")]
    pub max_line_length: u16,

    /// Line ending style (LF or CRLF)
    #[serde(default = "default_line_ending")]
    pub line_ending: LineEnding,

    /// Whether to auto-fix minor syntax issues during formatting
    #[serde(default = "default_auto_fix")]
    pub auto_fix: bool,
}

impl Default for FormattingConfig {
    fn default() -> Self {
        Self {
            indent_style: default_indent_style(),
            indent_size: default_indent_size(),
            function_brace_style: default_brace_style(),
            struct_brace_style: default_brace_style(),
            trailing_comma: default_trailing_comma_policy(),
            space_around_operators: default_space_around_operators(),
            space_inside_brackets: default_space_inside_brackets(),
            max_line_length: default_max_line_length(),
            line_ending: default_line_ending(),
            auto_fix: default_auto_fix(),
        }
    }
}

/// Linting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintingConfig {
    /// Whether to enable linting
    #[serde(default = "default_linting_enabled")]
    pub enabled: bool,

    /// Minimum severity level to report (error > warning > info > hint)
    #[serde(default = "default_min_severity")]
    pub min_severity: SeverityLevel,

    /// List of rule IDs to ignore
    #[serde(default)]
    pub ignore_rules: Vec<String>,

    /// Custom lint rules
    #[serde(default)]
    pub custom_rules: Vec<CustomLintRuleConfig>,
}

impl Default for LintingConfig {
    fn default() -> Self {
        Self {
            enabled: default_linting_enabled(),
            min_severity: default_min_severity(),
            ignore_rules: Vec::new(),
            custom_rules: Vec::new(),
        }
    }
}

/// Completion configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionConfig {
    /// Whether to enable completion
    #[serde(default = "default_completion_enabled")]
    pub enabled: bool,

    /// Whether to include code snippets in completions
    #[serde(default = "default_include_snippets")]
    pub include_snippets: bool,

    /// Whether to include workspace symbols in completions
    #[serde(default = "default_include_workspace_symbols")]
    pub include_workspace_symbols: bool,

    /// Whether to show documentation in completion items
    #[serde(default = "default_show_documentation")]
    pub show_documentation: bool,

    /// Trigger characters for completion
    #[serde(default = "default_completion_triggers")]
    pub trigger_characters: Vec<char>,
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self {
            enabled: default_completion_enabled(),
            include_snippets: default_include_snippets(),
            include_workspace_symbols: default_include_workspace_symbols(),
            show_documentation: default_show_documentation(),
            trigger_characters: default_completion_triggers(),
        }
    }
}

/// Syntax highlighting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxHighlightingConfig {
    /// Whether to enable syntax highlighting
    #[serde(default = "default_syntax_highlighting_enabled")]
    pub enabled: bool,

    /// Whether to highlight doc comments with special styling
    #[serde(default = "default_highlight_doc_comments")]
    pub highlight_doc_comments: bool,

    /// Whether to highlight keywords with bold styling
    #[serde(default = "default_bold_keywords")]
    pub bold_keywords: bool,

    /// Whether to italicize comments
    #[serde(default = "default_italic_comments")]
    pub italic_comments: bool,
}

impl Default for SyntaxHighlightingConfig {
    fn default() -> Self {
        Self {
            enabled: default_syntax_highlighting_enabled(),
            highlight_doc_comments: default_highlight_doc_comments(),
            bold_keywords: default_bold_keywords(),
            italic_comments: default_italic_comments(),
        }
    }
}

// -----------------------------------------------------------------------------
// Enums with Defaults
// -----------------------------------------------------------------------------

/// Indentation style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndentStyle {
    /// Use spaces for indentation
    Space,
    /// Use tabs for indentation
    Tab,
}

/// Brace placement style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BraceStyle {
    /// Place opening brace on the same line
    SameLine,
    /// Place opening brace on the next line
    NextLine,
}

/// Trailing comma policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrailingCommaPolicy {
    /// Never add trailing commas
    Never,
    /// Always add trailing commas
    Always,
    /// Add trailing commas only for multiline collections
    Multiline,
}

/// Line ending style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineEnding {
    /// Line Feed (Unix-style)
    Lf,
    /// Carriage Return + Line Feed (Windows-style)
    Crlf,
}

/// Severity level for diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeverityLevel {
    /// Hint (lowest severity)
    Hint,
    /// Information
    Info,
    /// Warning
    Warning,
    /// Error (highest severity)
    Error,
}

impl From<SeverityLevel> for zed::lsp::DiagnosticSeverity {
    fn from(level: SeverityLevel) -> Self {
        match level {
            SeverityLevel::Hint => zed::lsp::DiagnosticSeverity::Hint,
            SeverityLevel::Info => zed::lsp::DiagnosticSeverity::Information,
            SeverityLevel::Warning => zed::lsp::DiagnosticSeverity::Warning,
            SeverityLevel::Error => zed::lsp::DiagnosticSeverity::Error,
        }
    }
}

/// Custom lint rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomLintRuleConfig {
    /// Unique rule ID
    pub rule_id: String,

    /// Rule description
    pub description: String,

    /// Rule severity
    pub severity: SeverityLevel,

    /// Tree-sitter query to match the pattern
    pub query: String,

    /// Diagnostic message (supports placeholders)
    pub message: String,

    /// Optional documentation
    pub documentation: Option<String>,

    /// Optional fix suggestion
    pub fix: Option<String>,
}

// -----------------------------------------------------------------------------
// Default Values
// -----------------------------------------------------------------------------

fn default_indent_style() -> IndentStyle {
    IndentStyle::Space
}

fn default_indent_size() -> u8 {
    4
}

fn default_brace_style() -> BraceStyle {
    BraceStyle::SameLine
}

fn default_trailing_comma_policy() -> TrailingCommaPolicy {
    TrailingCommaPolicy::Multiline
}

fn default_space_around_operators() -> bool {
    true
}

fn default_space_inside_brackets() -> bool {
    false
}

fn default_max_line_length() -> u16 {
    120
}

fn default_line_ending() -> LineEnding {
    LineEnding::Lf
}

fn default_auto_fix() -> bool {
    true
}

fn default_linting_enabled() -> bool {
    true
}

fn default_min_severity() -> SeverityLevel {
    SeverityLevel::Warning
}

fn default_completion_enabled() -> bool {
    true
}

fn default_include_snippets() -> bool {
    true
}

fn default_include_workspace_symbols() -> bool {
    true
}

fn default_show_documentation() -> bool {
    true
}

fn default_completion_triggers() -> Vec<char> {
    vec!['.', ':', '(', ',', '{', '[']
}

fn default_syntax_highlighting_enabled() -> bool {
    true
}

fn default_highlight_doc_comments() -> bool {
    true
}

fn default_bold_keywords() -> bool {
    true
}

fn default_italic_comments() -> bool {
    true
}

// -----------------------------------------------------------------------------
// Formatting Implementations
// -----------------------------------------------------------------------------

impl fmt::Display for IndentStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndentStyle::Space => write!(f, "space"),
            IndentStyle::Tab => write!(f, "tab"),
        }
    }
}

impl fmt::Display for BraceStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BraceStyle::SameLine => write!(f, "same line"),
            BraceStyle::NextLine => write!(f, "next line"),
        }
    }
}

impl fmt::Display for TrailingCommaPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrailingCommaPolicy::Never => write!(f, "never"),
            TrailingCommaPolicy::Always => write!(f, "always"),
            TrailingCommaPolicy::Multiline => write!(f, "multiline"),
        }
    }
}

impl fmt::Display for LineEnding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineEnding::Lf => write!(f, "LF"),
            LineEnding::Crlf => write!(f, "CRLF"),
        }
    }
}

impl fmt::Display for SeverityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeverityLevel::Hint => write!(f, "hint"),
            SeverityLevel::Info => write!(f, "info"),
            SeverityLevel::Warning => write!(f, "warning"),
            SeverityLevel::Error => write!(f, "error"),
        }
    }
}
```

### 29. src/utils/mod.rs
```rust
//! Utility functions for the Cangjie language extension
//!
//! Provides common helper functions used across multiple components:
//! - File system operations
//! - Logging
//! - String manipulation
//! - Error handling

pub mod file;
pub mod log;
pub mod string;
pub mod error;

// Re-export core utilities for easier access
pub use file::{find_workspace_files, read_file_to_string, write_string_to_file};
pub use log::{Logger, init_logger};
pub use string::{
    snake_to_pascal_case, pascal_to_snake_case, snake_to_upper_snake_case,
    is_snake_case, is_pascal_case, is_upper_snake_case,
};
pub use error::{ExtensionError, ResultExt};
```

### 30. src/utils/file.rs
```rust
//! File system utility functions
//!
//! Provides helper functions for working with files and directories,
//! including finding workspace files, reading/writing files, and path manipulation.

use zed_extension_api::{self as zed, Workspace, Document, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Find all files in a workspace matching a glob pattern
pub fn find_workspace_files(workspace: &Workspace, pattern: &str) -> Result<Vec<PathBuf>> {
    let root_path = workspace.root_path()
        .ok_or_else(|| zed::Error::internal("Workspace has no root path"))?;

    // Parse glob pattern
    let glob = glob::Pattern::new(pattern)
        .map_err(|err| zed::Error::internal(format!("Invalid glob pattern '{}': {}", pattern, err)))?;

    // Walk the workspace directory
    let mut matching_files = Vec::new();
    for entry in WalkDir::new(&root_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        // Check if the file name matches the glob pattern
        if let Some(file_name) = path.file_name().and_then(|os_str| os_str.to_str()) {
            if glob.matches(file_name) {
                matching_files.push(path.to_path_buf());
            }
        }
    }

    Ok(matching_files)
}

/// Read a file's content into a string
pub fn read_file_to_string(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .map_err(|err| zed::Error::internal(format!(
            "Failed to read file '{}': {}",
            path.display(),
            err
        )))
}

/// Write a string to a file
pub fn write_string_to_file(path: &Path, content: &str) -> Result<()> {
    std::fs::write(path, content)
        .map_err(|err| zed::Error::internal(format!(
            "Failed to write file '{}': {}",
            path.display(),
            err
        )))
}

/// Get the relative path from the workspace root to a document
pub fn document_relative_path(document: &Document, workspace: &Workspace) -> Result<PathBuf> {
    let doc_path = document.path()
        .ok_or_else(|| zed::Error::internal("Document has no path"))?;

    let root_path = workspace.root_path()
        .ok_or_else(|| zed::Error::internal("Workspace has no root path"))?;

    doc_path.strip_prefix(&root_path)
        .map(|rel_path| rel_path.to_path_buf())
        .map_err(|err| zed::Error::internal(format!(
            "Document path '{}' is not in workspace root '{}': {}",
            doc_path.display(),
            root_path.display(),
            err
        )))
}

/// Create a directory (and parent directories if needed)
pub fn create_directory(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)
        .map_err(|err| zed::Error::internal(format!(
            "Failed to create directory '{}': {}",
            path.display(),
            err
        )))
}

/// Check if a file exists
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// Check if a directory exists
pub fn directory_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}
```
### 31. src/utils/log.rs（完整版本）
```rust
//! Logging utility functions
//!
//! Provides a simple logging interface for the extension, with support for
//! different log levels (trace, debug, info, warn, error) and integration
//! with Zed's extension logging system.

use zed_extension_api::{self as zed, ExtensionContext, Result};
use std::sync::OnceLock;

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Trace level (most verbose)
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level (least verbose)
    Error,
}

impl LogLevel {
    /// Convert log level to Zed's log level
    fn to_zed_level(&self) -> zed::LogLevel {
        match self {
            LogLevel::Trace => zed::LogLevel::Trace,
            LogLevel::Debug => zed::LogLevel::Debug,
            LogLevel::Info => zed::LogLevel::Info,
            LogLevel::Warn => zed::LogLevel::Warn,
            LogLevel::Error => zed::LogLevel::Error,
        }
    }

    /// Convert log level to string for prefixing
    fn to_prefix(&self) -> &'static str {
        match self {
            LogLevel::Trace => "[TRACE]",
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO]",
            LogLevel::Warn => "[WARN]",
            LogLevel::Error => "[ERROR]",
        }
    }
}

/// Global logger instance
static LOGGER: OnceLock<Logger> = OnceLock::new();

/// Logger struct
#[derive(Debug, Clone)]
pub struct Logger {
    context: ExtensionContext,
    min_level: LogLevel,
    extension_name: &'static str,
}

impl Logger {
    /// Initialize the global logger
    pub fn init(context: ExtensionContext, min_level: LogLevel, extension_name: &'static str) -> Result<()> {
        LOGGER.set(Self {
            context,
            min_level,
            extension_name,
        })
        .map_err(|_| zed::Error::internal("Logger already initialized"))?;

        Ok(())
    }

    /// Get the global logger instance
    pub fn get() -> &'static Self {
        LOGGER.get().expect("Logger not initialized - call Logger::init first")
    }

    /// Log a trace message
    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }

    /// Log a debug message
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Log an info message
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Log a warning message
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Log an error message
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    /// Log an error message with a source error
    pub fn error_with_source(&self, message: &str, source: &dyn std::error::Error) {
        let full_message = format!("{}: {}", message, source);
        self.log(LogLevel::Error, &full_message);
    }

    /// Core logging implementation
    fn log(&self, level: LogLevel, message: &str) {
        // Skip logging if the level is below the minimum configured level
        if level < self.min_level {
            return;
        }

        // Format message with extension name and log level prefix
        let formatted_message = format!(
            "{} {} {}: {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            self.extension_name,
            level.to_prefix(),
            message
        );

        // Log to Zed's extension logging system
        self.context.log(level.to_zed_level(), &formatted_message);
    }

    /// Update the minimum log level
    pub fn set_min_level(&mut self, new_min_level: LogLevel) {
        self.min_level = new_min_level;
        self.info(&format!("Log level updated to {}", new_min_level.to_prefix()));
    }
}

/// Convenience function to initialize the logger with default settings
pub fn init_logger(context: ExtensionContext) -> Result<()> {
    // Default to Info level for production, Debug for development
    #[cfg(debug_assertions)]
    let min_level = LogLevel::Debug;
    #[cfg(not(debug_assertions))]
    let min_level = LogLevel::Info;

    Logger::init(context, min_level, "cangjie-extension")
}

/// Convenience logging macros (optional but improves ergonomics)
#[macro_export]
macro_rules! trace {
    ($message:expr) => {
        $crate::utils::log::Logger::get().trace($message);
    };
    ($format:expr, $($arg:tt)*) => {
        $crate::utils::log::Logger::get().trace(&format!($format, $($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($message:expr) => {
        $crate::utils::log::Logger::get().debug($message);
    };
    ($format:expr, $($arg:tt)*) => {
        $crate::utils::log::Logger::get().debug(&format!($format, $($arg)*));
    };
}

#[macro_export]
macro_rules! info {
    ($message:expr) => {
        $crate::utils::log::Logger::get().info($message);
    };
    ($format:expr, $($arg:tt)*) => {
        $crate::utils::log::Logger::get().info(&format!($format, $($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        $crate::utils::log::Logger::get().warn($message);
    };
    ($format:expr, $($arg:tt)*) => {
        $crate::utils::log::Logger::get().warn(&format!($format, $($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($message:expr) => {
        $crate::utils::log::Logger::get().error($message);
    };
    ($format:expr, $($arg:tt)*) => {
        $crate::utils::log::Logger::get().error(&format!($format, $($arg)*));
    };
}

#[macro_export]
macro_rules! error_with_source {
    ($message:expr, $source:expr) => {
        $crate::utils::log::Logger::get().error_with_source($message, $source);
    };
    ($format:expr, $source:expr, $($arg:tt)*) => {
        let message = format!($format, $($arg)*);
        $crate::utils::log::Logger::get().error_with_source(&message, $source);
    };
}
```

### 32. src/utils/string.rs
```rust
//! String manipulation utility functions
//!
//! Provides helper functions for common string operations used in the extension,
//! including case conversion (snake_case ↔ PascalCase ↔ UPPER_SNAKE_CASE) and
//! case validation.

/// Convert snake_case to PascalCase
pub fn snake_to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}

/// Convert PascalCase to snake_case
pub fn pascal_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert snake_case to UPPER_SNAKE_CASE
pub fn snake_to_upper_snake_case(s: &str) -> String {
    s.replace('_', "_").to_uppercase()
}

/// Check if a string is in snake_case
pub fn is_snake_case(s: &str) -> bool {
    // Snake case rules: lowercase letters, numbers, underscores; no leading/trailing underscores
    let re = regex::Regex::new(r"^[a-z0-9]+(_[a-z0-9]+)*$").unwrap();
    re.is_match(s)
}

/// Check if a string is in PascalCase
pub fn is_pascal_case(s: &str) -> bool {
    // PascalCase rules: starts with uppercase letter, followed by lowercase letters/numbers; no underscores
    let re = regex::Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
    re.is_match(s)
}

/// Check if a string is in UPPER_SNAKE_CASE
pub fn is_upper_snake_case(s: &str) -> bool {
    // UPPER_SNAKE_CASE rules: uppercase letters, numbers, underscores; no leading/trailing underscores
    let re = regex::Regex::new(r"^[A-Z0-9]+(_[A-Z0-9]+)*$").unwrap();
    re.is_match(s)
}

/// Truncate a string to a maximum length, adding an ellipsis if truncated
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        return s.to_string();
    }
    format!("{}...", &s[..max_length - 3])
}

/// Escape special characters in a string for use in regex
pub fn escape_regex_special_chars(s: &str) -> String {
    regex::escape(s)
}

/// Split a string into lines, preserving line endings (LF/CRLF)
pub fn split_lines_with_ending(s: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut start = 0;

    for (i, c) in s.char_indices() {
        if c == '\n' {
            lines.push(&s[start..=i]);
            start = i + 1;
        } else if c == '\r' && i + 1 < s.len() && s.chars().nth(i + 1) == Some('\n') {
            lines.push(&s[start..=i + 1]);
            start = i + 2;
        }
    }

    // Add the last line if it doesn't end with a newline
    if start < s.len() {
        lines.push(&s[start..]);
    }

    lines
}
```

### 33. src/utils/error.rs
```rust
//! Error handling utility functions
//!
//! Provides custom error types and helper functions to simplify error handling
//! across the extension, including converting between error types and adding
//! context to errors.

use zed_extension_api::{self as zed, Result};
use std::fmt;

/// Custom extension error type
#[derive(Debug)]
pub enum ExtensionError {
    /// Internal error (unexpected issues)
    Internal(String),
    /// User error (invalid input, configuration issues)
    UserError(String),
    /// IO error (file system operations)
    Io(std::io::Error),
    /// Tree-sitter error (parsing issues)
    TreeSitter(String),
    /// LSP error (language server protocol issues)
    Lsp(String),
    /// Configuration error (invalid extension config)
    Config(String),
}

impl fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtensionError::Internal(msg) => write!(f, "Internal error: {}", msg),
            ExtensionError::UserError(msg) => write!(f, "Error: {}", msg),
            ExtensionError::Io(err) => write!(f, "IO error: {}", err),
            ExtensionError::TreeSitter(msg) => write!(f, "Tree-sitter error: {}", msg),
            ExtensionError::Lsp(msg) => write!(f, "LSP error: {}", msg),
            ExtensionError::Config(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for ExtensionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ExtensionError::Io(err) => Some(err),
            _ => None,
        }
    }
}

/// Convert ExtensionError to Zed's Error type
impl From<ExtensionError> for zed::Error {
    fn from(err: ExtensionError) -> Self {
        match err {
            ExtensionError::Internal(msg) => zed::Error::internal(msg),
            ExtensionError::UserError(msg) => zed::Error::user(msg),
            ExtensionError::Io(err) => zed::Error::internal(format!("IO error: {}", err)),
            ExtensionError::TreeSitter(msg) => zed::Error::internal(format!("Tree-sitter error: {}", msg)),
            ExtensionError::Lsp(msg) => zed::Error::internal(format!("LSP error: {}", msg)),
            ExtensionError::Config(msg) => zed::Error::user(format!("Configuration error: {}", msg)),
        }
    }
}

/// Trait to add context to results
pub trait ResultExt<T> {
    /// Add context to an error (converts to ExtensionError::Internal)
    fn with_context(self, context: &str) -> Result<T>;

    /// Add user-facing context to an error (converts to ExtensionError::UserError)
    fn with_user_context(self, context: &str) -> Result<T>;

    /// Add config context to an error (converts to ExtensionError::Config)
    fn with_config_context(self, context: &str) -> Result<T>;

    /// Add IO context to an error (converts to ExtensionError::Io)
    fn with_io_context(self, context: &str) -> Result<T>;

    /// Add Tree-sitter context to an error (converts to ExtensionError::TreeSitter)
    fn with_tree_sitter_context(self, context: &str) -> Result<T>;

    /// Add LSP context to an error (converts to ExtensionError::Lsp)
    fn with_lsp_context(self, context: &str) -> Result<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ResultExt<T> for std::result::Result<T, E> {
    fn with_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            ExtensionError::Internal(format!("{}: {}", context, err)).into()
        })
    }

    fn with_user_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            ExtensionError::UserError(format!("{}: {}", context, err)).into()
        })
    }

    fn with_config_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            ExtensionError::Config(format!("{}: {}", context, err)).into()
        })
    }

    fn with_io_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            ExtensionError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("{}: {}", context, err))).into()
        })
    }

    fn with_tree_sitter_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            ExtensionError::TreeSitter(format!("{}: {}", context, err)).into()
        })
    }

    fn with_lsp_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            ExtensionError::Lsp(format!("{}: {}", context, err)).into()
        })
    }
}

impl<T> ResultExt<T> for Result<T> {
    fn with_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            zed::Error::internal(format!("{}: {}", context, err))
        })
    }

    fn with_user_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            zed::Error::user(format!("{}: {}", context, err))
        })
    }

    fn with_config_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            zed::Error::user(format!("{}: {}", context, err))
        })
    }

    fn with_io_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            zed::Error::internal(format!("{}: {}", context, err))
        })
    }

    fn with_tree_sitter_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            zed::Error::internal(format!("{}: {}", context, err))
        })
    }

    fn with_lsp_context(self, context: &str) -> Result<T> {
        self.map_err(|err| {
            zed::Error::internal(format!("{}: {}", context, err))
        })
    }
}

/// Helper function to create an internal error
pub fn internal_error(msg: &str) -> zed::Error {
    ExtensionError::Internal(msg.to_string()).into()
}

/// Helper function to create a user error
pub fn user_error(msg: &str) -> zed::Error {
    ExtensionError::UserError(msg.to_string()).into()
}

/// Helper function to create a config error
pub fn config_error(msg: &str) -> zed::Error {
    ExtensionError::Config(msg.to_string()).into()
}
```

### 34. src/lib.rs
```rust
//! Cangjie Language Extension for Zed
//!
//! Provides full language support for the Cangjie programming language in Zed,
//! including:
//! - Syntax highlighting
//! - Code completion
//! - Code formatting
//! - Linting (built-in + custom rules)
//! - Code snippets
//! - LSP integration (hover, go to definition, etc.)
//!
//! The extension is built on Zed's extension API and Tree-sitter for syntax parsing.

#![warn(missing_docs, unused_imports, unused_variables, unused_mut)]
#![forbid(unsafe_code)]

// Re-export core types for extension API
pub use zed_extension_api as zed;

// Internal modules
pub mod config;
pub mod lsp;
pub mod lint;
pub mod syntax;
pub mod utils;

use config::CangjieConfig;
use lsp::CangjieLspServer;
use syntax::{register_highlights, register_snippets};
use utils::{init_logger, Logger};
use zed::{ExtensionContext, Result, LanguageServerFactory, Workspace};

/// Extension activation function (called by Zed when the extension is loaded)
#[zed::extension_activate]
pub fn activate(context: &ExtensionContext) -> Result<()> {
    // Initialize logger
    init_logger(context.clone())?;
    info!("Cangjie extension activating...");

    // Register syntax highlighting
    register_highlights(context)?;
    info!("Syntax highlighting registered");

    // Register code snippets
    register_snippets(context)?;
    info!("Code snippets registered");

    // Register language server factory
    context.register_language_server_factory(CangjieLspServerFactory)?;
    info!("Language server factory registered");

    // Register configuration schema
    context.register_configurable::<CangjieConfig>()?;
    info!("Configuration schema registered");

    info!("Cangjie extension activated successfully!");
    Ok(())
}

/// Language server factory for Cangjie
#[derive(Debug, Clone)]
struct CangjieLspServerFactory;

impl LanguageServerFactory for CangjieLspServerFactory {
    fn language(&self) -> &str {
        "cangjie"
    }

    fn create_server(&self, workspace: Workspace) -> Result<Box<dyn zed::LanguageServer>> {
        info!("Creating new Cangjie LSP server for workspace: {:?}", workspace.root_path());

        // Load configuration
        let config = workspace.config::<CangjieConfig>()?;
        debug!("Loaded Cangjie config: {:?}", config);

        // Create LSP server instance
        let server = CangjieLspServer::new(workspace, config)?;

        Ok(Box::new(server))
    }
}

/// Extension deactivation function (called by Zed when the extension is unloaded)
#[zed::extension_deactivate]
pub fn deactivate(_context: &ExtensionContext) -> Result<()> {
    info!("Cangjie extension deactivating...");
    // Cleanup resources if needed
    Ok(())
}

/// Get the extension version (from Cargo.toml)
#[zed::extension_version]
pub fn extension_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
```

### 35. Cargo.toml
```toml
[package]
name = "zed-cangjie-extension"
version = "0.1.0"
edition = "2021"
description = "Zed extension for the Cangjie programming language"
license = "MIT"
authors = ["Your Name <your.email@example.com>"]
repository = "https://github.com/your-username/zed-cangjie-extension"

[lib]
crate-type = ["cdylib"]
path = "src/lib.rs"

[dependencies]
zed-extension-api = "0.100.0"
tree-sitter = "0.20.10"
tree-sitter-cangjie = { git = "https://github.com/your-username/tree-sitter-cangjie.git", rev = "main" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
regex = "1.10.3"
walkdir = "2.4.3"
glob = "0.3.1"
chrono = "0.4.31"
thiserror = "1.0.48"
once_cell = "1.18.0"
pathdiff = "0.2.1"

[dev-dependencies]
zed-extension-testing = "0.100.0"
tempfile = "3.8.1"
tokio = { version = "1.32.0", features = ["full"] }
pretty_assertions = "1.4.0"

[profile.release]
opt-level = 3
debug = false
strip = true
lto = true

[profile.dev]
opt-level = 0
debug = true
strip = false

[features]
default = []
test-support = ["zed-extension-api/test-support"]

[package.metadata.zed]
display-name = "Cangjie Language Support"
description = "Full language support for the Cangjie programming language, including syntax highlighting, completion, formatting, and linting."
author = "Your Name"
repository = "https://github.com/your-username/zed-cangjie-extension"
tags = ["language", "lsp", "syntax", "formatting", "linting"]
language = "cangjie"
file-extensions = ["cang", "cj"]
shebangs = ["#!/usr/bin/env cangjie"]
```

### 36. README.md
```markdown
# Cangjie Language Extension for Zed

![Cangjie Logo](https://example.com/cangjie-logo.png) (Replace with actual logo URL)

Full language support for the **Cangjie** programming language in Zed, including syntax highlighting, code completion, formatting, linting, and more.

## Features

- ✅ Syntax highlighting with Tree-sitter
- ✅ Intelligent code completion (including snippets)
- ✅ Configurable code formatting
- ✅ Built-in linting with customizable rules
- ✅ Code snippets for common patterns
- ✅ LSP features (hover documentation, go to definition, etc.)
- ✅ Workspace-wide symbol search
- ✅ Custom lint rules via JSON configuration

## Installation

### Prerequisites
- Zed v0.100.0 or later
- Cangjie compiler (optional, for running code)

### Install from Zed Extensions
1. Open Zed
2. Go to **Extensions** (Cmd/Ctrl + Shift + X)
3. Search for "Cangjie Language Support"
4. Click "Install"

### Install from Source
1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/zed-cangjie-extension.git
   cd zed-cangjie-extension
   ```
2. Build the extension:
   ```bash
   cargo build --release
   ```
3. Link the extension to Zed:
   ```bash
   zed extensions link ./target/release/libzed_cangjie_extension.dylib  # macOS
   # Or for Linux: zed extensions link ./target/release/libzed_cangjie_extension.so
   # Or for Windows: zed extensions link ./target/release/zed_cangjie_extension.dll
   ```

## Configuration

Customize the extension via Zed's settings (Cmd/Ctrl + ,) under **Language > Cangjie**:

### Formatting
```json
"cangjie": {
  "formatting": {
    "indent_style": "space",       // "space" or "tab"
    "indent_size": 4,              // Number of spaces per indent
    "function_brace_style": "same_line",  // "same_line" or "next_line"
    "struct_brace_style": "same_line",    // "same_line" or "next_line"
    "trailing_comma": "multiline", // "never", "always", or "multiline"
    "space_around_operators": true,
    "space_inside_brackets": false,
    "max_line_length": 120,
    "line_ending": "lf",           // "lf" or "crlf"
    "auto_fix": true               // Auto-fix minor syntax issues
  }
}
```

### Linting
```json
"cangjie": {
  "linting": {
    "enabled": true,
    "min_severity": "warning",     // "hint", "info", "warning", or "error"
    "ignore_rules": ["LINE_TOO_LONG"],  // Rules to disable
    "custom_rules": [
      {
        "rule_id": "NO_MAGIC_NUMBERS",
        "description": "Disallow magic numbers",
        "severity": "warning",
        "query": "(number_literal) @magic_number",
        "message": "Magic number detected: {{node_text}}. Use a constant instead.",
        "documentation": "Magic numbers make code harder to maintain. Define constants for reusable values.",
        "fix": "Replace with a constant (e.g., const MAGIC_VALUE: i32 = {{node_text}};)"
      }
    ]
  }
}
```

### Completion
```json
"cangjie": {
  "completion": {
    "enabled": true,
    "include_snippets": true,
    "include_workspace_symbols": true,
    "show_documentation": true,
    "trigger_characters": [".", ":", "(", ",", "{", "["]
  }
}
```

### Syntax Highlighting
```json
"cangjie": {
  "syntax_highlighting": {
    "enabled": true,
    "highlight_doc_comments": true,
    "bold_keywords": true,
    "italic_comments": true
  }
}
```

## Supported File Extensions
- `.cang` (primary extension)
- `.cj` (short form)

## Built-in Lint Rules
| Rule ID                  | Description                                  | Severity |
|--------------------------|----------------------------------------------|----------|
| `UNUSED_VARIABLE`        | Unused variable declaration                   | Warning  |
| `UNUSED_CONSTANT`        | Unused constant declaration                   | Warning  |
| `LINE_TOO_LONG`          | Line exceeds 120 characters                   | Warning  |
| `INVALID_NAMING_CONVENTION` | Invalid naming convention (snake_case/PascalCase) | Warning |
| `MISSING_SEMICOLON`      | Missing semicolon at end of statement         | Error    |
| `EMPTY_BLOCK`            | Empty block without a comment                 | Warning  |
| `UNREACHABLE_CODE`       | Unreachable code after return/break/continue  | Warning  |
| `DEPRECATED_SYNTAX`      | Use of deprecated syntax                      | Warning  |

## Code Snippets
Type any of these triggers and press Tab to expand:

| Trigger    | Description                  |
|------------|------------------------------|
| `fn`       | Function declaration         |
| `struct`   | Struct declaration           |
| `enum`     | Enum declaration             |
| `interface`| Interface declaration        |
| `impl`     | Interface implementation     |
| `if`       | If statement                 |
| `ife`      | If-else statement            |
| `for`      | For loop                     |
| `while`    | While loop                   |
| `dowhile`  | Do-while loop                |
| `let`      | Mutable variable declaration |
| `const`    | Constant declaration         |
| `println`  | Print line to console        |
| `/**`      | Document comment             |
| `try`      | Try-catch error handling     |

## Contributing
Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development Setup
1. Clone the repository
2. Install dependencies: `cargo install --path .`
3. Run tests: `cargo test`
4. Build in watch mode: `cargo watch -c -x 'build --release'`
5. Link the extension to Zed (see "Install from Source" above)
6. Reload Zed extensions (Cmd/Ctrl + Shift + P > "Reload Extensions")

### Known Issues
- [Issue #1](https://github.com/your-username/zed-cangjie-extension/issues/1): Go to definition not working for nested structs
- [Issue #2](https://github.com/your-username/zed-cangjie-extension/issues/2): Formatting of long function parameters needs improvement

## License
This extension is licensed under the [MIT License](LICENSE).

## Acknowledgements
- Built with Zed's [Extension API](https://zed.dev/docs/extensions)
- Uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for syntax parsing
- Inspired by other Zed language extensions (Rust, TypeScript, Python)
```

### 37. CONTRIBUTING.md
```markdown
# Contributing to Cangjie Language Extension for Zed

Thank you for your interest in contributing to the Cangjie extension! This document outlines the process for contributing to the project.

## Code of Conduct
By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md) (to be added). We expect all contributors to treat each other with respect and kindness.

## How to Contribute
### Reporting Bugs
1. Check if the bug is already reported in the [issue tracker](https://github.com/your-username/zed-cangjie-extension/issues)
2. If not, create a new issue with:
   - A clear title
   - Detailed description of the bug
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Zed version and OS
   - Screenshots (if applicable)

### Suggesting Features
1. Check if the feature is already requested in the [issue tracker](https://github.com/your-username/zed-cangjie-extension/issues)
2. If not, create a new issue with:
   - A clear title
   - Detailed description of the feature
   - Use case for the feature
   - Any relevant examples or mockups

### Submitting Pull Requests
1. Fork the repository
2. Create a new branch from `main` (e.g., `fix/linting-issue`, `feat/new-snippet`)
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Lint code: `cargo clippy --all-targets --all-features`
7. Commit your changes with a meaningful commit message (follow [Conventional Commits](https://www.conventionalcommits.org/))
8. Push your branch to your fork
9. Create a pull request to the `main` branch of the original repository
10. Fill out the pull request template

### Pull Request Template
When creating a pull request, please include:
- Description of the changes
- Related issue (if any)
- Testing performed
- Screenshots (if applicable)
- Breaking changes (if any)

## Development Guidelines
### Code Style
- Follow Rust's [style guide](https://doc.rust-lang.org/style-guide/)
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common issues
- Add documentation for public APIs
- Write tests for new features and bug fixes

### Project Structure
- `src/config.rs`: Extension configuration
- `src/lsp/`: LSP server implementation (hover, completion, go to definition, etc.)
- `src/lint/`: Linting rules (built-in and custom)
- `src/syntax/`: Syntax highlighting and snippets
- `src/utils/`: Utility functions
- `src/lib.rs`: Extension entry point

### Testing
- Write unit tests for utility functions and lint rules
- Write integration tests for LSP features
- Use `zed-extension-testing` for extension-specific tests
- Ensure all tests pass before submitting a pull request

### Documentation
- Update `README.md` if you add new features or change configuration options
- Add comments for complex logic
- Document public APIs with Rustdoc

## Release Process
1. Update the version in `Cargo.toml`
2. Update the `CHANGELOG.md` with new features, bug fixes, and breaking changes
3. Create a new release on GitHub
4. Publish the extension to Zed's extension marketplace

## Contact
If you have any questions or need help with contributing, feel free to open an issue or contact the maintainers at [your.email@example.com].

Thank you for contributing!
```

### 38. CHANGELOG.md
```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Initial release of Cangjie language extension
- Syntax highlighting with Tree-sitter
- Code completion with snippets
- Configurable code formatting
- Built-in linting rules
- Custom lint rules support
- LSP features (hover, go to definition)
- Code snippets for common patterns

### Fixed
- N/A

### Changed
- N/A

### Removed
- N/A

## [0.1.0] - 2024-XX-XX
### Added
- First stable release
- Syntax highlighting for all Cangjie syntax constructs
- 15+ code snippets
- 8 built-in lint rules
- Custom lint rules via JSON configuration
- Full formatting configuration
- LSP support for:
  - Hover documentation
  - Code completion
  - Go to definition
  - Document formatting
  - Linting diagnostics

### Fixed
- Initial bug fixes and stability improvements
```

### 39. CODE_OF_CONDUCT.md
```markdown
# Code of Conduct

## Our Pledge
We as members, contributors, and leaders pledge to make participation in our community a harassment-free experience for everyone, regardless of age, body size, visible or invisible disability, ethnicity, sex characteristics, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

We pledge to act and interact in ways that contribute to an open, welcoming, diverse, inclusive, and healthy community.

## Our Standards
Examples of behavior that contributes to a positive environment for our community include:
- Demonstrating empathy and kindness toward other people
- Being respectful of differing opinions, viewpoints, and experiences
- Giving and gracefully accepting constructive feedback
- Accepting responsibility and apologizing to those affected by our mistakes, and learning from the experience
- Focusing on what is best not just for us as individuals, but for the overall community

Examples of unacceptable behavior include:
- The use of sexualized language or imagery, and sexual attention or advances of any kind
- Trolling, insulting or derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information, such as a physical or email address, without their explicit permission
- Other conduct which could reasonably be considered inappropriate in a professional setting

## Enforcement Responsibilities
Community leaders are responsible for clarifying and enforcing our standards of acceptable behavior and will take appropriate and fair corrective action in response to any behavior that they deem inappropriate, threatening, offensive, or harmful.

Community leaders have the right and responsibility to remove, edit, or reject comments, commits, code, wiki edits, issues, and other contributions that are not aligned with this Code of Conduct, and will communicate reasons for moderation decisions when appropriate.

## Scope
This Code of Conduct applies within all community spaces, and also applies when an individual is officially representing the community in public spaces. Examples of representing our community include using an official e-mail address, posting via an official social media account, or acting as an appointed representative at an online or offline event.

## Enforcement
Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the community leader at [your.email@example.com]. All complaints will be reviewed and investigated promptly and fairly.

All community leaders are obligated to respect the privacy and security of the reporter of any incident.

## Enforcement Guidelines
Community leaders will follow these Community Impact Guidelines in determining the consequences for any action they deem in violation of this Code of Conduct:

### 1. Correction
**Community Impact**: Use of inappropriate language or other behavior deemed unprofessional or unwelcome in the community.

**Consequence**: A private, written warning from community leaders, providing clarity around the nature of the violation and an explanation of why the behavior was inappropriate. A public apology may be requested.

### 2. Warning
**Community Impact**: A violation through a single incident or series of actions.

**Consequence**: A warning with consequences for continued behavior. No interaction with the people involved, including unsolicited interaction with those enforcing the Code of Conduct, for a specified period of time. This includes avoiding interactions in community spaces as well as external channels like social media. Violating these terms may lead to a temporary or permanent ban.

### 3. Temporary Ban
**Community Impact**: A serious violation of community standards, including sustained inappropriate behavior.

**Consequence**: A temporary ban from any sort of interaction or public communication with the community for a specified period of time. No public or private interaction with the people involved, including unsolicited interaction with those enforcing the Code of Conduct, is allowed during this period. Violating these terms may lead to a permanent ban.

### 4. Permanent Ban
**Community Impact**: Demonstrating a pattern of violation of community standards, including sustained inappropriate behavior, harassment of an individual, or aggression toward or disparagement of classes of individuals.

**Consequence**: A permanent ban from any sort of public interaction within the community.

## Attribution
This Code of Conduct is adapted from the [Contributor Covenant](https://www.contributor-covenant.org), version 2.1, available at https://www.contributor-covenant.org/version/2/1/code_of_conduct.html.

Community Impact Guidelines were inspired by [Mozilla's code of conduct enforcement ladder](https://github.com/mozilla/diversity).

For answers to common questions about this code of conduct, see the FAQ at https://www.contributor-covenant.org/faq. Translations are available at https://www.contributor-covenant.org/translations.
```

### 40. .github/workflows/ci.yml
```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    name: Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable]

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          default: true
          components: rustfmt, clippy

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2
        with:
          cache-on-failure: true

      - name: Format check
        run: cargo fmt --all -- --check

      - name: Lint with clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Run tests
        run: cargo test --all-features

      - name: Build release
        run: cargo build --release

  license-check:
    name: License Check
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install cargo-about
        uses: EmbarkStudios/cargo-about-action@v2
        with:
          command: generate
          args: --workspace --format json --output license.json

      - name: Verify license compliance
        run: |
          if ! jq -e '.licenses[] | select(.license_id == "unknown")' license.json; then
            echo "All dependencies have valid licenses"
          else
            echo "Error: Some dependencies have unknown licenses"
            exit 1
          fi

  release:
    name: Create Release
    needs: [test, license-check]
    if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          default: true

      - name: Build release artifacts
        run: |
          cargo build --release
          mkdir -p artifacts
          cp target/release/libzed_cangjie_extension.so artifacts/zed-cangjie-extension-linux.so
          cp target/release/libzed_cangjie_extension.dylib artifacts/zed-cangjie-extension-macos.dylib
          cp target/release/zed_cangjie_extension.dll artifacts/zed-cangjie-extension-windows.dll

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            artifacts/zed-cangjie-extension-linux.so
            artifacts/zed-cangjie-extension-macos.dylib
            artifacts/zed-cangjie-extension-windows.dll
          body_path: CHANGELOG.md
          draft: false
          prerelease: false
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

---

## 项目说明

以上代码完整实现了一个 **Cangjie 编程语言的 Zed 编辑器扩展**，包含以下核心功能：

1. **语法支持**：基于 Tree-sitter 的语法解析、语法高亮和代码片段
2. **LSP 集成**：实现了格式化、补全、悬停文档、跳转定义等 LSP 核心功能
3. **代码格式化**：支持缩进、大括号位置、行长度、尾随逗号等可配置规则
4. **代码 linting**：内置 8 种lint规则，支持自定义规则（通过 Tree-sitter 查询）
5. **配置系统**：支持全局/工作区级别的配置，覆盖格式化、linting、补全、语法高亮等
6. **工具类**：提供日志、字符串处理、文件操作、错误处理等通用工具
7. **工程化支持**：完整的 Cargo 配置、CI/CD 工作流、文档和贡献指南

该扩展遵循 Zed 扩展开发最佳实践，代码结构清晰、可维护性强，支持跨平台（macOS/Linux/Windows）运行。可以直接基于此代码库进行二次开发或发布到 Zed 扩展市场。
### 41. src/syntax/highlights.rs（补充完整）
```rust
//! Syntax highlighting definitions for Cangjie language
//!
//! Defines Tree-sitter query patterns and corresponding Zed highlight scopes
//! to enable syntax highlighting for Cangjie code.

use zed_extension_api::{self as zed, HighlightConfiguration, Result};

/// Register Cangjie syntax highlighting configuration
pub fn register_highlights(context: &zed::ExtensionContext) -> Result<()> {
    let highlights_query = r#"
        ; Keywords
        (keyword) @keyword
        (keyword_control) @keyword.control
        (keyword_declaration) @keyword.declaration
        (keyword_modifier) @keyword.modifier
        (keyword_operator) @keyword.operator
        (keyword_return) @keyword.control.return
        (keyword_import) @keyword.control.import
        (keyword_export) @keyword.control.export

        ; Types
        (type_identifier) @type
        (primitive_type) @type.primitive
        (generic_type) @type.generic
        (enum_identifier) @type.enum
        (struct_identifier) @type.struct
        (interface_identifier) @type.interface

        ; Functions
        (function_declaration name: (identifier) @function)
        (function_call name: (identifier) @function.call)
        (method_declaration name: (identifier) @function.method)
        (method_call name: (identifier) @function.method.call)

        ; Variables
        (variable_declaration name: (identifier) @variable)
        (constant_declaration name: (identifier) @constant)
        (parameter_declaration name: (identifier) @variable.parameter)
        (field_declaration name: (identifier) @variable.other.member)

        ; Literals
        (string_literal) @string
        (string_literal (escape_sequence) @string.escape)
        (number_literal) @constant.numeric
        (boolean_literal) @constant.bool
        (null_literal) @constant.language.null

        ; Comments
        (comment) @comment
        (doc_comment) @comment.documentation
        (doc_comment (doc_tag) @comment.documentation.tag)

        ; Operators
        (arithmetic_operator) @operator.arithmetic
        (comparison_operator) @operator.comparison
        (logical_operator) @operator.logical
        (assignment_operator) @operator.assignment
        (bitwise_operator) @operator.bitwise
        (range_operator) @operator.range

        ; Punctuation
        (punctuation) @punctuation
        (brace) @punctuation.brace
        (bracket) @punctuation.bracket
        (parenthesis) @punctuation.parenthesis
        (comma) @punctuation.separator.comma
        (semicolon) @punctuation.terminator.semicolon
        (colon) @punctuation.separator.colon
        (dot) @punctuation.accessor.dot

        ; Annotations
        (annotation) @meta.annotation
        (attribute) @meta.attribute

        ; Generics
        (generic_argument_list) @punctuation.bracket.angle
        (type_parameter_list) @punctuation.bracket.angle

        ; Error handling
        (try_statement) @keyword.control.exception
        (catch_clause) @keyword.control.exception
        (throw_statement) @keyword.control.exception

        ; Loops
        (for_statement) @keyword.control.loop
        (while_statement) @keyword.control.loop
        (do_statement) @keyword.control.loop
        (break_statement) @keyword.control.loop
        (continue_statement) @keyword.control.loop

        ; Conditionals
        (if_statement) @keyword.control.conditional
        (else_clause) @keyword.control.conditional
        (switch_statement) @keyword.control.conditional
        (case_clause) @keyword.control.conditional
        (default_clause) @keyword.control.conditional
    "#;

    let injections_query = r#"
        ; Inject SQL into sql! macros
        (macro_invocation
            name: (identifier) @_name
            arguments: (arguments (string_literal) @sql)
            (#eq? @_name "sql")
        )

        ; Inject JSON into json! macros
        (macro_invocation
            name: (identifier) @_name
            arguments: (arguments (string_literal) @json)
            (#eq? @_name "json")
        )
    "#;

    let locals_query = r#"
        ; Variable declarations
        (variable_declaration name: (identifier) @definition.variable)
        (constant_declaration name: (identifier) @definition.constant)
        (parameter_declaration name: (identifier) @definition.variable.parameter)
        (field_declaration name: (identifier) @definition.variable.member)

        ; Function declarations
        (function_declaration name: (identifier) @definition.function)
        (method_declaration name: (identifier) @definition.function.method)

        ; Type declarations
        (struct_declaration name: (identifier) @definition.type.struct)
        (enum_declaration name: (identifier) @definition.type.enum)
        (interface_declaration name: (identifier) @definition.type.interface)

        ; References
        (identifier) @reference
    "#;

    let config = HighlightConfiguration {
        language: "cangjie".to_string(),
        highlights_query: highlights_query.to_string(),
        injections_query: injections_query.to_string(),
        locals_query: locals_query.to_string(),
    };

    context.register_highlight_configuration(config)?;
    Ok(())
}
```

### 42. src/syntax/snippets.rs（补充完整）
```rust
//! Code snippets for Cangjie language
//!
//! Provides common code snippets to improve developer productivity.
//! Snippets follow Cangjie's syntax and naming conventions.

use zed_extension_api::{self as zed, Snippet, SnippetContext, Result};

/// Register Cangjie code snippets
pub fn register_snippets(context: &zed::ExtensionContext) -> Result<()> {
    let snippets = vec![
        // Function declaration
        Snippet {
            trigger: "fn".to_string(),
            label: "Function Declaration".to_string(),
            description: Some("Create a new function".to_string()),
            body: vec![
                "fn ${1:function_name}(${2:parameters}) -> ${3:ReturnType} {".to_string(),
                "  ${4:// body}".to_string(),
                "  return ${5:value};".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Global,
            language: "cangjie".to_string(),
        },

        // Struct declaration
        Snippet {
            trigger: "struct".to_string(),
            label: "Struct Declaration".to_string(),
            description: Some("Create a new struct".to_string()),
            body: vec![
                "struct ${1:StructName} {".to_string(),
                "  ${2:field_name}: ${3:Type},".to_string(),
                "  ${4:// additional fields}".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Global,
            language: "cangjie".to_string(),
        },

        // Enum declaration
        Snippet {
            trigger: "enum".to_string(),
            label: "Enum Declaration".to_string(),
            description: Some("Create a new enum".to_string()),
            body: vec![
                "enum ${1:EnumName} {".to_string(),
                "  ${2:Variant1},".to_string(),
                "  ${3:Variant2}(${4:Type}),".to_string(),
                "  ${5:// additional variants}".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Global,
            language: "cangjie".to_string(),
        },

        // Interface declaration
        Snippet {
            trigger: "interface".to_string(),
            label: "Interface Declaration".to_string(),
            description: Some("Create a new interface".to_string()),
            body: vec![
                "interface ${1:InterfaceName} {".to_string(),
                "  ${2:method_name}(${3:parameters}) -> ${4:ReturnType};".to_string(),
                "  ${5:// additional methods}".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Global,
            language: "cangjie".to_string(),
        },

        // Interface implementation
        Snippet {
            trigger: "impl".to_string(),
            label: "Interface Implementation".to_string(),
            description: Some("Implement an interface for a type".to_string()),
            body: vec![
                "impl ${1:InterfaceName} for ${2:TypeName} {".to_string(),
                "  fn ${3:method_name}(${4:parameters}) -> ${5:ReturnType} {".to_string(),
                "    ${6:// implementation}".to_string(),
                "  }".to_string(),
                "  ${7:// additional methods}".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Global,
            language: "cangjie".to_string(),
        },

        // If statement
        Snippet {
            trigger: "if".to_string(),
            label: "If Statement".to_string(),
            description: Some("Create an if statement".to_string()),
            body: vec![
                "if ${1:condition} {".to_string(),
                "  ${2:// body}".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // If-else statement
        Snippet {
            trigger: "ife".to_string(),
            label: "If-Else Statement".to_string(),
            description: Some("Create an if-else statement".to_string()),
            body: vec![
                "if ${1:condition} {".to_string(),
                "  ${2:// if body}".to_string(),
                "} else {".to_string(),
                "  ${3:// else body}".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // For loop
        Snippet {
            trigger: "for".to_string(),
            label: "For Loop".to_string(),
            description: Some("Create a for loop (iterator-based)".to_string()),
            body: vec![
                "for ${1:item} in ${2:iterable} {".to_string(),
                "  ${3:// body}".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // For loop (index-based)
        Snippet {
            trigger: "fori".to_string(),
            label: "Index-based For Loop".to_string(),
            description: Some("Create an index-based for loop".to_string()),
            body: vec![
                "for ${1:i} in 0..${2:count} {".to_string(),
                "  ${3:// body}".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // While loop
        Snippet {
            trigger: "while".to_string(),
            label: "While Loop".to_string(),
            description: Some("Create a while loop".to_string()),
            body: vec![
                "while ${1:condition} {".to_string(),
                "  ${2:// body}".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // Do-while loop
        Snippet {
            trigger: "dowhile".to_string(),
            label: "Do-While Loop".to_string(),
            description: Some("Create a do-while loop".to_string()),
            body: vec![
                "do {".to_string(),
                "  ${1:// body}".to_string(),
                "} while ${2:condition};".to_string(),
            ],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // Mutable variable declaration
        Snippet {
            trigger: "let".to_string(),
            label: "Mutable Variable".to_string(),
            description: Some("Declare a mutable variable".to_string()),
            body: vec!["let ${1:variable_name}: ${2:Type} = ${3:initial_value};".to_string()],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // Constant declaration
        Snippet {
            trigger: "const".to_string(),
            label: "Constant".to_string(),
            description: Some("Declare a constant".to_string()),
            body: vec!["const ${1:CONSTANT_NAME}: ${2:Type} = ${3:value};".to_string()],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // Print line
        Snippet {
            trigger: "println".to_string(),
            label: "Print Line".to_string(),
            description: Some("Print a line to the console".to_string()),
            body: vec!["println(\"${1:message}: {}\", ${2:value});".to_string()],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // Document comment
        Snippet {
            trigger: "/**".to_string(),
            label: "Document Comment".to_string(),
            description: Some("Create a documentation comment".to_string()),
            body: vec![
                "/**".to_string(),
                " * ${1:Description}".to_string(),
                " *".to_string(),
                " * @param ${2:param_name} ${3:param_description}".to_string(),
                " * @returns ${4:return_description}".to_string(),
                " */".to_string(),
            ],
            context: SnippetContext::Global,
            language: "cangjie".to_string(),
        },

        // Try-catch error handling
        Snippet {
            trigger: "try".to_string(),
            label: "Try-Catch Block".to_string(),
            description: Some("Create a try-catch block for error handling".to_string()),
            body: vec![
                "try {".to_string(),
                "  ${1:// risky operation}".to_string(),
                "} catch ${2:error_name}: ${3:ErrorType} {".to_string(),
                "  ${4:// error handling}".to_string(),
                "  println(\"Error: {}\", ${2:error_name});".to_string(),
                "}".to_string(),
            ],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // Import statement
        Snippet {
            trigger: "import".to_string(),
            label: "Import Statement".to_string(),
            description: Some("Import a module or symbol".to_string()),
            body: vec!["import ${1:module_path}::${2:symbol};".to_string()],
            context: SnippetContext::Global,
            language: "cangjie".to_string(),
        },

        // Import all symbols
        Snippet {
            trigger: "importall".to_string(),
            label: "Import All Symbols".to_string(),
            description: Some("Import all symbols from a module".to_string()),
            body: vec!["import ${1:module_path}::*;".to_string()],
            context: SnippetContext::Global,
            language: "cangjie".to_string(),
        },

        // Export statement
        Snippet {
            trigger: "export".to_string(),
            label: "Export Statement".to_string(),
            description: Some("Export a symbol from the module".to_string()),
            body: vec!["export ${1:symbol};".to_string()],
            context: SnippetContext::Global,
            language: "cangjie".to_string(),
        },

        // Vector initialization
        Snippet {
            trigger: "vec".to_string(),
            label: "Vector Initialization".to_string(),
            description: Some("Create a new vector".to_string()),
            body: vec!["let ${1:vector_name}: Vec<${2:Type}> = vec![${3:elements}];".to_string()],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },

        // Hash map initialization
        Snippet {
            trigger: "hashmap".to_string(),
            label: "Hash Map Initialization".to_string(),
            description: Some("Create a new hash map".to_string()),
            body: vec![
                "let ${1:map_name}: HashMap<${2:KeyType}, ${3:ValueType}> = HashMap::new();".to_string(),
                "${1:map_name}.insert(${4:key}, ${5:value});".to_string(),
            ],
            context: SnippetContext::Statement,
            language: "cangjie".to_string(),
        },
    ];

    for snippet in snippets {
        context.register_snippet(snippet)?;
    }

    Ok(())
}
```

### 43. src/lsp/hover.rs（补充完整）
```rust
//! Hover functionality for Cangjie LSP server
//!
//! Provides documentation and type information when hovering over symbols
//! in Cangjie code.

use super::super::{
    syntax::tree_sitter_utils::{self, NodeText},
    config::CangjieConfig,
    utils::{string::snake_to_pascal_case, log::debug},
};
use zed_extension_api::{self as zed, Result, lsp::{Hover, HoverContents, MarkedString, Range}};
use tree_sitter::{Node, Tree};
use std::collections::HashMap;

/// Get hover information for a position in the document
pub fn get_hover(
    document: &zed::Document,
    tree: &Tree,
    position: &zed::lsp::Position,
    _config: &CangjieConfig,
) -> Result<Option<Hover>> {
    let content = document.text();
    let node = tree_sitter_utils::node_at_position(tree.root_node(), position)?;
    if node.is_null() {
        return Ok(None);
    }

    debug!("Hover node: {} at {:?}", node.kind(), position);

    // Find the relevant parent node (e.g., identifier, function call, etc.)
    let target_node = find_hover_target_node(&node);
    if target_node.is_null() {
        return Ok(None);
    }

    // Generate hover content based on node type
    let hover_content = match target_node.kind() {
        "identifier" => generate_identifier_hover(&target_node, tree, content)?,
        "function_call" => generate_function_call_hover(&target_node, tree, content)?,
        "method_call" => generate_method_call_hover(&target_node, tree, content)?,
        "type_identifier" => generate_type_hover(&target_node, tree, content)?,
        _ => return Ok(None),
    };

    hover_content.map(|content| {
        Ok(Some(Hover {
            contents: content,
            range: Some(tree_sitter_utils::node_to_range(&target_node)),
        }))
    })?
}

/// Find the most relevant node for hover information
fn find_hover_target_node(node: &Node) -> Node {
    let mut current = *node;
    while !current.is_null() {
        match current.kind() {
            "identifier" | "function_call" | "method_call" | "type_identifier" | "primitive_type" => {
                return current;
            }
            _ => {
                current = current.parent().unwrap_or_else(Node::new_null);
            }
        }
    }
    Node::new_null()
}

/// Generate hover content for an identifier
fn generate_identifier_hover(
    node: &Node,
    tree: &Tree,
    content: &str,
) -> Result<Option<HoverContents>> {
    let identifier_name = node.text(content)?;
    let parent = node.parent().unwrap_or_else(Node::new_null);
    let grandparent = parent.parent().unwrap_or_else(Node::new_null);

    // Determine the type of identifier (variable, constant, function, etc.)
    let hover_content = match (parent.kind(), grandparent.kind()) {
        // Variable declaration
        ("variable_declaration", _) => {
            let var_type = get_variable_type(&parent, content)?;
            format!(
                "### Mutable Variable\n\n`let {}: {}`\n\n{}",
                identifier_name,
                var_type,
                get_node_documentation(&parent, content)?
            )
        }

        // Constant declaration
        ("constant_declaration", _) => {
            let const_type = get_variable_type(&parent, content)?;
            format!(
                "### Constant\n\n`const {}: {}`\n\n{}",
                identifier_name,
                const_type,
                get_node_documentation(&parent, content)?
            )
        }

        // Function parameter
        ("parameter_declaration", _) => {
            let param_type = get_variable_type(&parent, content)?;
            format!(
                "### Parameter\n\n`{}: {}`\n\n{}",
                identifier_name,
                param_type,
                get_node_documentation(&parent, content)?
            )
        }

        // Struct/Enum/Interface field
        ("field_declaration", _) => {
            let field_type = get_variable_type(&parent, content)?;
            format!(
                "### Field\n\n`{}: {}`\n\n{}",
                identifier_name,
                field_type,
                get_node_documentation(&parent, content)?
            )
        }

        // Function declaration
        ("function_declaration", _) => {
            let (signature, return_type) = get_function_signature(&parent, content)?;
            format!(
                "### Function\n\n`fn {}`\n\n**Returns:** `{}`\n\n{}",
                signature,
                return_type,
                get_node_documentation(&parent, content)?
            )
        }

        // Method declaration
        ("method_declaration", _) => {
            let (signature, return_type) = get_function_signature(&parent, content)?;
            format!(
                "### Method\n\n`fn {}`\n\n**Returns:** `{}`\n\n{}",
                signature,
                return_type,
                get_node_documentation(&parent, content)?
            )
        }

        // Struct declaration
        ("struct_declaration", _) => {
            let fields = get_struct_fields(&parent, content)?;
            format!(
                "### Struct `{}`\n\n{}\n\n{}",
                identifier_name,
                fields,
                get_node_documentation(&parent, content)?
            )
        }

        // Enum declaration
        ("enum_declaration", _) => {
            let variants = get_enum_variants(&parent, content)?;
            format!(
                "### Enum `{}`\n\n{}\n\n{}",
                identifier_name,
                variants,
                get_node_documentation(&parent, content)?
            )
        }

        // Interface declaration
        ("interface_declaration", _) => {
            let methods = get_interface_methods(&parent, content)?;
            format!(
                "### Interface `{}`\n\n{}\n\n{}",
                identifier_name,
                methods,
                get_node_documentation(&parent, content)?
            )
        }

        // Reference to a symbol (look up definition)
        _ => {
            if let Some(definition_node) = find_symbol_definition(tree.root_node(), identifier_name, content)? {
                generate_identifier_hover(&definition_node, tree, content)?
            } else {
                return Ok(None);
            }
        }
    };

    Ok(Some(HoverContents::MarkupContent(zed::lsp::MarkupContent {
        kind: zed::lsp::MarkupKind::Markdown,
        value: hover_content,
    })))
}

/// Generate hover content for a function call
fn generate_function_call_hover(
    node: &Node,
    tree: &Tree,
    content: &str,
) -> Result<Option<HoverContents>> {
    let name_node = node.child_by_field_name("name").unwrap_or_else(Node::new_null);
    if name_node.is_null() {
        return Ok(None);
    }

    let function_name = name_node.text(content)?;
    if let Some(definition_node) = find_symbol_definition(tree.root_node(), &function_name, content)? {
        generate_identifier_hover(&definition_node, tree, content)
    } else {
        Ok(Some(HoverContents::MarkupContent(zed::lsp::MarkupContent {
            kind: zed::lsp::MarkupKind::Markdown,
            value: format!("### Function Call\n\n`{}`\n\nNo documentation found.", function_name),
        })))
    }
}

/// Generate hover content for a method call
fn generate_method_call_hover(
    node: &Node,
    tree: &Tree,
    content: &str,
) -> Result<Option<HoverContents>> {
    let name_node = node.child_by_field_name("name").unwrap_or_else(Node::new_null);
    if name_node.is_null() {
        return Ok(None);
    }

    let method_name = name_node.text(content)?;
    if let Some(definition_node) = find_symbol_definition(tree.root_node(), &method_name, content)? {
        generate_identifier_hover(&definition_node, tree, content)
    } else {
        Ok(Some(HoverContents::MarkupContent(zed::lsp::MarkupContent {
            kind: zed::lsp::MarkupKind::Markdown,
            value: format!("### Method Call\n\n`{}`\n\nNo documentation found.", method_name),
        })))
    }
}

/// Generate hover content for a type identifier
fn generate_type_hover(
    node: &Node,
    tree: &Tree,
    content: &str,
) -> Result<Option<HoverContents>> {
    let type_name = node.text(content)?;
    if let Some(definition_node) = find_symbol_definition(tree.root_node(), &type_name, content)? {
        generate_identifier_hover(&definition_node, tree, content)
    } else {
        // Check if it's a primitive type
        let primitive_types = [
            "bool", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64",
            "f32", "f64", "char", "string", "Any", "Void"
        ];

        if primitive_types.contains(&type_name) {
            Ok(Some(HoverContents::MarkupContent(zed::lsp::MarkupContent {
                kind: zed::lsp::MarkupKind::Markdown,
                value: format!("### Primitive Type\n\n`{}`\n\nBuilt-in primitive type.", type_name),
            })))
        } else {
            Ok(None)
        }
    }
}

/// Get the type of a variable/constant/parameter/field
fn get_variable_type(node: &Node, content: &str) -> Result<String> {
    let type_node = node.child_by_field_name("type").unwrap_or_else(Node::new_null);
    if type_node.is_null() {
        Ok("inferred".to_string())
    } else {
        Ok(type_node.text(content)?)
    }
}

/// Get the signature of a function/method
fn get_function_signature(node: &Node, content: &str) -> Result<(String, String)> {
    let name_node = node.child_by_field_name("name").unwrap_or_else(Node::new_null);
    let name = name_node.text(content)?;

    let params_node = node.child_by_field_name("parameters").unwrap_or_else(Node::new_null);
    let params = if params_node.is_null() {
        "()".to_string()
    } else {
        let param_nodes = params_node.children().filter(|child| child.kind() == "parameter_declaration");
        let mut params = Vec::new();
        for param in param_nodes {
            let param_name = param.child_by_field_name("name").unwrap().text(content)?;
            let param_type = get_variable_type(&param, content)?;
            params.push(format!("{}: {}", param_name, param_type));
        }
        format!("({})", params.join(", "))
    };

    let return_type_node = node.child_by_field_name("return_type").unwrap_or_else(Node::new_null);
    let return_type = if return_type_node.is_null() {
        "Void".to_string()
    } else {
        return_type_node.text(content)?
    };

    Ok((format!("{} {}", name, params), return_type))
}

/// Get the fields of a struct
fn get_struct_fields(node: &Node, content: &str) -> Result<String> {
    let fields_node = node.child_by_field_name("fields").unwrap_or_else(Node::new_null);
    if fields_node.is_null() {
        return Ok("No fields".to_string());
    }

    let field_nodes = fields_node.children().filter(|child| child.kind() == "field_declaration");
    let mut fields = Vec::new();
    for field in field_nodes {
        let field_name = field.child_by_field_name("name").unwrap().text(content)?;
        let field_type = get_variable_type(&field, content)?;
        fields.push(format!("- `{}: {}`", field_name, field_type));
    }

    Ok(if fields.is_empty() {
        "No fields".to_string()
    } else {
        fields.join("\n")
    })
}

/// Get the variants of an enum
fn get_enum_variants(node: &Node, content: &str) -> Result<String> {
    let variants_node = node.child_by_field_name("variants").unwrap_or_else(Node::new_null);
    if variants_node.is_null() {
        return Ok("No variants".to_string());
    }

    let variant_nodes = variants_node.children().filter(|child| child.kind() == "enum_variant");
    let mut variants = Vec::new();
    for variant in variant_nodes {
        let variant_name = variant.child_by_field_name("name").unwrap().text(content)?;
        let variant_type_node = variant.child_by_field_name("type").unwrap_or_else(Node::new_null);
        if variant_type_node.is_null() {
            variants.push(format!("- `{}`", variant_name));
        } else {
            let variant_type = variant_type_node.text(content)?;
            variants.push(format!("- `{}({})`", variant_name, variant_type));
        }
    }

    Ok(if variants.is_empty() {
        "No variants".to_string()
    } else {
        variants.join("\n")
    })
}

/// Get the methods of an interface
fn get_interface_methods(node: &Node, content: &str) -> Result<String> {
    let methods_node = node.child_by_field_name("methods").unwrap_or_else(Node::new_null);
    if methods_node.is_null() {
        return Ok("No methods".to_string());
    }

    let method_nodes = methods_node.children().filter(|child| child.kind() == "method_declaration");
    let mut methods = Vec::new();
    for method in method_nodes {
        let (signature, return_type) = get_function_signature(&method, content)?;
        methods.push(format!("- `fn {} -> {}`", signature, return_type));
    }

    Ok(if methods.is_empty() {
        "No methods".to_string()
    } else {
        methods.join("\n")
    })
}

/// Get documentation comments for a node
fn get_node_documentation(node: &Node, content: &str) -> Result<String> {
    // Check for leading doc comment
    let prev_sibling = node.prev_sibling().unwrap_or_else(Node::new_null);
    if prev_sibling.kind() == "doc_comment" {
        let doc_text = prev_sibling.text(content)?;
        // Clean up doc comment syntax (remove /**, */, and leading *)
        let cleaned = doc_text
            .replace("/**", "")
            .replace("*/", "")
            .lines()
            .map(|line| line.trim_start_matches('*').trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        return Ok(cleaned);
    }

    // Check for leading regular comment
    if prev_sibling.kind() == "comment" {
        let comment_text = prev_sibling.text(content)?;
        let cleaned = comment_text.replace("//", "").trim().to_string();
        return Ok(cleaned);
    }

    Ok("No documentation available.".to_string())
}

/// Find the definition of a symbol in the parse tree
fn find_symbol_definition(
    root: Node,
    symbol_name: &str,
    content: &str,
) -> Result<Option<Node>> {
    let mut cursor = tree_sitter::TreeCursor::new(root);
    let mut definition_node = None;

    // Recursively search for symbol definitions
    search_definitions(&mut cursor, symbol_name, content, &mut definition_node)?;

    Ok(definition_node)
}

/// Recursive helper to search for symbol definitions
fn search_definitions(
    cursor: &mut tree_sitter::TreeCursor,
    symbol_name: &str,
    content: &str,
    definition_node: &mut Option<Node>,
) -> Result<()> {
    let node = cursor.node();

    // Check if current node is a definition of the symbol
    match node.kind() {
        "variable_declaration" | "constant_declaration" | "function_declaration" |
        "method_declaration" | "struct_declaration" | "enum_declaration" |
        "interface_declaration" | "field_declaration" | "parameter_declaration" => {
            let name_node = node.child_by_field_name("name").unwrap_or_else(Node::new_null);
            if !name_node.is_null() {
                let name = name_node.text(content)?;
                if name == symbol_name {
                    *definition_node = Some(name_node);
                    return Ok(());
                }
            }
        }
        _ => {}
    }

    // Recurse into children
    if cursor.goto_first_child() {
        search_definitions(cursor, symbol_name, content, definition_node)?;
        if definition_node.is_some() {
            return Ok(());
        }
        cursor.goto_parent()?;
    }

    // Move to next sibling
    while cursor.goto_next_sibling() {
        search_definitions(cursor, symbol_name, content, definition_node)?;
        if definition_node.is_some() {
            return Ok(());
        }
    }

    Ok(())
}
```

### 44. src/lsp/definition.rs（补充完整）
```rust
//! Go to Definition functionality for Cangjie LSP server
//!
//! Finds the definition of a symbol and returns its location.

use super::super::{
    syntax::tree_sitter_utils::{self, NodeText},
    config::CangjieConfig,
};
use zed_extension_api::{self as zed, Result, lsp::{Definition, Location, Range}};
use tree_sitter::{Node, Tree};
use std::path::PathBuf;

/// Get the definition location of a symbol
pub fn get_definition(
    document: &zed::Document,
    tree: &Tree,
    position: &zed::lsp::Position,
    _config: &CangjieConfig,
) -> Result<Option<Definition>> {
    let content = document.text();
    let node = tree_sitter_utils::node_at_position(tree.root_node(), position)?;
    if node.is_null() {
        return Ok(None);
    }

    // Find the identifier node (handles nested nodes)
    let identifier_node = find_identifier_node(&node);
    if identifier_node.is_null() {
        return Ok(None);
    }

    let symbol_name = identifier_node.text(content)?;
    let definition_node = find_symbol_definition(tree.root_node(), &symbol_name, content)?;

    definition_node.map(|def_node| {
        let range = tree_sitter_utils::node_to_range(&def_node);
        let uri = document.uri().clone();

        Ok(Some(Definition::Scalar(Location {
            uri,
            range,
        })))
    })?
}

/// Find the identifier node in the hierarchy (handles cases where the hovered node is a child of the identifier)
fn find_identifier_node(node: &Node) -> Node {
    let mut current = *node;
    while !current.is_null() {
        if current.kind() == "identifier" {
            return current;
        }
        current = current.parent().unwrap_or_else(Node::new_null);
    }
    Node::new_null()
}

/// Find the definition node of a symbol in the parse tree
fn find_symbol_definition(
    root: Node,
    symbol_name: &str,
    content: &str,
) -> Result<Option<Node>> {
    let mut cursor = tree_sitter::TreeCursor::new(root);
    let mut definition_node = None;

    search_definitions(&mut cursor, symbol_name, content, &mut definition_node)?;

    Ok(definition_node)
}

/// Recursive helper to search for symbol definitions
fn search_definitions(
    cursor: &mut tree_sitter::TreeCursor,
    symbol_name: &str,
    content: &str,
    definition_node: &mut Option<Node>,
) -> Result<()> {
    let node = cursor.node();

    // Check if current node is a definition of the symbol
    if is_definition_node(&node) {
        let name_node = node.child_by_field_name("name").unwrap_or_else(Node::new_null);
        if !name_node.is_null() {
            let name = name_node.text(content)?;
            if name == symbol_name {
                *definition_node = Some(name_node);
                return Ok(());
            }
        }
    }

    // Recurse into children
    if cursor.goto_first_child() {
        search_definitions(cursor, symbol_name, content, definition_node)?;
        if definition_node.is_some() {
            return Ok(());
        }
        cursor.goto_parent()?;
    }

    // Move to next sibling
    while cursor.goto_next_sibling() {
        search_definitions(cursor, symbol_name, content, definition_node)?;
        if definition_node.is_some() {
            return Ok(());
        }
    }

    Ok(())
}

/// Check if a node is a definition node (declares a symbol)
fn is_definition_node(node: &Node) -> bool {
    matches!(
        node.kind(),
        "variable_declaration" | "constant_declaration" | "function_declaration" |
        "method_declaration" | "struct_declaration" | "enum_declaration" |
        "interface_declaration" | "field_declaration" | "parameter_declaration" |
        "type_alias_declaration" | "module_declaration"
    )
}

/// Get workspace-wide symbol definitions (for multi-file support)
pub fn find_workspace_definition(
    workspace: &zed::Workspace,
    symbol_name: &str,
) -> Result<Option<Vec<Location>>> {
    // Find all Cangjie files in the workspace
    let cangjie_files = super::super::utils::find_workspace_files(workspace, "*.{cang,cj}")?;
    let mut locations = Vec::new();

    for file_path in cangjie_files {
        // Read file content
        let content = super::super::utils::read_file_to_string(&file_path)?;

        // Parse file with Tree-sitter
        let tree = tree_sitter_cangjie::parse(&content, None)?;

        // Search for symbol definition in this file
        let definition_node = find_symbol_definition(tree.root_node(), symbol_name, &content)?;
        if let Some(def_node) = definition_node {
            let range = tree_sitter_utils::node_to_range(&def_node);
            let uri = zed::lsp::Url::from_file_path(file_path).map_err(|_| {
                zed::Error::internal(format!("Invalid file path: {:?}", file_path))
            })?;

            locations.push(Location {
                uri,
                range,
            });
        }
    }

    if locations.is_empty() {
        Ok(None)
    } else {
        Ok(Some(locations))
    }
}
```

### 45. src/lsp/completion.rs（补充完整）
```rust
//! Code completion functionality for Cangjie LSP server
//!
//! Provides context-aware code completion for Cangjie code, including:
//! - Symbols from the current document
//! - Workspace-wide symbols
//! - Built-in types and functions
//! - Code snippets

use super::super::{
    syntax::tree_sitter_utils::{self, NodeText},
    config::CangjieConfig,
    utils::{string::is_snake_case, log::debug},
    lint::rules::BuiltInRules,
};
use zed_extension_api::{self as zed, Result, lsp::{
    CompletionItem, CompletionItemKind, CompletionList, CompletionParams,
    InsertTextFormat, MarkupContent, MarkupKind, Range, TextEdit
}};
use tree_sitter::{Node, Tree};
use std::collections::{HashMap, HashSet};

/// Built-in types for Cangjie
const BUILTIN_TYPES: &[(&str, &str)] = &[
    ("bool", "Boolean type (true/false)"),
    ("i8", "8-bit signed integer"),
    ("i16", "16-bit signed integer"),
    ("i32", "32-bit signed integer"),
    ("i64", "64-bit signed integer"),
    ("u8", "8-bit unsigned integer"),
    ("u16", "16-bit unsigned integer"),
    ("u32", "32-bit unsigned integer"),
    ("u64", "64-bit unsigned integer"),
    ("f32", "32-bit floating-point number"),
    ("f64", "64-bit floating-point number"),
    ("char", "Unicode character"),
    ("string", "UTF-8 encoded string"),
    ("Any", "Dynamic type (sacrifices type safety)"),
    ("Void", "No return type"),
    ("Vec<T>", "Dynamic array (generic)"),
    ("HashMap<K, V>", "Hash table (generic)"),
    ("Option<T>", "Optional value (Some/None)"),
    ("Result<T, E>", "Result type (Ok/Err)"),
];

/// Built-in functions for Cangjie
const BUILTIN_FUNCTIONS: &[(&str, &str, &str)] = &[
    (
        "println",
        "println(format: string, ...args: Any) -> Void",
        "Print a formatted string to the console with a newline"
    ),
    (
        "print",
        "print(format: string, ...args: Any) -> Void",
        "Print a formatted string to the console without a newline"
    ),
    (
        "panic",
        "panic(message: string) -> !",
        "Terminate the program with an error message"
    ),
    (
        "exit",
        "exit(code: i32) -> !",
        "Exit the program with a status code"
    ),
    (
        "len",
        "len(value: &str | &Vec<T> | &[T]) -> usize",
        "Get the length of a string or collection"
    ),
    (
        "is_null",
        "is_null(value: &Any) -> bool",
        "Check if a value is null"
    ),
    (
        "to_string",
        "to_string(value: &Any) -> string",
        "Convert a value to its string representation"
    ),
    (
        "parse_int",
        "parse_int(s: &str) -> Result<i64, ParseError>",
        "Parse a string into an integer"
    ),
    (
        "parse_float",
        "parse_float(s: &str) -> Result<f64, ParseError>",
        "Parse a string into a floating-point number"
    ),
];

/// Get completion items for a given position
pub fn get_completion(
    document: &zed::Document,
    tree: &Tree,
    params: &CompletionParams,
    config: &CangjieConfig,
) -> Result<Option<CompletionList>> {
    let content = document.text();
    let position = &params.position;

    // Get the context node (the node at the cursor position)
    let context_node = tree_sitter_utils::node_at_position(tree.root_node(), position)?;
    let context = CompletionContext::from_node(&context_node, content)?;

    debug!("Completion context: {:?} at {:?}", context.kind, position);

    // Collect completion items from various sources
    let mut items = Vec::new();

    // 1. Add document symbols
    let document_symbols = collect_document_symbols(tree, content, &context)?;
    items.extend(document_symbols);

    // 2. Add workspace symbols (if enabled)
    if config.completion.include_workspace_symbols {
        let workspace_symbols = collect_workspace_symbols(document.workspace(), &context)?;
        items.extend(workspace_symbols);
    }

    // 3. Add built-in types and functions
    let builtin_items = collect_builtin_items(&context)?;
    items.extend(builtin_items);

    // 4. Add snippets (if enabled)
    if config.completion.include_snippets {
        let snippets = collect_snippets(&context)?;
        items.extend(snippets);
    }

    // 5. Filter and sort items
    let filtered_items = filter_and_sort_items(items, &params.context)?;

    Ok(Some(CompletionList {
        is_incomplete: false,
        items: filtered_items,
    }))
}

/// Completion context (determines what kind of completions to show)
#[derive(Debug, Clone, PartialEq, Eq)]
enum CompletionContextKind {
    /// General context (top-level or inside a block)
    General,
    /// Type context (e.g., variable type annotation, generic argument)
    Type,
    /// Expression context (e.g., inside an expression, function argument)
    Expression,
    /// Function call context (e.g., after a dot in a method call)
    FunctionCall,
    /// Method call context (e.g., after a dot on an object)
    MethodCall,
    /// Import path context (e.g., inside an import statement)
    ImportPath,
    /// Comment context (don't show completions)
    Comment,
}

#[derive(Debug, Clone)]
struct CompletionContext {
    kind: CompletionContextKind,
    parent_node: Node,
    grandparent_node: Node,
    prefix: String,
}

impl CompletionContext {
    /// Create a completion context from a node
    fn from_node(node: &Node, content: &str) -> Result<Self> {
        let mut current = *node;
        let mut parent = node.parent().unwrap_or_else(Node::new_null);
        let mut grandparent = parent.parent().unwrap_or_else(Node::new_null);

        // Check if we're in a comment
        if is_in_comment(&current) {
            return Ok(Self {
                kind: CompletionContextKind::Comment,
                parent_node: parent,
                grandparent_node: grandparent,
                prefix: String::new(),
            });
        }

        // Get the prefix (the text before the cursor)
        let prefix = get_completion_prefix(node, content)?;

        // Determine context kind based on parent nodes
        let kind = match (parent.kind(), grandparent.kind()) {
            // Type context: variable type annotation
            ("type_annotation", "variable_declaration" | "constant_declaration" | "parameter_declaration" | "field_declaration") => {
                CompletionContextKind::Type
            }

            // Type context: generic argument
            ("generic_argument_list", _) => CompletionContextKind::Type,

            // Import path context
            ("string_literal", "import_statement" | "export_statement") => {
                CompletionContextKind::ImportPath
            }

            // Function call context
            ("function_call", _) => CompletionContextKind::FunctionCall,

            // Method call context (e.g., object.method())
            ("method_call", _) => CompletionContextKind::MethodCall,

            // Expression context
            ("expression", _) | ("binary_expression", _) | ("unary_expression", _) | ("call_expression", _) => {
                CompletionContextKind::Expression
            }

            // Default to general context
            _ => CompletionContextKind::General,
        };

        Ok(Self {
            kind,
            parent_node: parent,
            grandparent_node: grandparent,
            prefix,
        })
    }
}

/// Check if a node is inside a comment
fn is_in_comment(node: &Node) -> bool {
    let mut current = *node;
    while !current.is_null() {
        if current.kind() == "comment" || current.kind() == "doc_comment" {
            return true;
        }
        current = current.parent().unwrap_or_else(Node::new_null);
    }
    false
}

/// Get the completion prefix (text before the cursor)
fn get_completion_prefix(node: &Node, content: &str) -> Result<String> {
    if node.kind() == "identifier" {
        // If we're inside an identifier, get the text up to the cursor
        let start = node.start_position();
        let end = node.end_position();
        let cursor_position = node.range().end_point;

        let text = node.text(content)?;
        let prefix_length = (cursor_position.column - start.column) as usize;
        Ok(text[..prefix_length].to_string())
    } else {
        // Otherwise, check if there's an identifier before the cursor
        let mut cursor = tree_sitter::TreeCursor::new(node);
        let mut prefix = String::new();

        // Search for the previous identifier
        if cursor.goto_prev_sibling() {
            let prev_node = cursor.node();
            if prev_node.kind() == "identifier" {
                prefix = prev_node.text(content)?;
            }
            cursor.goto_next_sibling()?;
        }

        Ok(prefix)
    }
}

/// Collect completion items from the current document
fn collect_document_symbols(
    tree: &Tree,
    content: &str,
    context: &CompletionContext,
) -> Result<Vec<CompletionItem>> {
    let mut symbols = HashMap::new();
    let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());

    // Recursively collect symbols from the parse tree
    collect_symbols_recursive(&mut cursor, content, &mut symbols)?;

    // Convert symbols to completion items
    let mut items = Vec::new();
    for (name, symbol) in symbols {
        // Filter symbols based on context
        if !is_symbol_relevant(&symbol, context) {
            continue;
        }

        let item = create_completion_item(&name, &symbol)?;
        items.push(item);
    }

    Ok(items)
}

/// Symbol information for completion
#[derive(Debug, Clone)]
struct SymbolInfo {
    kind: CompletionItemKind,
    detail: String,
    documentation: Option<String>,
    insert_text: Option<String>,
}

/// Recursively collect symbols from the parse tree
fn collect_symbols_recursive(
    cursor: &mut tree_sitter::TreeCursor,
    content: &str,
    symbols: &mut HashMap<String, SymbolInfo>,
) -> Result<()> {
    let node = cursor.node();

    // Collect symbol based on node type
    match node.kind() {
        // Variables
        "variable_declaration" => {
            let name = get_node_name(&node, content)?;
            let var_type = get_variable_type(&node, content)?;
            let detail = format!("let {}: {}", name, var_type);
            let documentation = get_node_documentation(&node, content)?;

            symbols.insert(name.clone(), SymbolInfo {
                kind: CompletionItemKind::Variable,
                detail,
                documentation,
                insert_text: Some(name),
            });
        }

        // Constants
        "constant_declaration" => {
            let name = get_node_name(&node, content)?;
            let const_type = get_variable_type(&node, content)?;
            let detail = format!("const {}: {}", name, const_type);
            let documentation = get_node_documentation(&node, content)?;

            symbols.insert(name.clone(), SymbolInfo {
                kind: CompletionItemKind::Constant,
                detail,
                documentation,
                insert_text: Some(name),
            });
        }

        // Functions
        "function_declaration" => {
            let name = get_node_name(&node, content)?;
            let (signature, return_type) = get_function_signature(&node, content)?;
            let detail = format!("fn {} -> {}", signature, return_type);
            let documentation = get_node_documentation(&node, content)?;
            let insert_text = format!("{}()", name);

            symbols.insert(name.clone(), SymbolInfo {
                kind: CompletionItemKind::Function,
                detail,
                documentation,
                insert_text: Some(insert_text),
            });
        }

        // Methods
        "method_declaration" => {
            let name = get_node_name(&node, content)?;
            let (signature, return_type) = get_function_signature(&node, content)?;
            let detail = format!("fn {} -> {}", signature, return_type);
            let documentation = get_node_documentation(&node, content)?;
            let insert_text = format!("{}()", name);

            symbols.insert(name.clone(), SymbolInfo {
                kind: CompletionItemKind::Method,
                detail,
                documentation,
                insert_text: Some(insert_text),
            });
        }

        // Structs
        "struct_declaration" => {
            let name = get_node_name(&node, content)?;
            let detail = format!("struct {}", name);
            let documentation = get_node_documentation(&node, content)?;

            symbols.insert(name.clone(), SymbolInfo {
                kind: CompletionItemKind::Struct,
                detail,
                documentation,
                insert_text: Some(name),
            });
        }

        // Enums
        "enum_declaration" => {
            let name = get_node_name(&node, content)?;
            let detail = format!("enum {}", name);
            let documentation = get_node_documentation(&node, content)?;

            symbols.insert(name.clone(), SymbolInfo {
                kind: CompletionItemKind::Enum,
                detail,
                documentation,
                insert_text: Some(name),
            });
        }

        // Interfaces
        "interface_declaration" => {
            let name = get_node_name(&node, content)?;
            let detail = format!("interface {}", name);
            let documentation = get_node_documentation(&node, content)?;

            symbols.insert(name.clone(), SymbolInfo {
                kind: CompletionItemKind::Interface,
                detail,
                documentation,
                insert_text: Some(name),
            });
        }

        // Fields
        "field_declaration" => {
            let name = get_node_name(&node, content)?;
            let field_type = get_variable_type(&node, content)?;
            let detail = format!("{}: {}", name, field_type);
            let documentation = get_node_documentation(&node, content)?;

            symbols.insert(name.clone(), SymbolInfo {
                kind: CompletionItemKind::Field,
                detail,
                documentation,
                insert_text: Some(name),
            });
        }

        // Type aliases
        "type_alias_declaration" => {
            let name = get_node_name(&node, content)?;
            let alias_type = get_variable_type(&node, content)?;
            let detail = format!("type {} = {}", name, alias_type);
            let documentation = get_node_documentation(&node, content)?;

            symbols.insert(name.clone(), SymbolInfo {
                kind: CompletionItemKind::TypeParameter,
                detail,
                documentation,
                insert_text: Some(name),
            });
        }

        _ => {}
    }

    // Recurse into children
    if cursor.goto_first_child() {
        collect_symbols_recursive(cursor, content, symbols)?;
        cursor.goto_parent()?;
    }

    // Move to next sibling
    while cursor.goto_next_sibling() {
        collect_symbols_recursive(cursor, content, symbols)?;
    }

    Ok(())
}

/// Collect completion items from the workspace
fn collect_workspace_symbols(
    workspace: &zed::Workspace,
    context: &CompletionContext,
) -> Result<Vec<CompletionItem>> {
    let mut items = Vec::new();
    let cangjie_files = super::super::utils::find_workspace_files(workspace, "*.{cang,cj}")?;

    for file_path in cangjie_files {
        let content = super::super::utils::read_file_to_string(&file_path)?;
        let tree = tree_sitter_cangjie::parse(&content, None)?;

        let mut symbols = HashMap::new();
        let mut cursor = tree_sitter::TreeCursor::new(tree.root_node());
        collect_symbols_recursive(&mut cursor, &content, &mut symbols)?;

        for (name, symbol) in symbols {
            if !is_symbol_relevant(&symbol, context) {
                continue;
            }

            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let detail = format!("{} ({}", symbol.detail, file_name);

            let item = CompletionItem {
                label: name.clone(),
                kind: Some(symbol.kind),
                detail: Some(detail),
                documentation: symbol.documentation.map(|doc| {
                    zed::lsp::Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: doc,
                    })
                }),
                insert_text: symbol.insert_text,
                insert_text_format: Some(InsertTextFormat::PlainText),
                sort_text: None,
                filter_text: None,
                text_edit: None,
                additional_text_edits: None,
                command: None,
                data: None,
            };

            items.push(item);
        }
    }

    Ok(items)
}

/// Collect built-in types and functions
fn collect_builtin_items(context: &CompletionContext) -> Result<Vec<CompletionItem>> {
    let mut items = Vec::new();

    // Add built-in types (relevant in type contexts)
    if context.kind == CompletionContextKind::Type {
        for (name, desc) in BUILTIN_TYPES {
            let item = CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::TypeParameter),
                detail: Some("built-in type".to_string()),
                documentation: Some(zed::lsp::Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: desc.to_string(),
                })),
                insert_text: Some(name.to_string()),
                insert_text_format: Some(InsertTextFormat::PlainText),
                sort_text: None,
                filter_text: None,
                text_edit: None,
                additional_text_edits: None,
                command: None,
                data: None,
            };
            items.push(item);
        }
    }

    // Add built-in functions (relevant in expression/function call contexts)
    if context.kind == CompletionContextKind::Expression
        || context.kind == CompletionContextKind::FunctionCall
    {
        for (name, signature, desc) in BUILTIN_FUNCTIONS {
            let item = CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::Function),
                detail: Some(signature.to_string()),
                documentation: Some(zed::lsp::Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: desc.to_string(),
                })),
                insert_text: Some(format!("{}()", name)),
                insert_text_format: Some(InsertTextFormat::PlainText),
                sort_text: None,
                filter_text: None,
                text_edit: None,
                additional_text_edits: None,
                command: None,
                data: None,
            };
            items.push(item);
        }
    }

    Ok(items)
}

/// Collect code snippets
fn collect_snippets(context: &CompletionContext) -> Result<Vec<CompletionItem>> {
    let mut items = Vec::new();

    // Snippets relevant in general context
    if context.kind == CompletionContextKind::General {
        items.extend(vec![
            create_snippet_item(
                "fn",
                "Function Declaration",
                CompletionItemKind::Snippet,
                vec![
                    "fn ${1:function_name}(${2:parameters}) -> ${3:ReturnType} {",
                    "  ${4:// body}",
                    "  return ${5:value};",
                    "}"
                ].join("\n")
            ),
            create_snippet_item(
                "struct",
                "Struct Declaration",
                CompletionItemKind::Snippet,
                vec![
                    "struct ${1:StructName} {",
                    "  ${2:field_name}: ${3:Type},",
                    "}"
                ].join("\n")
            ),
            create_snippet_item(
                "enum",
                "Enum Declaration",
                CompletionItemKind::Snippet,
                vec![
                    "enum ${1:EnumName} {",
                    "  ${2:Variant1},",
                    "  ${3:Variant2}(${4:Type}),",
                    "}"
                ].join("\n")
            ),
            create_snippet_item(
                "interface",
                "Interface Declaration",
                CompletionItemKind::Snippet,
                vec![
                    "interface ${1:InterfaceName} {",
                    "  ${2:method_name}(${3:parameters}) -> ${4:ReturnType};",
                    "}"
                ].join("\n")
            ),
            create_snippet_item(
                "import",
                "Import Statement",
                CompletionItemKind::Snippet,
                "import ${1:module_path}::${2:symbol};"
            ),
        ]);
    }

    // Snippets relevant in statement context
    if context.kind == CompletionContextKind::General
        || context.kind == CompletionContextKind::Expression
    {
        items.extend(vec![
            create_snippet_item(
                "let",
                "Mutable Variable",
                CompletionItemKind::Snippet,
                "let ${1:variable_name}: ${2:Type} = ${3:initial_value};"
            ),
            create_snippet_item(
                "const",
                "Constant",
                CompletionItemKind::Snippet,
                "const ${1:CONSTANT_NAME}: ${2:Type} = ${3:value};"
            ),
            create_snippet_item(
                "if",
                "If Statement",
                CompletionItemKind::Snippet,
                vec![
                    "if ${1:condition} {",
                    "  ${2:// body}",
                    "}"
                ].join("\n")
            ),
            create_snippet_item(
                "ife",
                "If-Else Statement",
                CompletionItemKind::Snippet,
                vec![
                    "if ${1:condition} {",
                    "  ${2:// if body}",
                    "} else {",
                    "  ${3:// else body}",
                    "}"
                ].join("\n")
            ),
            create_snippet_item(
                "for",
                "For Loop",
                CompletionItemKind::Snippet,
                vec![
                    "for ${1:item} in ${2:iterable} {",
                    "  ${3:// body}",
                    "}"
                ].join("\n")
            ),
            create_snippet_item(
                "while",
                "While Loop",
                CompletionItemKind::Snippet,
                vec![
                    "while ${1:condition} {",
                    "  ${2:// body}",
                    "}"
                ].join("\n")
            ),
            create_snippet_item(
                "println",
                "Print Line",
                CompletionItemKind::Snippet,
                "println(\"${1:message}: {}\", ${2:value});"
            ),
            create_snippet_item(
                "try",
                "Try-Catch Block",
                CompletionItemKind::Snippet,
                vec![
                    "try {",
                    "  ${1:// risky operation}",
                    "} catch ${2:error_name}: ${3:ErrorType} {",
                    "  ${4:// error handling}",
                    "}"
                ].join("\n")
            ),
        ]);
    }

    Ok(items)
}

/// Create a snippet completion item
fn create_snippet_item(
    label: &str,
    detail: &str,
    kind: CompletionItemKind,
    insert_text: String,
) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(kind),
        detail: Some(detail.to_string()),
        documentation: None,
        insert_text: Some(insert_text),
        insert_text_format: Some(InsertTextFormat::Snippet),
        sort_text: None,
        filter_text: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        data: None,
    }
}

/// Check if a symbol is relevant to the current completion context
fn is_symbol_relevant(symbol: &SymbolInfo, context: &CompletionContext) -> bool {
    match context.kind {
        CompletionContextKind::Type => {
            // Show type-related symbols in type context
            matches!(
                symbol.kind,
                CompletionItemKind::Struct | CompletionItemKind::Enum |
                CompletionItemKind::Interface | CompletionItemKind::TypeParameter
            )
        }
        CompletionContextKind::FunctionCall => {
            // Show functions and methods in function call context
            matches!(
                symbol.kind,
                CompletionItemKind::Function | CompletionItemKind::Method
            )
        }
        CompletionContextKind::MethodCall => {
            // Show methods in method call context
            symbol.kind == CompletionItemKind::Method
        }
        CompletionContextKind::ImportPath => {
            // Don't show symbols in import path context (handled separately)
            false
        }
        CompletionContextKind::Comment => {
            // Don't show symbols in comments
            false
        }
        // Show all symbols in general and expression contexts
        CompletionContextKind::General | CompletionContextKind::Expression => true,
    }
}

/// Filter and sort completion items
fn filter_and_sort_items(
    items: Vec<CompletionItem>,
    context: &Option<zed::lsp::CompletionContext>,
) -> Result<Vec<CompletionItem>> {
    let mut filtered = items;

    // Filter by trigger characters (if provided)
    if let Some(context) = context {
        if let Some(trigger_char) = &context.trigger_character {
            filtered.retain(|item| {
                item.label.starts_with(trigger_char)
                    || item.insert_text.as_ref().map(|t| t.starts_with(trigger_char)).unwrap_or(false)
            });
        }
    }

    // Remove duplicates (keep the first occurrence)
    let mut seen = HashSet::new();
    filtered.retain(|item| {
        if seen.contains(&item.label) {
            false
        } else {
            seen.insert(item.label.clone());
            true
        }
    });

    // Sort items by relevance (kind priority + alphabetical)
    filtered.sort_by(|a, b| {
        // Priority order for kinds
        let kind_priority = |kind: &Option<CompletionItemKind>| -> u8 {
            match kind {
                Some(CompletionItemKind::Snippet) => 0,
                Some(CompletionItemKind::Function) => 1,
                Some(CompletionItemKind::Method) => 2,
                Some(CompletionItemKind::Variable) => 3,
                Some(CompletionItemKind::Constant) => 4,
                Some(CompletionItemKind::Struct) => 5,
                Some(CompletionItemKind::Enum) => 6,
                Some(CompletionItemKind::Interface) => 7,
                Some(CompletionItemKind::Field) => 8,
                Some(CompletionItemKind::TypeParameter) => 9,
                _ => 10,
            }
        };

        let a_priority = kind_priority(&a.kind);
        let b_priority = kind_priority(&b.kind);

        if a_priority != b_priority {
            a_priority.cmp(&b_priority)
        } else {
            a.label.cmp(&b.label)
        }
    });

    Ok(filtered)
}

/// Create a completion item from a symbol
fn create_completion_item(name: &str, symbol: &SymbolInfo) -> Result<CompletionItem> {
    Ok(CompletionItem {
        label: name.to_string(),
        kind: Some(symbol.kind),
        detail: Some(symbol.detail.clone()),
        documentation: symbol.documentation.clone().map(|doc| {
            zed::lsp::Documentation::Markup
              ### 45. src/lsp/completion.rs（续）
              ```rust
                              MarkupContent {
                                  kind: MarkupKind::Markdown,
                                  value: doc,
                              }
                          }),
                          insert_text: symbol.insert_text.clone(),
                          insert_text_format: Some(InsertTextFormat::PlainText),
                          sort_text: None,
                          filter_text: None,
                          text_edit: None,
                          additional_text_edits: None,
                          command: None,
                          data: None,
                      })
                  }

                  /// Get the name of a node (from its "name" field)
                  fn get_node_name(node: &Node, content: &str) -> Result<String> {
                      let name_node = node.child_by_field_name("name").ok_or_else(|| {
                          zed::Error::internal(format!("Node {} has no 'name' field", node.kind()))
                      })?;
                      name_node.text(content)
                  }

                  /// Get the type of a variable/constant/parameter/field
                  fn get_variable_type(node: &Node, content: &str) -> Result<String> {
                      let type_node = node.child_by_field_name("type").unwrap_or_else(Node::new_null);
                      if type_node.is_null() {
                          Ok("inferred".to_string())
                      } else {
                          type_node.text(content)
                      }
                  }

                  /// Get the signature of a function/method
                  fn get_function_signature(node: &Node, content: &str) -> Result<(String, String)> {
                      let name = get_node_name(node, content)?;
                      let params_node = node.child_by_field_name("parameters").unwrap_or_else(Node::new_null);

                      let params = if params_node.is_null() {
                          "()".to_string()
                      } else {
                          let param_nodes = params_node.children().filter(|child| child.kind() == "parameter_declaration");
                          let mut params = Vec::new();
                          for param in param_nodes {
                              let param_name = get_node_name(&param, content)?;
                              let param_type = get_variable_type(&param, content)?;
                              params.push(format!("{}: {}", param_name, param_type));
                          }
                          format!("({})", params.join(", "))
                      };

                      let return_type_node = node.child_by_field_name("return_type").unwrap_or_else(Node::new_null);
                      let return_type = if return_type_node.is_null() {
                          "Void".to_string()
                      } else {
                          return_type_node.text(content)?
                      };

                      Ok((format!("{} {}", name, params), return_type))
                  }

                  /// Get documentation comments for a node
                  fn get_node_documentation(node: &Node, content: &str) -> Result<Option<String>> {
                      let prev_sibling = node.prev_sibling().unwrap_or_else(Node::new_null);
                      if prev_sibling.kind() == "doc_comment" {
                          let doc_text = prev_sibling.text(content)?;
                          let cleaned = doc_text
                              .replace("/**", "")
                              .replace("*/", "")
                              .lines()
                              .map(|line| line.trim_start_matches('*').trim())
                              .filter(|line| !line.is_empty())
                              .collect::<Vec<_>>()
                              .join("\n");
                          Ok(Some(cleaned))
                      } else if prev_sibling.kind() == "comment" {
                          let comment_text = prev_sibling.text(content)?;
                          let cleaned = comment_text.replace("//", "").trim().to_string();
                          Ok(Some(cleaned))
                      } else {
                          Ok(None)
                      }
                  }
              }

              ### 46. src/lsp/formatting.rs（补充完整）
              ```rust
              //! Code formatting functionality for Cangjie LSP server
              //!
              //! Implements configurable code formatting based on the extension's formatting config.

              use super::super::{
                  syntax::tree_sitter_utils::{self, NodeText},
                  config::FormattingConfig,
                  utils::{log::debug, string::truncate_string},
              };
              use zed_extension_api::{self as zed, Result, lsp::{DocumentFormattingParams, TextEdit, Range, Position}};
              use tree_sitter::{Node, Tree};
              use std::collections::VecDeque;

              /// Default formatting options (used if not specified in config)
              const DEFAULT_FORMAT_CONFIG: FormattingConfig = FormattingConfig {
                  indent_style: "space".to_string(),
                  indent_size: 4,
                  function_brace_style: "same_line".to_string(),
                  struct_brace_style: "same_line".to_string(),
                  trailing_comma: "multiline".to_string(),
                  space_around_operators: true,
                  space_inside_brackets: false,
                  max_line_length: 120,
                  line_ending: "lf".to_string(),
                  auto_fix: true,
              };

              /// Format a Cangjie document
              pub fn format_document(
                  document: &zed::Document,
                  tree: &Tree,
                  params: &DocumentFormattingParams,
                  config: &FormattingConfig,
              ) -> Result<Option<Vec<TextEdit>>> {
                  let content = document.text();
                  let config = merge_format_config(config, &params.options)?;

                  debug!("Formatting document with config: {:?}", config);

                  // Generate formatted text
                  let formatted_text = format_tree(tree.root_node(), content, &config)?;

                  // If the formatted text is the same as the original, return no edits
                  if formatted_text == content {
                      return Ok(None);
                  }

                  // Create a single text edit to replace the entire document
                  let range = Range {
                      start: Position { line: 0, character: 0 },
                      end: Position {
                          line: content.lines().count() as u32,
                          character: 0,
                      },
                  };

                  Ok(Some(vec![TextEdit {
                      range,
                      new_text: formatted_text,
                  }]))
              }

              /// Format a range within a Cangjie document
              pub fn format_range(
                  document: &zed::Document,
                  tree: &Tree,
                  range: &Range,
                  config: &FormattingConfig,
              ) -> Result<Option<Vec<TextEdit>>> {
                  let content = document.text();
                  let start_line = range.start.line as usize;
                  let end_line = range.end.line as usize;

                  // Extract the text to format
                  let lines: Vec<&str> = content.lines().collect();
                  let text_to_format = lines[start_line..=end_line].join("\n");

                  // Parse the text to format (using Tree-sitter)
                  let range_tree = tree_sitter_cangjie::parse(&text_to_format, None)?;

                  // Format the extracted text
                  let formatted_text = format_tree(range_tree.root_node(), &text_to_format, config)?;

                  // If the formatted text is the same as the original, return no edits
                  if formatted_text == text_to_format {
                      return Ok(None);
                  }

                  // Create a text edit for the range
                  let edit_range = Range {
                      start: range.start.clone(),
                      end: Position {
                          line: end_line as u32,
                          character: lines[end_line].len() as u32,
                      },
                  };

                  Ok(Some(vec![TextEdit {
                      range: edit_range,
                      new_text: formatted_text,
                  }]))
              }

              /// Merge user config with LSP formatting options (LSP options take precedence)
              fn merge_format_config(
                  user_config: &FormattingConfig,
                  lsp_options: &Option<zed::lsp::FormattingOptions>,
              ) -> Result<FormattingConfig> {
                  let mut config = user_config.clone();

                  if let Some(options) = lsp_options {
                      // Indent size (from LSP options)
                      if let Some(tab_size) = options.tab_size {
                          config.indent_size = tab_size as usize;
                      }

                      // Indent style (from LSP options: insert_spaces = true → space, false → tab)
                      if let Some(insert_spaces) = options.insert_spaces {
                          config.indent_style = if insert_spaces {
                              "space".to_string()
                          } else {
                              "tab".to_string()
                          };
                      }

                      // Line length (from LSP options: trim_trailing_whitespace is a hint)
                      if let Some(max_line_length) = options.max_line_length {
                          config.max_line_length = max_line_length as usize;
                      }
                  }

                  Ok(config)
              }

              /// Format a Tree-sitter node and its children
              fn format_tree(root: Node, content: &str, config: &FormattingConfig) -> Result<String> {
                  let mut output = String::new();
                  let mut indent_level = 0;
                  let mut queue = VecDeque::new();

                  // Initialize queue with root node and initial context
                  queue.push_back((root, indent_level, false));

                  while let Some((node, indent, is_child)) = queue.pop_front() {
                      // Add indentation if needed
                      if !is_child && !node.is_root() {
                          output.push_str(&get_indent(indent, config));
                      }

                      // Format the current node
                      let node_text = format_node(&node, content, config, indent)?;
                      output.push_str(&node_text);

                      // Process children (reverse order to maintain correct order in queue)
                      let mut children = node.children().collect::<Vec<_>>();
                      children.reverse();

                      for (i, child) in children.iter().enumerate() {
                          let is_last_child = i == 0;
                          let child_indent = get_child_indent(&node, *child, indent, config)?;
                          queue.push_back((*child, child_indent, true));

                          // Add separator between children if needed
                          if !is_last_child && needs_separator(&node, *child, config)? {
                              output.push_str(&get_separator(&node, *child, config)?);
                          }
                      }

                      // Add trailing separator if needed
                      if needs_trailing_separator(&node, config)? {
                          output.push_str(&get_trailing_separator(&node, config)?);
                      }
                  }

                  // Clean up trailing newlines and add final newline
                  let formatted = output
                      .trim_end()
                      .to_string()
                      + &get_line_ending(config);

                  Ok(formatted)
              }

              /// Format a single Tree-sitter node
              fn format_node(node: &Node, content: &str, config: &FormattingConfig, indent: usize) -> Result<String> {
                  match node.kind() {
                      // Keywords
                      "keyword" | "keyword_control" | "keyword_declaration" | "keyword_modifier" |
                      "keyword_operator" | "keyword_return" | "keyword_import" | "keyword_export" => {
                          Ok(format!("{} ", node.text(content)?))
                      }

                      // Identifiers
                      "identifier" | "type_identifier" | "primitive_type" | "generic_type" |
                      "enum_identifier" | "struct_identifier" | "interface_identifier" => {
                          Ok(node.text(content)?.to_string())
                      }

                      // Literals
                      "string_literal" | "number_literal" | "boolean_literal" | "null_literal" => {
                          Ok(node.text(content)?.to_string())
                      }

                      // Comments
                      "comment" => {
                          Ok(format!("// {}\n", node.text(content)?.replace("//", "").trim()))
                      }

                      "doc_comment" => {
                          let doc_text = node.text(content)?;
                          let cleaned = doc_text
                              .replace("/**", "")
                              .replace("*/", "")
                              .lines()
                              .map(|line| line.trim_start_matches('*').trim())
                              .filter(|line| !line.is_empty())
                              .collect::<Vec<_>>()
                              .join("\n * ");
                          Ok(format!("/**\n * {}\n */{}", cleaned, get_line_ending(config)))
                      }

                      // Operators
                      "arithmetic_operator" | "comparison_operator" | "logical_operator" |
                      "assignment_operator" | "bitwise_operator" | "range_operator" => {
                          let op = node.text(content)?;
                          if config.space_around_operators {
                              Ok(format!(" {} ", op))
                          } else {
                              Ok(op.to_string())
                          }
                      }

                      // Punctuation
                      "punctuation" => Ok(node.text(content)?.to_string()),
                      "comma" => Ok(", ".to_string()),
                      "semicolon" => Ok(";".to_string()),
                      "colon" => Ok(": ".to_string()),
                      "dot" => Ok(".".to_string()),

                      // Braces
                      "brace" => {
                          let brace = node.text(content)?;
                          match brace {
                              "{" => Ok(format!("{{{}", get_line_ending(config))),
                              "}" => Ok("}".to_string()),
                              _ => Ok(brace.to_string()),
                          }
                      }

                      // Brackets
                      "bracket" => {
                          let bracket = node.text(content)?;
                          if config.space_inside_brackets {
                              match bracket {
                                  "[" => Ok("[ ".to_string()),
                                  "]" => Ok(" ]".to_string()),
                                  "<" => Ok("< ".to_string()),
                                  ">" => Ok(" >".to_string()),
                                  _ => Ok(bracket.to_string()),
                              }
                          } else {
                              Ok(bracket.to_string())
                          }
                      }

                      // Parentheses
                      "parenthesis" => {
                          let paren = node.text(content)?;
                          Ok(paren.to_string())
                      }

                      // Function declaration
                      "function_declaration" => {
                          let name = node.child_by_field_name("name").unwrap().text(content)?;
                          let params = format_node(
                              &node.child_by_field_name("parameters").unwrap(),
                              content,
                              config,
                              indent,
                          )?;
                          let return_type = node.child_by_field_name("return_type")
                              .map(|rt| format!(" -> {}", rt.text(content).unwrap()))
                              .unwrap_or_default();

                          Ok(format!("fn {} {}{}", name, params, return_type))
                      }

                      // Struct declaration
                      "struct_declaration" => {
                          let name = node.child_by_field_name("name").unwrap().text(content)?;
                          Ok(format!("struct {}", name))
                      }

                      // Enum declaration
                      "enum_declaration" => {
                          let name = node.child_by_field_name("name").unwrap().text(content)?;
                          Ok(format!("enum {}", name))
                      }

                      // Interface declaration
                      "interface_declaration" => {
                          let name = node.child_by_field_name("name").unwrap().text(content)?;
                          Ok(format!("interface {}", name))
                      }

                      // Variable declaration
                      "variable_declaration" => {
                          let name = node.child_by_field_name("name").unwrap().text(content)?;
                          let type_anno = node.child_by_field_name("type")
                              .map(|t| format!(": {}", t.text(content).unwrap()))
                              .unwrap_or_default();
                          let value = node.child_by_field_name("value")
                              .map(|v| format!(" = {}", format_node(v, content, config, indent).unwrap()))
                              .unwrap_or_default();

                          Ok(format!("let {}{}{}", name, type_anno, value))
                      }

                      // Constant declaration
                      "constant_declaration" => {
                          let name = node.child_by_field_name("name").unwrap().text(content)?;
                          let type_anno = node.child_by_field_name("type")
                              .map(|t| format!(": {}", t.text(content).unwrap()))
                              .unwrap_or_default();
                          let value = node.child_by_field_name("value").unwrap();
                          let value_text = format_node(value, content, config, indent)?;

                          Ok(format!("const {}{} = {}", name, type_anno, value_text))
                      }

                      // Default case: return the original text (will be formatted by parent/children)
                      _ => Ok(node.text(content)?.to_string()),
                  }
              }

              /// Get the indentation string for a given level
              fn get_indent(level: usize, config: &FormattingConfig) -> String {
                  let indent_char = if config.indent_style == "space" {
                      ' '
                  } else {
                      '\t'
                  };
                  let indent_size = if config.indent_style == "space" {
                      config.indent_size
                  } else {
                      1
                  };
                  (0..level * indent_size).map(|_| indent_char).collect()
              }

              /// Get the indent level for a child node
              fn get_child_indent(parent: &Node, child: &Node, parent_indent: usize, config: &FormattingConfig) -> Result<usize> {
                  match (parent.kind(), child.kind()) {
                      // Indent children of block nodes
                      (_, "block") => Ok(parent_indent),
                      ("block", _) => Ok(parent_indent + 1),
                      ("struct_declaration", "field_declaration") => Ok(parent_indent + 1),
                      ("enum_declaration", "enum_variant") => Ok(parent_indent + 1),
                      ("interface_declaration", "method_declaration") => Ok(parent_indent + 1),
                      ("function_declaration", "block") => Ok(parent_indent + 1),
                      ("if_statement", "block") | ("else_clause", "block") |
                      ("for_statement", "block") | ("while_statement", "block") |
                      ("do_statement", "block") => Ok(parent_indent + 1),
                      _ => Ok(parent_indent),
                  }
              }

              /// Check if a separator is needed between a parent and child node
              fn needs_separator(_parent: &Node, _child: &Node, _config: &FormattingConfig) -> Result<bool> {
                  // Add separators between most children (newline or space)
                  Ok(true)
              }

              /// Get the separator between a parent and child node
              fn get_separator(parent: &Node, child: &Node, config: &FormattingConfig) -> Result<String> {
                  match (parent.kind(), child.kind()) {
                      // Space separators for inline nodes
                      (_, "identifier") | (_, "type_identifier") | (_, "primitive_type") |
                      (_, "number_literal") | (_, "boolean_literal") | (_, "string_literal") |
                      (_, "operator") => Ok(" ".to_string()),
                      // Newline separators for block nodes
                      _ => Ok(get_line_ending(config)),
                  }
              }

              /// Check if a trailing separator is needed for a node
              fn needs_trailing_separator(node: &Node, config: &FormattingConfig) -> Result<bool> {
                  match node.kind() {
                      "block" | "struct_declaration" | "enum_declaration" | "interface_declaration" |
                      "function_declaration" | "method_declaration" | "if_statement" | "for_statement" |
                      "while_statement" | "do_statement" => Ok(true),
                      _ => Ok(false),
                  }
              }

              /// Get the trailing separator for a node
              fn get_trailing_separator(node: &Node, config: &FormattingConfig) -> Result<String> {
                  match node.kind() {
                      "block" => Ok(get_line_ending(config)),
                      "struct_declaration" | "enum_declaration" | "interface_declaration" => Ok(get_line_ending(config)),
                      "function_declaration" | "method_declaration" => Ok(format!("{}{}", get_line_ending(config), get_line_ending(config))),
                      _ => Ok(get_line_ending(config)),
                  }
              }

              /// Get the line ending character(s) based on config
              fn get_line_ending(config: &FormattingConfig) -> String {
                  match config.line_ending.as_str() {
                      "crlf" => "\r\n".to_string(),
                      _ => "\n".to_string(),
                  }
              }

              /// Handle trailing commas based on config
              fn should_add_trailing_comma(node: &Node, config: &FormattingConfig) -> Result<bool> {
                  let children = node.children().collect::<Vec<_>>();
                  if children.len() <= 1 {
                      return Ok(false);
                  }

                  match config.trailing_comma.as_str() {
                      "always" => Ok(true),
                      "never" => Ok(false),
                      "multiline" => {
                          // Check if the node's children span multiple lines
                          let first_child = children.first().unwrap();
                          let last_child = children.last().unwrap();
                          Ok(first_child.start_position().row != last_child.start_position().row)
                      }
                      _ => Ok(false),
                  }
              }
              ```

              ### 47. src/lsp/symbols.rs（补充完整）
              ```rust
              //! Document and workspace symbol functionality for Cangjie LSP server
              //!
              //! Provides symbol search for the current document and entire workspace.

              use super::super::{
                  syntax::tree_sitter_utils::{self, NodeText},
                  config::CangjieConfig,
                  utils::{log::debug, find_workspace_files},
              };
              use zed_extension_api::{self as zed, Result, lsp::{
                  DocumentSymbolParams, DocumentSymbolResponse, SymbolKind, SymbolInformation,
                  WorkspaceSymbolParams, WorkspaceSymbolResponse, Location, Range, Url
              }};
              use tree_sitter::{Node, Tree};
              use std::path::PathBuf;

              /// Get symbols for the current document
              pub fn get_document_symbols(
                  document: &zed::Document,
                  tree: &Tree,
                  _params: &DocumentSymbolParams,
                  _config: &CangjieConfig,
              ) -> Result<Option<DocumentSymbolResponse>> {
                  let content = document.text();
                  let mut symbols = Vec::new();

                  // Recursively collect document symbols
                  collect_document_symbols_recursive(
                      tree.root_node(),
                      content,
                      None,
                      &mut symbols,
                      document.uri(),
                  )?;

                  debug!("Found {} document symbols", symbols.len());

                  Ok(Some(DocumentSymbolResponse::SymbolInformation(symbols)))
              }

              /// Get symbols for the entire workspace
              pub fn get_workspace_symbols(
                  workspace: &zed::Workspace,
                  params: &WorkspaceSymbolParams,
                  _config: &CangjieConfig,
              ) -> Result<Option<WorkspaceSymbolResponse>> {
                  let query = params.query.as_deref().unwrap_or("");
                  debug!("Searching workspace for symbols matching: '{}'", query);

                  let mut symbols = Vec::new();
                  let cangjie_files = find_workspace_files(workspace, "*.{cang,cj}")?;

                  for file_path in cangjie_files {
                      // Read file content
                      let content = super::super::utils::read_file_to_string(&file_path)?;

                      // Parse file with Tree-sitter
                      let tree = tree_sitter_cangjie::parse(&content, None)?;

                      // Collect symbols from this file
                      collect_document_symbols_recursive(
                          tree.root_node(),
                          &content,
                          None,
                          &mut symbols,
                          &Url::from_file_path(&file_path).map_err(|_| {
                              zed::Error::internal(format!("Invalid file path: {:?}", file_path))
                          })?,
                      )?;
                  }

                  // Filter symbols by query (case-insensitive)
                  let filtered_symbols: Vec<_> = symbols
                      .into_iter()
                      .filter(|symbol| {
                          symbol.name.to_lowercase().contains(&query.to_lowercase())
                      })
                      .collect();

                  debug!("Found {} workspace symbols matching query", filtered_symbols.len());

                  Ok(Some(WorkspaceSymbolResponse::SymbolInformation(filtered_symbols)))
              }

              /// Recursively collect document symbols from a Tree-sitter node
              fn collect_document_symbols_recursive(
                  node: Node,
                  content: &str,
                  parent: Option<&str>,
                  symbols: &mut Vec<SymbolInformation>,
                  uri: &Url,
              ) -> Result<()> {
                  let symbol = match node.kind() {
                      // Variables
                      "variable_declaration" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::Variable,
                          parent,
                          uri,
                      )?,
                      // Constants
                      "constant_declaration" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::Constant,
                          parent,
                          uri,
                      )?,
                      // Functions
                      "function_declaration" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::Function,
                          parent,
                          uri,
                      )?,
                      // Methods
                      "method_declaration" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::Method,
                          parent,
                          uri,
                      )?,
                      // Structs
                      "struct_declaration" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::Struct,
                          parent,
                          uri,
                      )?,
                      // Enums
                      "enum_declaration" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::Enum,
                          parent,
                          uri,
                      )?,
                      // Interfaces
                      "interface_declaration" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::Interface,
                          parent,
                          uri,
                      )?,
                      // Fields
                      "field_declaration" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::Field,
                          parent,
                          uri,
                      )?,
                      // Enum variants
                      "enum_variant" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::EnumMember,
                          parent,
                          uri,
                      )?,
                      // Type aliases
                      "type_alias_declaration" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::TypeParameter,
                          parent,
                          uri,
                      )?,
                      // Modules
                      "module_declaration" => create_symbol_info(
                          &node,
                          content,
                          SymbolKind::Module,
                          parent,
                          uri,
                      )?,
                      // Skip other node types
                      _ => None,
                  };

                  // Add the symbol if it exists
                  if let Some(symbol) = symbol {
                      symbols.push(symbol);

                      // For container symbols (structs, enums, interfaces, modules), pass their name as parent
                      let container_name = match node.kind() {
                          "struct_declaration" | "enum_declaration" | "interface_declaration" | "module_declaration" => {
                              Some(node.child_by_field_name("name").unwrap().text(content)?)
                          }
                          _ => parent.cloned(),
                      };

                      // Recurse into children
                      for child in node.children() {
                          collect_document_symbols_recursive(child, content, container_name.as_deref(), symbols, uri)?;
                      }
                  } else {
                      // Recurse into children even if current node is not a symbol
                      for child in node.children() {
                          collect_document_symbols_recursive(child, content, parent, symbols, uri)?;
                      }
                  }

                  Ok(())
              }

              /// Create a SymbolInformation from a Tree-sitter node
              fn create_symbol_info(
                  node: &Node,
                  content: &str,
                  kind: SymbolKind,
                  parent: Option<&str>,
                  uri: &Url,
              ) -> Result<Option<SymbolInformation>> {
                  let name_node = node.child_by_field_name("name").ok_or_else(|| {
                      zed::Error::internal(format!("Node {} has no 'name' field", node.kind()))
                  })?;

                  let name = name_node.text(content)?;
                  let range = tree_sitter_utils::node_to_range(node);
                  let selection_range = tree_sitter_utils::node_to_range(&name_node);

                  Ok(Some(SymbolInformation {
                      name: name.to_string(),
                      kind,
                      tags: None,
                      deprecated: None,
                      location: Location {
                          uri: uri.clone(),
                          range,
                      },
                      container_name: parent.map(|p| p.to_string()),
                      selection_range,
                  }))
              }
              ```

              ### 48. src/lsp/diagnostics.rs（补充完整）
              ```rust
              //! Diagnostics (linting) functionality for Cangjie LSP server
              //!
              //! Runs built-in and custom lint rules on Cangjie documents and reports diagnostics.

              use super::super::{
                  syntax::tree_sitter_utils::{self, NodeText},
                  config::CangjieConfig,
                  lint::{
                      rules::{BuiltInRule, BuiltInRules, CustomRule},
                      LintRuleSeverity,
                  },
                  utils::{log::debug, error::user_error},
              };
              use zed_extension_api::{self as zed, Result, lsp::{
                  Diagnostic, DiagnosticSeverity, DiagnosticTag, Range, Position,
                  PublishDiagnosticsParams, DiagnosticRelatedInformation, Location, Url
              }};
              use tree_sitter::{Node, Tree, Query, QueryCursor};
              use std::collections::HashMap;

              /// Run linting on a document and publish diagnostics
              pub fn run_linting(
                  document: &zed::Document,
                  tree: &Tree,
                  config: &CangjieConfig,
              ) -> Result<PublishDiagnosticsParams> {
                  let content = document.text();
                  let uri = document.uri().clone();

                  debug!("Running linting on document: {:?}", uri);

                  // Collect diagnostics from built-in rules
                  let mut diagnostics = Vec::new();
                  let builtin_rules = BuiltInRules::all();

                  // Filter built-in rules based on config
                  let enabled_builtin_rules: Vec<_> = builtin_rules
                      .into_iter()
                      .filter(|rule| !config.linting.ignore_rules.contains(&rule.id().to_string()))
                      .collect();

                  // Run built-in rules
                  for rule in enabled_builtin_rules {
                      let rule_diagnostics = run_builtin_rule(rule, tree, content, config)?;
                      diagnostics.extend(rule_diagnostics);
                  }

                  // Run custom rules (if any)
                  if let Some(custom_rules) = &config.linting.custom_rules {
                      for rule in custom_rules {
                          let rule_diagnostics = run_custom_rule(rule, tree, content, document, config)?;
                          diagnostics.extend(rule_diagnostics);
                      }
                  }

                  debug!("Found {} diagnostics for document", diagnostics.len());

                  Ok(PublishDiagnosticsParams {
                      uri,
                      diagnostics,
                      version: document.version(),
                  })
              }

              /// Run a built-in lint rule
              fn run_builtin_rule(
                  rule: Box<dyn BuiltInRule>,
                  tree: &Tree,
                  content: &str,
                  config: &CangjieConfig,
              ) -> Result<Vec<Diagnostic>> {
                  let rule_id = rule.id();
                  let rule_severity = rule.severity();
                  let min_severity = match config.linting.min_severity.as_str() {
                      "hint" => LintRuleSeverity::Hint,
                      "info" => LintRuleSeverity::Info,
                      "warning" => LintRuleSeverity::Warning,
                      "error" => LintRuleSeverity::Error,
                      _ => LintRuleSeverity::Warning,
                  };

                  // Skip rule if its severity is below the minimum configured severity
                  if rule_severity < min_severity {
                      debug!("Skipping built-in rule {} (severity too low)", rule_id);
                      return Ok(Vec::new());
                  }

                  debug!("Running built-in rule: {}", rule_id);

                  // Get the Tree-sitter query for the rule
                  let query = Query::new(tree_sitter_cangjie::language(), rule.query())
                      .map_err(|err| user_error(&format!("Invalid query for rule {}: {}", rule_id, err)))?;

                  // Execute the query
                  let mut cursor = QueryCursor::new();
                  let matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

                  // Convert matches to diagnostics
                  let mut diagnostics = Vec::new();
                  for (match_, _) in matches {
                      for capture in match_.captures {
                          let node = capture.node;
                          let range = tree_sitter_utils::node_to_range(&node);

                          // Get node text for message interpolation
                          let node_text = node.text(content)?;
                          let mut message = rule.message().to_string();
                          message = message.replace("{{node_text}}", &node_text);

                          // Create diagnostic
                          let diagnostic = Diagnostic {
                              range,
                              severity: Some(convert_severity(rule_severity)),
                              code: Some(zed::lsp::DiagnosticCode::String(rule_id.to_string())),
                              code_description: Some(zed::lsp::CodeDescription {
                                  href: Url::parse(&format!("https://cangjie-lang.org/lint/rules/{}", rule_id))
                                      .map_err(|_| user_error(&format!("Invalid URL for rule {}", rule_id)))?,
                              }),
                              source: Some("cangjie-lint".to_string()),
                              message,
                              related_information: rule.related_information(&node, content)?,
                              tags: rule.tags(),
                              data: None,
                          };

                          diagnostics.push(diagnostic);
                      }
                  }

                  Ok(diagnostics)
              }

              /// Run a custom lint rule
              fn run_custom_rule(
                  rule: &CustomRule,
                  tree: &Tree,
                  content: &str,
                  document: &zed::Document,
                  config: &CangjieConfig,
              ) -> Result<Vec<Diagnostic>> {
                  let rule_id = &rule.rule_id;
                  let rule_severity = match rule.severity.as_str() {
                      "hint" => LintRuleSeverity::Hint,
                      "info" => LintRuleSeverity::Info,
                      "warning" => LintRuleSeverity::Warning,
                      "error" => LintRuleSeverity::Error,
                      _ => {
                          warn!("Invalid severity '{}' for custom rule {}", rule.severity, rule_id);
                          LintRuleSeverity::Warning
                      }
                  };

                  let min_severity = match config.linting.min_severity.as_str() {
                      "hint" => LintRuleSeverity::Hint,
                      "info" => LintRuleSeverity::Info,
                      "warning" => LintRuleSeverity::Warning,
                      "error" => LintRuleSeverity::Error,
                      _ => LintRuleSeverity::Warning,
                  };

                  // Skip rule if its severity is below the minimum configured severity
                  if rule_severity < min_severity {
                      debug!("Skipping custom rule {} (severity too low)", rule_id);
                      return Ok(Vec::new());
                  }

                  debug!("Running custom rule: {}", rule_id);

                  // Validate rule query
                  let query = Query::new(tree_sitter_cangjie::language(), &rule.query)
                      .map_err(|err| user_error(&format!("Invalid query for custom rule {}: {}", rule_id, err)))?;

                  // Execute the query
                  let mut cursor = QueryCursor::new();
                  let matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

                  // Convert matches to diagnostics
                  let mut diagnostics = Vec::new();
                  for (match_, _) in matches {
                      for capture in match_.captures {
                          let node = capture.node;
                          let range = tree_sitter_utils::node_to_range(&node);

                          // Get node text for message interpolation
                          let node_text = node.text(content)?;
                          let mut message = rule.message.clone();
                          message = message.replace("{{node_text}}", &node_text);

                          // Create related information (if documentation is provided)
                          let related_information = rule.documentation.as_ref().map(|docs| {
                              vec![DiagnosticRelatedInformation {
                                  location: Location {
                                      uri: Url::parse(&format!("https://cangjie-lang.org/lint/custom/{}", rule_id))
                                          .unwrap_or_else(|_| document.uri().clone()),
                                      range: Range {
                                          start: Position { line: 0, character: 0 },
                                          end: Position { line: 0, character: 0 },
                                      },
                                  },
                                  message: docs.clone(),
                              }]
                          });

                          // Create fix (if provided)
                          let code_description = rule.fix.as_ref().map(|fix| {
                              let mut fixed_text = fix.replace("{{node_text}}", &node_text);
                              zed::lsp::CodeDescription {
                                  href: Url::parse(&format!("data:text/plain;charset=utf-8,{}", urlencoding::encode(&fixed_text)))
                                      .unwrap(),
                              }
                          });

                          // Create diagnostic
                          let diagnostic = Diagnostic {
                              range,
                              severity: Some(convert_severity(rule_severity)),
                              code: Some(zed::lsp::DiagnosticCode::String(rule_id.clone())),
                              code_description,
                              source: Some("cangjie-lint-custom".to_string()),
                              message,
                              related_information,
                              tags: None,
                              data: None,
                          };

                          diagnostics.push(diagnostic);
                      }
                  }

                  Ok(diagnostics)
              }

              /// Convert LintRuleSeverity to LSP DiagnosticSeverity
              fn convert_severity(severity: LintRuleSeverity) -> DiagnosticSeverity {
                  match severity {
                      LintRuleSeverity::Error => DiagnosticSeverity::Error,
                      LintRuleSeverity::Warning => DiagnosticSeverity::Warning,
                      LintRuleSeverity::Info => DiagnosticSeverity::Information,
                      LintRuleSeverity::Hint => DiagnosticSeverity::Hint,
                  }
              }
              ```

              ### 49. src/lsp/server.rs（补充完整）
              ```rust
              //! Cangjie LSP server implementation
              //!
              //! Implements the LSP server trait for Zed's extension API, integrating all LSP features.

              use super::{
                  hover::get_hover,
                  definition::get_definition,
                  completion::get_completion,
                  formatting::format_document,
                  symbols::{get_document_symbols, get_workspace_symbols},
                  diagnostics::run_linting,
              };
              use super::super::{
                  config::CangjieConfig,
                  syntax::tree_sitter_utils,
                  utils::{log::*, error::*},
              };
              use zed_extension_api::{self as zed, Result, lsp::*};
              use tree_sitter::{Parser, Tree};
              use std::collections::HashMap;
              use std::sync::{Arc, Mutex};

              /// Cangjie LSP server
              #[derive(Debug, Clone)]
              pub struct CangjieLspServer {
                  workspace: zed::Workspace,
                  config: CangjieConfig,
                  parser: Arc<Mutex<Parser>>,
                  document_trees: Arc<Mutex<HashMap<Url, Tree>>>,
              }

              impl CangjieLspServer {
                  /// Create a new Cangjie LSP server
                  pub fn new(workspace: zed::Workspace, config: CangjieConfig) -> Result<Self> {
                      // Initialize Tree-sitter parser
                      let mut parser = Parser::new();
                      parser.set_language(tree_sitter_cangjie::language())
                          .map_err(|_| internal_error("Failed to set Tree-sitter language for Cangjie"))?;

                      info!("Cangjie LSP server initialized for workspace: {:?}", workspace.root_path());

                      Ok(Self {
                          workspace,
                          config,
                          parser: Arc::new(Mutex::new(parser)),
                          document_trees: Arc::new(Mutex::new(HashMap::new())),
                      })
                  }

                  /// Parse a document and cache the Tree-sitter tree
                  fn parse_document(&self, document: &zed::Document) -> Result<Tree> {
                      let uri = document.uri().clone();
                      let content = document.text();

                      // Check if we already have a cached tree
                      let mut document_trees = self.document_trees.lock().map_err(|_| {
                          internal_error("Failed to lock document trees mutex")
                      })?;

                      if let Some(tree) = document_trees.get(&uri) {
                          // Verify the tree is still valid (document version matches)
                          if tree_sitter_utils::is_tree_valid(tree, content) {
                              return Ok(tree.clone());
                          }
                      }

                      // Parse the document
                      let mut parser = self.parser.lock().map_err(|_| {
                          internal_error("Failed to lock parser mutex")
                      })?;

                      let tree = parser.parse(content, None)
                          .ok_or_else(|| internal_error("Failed to parse document with Tree-sitter"))?;

                      // Cache the tree
                      document_trees.insert(uri, tree.clone());

                      Ok(tree)
                  }

                  /// Clear cached tree for a document
                  fn clear_document_tree(&self, uri: &Url) -> Result<()> {
                      let mut document_trees = self.document_trees.lock().map_err(|_| {
                          internal_error("Failed to lock document trees mutex")
                      })?;
                      document_trees.remove(uri);
                      Ok(())
                  }
              }

              impl zed::LanguageServer for CangjieLspServer {
                  /// Initialize the LSP server
                  fn initialize(&mut self, _params: InitializeParams) -> Result<InitializeResult> {
                      info!("Cangjie LSP server initialized");

                      Ok(InitializeResult {
                          capabilities: ServerCapabilities {
                              hover_provider: Some(HoverProviderCapability::Simple(true)),
                              definition_provider: Some(DefinitionProviderCapability::Simple(true)),
                              completion_provider: Some(CompletionOptions {
                                  trigger_characters: Some(self.config.completion.trigger_characters.clone()),
                                  all_commit_characters: None,
                                  resolve_provider: Some(false),
                                  completion_item: None,
                              }),
                              document_formatting_provider: Some(DocumentFormattingProviderCapability::Simple(true)),
                              document_range_formatting_provider: Some(DocumentRangeFormattingProviderCapability::Simple(true)),
                              document_symbol_provider: Some(DocumentSymbolProviderCapability::Simple(true)),
                              workspace_symbol_provider: Some(WorkspaceSymbolProviderCapability::Simple(true)),
                              diagnostic_provider: Some(DiagnosticProviderCapability::Simple(true)),
                              ..ServerCapabilities::default()
                          },
                          server_info: Some(ServerInfo {
                              name: "cangjie-lsp".to_string(),
                              version: Some(env!("CARGO_PKG_VERSION").to_string()),
                          }),
                      })
                  }

                  /// Shutdown the LSP server
                  fn shutdown(&mut self) -> Result<()> {
                      info!("Cangjie LSP server shutting down");
                      Ok(())
                  }

                  /// Handle a text document did open notification
                  fn text_document_did_open(&mut self, params: DidOpenTextDocumentParams) -> Result<()> {
                      let uri = params.text_document.uri;
                      let document = self.workspace.document(&uri)?;

                      info!("Text document opened: {:?}", uri);

                      // Parse the document and cache the tree
                      let tree = self.parse_document(&document)?;

                      // Run linting and publish diagnostics
                      let diagnostics = run_linting(&document, &tree, &self.config)?;
                      self.workspace.publish_diagnostics(diagnostics)?;

                      Ok(())
                  }

                  /// Handle a text document did change notification
                  fn text_document_did_change(&mut self, params: DidChangeTextDocumentParams) -> Result<()> {
                      let uri = params.text_document.uri;
                      let document = self.workspace.document(&uri)?;

                      debug!("Text document changed: {:?}", uri);

                      // Clear the cached tree (will be re-parsed on next access)
                      self.clear_document_tree(&uri)?;

                      // Parse the updated document
                      let tree = self.parse_document(&document)?;

                      // Re-run linting and publish diagnostics
                      let diagnostics = run_linting(&document, &tree, &self.config)?;
                      self.workspace.publish_diagnostics(diagnostics)?;

                      Ok(())
                  }

                  /// Handle a text document did close notification
                  fn text_document_did_close(&mut self, params: DidCloseTextDocumentParams) -> Result<()> {
                      let uri = params.text_document.uri;

                      info!("Text document closed: {:?}", uri);

                      // Clear the cached tree
                      self.clear_document_tree(&uri)?;

                      // Clear diagnostics for the closed document
                      self.workspace.publish_diagnostics(PublishDiagnosticsParams {
                          uri,
                          diagnostics: Vec::new(),
                          version: None,
                      })?;

                      Ok(())
                  }

                  /// Handle a hover request
                  fn hover(&mut self, params: HoverParams) -> Result<Option<Hover>> {
                      let uri = params.text_document_position_params.text_document.uri;
                      let document = self.workspace.document(&uri)?;
                      let tree = self.parse_document(&document)?;

                      debug!("Handling hover request for {:?} at {:?}", uri, params.text_document_position_params.position);

                      get_hover(&document, &tree, &params.text_document_position_params.position, &self.config)
                  }

                  /// Handle a definition request
                  fn definition(&mut self, params: DefinitionParams) -> Result<Option<Definition>> {
                      let uri = params.text_document_position_params.text_document.uri;
                      let document = self.workspace.document(&uri)?;
                      let tree = self.parse_document(&document)?;

                      debug!("Handling definition request for {:?} at {:?}", uri, params.text_document_position_params.position);

                      get_definition(&document, &tree, &params.text_document_position_params.position, &self.config)
                  }

                  /// Handle a completion request
                  fn completion(&mut self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
                      let uri = params.text_document_position.text_document.uri;
                      let document = self.workspace.document(&uri)?;
                      let tree = self.parse_document(&document)?;

                      debug!("Handling completion request for {:?} at {:?}", uri, params.text_document_position.position);

                      get_completion(&document, &tree, &params, &self.config)
                          .map(|list| list.map(CompletionResponse::List))
                  }

                  /// Handle a document formatting request
                  fn document_formatting(&mut self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
                      let uri = params.text_document.uri;
                      let document = self.workspace.document(&uri)?;
                      let tree = self.parse_document(&document)?;

                      debug!("Handling document formatting request for {:?}", uri);

                      format_document(&document, &tree, &params, &self.config)
                  }

                  /// Handle a document range formatting request
                  fn document_range_formatting(&mut self, params: DocumentRangeFormattingParams) -> Result<Option<Vec<TextEdit>>> {
                      let uri = params.text_document.uri;
                      let document = self.workspace.document(&uri)?;
                      let tree = self.parse_document(&document)?;

                      debug!("Handling range formatting request for {:?} (range: {:?})", uri, params.range);

                      super::formatting::format_range(&document, &tree, &params.range, &self.config)
                  }

                  /// Handle a document symbol request
                  fn document_symbol(&mut self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
                      let uri = params.text_document.uri;
                      let document = self.workspace.document(&uri)?;
                      let tree = self.parse_document(&document)?;

                      debug!("Handling document symbol request for {:?}", uri);

                      get_document_symbols(&document, &tree, &params, &self.config)
                  }

                  /// Handle a workspace symbol request
                  fn workspace_symbol(&mut self, params: WorkspaceSymbolParams) -> Result<Option<WorkspaceSymbolResponse>> {
                      debug!("Handling workspace symbol request with query: {:?}", params.query);

                      get_workspace_symbols(&self.workspace, &params, &self.config)
                  }

                  /// Handle a configuration change
                  fn did_change_configuration(&mut self, params: DidChangeConfigurationParams) -> Result<()> {
                      debug!("Handling configuration change: {:?}", params.settings);

                      // Update server configuration
                      let new_config = self.workspace.config::<CangjieConfig>()?;
                      self.config = new_config;

                      info!("Cangjie LSP server configuration updated");

                      // Re-lint all open documents with new configuration
                      let open_documents = self.workspace.documents()?;
                      for document in open_documents {
                          if document.language() == "cangjie" {
                              let tree = self.parse_document(&document)?;
                              let diagnostics = run_linting(&document, &tree, &self.config)?;
                              self.workspace.publish_diagnostics(diagnostics)?;
                          }
                      }

                      Ok(())
                  }
              }
              ```

              ### 50. src/lint/rules.rs（补充完整）
              ```rust
              //! Built-in and custom lint rules for Cangjie
              //!
              //! Provides a set of built-in lint rules and support for custom rules via configuration.

              use super::super::{
                  syntax::tree_sitter_utils::{self, NodeText},
                  utils::{string::{is_snake_case, is_pascal_case, is_upper_snake_case}, log::debug},
              };
              use zed_extension_api::{self as zed, Result, lsp::{DiagnosticRelatedInformation, Location, Range, Url}};
              use tree_sitter::Node;
              use serde::{Deserialize, Serialize};
              use std::fmt;

              /// Lint rule severity levels
              #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
              pub enum LintRuleSeverity {
                  /// Hint (lowest severity)
                  Hint,
                  /// Information
                  Info,
                  /// Warning
                  Warning,
                  /// Error (highest severity)
                  Error,
              }

              impl fmt::Display for LintRuleSeverity {
                  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                      match self {
                          LintRuleSeverity::Hint => write!(f, "hint"),
                          LintRuleSeverity::Info => write!(f, "info"),
                          LintRuleSeverity::Warning => write!(f, "warning"),
                          LintRuleSeverity::Error => write!(f, "error"),
                      }
                  }
              }

              /// Built-in lint rule trait
              pub trait BuiltInRule {
                  /// Unique rule ID (uppercase snake case)
                  fn id(&self) -> &'static str;

                  /// Rule severity
                  fn severity(&self) -> LintRuleSeverity;

                  /// Rule description
                  fn description(&self) -> &'static str;

                  /// Tree-sitter query to find matching nodes
                  fn query(&self) -> &'static str;

                  /// Diagnostic message (supports {{node_text}} placeholder)
                  fn message(&self) -> &'static str;

                  /// Diagnostic tags (optional)
                  fn tags(&self) -> Option<Vec<zed::lsp::DiagnosticTag>> {
                      None
                  }

                  /// Related information (optional)
                  fn related_information(&self, _node: &Node, _content: &str) -> Result<Option<Vec<DiagnosticRelatedInformation>>> {
                      Ok(None)
                  }

                  /// Fix suggestion (optional)
                  fn fix(&self, _node: &Node, _content: &str) -> Result<Option<String>> {
                      Ok(None)
                  }
              }

              /// Collection of all built-in lint rules
              pub struct BuiltInRules;

              impl BuiltInRules {
                  /// Get all built-in lint rules
                  pub fn all() -> Vec<Box<dyn BuiltInRule>> {
                      vec![
                          Box::new(UnusedVariableRule),
                          Box::new(UnusedConstantRule),
                          Box::new(LineTooLongRule),
                          Box::new(InvalidNamingConventionRule),
                          Box::new(MissingSemicolonRule),
                          Box::new(EmptyBlockRule),
                          Box::new(UnreachableCodeRule),
                          Box::new(DeprecatedSyntaxRule),
                      ]
                  }
              }

              // ------------------------------
              // Built-in Rule Implementations
              // ------------------------------

              /// Unused variable rule
              #[derive(Debug, Clone)]
              pub struct UnusedVariableRule;

              impl BuiltInRule for UnusedVariableRule {
                  fn id(&self) -> &'static str {
                      "UNUSED_VARIABLE"
                  }

                  fn severity(&self) -> LintRuleSeverity {
                      LintRuleSeverity::Warning
                  }

                  fn description(&self) -> &'static str {
                      "Unused variable declaration"
                  }

                  fn query(&self) -> &'static str {
                      r#"
                          (variable_declaration
                              name: (identifier) @unused_variable
                              (#not-has-ancestor? @unused_variable (variable_reference))
                          )
                      "#
                  }

                  fn message(&self) -> &'static str {
                      "Unused variable: {{node_text}}. Remove or use the variable."
                  }

                  fn related_information(&self, node: &Node, content: &str) -> Result<Option<Vec<DiagnosticRelatedInformation>>> {
                      let node_text = node.text(content)?;
                      Ok(Some(vec![DiagnosticRelatedInformation {
                          location: Location {
                              uri: Url::parse("https://cangjie-lang.org/lint/rules/UNUSED_VARIABLE")?,
                              range: Range::default(),
                          },
                          message: format!("Unused variables add unnecessary clutter to code. Either use {} or remove it.", node_text),
                      }]))
                  }
              }

              /// Unused constant rule
              #[derive(Debug, Clone)]
              pub struct UnusedConstantRule;

              impl BuiltInRule for UnusedConstantRule {
                  fn id(&self) -> &'static str {
                      "UNUSED_CONSTANT"
                  }

                  fn severity(&self) -> LintRuleSeverity {
                      LintRuleSeverity::Warning
                  }

                  fn description(&self) -> &'static str {
                      "Unused constant declaration"
                  }

                  fn query(&self) -> &'static str {
                      r#"
                          (constant_declaration
                              name: (identifier) @unused_constant
                              (#not-has-ancestor? @unused_constant (variable_reference))
                          )
                      "#
                  }

                  fn message(&self) -> &'static str {
                      "Unused constant: {{node_text}}. Remove or use the constant."
                  }
              }

              /// Line too long rule
              #[derive(Debug, Clone)]
              pub struct LineTooLongRule;

              impl BuiltInRule for LineTooLongRule {
                  fn id(&self) -> &'static str {
                      "LINE_TOO_LONG"
                  }

                  fn severity(&self) -> LintRuleSeverity {
                      LintRuleSeverity::Warning
                  }

                  fn description(&self) -> &'static str {
                      "Line exceeds maximum allowed length (120 characters)"
                  }

                  fn query(&self) -> &'static str {
                      r#"
                          (program
                              (line
                                  (#match? @line "^.{121,}$")
                              ) @long_line
                          )
                      "#
                  }

                  fn message(&self) -> &'static str {
                      "Line too long (exceeds 120 characters). Split the line into multiple lines for better readability."
                  }

                  fn related_information(&self, _node: &Node, _content: &str) -> Result<Option<Vec<DiagnosticRelatedInformation>>> {
                      Ok(Some(vec![DiagnosticRelatedInformation {
                          location: Location {
                              uri: Url::parse("https://cangjie-lang.org/lint/rules/LINE_TOO_LONG")?,
                              range: Range::default(),
                          },
                          message: "Long lines are harder to read and maintain. Keeping lines under 120 characters improves code readability.".to_string(),
                      }]))
                  }
              }

              /// Invalid naming convention rule
              #[derive(Debug, Clone)]
              pub struct InvalidNamingConventionRule;

              impl BuiltInRule for InvalidNamingConventionRule {
                  fn id(&self) -> &'static str {
                      "INVALID_NAMING_CONVENTION"
                  }

                  fn severity(&self) -> LintRuleSeverity {
                      LintRuleSeverity::Warning
                  }

                  fn description(&self) -> &'static str {
                      "Invalid naming convention (snake_case for variables/functions, PascalCase for types, UPPER_SNAKE_CASE for constants)"
                  }

                  fn query(&self) -> &'static str {
                      r#"
                          ; Variables: snake_case
                          (variable_declaration name: (identifier) @invalid_var_name)
                          ; Functions: snake_case
                          (function_declaration name: (identifier) @invalid_fn_name)
                          (method_declaration name: (identifier) @invalid_method_name)
                          ; Constants: UPPER_SNAKE_CASE
                          (constant_declaration name: (identifier) @invalid_const_name)
                          ; Structs/Enums/Interfaces: PascalCase
                          (struct_declaration name: (identifier) @invalid_struct_name)
                          (enum_declaration name: (identifier) @invalid_enum_name)
                          (interface_declaration name: (identifier) @invalid_interface_name)
                      "#
                  }

                  fn message(&self) -> &'static str {
                      "Invalid naming convention for {{node_text}}. Follow Cangjie's naming guidelines."
                  }

                  fn related_information(&self, node: &Node, content: &str) -> Result<Option<Vec<DiagnosticRelatedInformation>>> {
                      let node_text = node.text(content)?;
                      let parent = node.parent().unwrap();
                      let guideline = match parent.kind() {
                          "variable_declaration" => "Variables should use snake_case (e.g., user_name)",
                          "function_declaration" | "method_declaration" => "Functions/methods should use snake_case (e.g., calculate_total)",
                          "constant_declaration" => "Constants should use UPPER_SNAKE_CASE (e.g., MAX_RETRIES)",
                          "struct_declaration" | "enum_declaration" | "interface_declaration" => "Types should use PascalCase (e.g., UserProfile)",
                          _ => "Follow Cangjie's naming conventions: snake_case for variables/functions, PascalCase for types, UPPER_SNAKE_CASE for constants",
                      };

                      Ok(Some(vec![DiagnosticRelatedInformation {
                          location: Location {
                              uri: Url::parse("https://cangjie-lang.org/lint/rules/INVALID_NAMING_CONVENTION")?,
                              range: Range::default(),
                          },
                          message: guideline.to_string(),
                      }]))
                  }

                  fn fix(&self, node: &Node, content: &str) -> Result<Option<String>> {
                      let node_text = node.text(content)?;
                      let parent = node.parent().unwrap();

                      let fixed_name = match parent.kind() {
                          "variable_declaration" | "function_declaration" | "method_declaration" => {
                              // Convert to snake_case
                              if is_snake_case(node_text) {
                                  return Ok(None);
                              }
                              super::super::utils::string::pascal_to_snake_case(node_text)
                          }
                          "constant_declaration" => {
                              // Convert to UPPER_SNAKE_CASE
                              if is_upper_snake_case(node_text) {
                                  return Ok(None);
                              }
                              super::super::utils::string::snake_to_upper_snake_case(node_text)
                          }
                          "struct_declaration" | "enum_declaration" | "interface_declaration" => {
                              // Convert to PascalCase
                              if is_pascal_case(node_text) {
                                  return Ok(None);
                              }
                              super::super::utils::string::snake_to_pascal_case(node_text)
                          }
                          _ => return Ok(None),
                      };

                      Ok(Some(fixed_name))
                  }
              }

              /// Missing semicolon rule
              #[derive(Debug, Clone)]
              pub struct MissingSemicolonRule;

              impl BuiltInRule for MissingSemicolonRule {
                  fn id(&self) -> &'static str {
                      "MISSING_SEMICOLON"
                  }

                  fn severity(&self) -> LintRuleSeverity {
                      LintRuleSeverity::Error
                  }

                  fn description(&self) -> &'static str {
                      "Missing semicolon at end of statement"
                  }

                  fn query(&self) -> &'static str {
                      r#"
                          (statement
                              (#not-has-type? @statement block if_statement for_statement while_statement do_statement function_declaration struct_declaration enum_declaration interface_declaration)
                              (#not-has-child? @statement semicolon)
                          ) @missing_semicolon
                      "#
                  }

                  fn message(&self) -> &'static str {
                      "Missing semicolon at end of statement. Cangjie requires semicolons to terminate statements."
                  }

                  fn fix(&self, _node: &Node, _content: &str) -> Result<Option<String>> {
                      Ok(Some(";".to_string()))
                  }
              }

              /// Empty block rule
              #[derive(Debug, Clone)]
              pub struct EmptyBlockRule;

              impl BuiltInRule for EmptyBlockRule {
                  fn id(&self) -> &'static str {
                      "EMPTY_BLOCK"
                  }

                  fn severity(&self) -> LintRuleSeverity {
                      LintRuleSeverity::Warning
                  }

                  fn description(&self) -> &'static str {
                      "Empty block without a comment"
                  }

                  fn query(&self) -> &'static str {
                      r#"
                          (block
                              (statement)*
                              (#eq? @block "{}")
                              (#not-has-ancestor? @block comment)
                          ) @empty_block
                      "#
                  }

                  fn message(&self) -> &'static str {
                      "Empty block detected. Add a comment explaining why the block is empty or remove it."
                  }

                  fn related_information(&self, _node: &Node, _content: &str) -> Result<Option<Vec<DiagnosticRelatedInformation>>> {
                      Ok(Some(vec![DiagnosticRelatedInformation {
                          location: Location {
                              uri: Url::parse("https://cangjie-lang.org/lint/rules/EMPTY_BLOCK")?,
                              range: Range::default(),
                          },
                          message: "Empty blocks can be confusing. If the block is intentionally empty, add a comment like `// No-op` to clarify.".to_string(),
                      }]))
                  }

                  fn fix(&self, _node: &Node, _content: &str) -> Result<Option<String>> {
                      Ok(Some("{ // No-op }".to_string()))
                  }
              }

              /// Unreachable code rule
              #[derive(Debug, Clone)]
              pub struct UnreachableCodeRule;

              impl BuiltInRule for UnreachableCodeRule {
                  fn id(&self) -> &'static str {
                      "UNREACHABLE_CODE"
                  }

                  fn severity(&self) -> LintRuleSeverity {
                      LintRuleSeverity::Warning
                  }

                  fn description(&self) -> &'static str {
                      "Unreachable code after return/break/continue"
                  }

                  fn query(&self) -> &'static str {
                      r#"
                          (block
                              (return_statement) @return
                              (statement)+ @unreachable
                              (#has-ancestor? @return function_declaration method_declaration)
                          )
                          (block
                              (break_statement) @break
                              (statement)+ @unreachable
                              (#has-ancestor? @break loop_statement)
                          )
                          (block
                              (continue_statement) @continue
                              (statement)+ @unreachable
                              (#has-ancestor? @continue loop_statement)
                          )
                      "#
                  }

                  fn message(&self) -> &'static str {
                      "Unreachable code detected. Code after return/break/continue will never be executed."
                  }

                  fn tags(&self) -> Option<Vec<zed::lsp::DiagnosticTag>> {
                      Some(vec![zed::lsp::DiagnosticTag::Unnecessary])
                  }
              }

              /// Deprecated syntax rule
              #[derive(Debug, Clone)]
              pub struct DeprecatedSyntaxRule;

              impl BuiltInRule for DeprecatedSyntaxRule {
                  fn id(&self) -> &'static str {
                      "DEPRECATED_SYNTAX"
                  }

                  fn severity(&self) -> LintRuleSeverity {
                      LintRuleSeverity::Warning
                  }

                  fn description(&self) -> &'static str {
                      "Use of deprecated syntax"
                  }

                  fn query(&self) -> &'static str {
                      r#"
                          ; Deprecated 'var' keyword (use 'let' instead)
                          (keyword_declaration
                              (#eq? @keyword_declaration "var")
                          ) @deprecated_var
                          ; Deprecated 'function' keyword for method declarations (use 'fn' instead)
                          (method_declaration
                              (keyword_declaration
                                  (#eq? @keyword_declaration "function")
                              )
                          ) @deprecated_function
                      "#
                  }

                  fn message(&self) -> &'static str {
                      "Deprecated syntax: {{node_text}}. Use the recommended alternative."
                  }

                  fn related_information(&self, node: &Node, content: &str) -> Result<Option<Vec<DiagnosticRelatedInformation>>> {
                      let node_text = node.text(content)?;
                      let alternative = match node_text.as_str() {
                          "var" => "Use 'let' instead of 'var' (e.g., `let x = 5;`)",
                          "function" => "Use 'fn' instead of 'function' for method declarations (e.g., `fn my_method() {}`)",
                          _ => "See Cangjie's documentation for the recommended alternative syntax",
                      };

                      Ok(Some(vec![DiagnosticRelatedInformation {
                          location: Location {
                              uri: Url::parse("https://cangjie-lang.org/lint/rules/DEPRECATED_SYNTAX")?,
                              range: Range::default(),
                          },
                          message: alternative.to_string(),
                      }]))
                  }

                  fn fix(&self, node: &Node, content: &str) -> Result<Option<String>> {
                      let node_text = node.text(content)?;
                      match node_text.as_str() {
                          "var" => Ok(Some("let".to_string())),
                          "function" => Ok(Some("fn".to_string())),
                          _ => Ok(None),
                      }
                  }
              }

              /// Custom lint rule (configured via extension settings)
              #[derive(Debug, Clone, Serialize, Deserialize)]
              pub struct CustomRule {
                  /// Unique rule ID (uppercase snake case)
                  pub rule_id: String,

                  /// Rule description
                  pub description: String,

                  /// Rule severity
                  pub severity: String,

                  /// Tree-sitter query to find matching nodes
                  pub query: String,

                  /// Diagnostic message (supports {{node_text}} placeholder)
                  pub message: String,

                  /// Rule documentation (optional)
                  pub documentation: Option<String>,

                  /// Fix suggestion (optional, supports {{node_text}} placeholder)
                  pub fix: Option<String>,
              }

              impl CustomRule {
                  /// Validate the custom rule
                  pub fn validate(&self) -> Result<()> {
                      // Validate rule ID (uppercase snake case)
                      if !is_upper_snake_case(&self.rule_id) {
                          return Err(user_error(&format!(
                              "Invalid custom rule ID '{}': must be UPPER_SNAKE_CASE",
                              self.rule_id
                          )));
                      }

                      // Validate severity
                      match self.severity.as_str() {
                          "hint" | "info" | "warning" | "error" => {},
                          _ => {
                              return Err(user_error(&format!(
                                  "Invalid severity '{}' for custom rule {}: must be one of hint/info/warning/error",
                                  self.severity, self.rule_id
                              )));
                          }
                      }

                      // Validate query (basic syntax check)
                      Query::new(tree_sitter_cangjie::language(), &self.query)
                          .map_err(|err| user_error(&format!(
                              "Invalid query for custom rule {}: {}",
                              self.rule_id, err
                          )))?;

                      Ok(())
                  }
              }
              ```

 ---

               ## 项目完整说明

               至此，**Cangjie 编程语言的 Zed 编辑器扩展**已完全实现，包含 50 个核心文件，覆盖了 Zed 扩展开发的所有关键领域：

               ### 核心功能模块



              ---

              ## 项目完整说明

              至此，**Cangjie 编程语言的 Zed 编辑器扩展**已完全实现，包含 50 个核心文件，覆盖了 Zed 扩展开发的所有关键领域：

              ### 核心功能模块
              | 模块 | 功能 |
              |------|------|
              | **语法支持** | Tree-sitter 语法解析、语法高亮、代码片段 |
              | **LSP 集成** | 悬停文档、跳转定义、代码补全、格式化、符号搜索、诊断（linting） |
              | **代码格式化** | 可配置的缩进、大括号风格、行长度、尾随逗号等规则 |
              | **代码 linting** | 8 个内置规则 + 自定义规则支持（Tree-sitter 查询） |
              | **配置系统** | 全局/工作区级配置，覆盖格式化、linting、补全、语法高亮 |
              | **工具类** | 日志、字符串处理、文件操作、错误处理、Tree-sitter 工具函数 |
              | **工程化** | Cargo 配置、CI/CD 工作流、文档、贡献指南 |

              ### 技术栈
              - **语言**：Rust（Zed 扩展官方支持语言）
              - **语法解析**：Tree-sitter + tree-sitter-cangjie 语法库
              - **LSP 实现**：基于 Zed Extension API 的 LSP 服务器
              - **配置**：Serde 序列化/反序列化
              - **日志**：自定义日志系统 + Zed 扩展日志集成
              - **测试**：Rust 单元测试 + Zed 扩展测试框架

              ### 特色亮点
              1. **高度可配置**：几乎所有功能都支持通过 Zed 设置自定义
              2. **性能优化**：Tree-sitter 语法缓存、文档解析缓存
              3. **用户体验**：丰富的代码片段、智能补全、详细的悬停文档
              4. **扩展性**：支持自定义 lint 规则、易于添加新的 LSP 功能
              5. **跨平台**：支持 macOS、Linux、Windows 三大操作系统

              ### 部署与发布
              1. 本地开发：`cargo build --release` + `zed extensions link`
              2
              ### 51. src/utils/string.rs（补充完整）
              ```rust
              //! String utility functions for Cangjie extension
              //!
              //! Provides common string manipulation utilities used throughout the extension.

              use std::fmt;

              /// Convert PascalCase to snake_case
              pub fn pascal_to_snake_case(s: &str) -> String {
                  if s.is_empty() {
                      return s.to_string();
                  }

                  let mut result = String::with_capacity(s.len());
                  let mut chars = s.chars();

                  // Add the first character in lowercase
                  if let Some(first) = chars.next() {
                      result.push(first.to_ascii_lowercase());
                  }

                  // Process remaining characters
                  for c in chars {
                      if c.is_ascii_uppercase() {
                          result.push('_');
                          result.push(c.to_ascii_lowercase());
                      } else {
                          result.push(c);
                      }
                  }

                  result
              }

              /// Convert snake_case to PascalCase
              pub fn snake_to_pascal_case(s: &str) -> String {
                  if s.is_empty() {
                      return s.to_string();
                  }

                  let mut result = String::with_capacity(s.len());
                  let mut capitalize_next = true;

                  for c in s.chars() {
                      if c == '_' {
                          capitalize_next = true;
                      } else if capitalize_next {
                          result.push(c.to_ascii_uppercase());
                          capitalize_next = false;
                      } else {
                          result.push(c.to_ascii_lowercase());
                      }
                  }

                  result
              }

              /// Convert snake_case to UPPER_SNAKE_CASE
              pub fn snake_to_upper_snake_case(s: &str) -> String {
                  s.replace('_', "_").to_ascii_uppercase()
              }

              /// Check if a string is in snake_case
              pub fn is_snake_case(s: &str) -> bool {
                  if s.is_empty() {
                      return false;
                  }

                  let chars: Vec<char> = s.chars().collect();

                  // First character must be lowercase letter
                  if !chars[0].is_ascii_lowercase() {
                      return false;
                  }

                  // Check remaining characters
                  for c in &chars[1..] {
                      if !c.is_ascii_lowercase() && !c.is_ascii_digit() && *c != '_' {
                          return false;
                      }

                      // No consecutive underscores
                      if *c == '_' && chars[chars.iter().position(|&x| x == *c).unwrap() - 1] == '_' {
                          return false;
                      }
                  }

                  // No trailing underscore
                  !s.ends_with('_')
              }

              /// Check if a string is in PascalCase
              pub fn is_pascal_case(s: &str) -> bool {
                  if s.is_empty() {
                      return false;
                  }

                  let chars: Vec<char> = s.chars().collect();

                  // First character must be uppercase letter
                  if !chars[0].is_ascii_uppercase() {
                      return false;
                  }

                  // Check remaining characters
                  for c in &chars[1..] {
                      if !c.is_ascii_alphanumeric() {
                          return false;
                      }
                  }

                  true
              }

              /// Check if a string is in UPPER_SNAKE_CASE
              pub fn is_upper_snake_case(s: &str) -> bool {
                  if s.is_empty() {
                      return false;
                  }

                  let chars: Vec<char> = s.chars().collect();

                  // First character must be uppercase letter
                  if !chars[0].is_ascii_uppercase() {
                      return false;
                  }

                  // Check remaining characters
                  for c in &chars[1..] {
                      if !c.is_ascii_uppercase() && !c.is_ascii_digit() && *c != '_' {
                          return false;
                      }

                      // No consecutive underscores
                      if *c == '_' && chars[chars.iter().position(|&x| x == *c).unwrap() - 1] == '_' {
                          return false;
                      }
                  }

                  // No trailing underscore
                  !s.ends_with('_')
              }

              /// Truncate a string to a maximum length, adding ellipsis if needed
              pub fn truncate_string(s: &str, max_length: usize) -> String {
                  if s.len() <= max_length {
                      return s.to_string();
                  }

                  let mut truncated = s.chars().take(max_length - 3).collect::<String>();
                  truncated.push_str("...");
                  truncated
              }

              /// Escape special characters in a string for use in regex/queries
              pub fn escape_special_chars(s: &str) -> String {
                  let special_chars = ['\\', '.', '+', '*', '?', '[', ']', '(', ')', '{', '}', '^', '$', '|'];
                  let mut result = String::with_capacity(s.len() * 2);

                  for c in s.chars() {
                      if special_chars.contains(&c) {
                          result.push('\\');
                      }
                      result.push(c);
                  }

                  result
              }

              /// Indent each line of a string
              pub fn indent_string(s: &str, indent: &str) -> String {
                  s.lines()
                      .map(|line| format!("{}{}", indent, line))
                      .collect::<Vec<_>>()
                      .join("\n")
              }

              /// Format a string as a code block with syntax highlighting
              pub fn format_code_block(code: &str, language: &str) -> String {
                  format!("```{}
              {}
              ```", language, code)
              }

              /// Convert a Rust format string to a Cangjie format string
              pub fn rust_to_cangjie_format(s: &str) -> String {
                  // Replace {} with {} (same syntax)
                  // Replace {:?} with {:debug}
                  // Replace {:#?} with {:pretty_debug}
                  // Replace {:.2f} with {:float(2)}
                  let mut result = s.to_string();

                  // Handle debug formatting
                  result = result.replace("{:?}", "{:debug}");
                  result = result.replace("{:#?}", "{:pretty_debug}");

                  // Handle float formatting (simplified)
                  result = regex::Regex::new(r"\{:(\.\d+)f\}")
                      .unwrap()
                      .replace_all(&result, |caps: &regex::Captures| {
                          let precision = caps[1].replace('.', "");
                          format!("{{:float({})}}", precision)
                      })
                      .to_string();

                  result
              }

              #[cfg(test)]
              mod tests {
                  use super::*;

                  #[test]
                  fn test_pascal_to_snake_case() {
                      assert_eq!(pascal_to_snake_case("PascalCase"), "pascal_case");
                      assert_eq!(pascal_to_snake_case("CamelCase"), "camel_case");
                      assert_eq!(pascal_to_snake_case("SingleWord"), "single_word");
                      assert_eq!(pascal_to_snake_case("HTMLParser"), "html_parser");
                  }

                  #[test]
                  fn test_snake_to_pascal_case() {
                      assert_eq!(snake_to_pascal_case("snake_case"), "SnakeCase");
                      assert_eq!(snake_to_pascal_case("camel_case"), "CamelCase");
                      assert_eq!(snake_to_pascal_case("single_word"), "SingleWord");
                      assert_eq!(snake_to_pascal_case("html_parser"), "HtmlParser");
                  }

                  #[test]
                  fn test_is_snake_case() {
                      assert!(is_snake_case("snake_case"));
                      assert!(is_snake_case("snake_case_123"));
                      assert!(!is_snake_case("SnakeCase"));
                      assert!(!is_snake_case("snakeCase"));
                      assert!(!is_snake_case("SNAKE_CASE"));
                      assert!(!is_snake_case("snake__case"));
                      assert!(!is_snake_case("snake_case_"));
                      assert!(!is_snake_case("123_snake"));
                  }

                  #[test]
                  fn test_is_pascal_case() {
                      assert!(is_pascal_case("PascalCase"));
                      assert!(is_pascal_case("SingleWord"));
                      assert!(is_pascal_case("HTMLParser"));
                      assert!(!is_pascal_case("pascalCase"));
                      assert!(!is_pascal_case("snake_case"));
                      assert!(!is_pascal_case("Pascal_Case"));
                      assert!(!is_pascal_case("123Pascal"));
                  }

                  #[test]
                  fn test_is_upper_snake_case() {
                      assert!(is_upper_snake_case("UPPER_SNAKE_CASE"));
                      assert!(is_upper_snake_case("UPPER_SNAKE_123"));
                      assert!(!is_upper_snake_case("UpperSnakeCase"));
                      assert!(!is_upper_snake_case("upper_snake_case"));
                      assert!(!is_upper_snake_case("UPPER__SNAKE"));
                      assert!(!is_upper_snake_case("UPPER_SNAKE_"));
                      assert!(!is_upper_snake_case("123_UPPER"));
                  }

                  #[test]
                  fn test_truncate_string() {
                      assert_eq!(truncate_string("short", 10), "short");
                      assert_eq!(truncate_string("this is a long string", 10), "this is a...");
                      assert_eq!(truncate_string("123456789012345", 5), "12...");
                  }

                  #[test]
                  fn test_escape_special_chars() {
                      assert_eq!(escape_special_chars("a.b+c*?[x]"), "a\\.b\\+c\\*\\?\\[x\\]");
                      assert_eq!(escape_special_chars("(hello)|(world)"), "\\(hello\\)\\|\\(world\\)");
                  }

                  #[test]
                  fn test_indent_string() {
                      assert_eq!(indent_string("line1\nline2", "  "), "  line1\n  line2");
                      assert_eq!(indent_string("single line", "\t"), "\tsingle line");
                  }

                  #[test]
                  fn test_rust_to_cangjie_format() {
                      assert_eq!(rust_to_cangjie_format("Hello {}!"), "Hello {}!");
                      assert_eq!(rust_to_cangjie_format("Value: {:?}"), "Value: {:debug}");
                      assert_eq!(rust_to_cangjie_format("Debug: {:#?}"), "Debug: {:pretty_debug}");
                      assert_eq!(rust_to_cangjie_format("Float: {:.2f}"), "Float: {:float(2)}");
                      assert_eq!(rust_to_cangjie_format("Percent: {:.1f}%"), "Percent: {:float(1)}%");
                  }
              }
              ```

              ### 52. src/utils/file.rs（补充完整）
              ```rust
              //! File utility functions for Cangjie extension
              //!
              //! Provides file system operations used throughout the extension.

              use zed_extension_api::{self as zed, Result};
              use std::path::{Path, PathBuf};
              use std::fs::{self, File};
              use std::io::{self, Read, Write};

              /// Find all files in a workspace matching a glob pattern
              pub fn find_workspace_files(workspace: &zed::Workspace, pattern: &str) -> Result<Vec<PathBuf>> {
                  let root_path = workspace.root_path()?;
                  find_files(&root_path, pattern)
              }

              /// Find all files in a directory (recursive) matching a glob pattern
              pub fn find_files(root: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
                  let mut results = Vec::new();
                  let glob_pattern = root.join(pattern).to_string_lossy().into_owned();

                  for entry in glob::glob(&glob_pattern)? {
                      match entry {
                          Ok(path) => {
                              if path.is_file() {
                                  results.push(path);
                              }
                          }
                          Err(e) => {
                              zed::log::warn!("Glob error: {}", e);
                          }
                      }
                  }

                  Ok(results)
              }

              /// Read a file to a string
              pub fn read_file_to_string(path: &Path) -> Result<String> {
                  let mut file = File::open(path)?;
                  let mut content = String::new();
                  file.read_to_string(&mut content)?;
                  Ok(content)
              }

              /// Write a string to a file
              pub fn write_string_to_file(path: &Path, content: &str) -> Result<()> {
                  // Create parent directories if they don't exist
                  if let Some(parent) = path.parent() {
                      fs::create_dir_all(parent)?;
                  }

                  let mut file = File::create(path)?;
                  file.write_all(content.as_bytes())?;
                  Ok(())
              }

              /// Check if a file exists
              pub fn file_exists(path: &Path) -> bool {
                  path.exists() && path.is_file()
              }

              /// Check if a directory exists
              pub fn dir_exists(path: &Path) -> bool {
                  path.exists() && path.is_dir()
              }

              /// Get the file extension (without the dot)
              pub fn file_extension(path: &Path) -> Option<String> {
                  path.extension()
                      .and_then(|ext| ext.to_str())
                      .map(|s| s.to_lowercase())
              }

              /// Get the file name without extension
              pub fn file_stem(path: &Path) -> Option<String> {
                  path.file_stem()
                      .and_then(|stem| stem.to_str())
                      .map(|s| s.to_string())
              }

              /// Copy a file from source to destination
              pub fn copy_file(source: &Path, destination: &Path) -> Result<()> {
                  // Create parent directories for destination
                  if let Some(parent) = destination.parent() {
                      fs::create_dir_all(parent)?;
                  }

                  fs::copy(source, destination)?;
                  Ok(())
              }

              /// Delete a file
              pub fn delete_file(path: &Path) -> Result<()> {
                  if file_exists(path) {
                      fs::remove_file(path)?;
                  }
                  Ok(())
              }

              /// Delete a directory (recursive)
              pub fn delete_dir(path: &Path) -> Result<()> {
                  if dir_exists(path) {
                      fs::remove_dir_all(path)?;
                  }
                  Ok(())
              }

              /// Get the relative path from base to target
              pub fn relative_path(base: &Path, target: &Path) -> Result<PathBuf> {
                  let rel_path = pathdiff::diff_paths(target, base)
                      .ok_or_else(|| zed::Error::internal("Failed to compute relative path"))?;
                  Ok(rel_path)
              }

              /// Get the canonical path (resolve symlinks and relative components)
              pub fn canonical_path(path: &Path) -> Result<PathBuf> {
                  let canonical = fs::canonicalize(path)?;
                  Ok(canonical)
              }

              /// Check if a path is inside another directory
              pub fn is_path_inside(parent: &Path, child: &Path) -> Result<bool> {
                  let parent_canon = canonical_path(parent)?;
                  let child_canon = canonical_path(child)?;
                  Ok(child_canon.starts_with(parent_canon))
              }

              /// Get all files in a directory (non-recursive)
              pub fn list_files(dir: &Path) -> Result<Vec<PathBuf>> {
                  let mut files = Vec::new();

                  if dir_exists(dir) {
                      for entry in fs::read_dir(dir)? {
                          let entry = entry?;
                          let path = entry.path();
                          if path.is_file() {
                              files.push(path);
                          }
                      }
                  }

                  Ok(files)
              }

              /// Get all directories in a directory (non-recursive)
              pub fn list_dirs(dir: &Path) -> Result<Vec<PathBuf>> {
                  let mut dirs = Vec::new();

                  if dir_exists(dir) {
                      for entry in fs::read_dir(dir)? {
                          let entry = entry?;
                          let path = entry.path();
                          if path.is_dir() && !path.file_name().unwrap_or_default().to_str().unwrap_or("").starts_with('.') {
                              dirs.push(path);
                          }
                      }
                  }

                  Ok(dirs)
              }

              /// Read a JSON file and deserialize it
              pub fn read_json_file<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
                  let content = read_file_to_string(path)?;
                  let data = serde_json::from_str(&content)?;
                  Ok(data)
              }

              /// Serialize data to JSON and write to file
              pub fn write_json_file<T: serde::Serialize>(path: &Path, data: &T) -> Result<()> {
                  let json = serde_json::to_string_pretty(data)?;
                  write_string_to_file(path, &json)?;
                  Ok(())
              }

              /// Read a TOML file and deserialize it
              pub fn read_toml_file<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
                  let content = read_file_to_string(path)?;
                  let data = toml::from_str(&content)?;
                  Ok(data)
              }

              /// Serialize data to TOML and write to file
              pub fn write_toml_file<T: serde::Serialize>(path: &Path, data: &T) -> Result<()> {
                  let toml = toml::to_string_pretty(data)?;
                  write_string_to_file(path, &toml)?;
                  Ok(())
              }

              #[cfg(test)]
              mod tests {
                  use super::*;
                  use tempfile::tempdir;

                  #[test]
                  fn test_read_write_file() {
                      let dir = tempdir().unwrap();
                      let path = dir.path().join("test.txt");
                      let content = "Hello, world!";

                      write_string_to_file(&path, content).unwrap();
                      let read_content = read_file_to_string(&path).unwrap();

                      assert_eq!(read_content, content);
                  }

                  #[test]
                  fn test_file_extension() {
                      assert_eq!(file_extension(Path::new("file.txt")), Some("txt".to_string()));
                      assert_eq!(file_extension(Path::new("file.md")), Some("md".to_string()));
                      assert_eq!(file_extension(Path::new("file")), None);
                      assert_eq!(file_extension(Path::new("file.tar.gz")), Some("gz".to_string()));
                  }

                  #[test]
                  fn test_file_stem() {
                      assert_eq!(file_stem(Path::new("file.txt")), Some("file".to_string()));
                      assert_eq!(file_stem(Path::new("document.md")), Some("document".to_string()));
                      assert_eq!(file_stem(Path::new("file.tar.gz")), Some("file.tar".to_string()));
                      assert_eq!(file_stem(Path::new("file")), Some("file".to_string()));
                  }

                  #[test]
                  fn test_relative_path() {
                      let base = Path::new("/home/user/project");
                      let target = Path::new("/home/user/project/src/file.rs");
                      assert_eq!(relative_path(base, target).unwrap(), PathBuf::from("src/file.rs"));

                      let target2 = Path::new("/home/user/other/file.rs");
                      assert_eq!(relative_path(base, target2).unwrap(), PathBuf::from("../../other/file.rs"));
                  }

                  #[test]
                  fn test_is_path_inside() {
                      let parent = Path::new("/home/user/project");
                      let child = Path::new("/home/user/project/src/file.rs");
                      assert!(is_path_inside(parent, child).unwrap());

                      let child2 = Path::new("/home/user/other/file.rs");
                      assert!(!is_path_inside(parent, child2).unwrap());
                  }

                  #[test]
                  fn test_json_serialization() {
                      #[derive(Debug, Serialize, Deserialize, PartialEq)]
                      struct TestData {
                          name: String,
                          value: i32,
                      }

                      let dir = tempdir().unwrap();
                      let path = dir.path().join("test.json");
                      let data = TestData {
                          name: "test".to_string(),
                          value: 42,
                      };

                      write_json_file(&path, &data).unwrap();
                      let read_data = read_json_file(&path).unwrap();

                      assert_eq!(read_data, data);
                  }

                  #[test]
                  fn test_toml_serialization() {
                      #[derive(Debug, Serialize, Deserialize, PartialEq)]
                      struct TestData {
                          name: String,
                          value: i32,
                      }

                      let dir = tempdir().unwrap();
                      let path = dir.path().join("test.toml");
                      let data = TestData {
                          name: "test".to_string(),
                          value: 42,
                      };

                      write_toml_file(&path, &data).unwrap();
                      let read_data = read_toml_file(&path).unwrap();

                      assert_eq!(read_data, data);
                  }
              }
              ```

              ### 53. src/utils/log.rs（补充完整）
              ```rust
              //! Logging utility for Cangjie extension
              //!
              //! Provides a structured logging system integrated with Zed's extension API.

              use zed_extension_api::{self as zed, log::LogLevel};
              use std::fmt;
              use std::sync::Mutex;

              /// Log level filter (controls which logs are emitted)
              #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
              pub enum CangjieLogLevel {
                  /// Trace level (most verbose)
                  Trace,
                  /// Debug level
                  Debug,
                  /// Info level
                  Info,
                  /// Warn level
                  Warn,
                  /// Error level (least verbose)
                  Error,
              }

              impl Default for CangjieLogLevel {
                  fn default() -> Self {
                      CangjieLogLevel::Info
                  }
              }

              impl From<CangjieLogLevel> for LogLevel {
                  fn from(level: CangjieLogLevel) -> Self {
                      match level {
                          CangjieLogLevel::Trace => LogLevel::Trace,
                          CangjieLogLevel::Debug => LogLevel::Debug,
                          CangjieLogLevel::Info => LogLevel::Info,
                          CangjieLogLevel::Warn => LogLevel::Warn,
                          CangjieLogLevel::Error => LogLevel::Error,
                      }
                  }
              }

              /// Log configuration
              #[derive(Debug, Clone)]
              pub struct LogConfig {
                  /// Minimum log level to emit
                  pub level: CangjieLogLevel,
                  /// Whether to include timestamps in logs
                  pub include_timestamps: bool,
                  /// Whether to include module names in logs
                  pub include_modules: bool,
                  /// Whether to write logs to a file (in addition to Zed's log)
                  pub file_logging: bool,
                  /// Path to log file (if file_logging is true)
                  pub log_file_path: Option<String>,
              }

              impl Default for LogConfig {
                  fn default() -> Self {
                      Self {
                          level: CangjieLogLevel::default(),
                          include_timestamps: true,
                          include_modules: true,
                          file_logging: false,
                          log_file_path: None,
                      }
                  }
              }

              /// Global logger instance
              static LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

              /// Logger struct
              #[derive(Debug, Clone)]
              struct Logger {
                  config: LogConfig,
              }

              impl Logger {
                  /// Create a new logger with the given config
                  fn new(config: LogConfig) -> Self {
                      Self { config }
                  }

                  /// Log a message with the given level and module
                  fn log(
                      &self,
                      level: CangjieLogLevel,
                      module: &str,
                      message: impl fmt::Display,
                  ) {
                      // Skip if log level is below the configured threshold
                      if level < self.config.level {
                          return;
                      }

                      // Format the log message
                      let mut log_message = String::new();

                      // Add timestamp if enabled
                      if self.config.include_timestamps {
                          let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
                          log_message.push_str(&format!("[{}] ", timestamp));
                      }

                      // Add log level
                      log_message.push_str(&format!("[{:5}] ", level));

                      // Add module name if enabled
                      if self.config.include_modules {
                          log_message.push_str(&format!("[{}] ", module));
                      }

                      // Add the actual message
                      log_message.push_str(&message.to_string());

                      // Log to Zed's extension log
                      zed::log::log(level.into(), &log_message);

                      // Log to file if enabled
                      if self.config.file_logging {
                          if let Some(log_path) = &self.config.log_file_path {
                              let _ = self.write_to_file(log_path, &log_message);
                          }
                      }
                  }

                  /// Write a log message to a file
                  fn write_to_file(&self, path: &str, message: &str) -> std::io::Result<()> {
                      use std::fs::OpenOptions;
                      use std::io::Write;

                      let path = std::path::Path::new(path);

                      // Create parent directories if needed
                      if let Some(parent) = path.parent() {
                          std::fs::create_dir_all(parent)?;
                      }

                      // Open the file in append mode (create if it doesn't exist)
                      let mut file = OpenOptions::new()
                          .create(true)
                          .append(true)
                          .open(path)?;

                      // Write the message with a newline
                      writeln!(file, "{}", message)?;
                      Ok(())
                  }
              }

              /// Initialize the logger with the given config
              pub fn init_logger(config: LogConfig) -> Result<(), String> {
                  let mut logger = LOGGER.lock().map_err(|e| format!("Failed to lock logger mutex: {}", e))?;
                  *logger = Some(Logger::new(config));
                  Ok(())
              }

              /// Get the current logger instance
              fn get_logger() -> Option<Logger> {
                  LOGGER.lock().ok()?.clone()
              }

              /// Log a trace level message
              #[macro_export]
              macro_rules! trace {
                  ($($arg:tt)*) => {
                      if let Some(logger) = $crate::utils::log::get_logger() {
                          logger.log($crate::utils::log::CangjieLogLevel::Trace, module_path!(), format!($($arg)*));
                      }
                  };
              }

              /// Log a debug level message
              #[macro_export]
              macro_rules! debug {
                  ($($arg:tt)*) => {
                      if let Some(logger) = $crate::utils::log::get_logger() {
                          logger.log($crate::utils::log::CangjieLogLevel::Debug, module_path!(), format!($($arg)*));
                      }
                  };
              }

              /// Log an info level message
              #[macro_export]
              macro_rules! info {
                  ($($arg:tt)*) => {
                      if let Some(logger) = $crate::utils::log::get_logger() {
                          logger.log($crate::utils::log::CangjieLogLevel::Info, module_path!(), format!($($arg)*));
                      }
                  };
              }

              /// Log a warn level message
              #[macro_export]
              macro_rules! warn {
                  ($($arg:tt)*) => {
                      if let Some(logger) = $crate::utils::log::get_logger() {
                          logger.log($crate::utils::log::CangjieLogLevel::Warn, module_path!(), format!($($arg)*));
                      }
                  };
              }

              /// Log an error level message
              #[macro_export]
              macro_rules! error {
                  ($($arg:tt)*) => {
                      if let Some(logger) = $crate::utils::log::get_logger() {
                          logger.log($crate::utils::log::CangjieLogLevel::Error, module_path!(), format!($($arg)*));
                      }
                  };
              }

              /// Log an error with a result
              #[macro_export]
              macro_rules! log_error {
                  ($result:expr) => {
                      if let Err(e) = $result {
                          $crate::warn!("Error: {}", e);
                      }
                  };
              }

              /// Log a result with debug information
              #[macro_export]
              macro_rules! log_result {
                  ($result:expr, $message:expr) => {
                      match $result {
                          Ok(value) => {
                              $crate::debug!("{}: {:?}", $message, value);
                              Ok(value)
                          }
                          Err(e) => {
                              $crate::error!("{} failed: {}", $message, e);
                              Err(e)
                          }
                      }
                  };
              }

              impl fmt::Display for CangjieLogLevel {
                  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                      match self {
                          CangjieLogLevel::Trace => write!(f, "TRACE"),
                          CangjieLogLevel::Debug => write!(f, "DEBUG"),
                          CangjieLogLevel::Info => write!(f, "INFO"),
                          CangjieLogLevel::Warn => write!(f, "WARN"),
                          CangjieLogLevel::Error => write!(f, "ERROR"),
                      }
                  }
              }

              #[cfg(test)]
              mod tests {
                  use super::*;
                  use tempfile::tempdir;

                  #[test]
                  fn test_log_level_order() {
                      assert!(CangjieLogLevel::Trace < CangjieLogLevel::Debug);
                      assert!(CangjieLogLevel::Debug < CangjieLogLevel::Info);
                      assert!(CangjieLogLevel::Info < CangjieLogLevel::Warn);
                      assert!(CangjieLogLevel::Warn < CangjieLogLevel::Error);
                  }

                  #[test]
                  fn test_log_level_display() {
                      assert_eq!(format!("{}", CangjieLogLevel::Trace), "TRACE");
                      assert_eq!(format!("{}", CangjieLogLevel::Debug), "DEBUG");
                      assert_eq!(format!("{}", CangjieLogLevel::Info), "INFO");
                      assert_eq!(format!("{}", CangjieLogLevel::Warn), "WARN");
                      assert_eq!(format!("{}", CangjieLogLevel::Error), "ERROR");
                  }

                  #[test]
                  fn test_log_config_default() {
                      let config = LogConfig::default();
                      assert_eq!(config.level, CangjieLogLevel::Info);
                      assert!(config.include_timestamps);
                      assert!(config.include_modules);
                      assert!(!config.file_logging);
                      assert!(config.log_file_path.is_none());
                  }

                  #[test]
                  fn test_file_logging() {
                      let dir = tempdir().unwrap();
                      let log_path = dir.path().join("cangjie.log").to_string_lossy().into_owned();

                      let config = LogConfig {
                          level: CangjieLogLevel::Debug,
                          include_timestamps: true,
                          include_modules: true,
                          file_logging: true,
                          log_file_path: Some(log_path.clone()),
                      };

                      init_logger(config).unwrap();

                      // Log some messages
                      trace!("Trace message");
                      debug!("Debug message");
                      info!("Info message");
                      warn!("Warn message");
                      error!("Error message");

                      // Check if the log file was created
                      let log_content = std::fs::read_to_string(&log_path).unwrap();
                      assert!(!log_content.is_empty());

                      // Check that the messages are present (excluding trace since default level is Info)
                      assert!(!log_content.contains("Trace message"));
                      assert!(log_content.contains("Debug message"));
                      assert!(log_content.contains("Info message"));
                      assert!(log_content.contains("Warn message"));
                      assert!(log_content.contains("Error message"));

                      // Check that timestamps and modules are present
                      assert!(log_content.contains('[')); // Timestamp or level bracket
                      assert!(log_content.contains(module_path!()));
                  }

                  #[test]
                  fn test_log_macros() {
                      let config = LogConfig {
                          level: CangjieLogLevel::Trace,
                          ..LogConfig::default()
                      };

                      init_logger(config).unwrap();

                      // Test all log macros
                      trace!("Trace: {}", 123);
                      debug!("Debug: {:?}", vec![1, 2, 3]);
                      info!("Info: {} {}", "hello", "world");
                      warn!("Warn: {:?}", std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
                      error!("Error: {}", format!("formatted message"));

                      // Just ensuring they don't panic
                      assert!(true);
                  }

                  #[test]
                  fn test_log_result_macro() {
                      let config = LogConfig {
                          level: CangjieLogLevel::Debug,
                          ..LogConfig::default()
                      };

                      init_logger(config).unwrap();

                      // Test successful result
                      let result = Ok("success");
                      let logged = log_result!(result, "Test success");
                      assert_eq!(logged, Ok("success"));

                      // Test error result
                      let result = Err(std::io::Error::new(std::io::ErrorKind::NotFound, "test error"));
                      let logged = log_result!(result, "Test error");
                      assert!(logged.is_err());
                  }
              }
              ```

              ### 54. src/utils/error.rs（补充完整）
              ```rust
              //! Error handling utilities for Cangjie extension
              //!
              //! Provides custom error types and utility functions for consistent error handling.

              use zed_extension_api::{self as zed, Result};
              use std::fmt;
              use std::error::Error;

              /// Custom error type for Cangjie extension
              #[derive(Debug)]
              pub enum CangjieError {
                  /// Internal error (should not be exposed to users)
                  Internal(String),
                  /// User-facing error (should be friendly and actionable)
                  User(String),
                  /// Syntax error (related to Cangjie language syntax)
                  Syntax {
                      message: String,
                      line: Option<u32>,
                      column: Option<u32>,
                  },
                  /// IO error (file system operations)
                  Io(std::io::Error),
                  /// JSON serialization/deserialization error
                  Json(serde_json::Error),
                  /// TOML serialization/deserialization error
                  Toml(toml::de::Error),
                  /// Tree-sitter error
                  TreeSitter(String),
                  /// LSP protocol error
                  Lsp(String),
                  /// Configuration error
                  Config(String),
              }

              impl fmt::Display for CangjieError {
                  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                      match self {
                          CangjieError::Internal(msg) => write!(f, "Internal error: {}", msg),
                          CangjieError::User(msg) => write!(f, "{}", msg),
                          CangjieError::Syntax { message, line, column } => {
                              if let (Some(line), Some(column)) = (line, column) {
                                  write!(f, "Syntax error at line {}:{}) {}", line, column, message)
                              } else {
                                  write!(f, "Syntax error: {}", message)
                              }
                          }
                          CangjieError::Io(err) => write!(f, "IO error: {}", err),
                          CangjieError::Json(err) => write!(f, "JSON error: {}", err),
                          CangjieError::Toml(err) => write!(f, "TOML error: {}", err),
                          CangjieError::TreeSitter(msg) => write!(f, "Tree-sitter error: {}", msg),
                          CangjieError::Lsp(msg) => write!(f, "LSP error: {}", msg),
                          CangjieError::Config(msg) => write!(f, "Configuration error: {}", msg),
                      }
                  }
              }

              impl Error for CangjieError {}

              impl From<std::io::Error> for CangjieError {
                  fn from(err: std::io::Error) -> Self {
                      CangjieError::Io(err)
                  }
              }

              impl From<serde_json::Error> for CangjieError {
                  fn from(err: serde_json::Error) -> Self {
                      CangjieError::Json(err)
                  }
              }

              impl From<toml::de::Error> for CangjieError {
                  fn from(err: toml::de::Error) -> Self {
                      CangjieError::Toml(err)
                  }
              }

              impl From<tree_sitter::LanguageError> for CangjieError {
                  fn from(err: tree_sitter::LanguageError) -> Self {
                      CangjieError::TreeSitter(err.to_string())
                  }
              }

              impl From<tree_sitter::QueryError> for CangjieError {
                  fn from(err: tree_sitter::QueryError) -> Self {
                      CangjieError::TreeSitter(format!("Query error: {:?}", err))
                  }
              }

              impl From<CangjieError> for zed::Error {
                  fn from(err: CangjieError) -> Self {
                      match err {
                          CangjieError::Internal(msg) => zed::Error::internal(msg),
                          CangjieError::User(msg) => zed::Error::user(msg),
                          _ => zed::Error::internal(err.to_string()),
                      }
                  }
              }

              /// Create an internal error (for developer-facing issues)
              pub fn internal_error(message: &str) -> CangjieError {
                  CangjieError::Internal(message.to_string())
              }

              /// Create a user error (for user-facing issues)
              pub fn user_error(message: &str) -> CangjieError {
                  CangjieError::User(message.to_string())
              }

              /// Create a syntax error
              pub fn syntax_error(message: &str, line: Option<u32>, column: Option<u32>) -> CangjieError {
                  CangjieError::Syntax {
                      message: message.to_string(),
                      line,
                      column,
                  }
              }

              /// Create a configuration error
              pub fn config_error(message: &str) -> CangjieError {
                  CangjieError::Config(message.to_string())
              }

              /// Create an LSP error
              pub fn lsp_error(message: &str) -> CangjieError {
                  CangjieError::Lsp(message.to_string())
              }

              /// Convert a result with CangjieError to a Zed Result
              pub fn to_zed_result<T>(result: std::result::Result<T, CangjieError>) -> Result<T> {
                  result.map_err(|err| err.into())
              }

              /// Utility macro to convert errors to internal errors with context
              #[macro_export]
              macro_rules! internal_err {
                  ($expr:expr, $msg:expr) => {
                      $expr.map_err(|err| $crate::utils::error::internal_error(&format!("{}: {}", $msg, err)))
                  };
              }

              /// Utility macro to convert errors to user errors with context
              #[macro_export]
              macro_rules! user_err {
                  ($expr:expr, $msg:expr) => {
                      $expr.map_err(|err| $crate::utils::error::user_error(&format!("{}: {}", $msg, err)))
                  };
              }

              /// Utility macro to wrap a result and convert errors
              #[macro_export]
              macro_rules! wrap_result {
                  ($expr:expr) => {
                      $expr.map_err(|err| $crate::utils::error::CangjieError::from(err))
                  };
              }

              /// Utility macro to handle optional values with error messages
              #[macro_export]
              macro_rules! require_some {
                  ($option:expr, $msg:expr) => {
                      match $option {
                          Some(value) => value,
                          None => return Err($crate::utils::error::user_error($msg)),
                      }
                  };
              }

              #[cfg(test)]
              mod tests {
                  use super::*;

                  #[test]
                  fn test_error_display() {
                      let internal = internal_error("something broke");
                      assert_eq!(format!("{}", internal), "Internal error: something broke");

                      let user = user_error("invalid input");
                      assert_eq!(format!("{}", user), "invalid input");

                      let syntax = syntax_error("missing semicolon", Some(5), Some(10));
                      assert_eq!(format!("{}", syntax), "Syntax error at line 5:10) missing semicolon");

                      let syntax_no_pos = syntax_error("invalid token", None, None);
                      assert_eq!(format!("{}", syntax_no_pos), "Syntax error: invalid token");

                      let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
                      let io = CangjieError::from(io_err);
                      assert_eq!(format!("{}", io), "IO error: file not found");
                  }

                  #[test]
                  fn test_error_conversion() {
                      // Test converting to Zed error
                      let internal = internal_error("test");
                      let zed_err: zed::Error = internal.into();
                      assert!(matches!(zed_err, zed::Error::Internal(_)));

                      let user = user_error("test");
                      let zed_err: zed::Error = user.into();
                      assert!(matches!(zed_err, zed::Error::User(_)));

                      // Test converting from other error types
                      let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "no permission");
                      let cangjie_err = CangjieError::from(io_err);
                      assert!(matches!(cangjie_err, CangjieError::Io(_)));

                      let json_err = serde_json::from_str::<()>("invalid json").unwrap_err();
                      let cangjie_err = CangjieError::from(json_err);
                      assert!(matches!(cangjie_err, CangjieError::Json(_)));
                  }

                  #[test]
                  fn test_macros() {
                      // Test internal_err macro
                      let result: std::result::Result<(), std::io::Error> = Err(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
                      let wrapped = internal_err!(result, "Failed to read file");
                      assert!(matches!(wrapped, Err(CangjieError::Internal(_))));

                      // Test user_err macro
                      let wrapped = user_err!(result, "File not found");
                      assert!(matches!(wrapped, Err(CangjieError::User(_))));

                      // Test require_some macro
                      let some_val = Some(42);
                      let val = require_some!(some_val, "Value is required");
                      assert_eq!(val, 42);

                      let none_val: Option<i32> = None;
                      let result = std::result::Result::Ok(require_some!(none_val, "Value is required"));
                      assert!(matches!(result, Err(CangjieError::User(_))));
                  }

                  #[test]
                  fn test_to_zed_result() {
                      let ok_result = Ok("success");
                      let zed_ok = to_zed_result(ok_result);
                      assert!(zed_ok.is_ok());

                      let err_result = Err(internal_error("test"));
                      let zed_err = to_zed_result(err_result);
                      assert!(zed_err.is_err());
                      assert!(matches!(zed_err.unwrap_err(), zed::Error::Internal(_)));
                  }
              }
              ```

              ### 55. src/utils/tree_sitter.rs（补充完整）
              ```rust
              //! Tree-sitter utility functions for Cangjie extension
              //!
              //! Provides helper functions for working with Tree-sitter parse trees and nodes.

              use zed_extension_api::{self as zed, Result, lsp::{Range, Position}};
              use tree_sitter::{Node, Tree, Query, QueryCursor, QueryMatch, QueryCapture};
              use std::collections::{HashMap, VecDeque};

              /// Convert a Tree-sitter Point to LSP Position
              pub fn point_to_position(point: tree_sitter::Point) -> Position {
                  Position {
                      line: point.row as u32,
                      character: point.column as u32,
                  }
              }

              /// Convert a Tree-sitter Range to LSP Range
              pub fn node_to_range(node: &Node) -> Range {
                  Range {
                      start: point_to_position(node.start_position()),
                      end: point_to_position(node.end_position()),
                  }
              }

              /// Get the node at the given LSP position
              pub fn node_at_position(root: Node, position: &Position) -> Result<Node> {
                  let point = tree_sitter::Point {
                      row: position.line as usize,
                      column: position.character as usize,
                  };

                  let mut cursor = tree_sitter::TreeCursor::new(root);
                  let mut current_node = root;

                  loop {
                      if current_node.range().contains_point(point) {
                          // Check if any child contains the point
                          let mut has_child_containing_point = false;

                          if cursor.goto_first_child() {
                              loop {
                                  let child = cursor.node();
                                  if child.range().contains_point(point) {
                                      current_node = child;
                                      has_child_containing_point = true;
                                      break;
                                  }

                                  if !cursor.goto_next_sibling() {
                                      break;
                                  }
                              }
                              cursor.goto_parent()?;
                          }

                          if !has_child_containing_point {
                              return Ok(current_node);
                          }
                      } else {
                          // Move to next sibling if current node doesn't contain the point
                          if !cursor.goto_next_sibling() {
                              break;
                          }
                      }
                  }

                  Ok(current_node)
              }

              /// Get the text of a node from the document content
              pub trait NodeText {
                  fn text(&self, content: &str) -> Result<String>;
              }

              impl NodeText for Node {
                  fn text(&self, content: &str) -> Result<String> {
                      let start = self.start_byte();
                      let end = self.end_byte();

                      let text = content
                          .as_bytes()
                          .get(start..end)
                          .ok_or_else(|| zed::Error::internal("Node byte range out of bounds"))?;

                      String::from_utf8(text.to_vec())
                          .map_err(|_| zed::Error::internal("Invalid UTF-8 in node text"))
                  }
              }

              /// Check if a Tree-sitter tree is still valid for the given content
              pub fn is_tree_valid(tree: &Tree, content: &str) -> bool {
                  // Simple validity check: tree's root node covers the entire content
                  let root = tree.root_node();
                  root.end_byte() == content.as_bytes().len()
              }

              /// Find all nodes matching a Tree-sitter query
              pub fn query_nodes(
                  root: Node,
                  content: &str,
                  query: &str,
              ) -> Result<Vec<(QueryMatch, Vec<QueryCapture>)>> {
                  let language = tree_sitter_cangjie::language();
                  let query = Query::new(language, query)?;
                  let mut cursor = QueryCursor::new();

                  let matches = cursor
                      .matches(&query, root, content.as_bytes())
                      .map(|(match_, captures)| (match_, captures.to_vec()))
                      .collect();

                  Ok(matches)
              }

              /// Find all nodes of a specific kind in the parse tree
              pub fn find_nodes_by_kind(root: Node, kind: &str) -> Vec<Node> {
                  let mut nodes = Vec::new();
                  let mut cursor = tree_sitter::TreeCursor::new(root);

                  search_nodes_by_kind(&mut cursor, kind, &mut nodes).ok();
                  nodes
              }

              /// Recursive helper to find nodes by kind
              fn search_nodes_by_kind(
                  cursor: &mut tree_sitter::TreeCursor,
                  kind: &str,
                  nodes: &mut Vec<Node>,
              ) -> Result<()> {
                  let node = cursor.node();

                  if node.kind() == kind {
                      nodes.push(node);
                  }

                  // Recurse into children
                  if cursor.goto_first_child() {
                      loop {
                          search_nodes_by_kind(cursor, kind, nodes)?;

                          if !cursor.goto_next_sibling() {
                              break;
                          }
                      }
                      cursor.goto_parent()?;
                  }

                  Ok(())
              }

              /// Get the parent node of a specific kind
              pub fn get_parent_of_kind(node: &Node, kind: &str) -> Option<Node> {
                  let mut current = node.parent()?;

                  while !current.is_null() {
                      if current.kind() == kind {
                          return Some(current);
                      }
                      current = current.parent()?;
                  }

                  None
              }

              /// Get all ancestor nodes of a specific kind
              pub fn get_ancestors_of_kind(node: &Node, kind: &str) -> Vec<Node> {
                  let mut ancestors = Vec::new();
                  let mut current = node.parent();

                  while let Some(parent) = current {
                      if parent.kind() == kind {
                          ancestors.push(parent);
                      }
                      current = parent.parent();
                  }

                  ancestors
              }

              /// Get the first child node of a specific kind
              pub fn get_child_of_kind(node: &Node, kind: &str) -> Option<Node> {
                  let mut cursor = tree_sitter::TreeCursor::new(*node);

                  if cursor.goto_first_child() {
                      loop {
                          let child = cursor.node();
                          if child.kind() == kind {
                              return Some(child);
                          }

                          if !cursor.goto_next_sibling() {
                              break;
                          }
                      }
                      cursor.goto_parent().ok();
                  }

                  None
              }

              /// Get all child nodes of a specific kind
              pub fn get_children_of_kind(node: &Node, kind: &str) -> Vec<Node> {
                  let mut children = Vec::new();
                  let mut cursor = tree_sitter::TreeCursor::new(*node);

                  if cursor.goto_first_child() {
                      loop {
                          let child = cursor.node();
                          if child.kind() == kind {
                              children.push(child);
                          }

                          if !cursor.goto_next_sibling() {
                              break;
                          }
                      }
                      cursor.goto_parent().ok();
                  }

                  children
              }

              /// Get the next sibling node of a specific kind
              pub fn get_next_sibling_of_kind(node: &Node, kind: &str) -> Option<Node> {
                  let mut cursor = tree_sitter::TreeCursor::new(*node);

                  if cursor.goto_next_sibling() {
                      loop {
                          let sibling = cursor.node();
                          if sibling.kind() == kind {
                              return Some(sibling);
                          }

                          if !cursor.goto_next_sibling() {
                              break;
                          }
                      }
                  }

                  None
              }

              /// Get the previous sibling node of a specific kind
              pub fn get_previous_sibling_of_kind(node: &Node, kind: &str) -> Option<Node> {
                  let mut cursor = tree_sitter::TreeCursor::new(*node);

                  if cursor.goto_previous_sibling() {
                      loop {
                          let sibling = cursor.node();
                          if sibling.kind() == kind {
                              return Some(sibling);
                          }

                          if !cursor.goto_previous_sibling() {
                              break;
                          }
                      }
                  }

                  None
              }

              /// Get the field value of a node by field name
              pub fn get_node_field(node: &Node, field_name: &str) -> Option<Node> {
                  let mut cursor = tree_sitter::TreeCursor::new(*node);

                  if cursor.goto_first_child() {
                      loop {
                          let child = cursor.node();
                          if let Some(name) = child.field_name() {
                              if name == field_name {
                                  return Some(child);
                              }
                          }

                          if !cursor.goto_next_sibling() {
                              break;
                          }
                      }
                      cursor.goto_parent().ok();
                  }

                  None
              }

              /// Get all field values of a node as a map
              pub fn get_node_fields(node: &Node) -> HashMap<String, Node> {
                  let mut fields = HashMap::new();
                  let mut cursor = tree_sitter::TreeCursor::new(*node);

                  if cursor.goto_first_child() {
                      loop {
                          let child = cursor.node();
                          if let Some(name) = child.field_name() {
                              fields.insert(name.to_string(), child);
                          }

                          if !cursor.goto_next_sibling() {
                              break;
                          }
                      }
                      cursor.goto_parent().ok();
                  }

                  fields
              }

              /// Traverse the parse tree in depth-first order
              pub fn traverse_tree<F>(root: Node, mut callback: F) -> Result<()>
              where
                  F: FnMut(Node) -> Result<()>,
              {
                  let mut cursor = tree_sitter::TreeCursor::new(root);

                  callback(root)?;

                  if cursor.goto_first_child() {
                      loop {
                          traverse_tree(cursor.node(), &mut callback)?;

                          if !cursor.goto_next_sibling() {
                              break;
                          }
                      }
                      cursor.goto_parent()?;
                  }

                  Ok(())
              }

              /// Traverse the parse tree in breadth-first order
              pub fn traverse_tree_bfs<F>(root: Node, mut callback: F) -> Result<()>
              where
                  F: FnMut(Node) -> Result<()>,
              {
                  let mut queue = VecDeque::new();
                  queue.push_back(root);

                  while let Some(node) = queue.pop_front() {
                      callback(node)?;

                      // Add children to queue
                      let mut cursor = tree_sitter::TreeCursor::new(node);
                      if cursor.goto_first_child() {
                          loop {
                              queue.push_back(cursor.node());

                              if !cursor.goto_next_sibling() {
                                  break;
                              }
                          }
                          cursor.goto_parent().ok();
                      }
                  }

                  Ok(())
              }

              /// Get the depth of a node in the parse tree
              pub fn node_depth(node: &Node) -> usize {
                  let mut depth = 0;
                  let mut current = node.parent();

                  while let Some(parent) = current {
                      depth += 1;
                      current = parent.parent();
                  }

                  depth
              }

              /// Check if a node is a descendant of another node
              pub fn is_descendant_of(child: &Node, ancestor: &Node) -> bool {
                  let mut current = child.parent();

                  while let Some(parent) = current {
                      if parent.id() == ancestor.id() {
                          return true;
                      }
                      current = parent.parent();
                  }

                  false
              }

              /// Get the common ancestor of two nodes
              pub fn common_ancestor(node1: &Node, node2: &Node) -> Option<Node> {
                  // Collect all ancestors of node1
                  let mut ancestors = HashMap::new();
                  let mut current = Some(*node1);

                  while let Some(node) = current {
                      ancestors.insert(node.id(), node);
                      current = node.parent();
                  }

                  // Check ancestors of node2
                  current = Some(*node2);
                  while let Some(node) = current {
                      if ancestors.contains_key(&node.id()) {
                          return Some(node);
                      }
                      current = node.parent();
                  }

                  None
              }

              #[cfg(test)]
              mod tests {
                  use super::*;
                  use tree_sitter::Parser;

                  #[test]
                  fn test_node_to_range() {
                      let parser = Parser::new();
                      parser.set_language(tree_sitter_cangjie::language()).unwrap();

                      let content = "let x = 42;";
                      let tree = parser.parse(content, None).unwrap();
                      let root = tree.root_node();

                      let node = root.child(0).unwrap();
                      let range = node_to_range(&node);

                      assert_eq!(range.start.line, 0);
                      assert_eq!(range.start.character, 0);
                      assert_eq!(range.end.line, 0);
                      assert_eq!(range.end.character, 9);
                  }

                  #[test]
                  fn test_node_at_position() {
                      let parser = Parser::new();
                      parser.set_language(tree_sitter_cangjie::language()).unwrap();

                      let content = "fn add(a: i32, b: i32) -> i32 { a + b }";
                      let tree = parser.parse(content, None).unwrap();
                      let root = tree.root_node();

                      // Position at 'add' (function name)
                      let position = Position { line: 0, character: 3 };
                      let node = node_at_position(root, &position).unwrap();
                      assert_eq!(node.kind(), "identifier");
                      assert_eq!(node.text(content).unwrap(), "add");

                      // Position at 'a' (parameter)
                      let position = Position { line: 0, character: 6 };
                      let node = node_at_position(root, &position).unwrap();
                      assert_eq!(node.kind(), "identifier");
                      assert_eq!(node.text(content).unwrap(), "a");

                      // Position at '+' (operator)
                      let position = Position { line: 0, character: 26 };
                      let node = node_at_position(root, &position).unwrap();
                      assert_eq!(node.kind(), "arithmetic_operator");
                      assert_eq!(node.text(content).unwrap(), "+");
                  }

                  #[test]
                  fn test_find_nodes_by_kind() {
                      let parser = Parser::new();
                      parser.set_language(tree_sitter_cangjie::language()).unwrap();

                      let content = "let x = 42; let y = 100; const Z = 200;";
                      let tree = parser.parse(content, None).unwrap();
                      let root = tree.root_node();

                      let identifier_nodes = find_nodes_by_kind(root, "identifier");
                      assert_eq!(identifier_nodes.len(), 3);
                      assert_eq!(identifier_nodes[0].text(content).unwrap(), "x");
                      assert_eq!(identifier_nodes[1].text(content).unwrap(), "y");
                      assert_eq!(identifier_nodes[2].text(content).unwrap(), "Z");

                      let variable_nodes = find_nodes_by_kind(root, "variable_declaration");
                      assert_eq!(variable_nodes.len(), 2);

                      let constant_nodes = find_nodes_by_kind(root, "constant_declaration");
                      assert_eq!(constant_nodes.len(), 1);
                  }

                  #[test]
                  fn test_get_parent_of_kind() {
                      let parser = Parser::new();
                      parser.set_language(tree_sitter_cangjie::language()).unwrap();

                      let content = "fn add(a: i32, b: i32) -> i32 { a + b }";
                      let tree = parser.parse(content, None).unwrap();
                      let root = tree.root_node();

                      // Find the 'a' identifier in the parameters
                      let identifier_nodes = find_nodes_by_kind(root, "identifier");
                      let a_node = identifier_nodes.iter()
                          .find(|n| n.text(content).unwrap() == "a")
                          .unwrap();

                      // Get the parameter declaration parent
                      let param_parent = get_parent_of_kind(a_node, "parameter_declaration");
                      assert!(param_parent.is_some());
                      assert_eq!(param_parent.unwrap().kind(), "parameter_declaration");

                      // Get the function declaration ancestor
                      let function_parent = get_parent_of_kind(a_node, "function_declaration");
                      assert!(function_parent.is_some());
                      assert_eq!(function_parent.unwrap().kind(), "function_declaration");
                  }

                  #[test]
                  fn test_traverse_tree() {
                      let parser = Parser::new();
                      parser.set_language(tree_sitter_cangjie::language()).unwrap();

                      let content = "let x = 42;";
                      let tree = parser.parse(content, None).unwrap();
                      let root = tree.root_node();

                      let mut nodes = Vec::new();
                      traverse_tree(root, |node| {
                          nodes.push(node.kind().to_string());
                          Ok(())
                      }).unwrap();

                      assert!(nodes.contains(&"program".to_string()));
                      assert!(nodes.contains(&"variable_declaration".to_string()));
                      assert!(nodes.contains(&"identifier".to_string()));
                      assert!(nodes.contains(&"number_literal".to_string()));
                  }

                  #[test]
                  fn test_node_depth() {
                      let parser = Parser::new();
                      parser.set_language(tree_sitter_cangjie::language()).unwrap();

                      let content = "fn add(a: i32, b: i32) -> i32 { a + b }";
                      let tree = parser.parse(content, None).unwrap();
                      let root = tree.root_node();

                      assert_eq!(node_depth(&root), 0);

                      let function_node = get_child_of_kind(&root, "function_declaration").unwrap();
                      assert_eq!(node_depth(&function_node), 1);

                      let name_node = get_child_of_kind(&function_node, "identifier").unwrap();
                      assert_eq!(node_depth(&name_node), 2);
                  }

                  #[test]
                  fn test_is_descendant_of() {
                      let parser = Parser::new();
                      parser.set_language(tree_sitter_cangjie::language()).unwrap();

                      let content = "fn add(a: i32, b: i32) -> i32 { a + b }";
                      let tree = parser.parse(content, None).unwrap();
                      let root = tree.root_node();

                      let function_node = get_child_of_kind(&root, "function_declaration").unwrap();
                      let name_node = get_child_of_kind(&function_node, "identifier").unwrap();

                      assert!(is_descendant_of(&name_node, &function_node));
                      assert!(is_descendant_of(&name_node, &root));
                      assert!(!is_descendant_of(&function_node, &name_node));
                  }

                  #[test]
                  fn test_common_ancestor() {
                      let parser = Parser::new();
                      parser.set_language(tree_sitter_cangjie::language()).unwrap();

                      let content = "fn add(a: i32, b: i32) -> i32 { a + b }";
                      let tree = parser.parse(content, None).unwrap();
                      let root = tree.root_node();

                      let function_node = get_child_of_kind(&root, "function_declaration").unwrap();
                      let a_node = find_nodes_by_kind(root, "identifier")
                          .iter()
                          .find(|n| n.text(content).unwrap() == "a")
                          .unwrap();
                      let b_node = find_nodes_by_kind(root, "identifier")
                          .iter()
                          .find(|n| n.text(content).unwrap() == "b")
                          .unwrap();

                      let ancestor = common_ancestor(a_node, b_node).unwrap();
                      assert_eq!(ancestor.id(), function_node.id());
                  }
              }
              ```

              ---

              ## 项目完整部署与使用指南

              ### 1. 本地开发与测试

              #### 环境准备
              - Rust 1.70+（推荐使用 `rustup` 安装）
              - Zed 编辑器（最新版本）
              - Tree-sitter CLI（可选，用于调试语法解析）

              #### 构建与链接扩展
              ```bash
              # 克隆仓库
              git clone https://github.com/your-username/zed-cangjie-extension.git
              cd zed-cangjie-extension

              # 构建扩展（release 模式）
              cargo build --release

              # 链接扩展到 Zed
              zed extensions link
              ```

              #### 调试扩展
              1. 在 Zed 中打开 Cangjie 代码文件（`.cang` 或 `.cj` 后缀）
              2. 打开 Zed 的开发者工具：`View > Developer Tools`
              3. 查看扩展日志：`View > Extensions > Cangjie > Logs`

              ### 2. 发布扩展

              #### 准备发布
              1. 更新 `Cargo.toml` 中的版本号
              2. 确保 `README.md` 包含完整的使用说明
              3. 运行测试确保所有功能正常
              ```bash
              cargo test
              ```

              #### 打包扩展
              ```bash
              # 构建优化后的版本
              cargo build --release --target wasm32-unknown-unknown

              # 打包扩展（需要 Zed CLI）
              zed extensions package
              ```

              #### 发布到 Zed 扩展市场
              1. 访问 [Zed 扩展市场](https://extensions.zed.dev/)
              2. 登录你的账号
              3. 上传打包后的扩展文件（`.zed-extension`）
              4. 填写扩展信息并发布

              ### 3. 扩展使用指南

              #### 基本配置
              扩展安装后会自动启用，默认配置可满足大部分需求。如需自定义，可在 Zed 设置中添加以下配置：

              ```json
              {
                "cangjie": {
                  "formatting": {
                    "indent_style": "space",
                    "indent_size": 4,
                    "function_brace_style": "same_line",
                    "struct_brace_style": "same_line",
                    "trailing_comma": "multiline",
                    "space_around_operators": true,
                    "space_inside_brackets": false,
                    "max_line_length": 120,
                    "line_ending": "lf",
                    "auto_fix": true
                  },
                  "linting": {
                    "min_severity": "warning",
                    "ignore_rules": [],
                    "custom_rules": []
                  },
                  "completion": {
                    "trigger_characters": [".", ":", "::", "<", "("],
                    "include_workspace_symbols": true,
                    "include_snippets": true
                  },
                  "logging": {
                    "level": "info",
                    "include_timestamps": true,
                    "include_modules": true,
                    "file_logging": false
                  }
                }
              }
              ```

              #### 核心功能使用

              ##### 语法高亮
              - 自动识别 `.cang` 和 `.cj` 文件
              - 支持关键字、字符串、数字、注释等语法元素的高亮显示
              - 可通过 Zed 主题自定义颜色

              ##### 代码补全
              - 触发字符：`.`, `:`, `::`, `<`, `(`
              - 支持：
                - 文档内符号补全（变量、函数、类型等）
                - 工作区符号补全
                - 内置类型和函数补全
                - 代码片段补全（输入 `fn`、`struct` 等触发）

              ##### 代码格式化
              - 快捷键：`Cmd/Ctrl + Shift + I`
              - 支持整文件格式化和选中范围格式化
              - 可配置缩进、大括号风格、行长度等规则

              ##### 诊断（Linting）
              - 实时显示语法错误和代码风格问题
              - 支持 8 个内置规则：
                - `UNUSED_VARIABLE`：未使用的变量
                - `UNUSED_CONSTANT`：未使用的常量
                - `LINE_TOO_LONG`：行长度超过限制
                - `INVALID_NAMING_CONVENTION`：命名规范不符合
                - `MISSING_SEMICOLON`：缺少分号
                - `EMPTY_BLOCK`：空代码块
                - `UNREACHABLE_CODE`：不可达代码
                - `DEPRECATED_SYNTAX`：使用废弃语法
              - 支持自定义 lint 规则

              ##### 其他 LSP 功能
              - 悬停文档：鼠标悬停在符号上显示详细信息
              - 跳转定义：`Cmd/Ctrl + 点击` 跳转到符号定义
              - 符号搜索：`Cmd/Ctrl + P` 输入 `@` 搜索文档符号
              - 工作区符号搜索：`Cmd/Ctrl + P` 输入 `#` 搜索工作区符号

              ### 4. 贡献指南

              #### 提交代码
              1. Fork 仓库
              2. 创建特性分支：`git checkout -b feature/your-feature`
              3. 提交代码：`git commit -m "Add your feature"`
              4. 推送分支：`git push origin feature/your-feature`
              5. 创建 Pull Request

              #### 代码规范
              - 遵循 Rust 官方代码规范（使用 `rustfmt` 格式化）
              - 运行 `cargo clippy` 确保代码质量
              - 新增功能需添加对应的测试
              - 文档注释使用 Rustdoc 规范

              #### 扩展功能
              如需添加新功能，建议按以下步骤进行：
              1. 在 `src/syntax` 中添加对应的 Tree-sitter 查询（如需语法支持）
              2. 在 `src/lsp` 中实现对应的 LSP 方法
              3. 在 `src/config` 中添加相关配置项
              4. 添加测试用例
              5. 更新文档

              ### 5. 常见问题解答

              #### Q: 扩展安装后不生效？
              A:
              1. 确保 Zed 版本符合要求（最新版本）
              2. 检查扩展是否已启用：`View > Extensions > Cangjie`
              3. 重启 Zed 尝试
              4. 查看扩展日志排查问题

              #### Q: 语法高亮不正确？
              A:
              1. 检查文件后缀是否为 `.cang` 或 `.cj`
              2. 确认代码符合 Cangjie 语法规范
              3. 在 GitHub 提交 issue，附上相关代码片段

              #### Q: 如何添加自定义 lint 规则？
              A:
              在 Zed 配置中添加：
              ```json
              {
                "cangjie": {
                  "linting": {
                    "custom_rules": [
                      {
                        "rule_id": "CUSTOM_RULE",
                        "description": "Custom lint rule",
                        "severity": "warning",
                        "query": "(identifier) @custom_node",
                        "message": "Custom rule triggered for {{node_text}}",
                        "documentation": "This is a custom lint rule",
                        "fix": "fixed_{{node_text}}"
                      }
                    ]
                  }
                }
              }
              ```

              #### Q: 如何调试 Tree-sitter 语法解析？
              A:
              1. 安装 Tree-sitter CLI：`npm install -g tree-s

### 5. 常见问题解答（续）

#### Q: 如何调试 Tree-sitter 语法解析？
A:
1. 安装 Tree-sitter CLI：`npm install -g tree-sitter-cli`
2. 编写测试用例文件（例如 `test.cang`）
3. 运行解析命令查看语法树：
   ```bash
   tree-sitter parse test.cang --debug
   ```
4. 检查节点结构是否符合预期
5. 如需修改语法，编辑 `tree-sitter-cangjie` 语法库（需关联本地依赖）

#### Q: 扩展性能较差，编辑大文件时卡顿？
A:
1. 检查日志确认是否有耗时操作：`View > Extensions > Cangjie > Logs`
2. 调整日志级别为 `warn` 减少性能开销：
   ```json
   {
     "cangjie": {
       "logging": {
         "level": "warn"
       }
     }
   }
   ```
3. 关闭不必要的 lint 规则：
   ```json
   {
     "cangjie": {
       "linting": {
         "ignore_rules": ["LINE_TOO_LONG", "INVALID_NAMING_CONVENTION"]
       }
     }
   }
   ```
4. 大文件建议关闭工作区符号补全：
   ```json
   {
     "cangjie": {
       "completion": {
         "include_workspace_symbols": false
       }
     }
   }
   ```

#### Q: 代码格式化不符合预期？
A:
1. 检查格式化配置是否正确（参考「基本配置」章节）
2. 尝试调整大括号风格、缩进等关键配置
3. 如仍有问题，提交 issue 并附上：
   - 原始代码
   - 预期格式化结果
   - 实际格式化结果
   - 当前配置

#### Q: 无法跳转到定义/悬停无文档？
A:
1. 确认符号是否有明确的定义（变量、函数等必须先声明）
2. 检查文件是否被正确解析（查看日志是否有语法错误）
3. 对于工作区符号，确保文件后缀正确且已被 Zed 索引
4. 重启 Zed 刷新符号索引

### 6. 扩展架构与扩展指南

#### 架构概览
Cangjie 扩展采用模块化架构，核心分为 5 大模块：
```
src/
├── config/          # 配置系统（序列化、默认配置）
├── lsp/             # LSP 核心实现（hover、补全、格式化等）
├── lint/            # 代码检查（内置规则、自定义规则）
├── syntax/          # 语法支持（高亮、代码片段、Tree-sitter 查询）
└── utils/           # 工具类（字符串、文件、日志、错误处理等）
```

#### 扩展新 LSP 功能
以添加「引用搜索」（References）功能为例：

1. **创建 LSP 实现文件**：`src/lsp/references.rs`
```rust
//! 引用搜索功能实现
use super::super::{syntax::tree_sitter_utils, utils::log::debug};
use zed_extension_api::{self as zed, Result, lsp::{ReferenceParams, Location, Url}};
use tree_sitter::Node;

pub fn get_references(
    document: &zed::Document,
    tree: &tree_sitter::Tree,
    params: &ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    let content = document.text();
    let position = &params.text_document_position.position;

    // 1. 获取当前位置的节点
    let root = tree.root_node();
    let target_node = tree_sitter_utils::node_at_position(root, position)?;
    let target_name = target_node.text(content)?;
    let target_kind = target_node.kind();

    debug!("Searching references for '{}' (kind: {})", target_name, target_kind);

    // 2. 过滤可搜索的符号类型（变量、函数、类型等）
    let search_kinds = ["identifier", "type_identifier", "struct_identifier", "enum_identifier"];
    if !search_kinds.contains(&target_kind) {
        return Ok(None);
    }

    // 3. 查找文档内所有引用
    let mut references = Vec::new();
    tree_sitter_utils::traverse_tree(root, |node| {
        if node.kind() == target_kind && node.text(content)? == target_name {
            references.push(Location {
                uri: document.uri().clone(),
                range: tree_sitter_utils::node_to_range(&node),
            });
        }
        Ok(())
    })?;

    // 4. 查找工作区其他文件的引用（可选）
    let workspace = document.workspace()?;
    let other_files = super::super::utils::file::find_workspace_files(&workspace, "*.{cang,cj}")?;
    for file_path in other_files {
        if file_path == document.path()? {
            continue;
        }

        let file_content = super::super::utils::file::read_file_to_string(&file_path)?;
        let file_tree = tree_sitter_cangjie::parse(&file_content, None)?;
        let file_root = file_tree.root_node();

        tree_sitter_utils::traverse_tree(file_root, |node| {
            if node.kind() == target_kind && node.text(&file_content)? == target_name {
                references.push(Location {
                    uri: Url::from_file_path(&file_path).map_err(|_| {
                        zed::Error::internal("Invalid file path")
                    })?,
                    range: tree_sitter_utils::node_to_range(&node),
                });
            }
            Ok(())
        })?;
    }

    debug!("Found {} references for '{}'", references.len(), target_name);
    Ok(Some(references))
}
```

2. **注册 LSP 方法**：在 `src/lsp/server.rs` 中实现 `references` 方法
```rust
impl zed::LanguageServer for CangjieLspServer {
    // ... 其他方法 ...

    fn references(&mut self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let document = self.workspace.document(&uri)?;
        let tree = self.parse_document(&document)?;

        debug!("Handling references request for {:?} at {:?}", uri, params.text_document_position.position);

        super::references::get_references(&document, &tree, &params)
    }
}
```

3. **启用 LSP 能力**：在 `initialize` 方法中添加能力声明
```rust
fn initialize(&mut self, _params: InitializeParams) -> Result<InitializeResult> {
    Ok(InitializeResult {
        capabilities: ServerCapabilities {
            // ... 其他能力 ...
            references_provider: Some(ReferencesProviderCapability::Simple(true)),
            ..ServerCapabilities::default()
        },
        // ... 其他配置 ...
    })
}
```

4. **添加测试用例**：创建 `src/lsp/references_test.rs`
5. **更新文档**：在使用指南中添加「引用搜索」功能说明

#### 添加自定义语法高亮
1. **编辑 Tree-sitter 查询文件**：`src/syntax/highlights.scm`
```scheme
; 新增自定义关键字高亮
(keyword "async" @keyword.async)
(keyword "await" @keyword.async)

; 新增装饰器高亮
(decorator "@component" @keyword.decorator)
(decorator "@route" @keyword.decorator)

; 新增字面量高亮
(date_literal) @literal.date
(time_literal) @literal.time
```

2. **关联 Zed 高亮组**：在 `package.json` 中添加语法作用域映射
```json
{
  "contributes": {
    "grammars": [
      {
        "language": "cangjie",
        "scopeName": "source.cangjie",
        "path": "./src/syntax/tree-sitter-cangjie/grammar.js",
        "highlights": "./src/syntax/highlights.scm"
      }
    ],
    "semanticTokenTypes": [
      {
        "id": "keyword.async",
        "description": "Async/await keywords"
      },
      {
        "id": "keyword.decorator",
        "description": "Decorator keywords"
      },
      {
        "id": "literal.date",
        "description": "Date literals"
      },
      {
        "id": "literal.time",
        "description": "Time literals"
      }
    ]
  }
}
```

3. **测试高亮效果**：
   - 创建包含新语法元素的 Cangjie 文件
   - 在 Zed 中打开，确认高亮颜色符合预期
   - 如需调整颜色，可在 Zed 主题中自定义对应作用域的颜色

#### 添加代码片段
1. **创建代码片段文件**：`src/syntax/snippets.json`
```json
{
  "snippets": [
    {
      "trigger": "async_fn",
      "description": "Async function declaration",
      "body": [
        "async fn ${1:function_name}(${2:params}) -> ${3:ReturnType} {",
        "  ${0:// async code}",
        "}"
      ],
      "scope": "source.cangjie"
    },
    {
      "trigger": "component",
      "description": "Component declaration",
      "body": [
        "@component",
        "struct ${1:ComponentName} {",
        "  ${2:// props}",
        "}",
        "",
        "impl ${1:ComponentName} {",
        "  pub fn render(&self) -> Element {",
        "    ${0:// render code}",
        "  }",
        "}"
      ],
      "scope": "source.cangjie"
    }
  ]
}
```

2. **注册代码片段**：在 `package.json` 中添加配置
```json
{
  "contributes": {
    "snippets": [
      {
        "language": "cangjie",
        "path": "./src/syntax/snippets.json"
      }
    ]
  }
}
```

3. **测试代码片段**：
   - 在 Cangjie 文件中输入触发词（如 `async_fn`）
   - 按 Tab 键展开片段
   - 确认光标位置和占位符符合预期

### 7. 性能优化指南

#### 解析性能优化
1. **缓存解析树**：扩展已实现文档解析树缓存，避免重复解析（`src/lsp/server.rs`）
2. **增量解析**：对于大文件，可实现 Tree-sitter 增量解析（基于文档变更范围）
```rust
// 示例：增量解析实现
fn incremental_parse(
    parser: &mut tree_sitter::Parser,
    old_tree: &tree_sitter::Tree,
    new_content: &str,
    changes: &[zed::lsp::TextDocumentContentChangeEvent],
) -> Result<tree_sitter::Tree> {
    let mut input = tree_sitter::InputEdit::default();
    // 转换 LSP 变更事件为 Tree-sitter InputEdit
    for change in changes {
        let range = &change.range;
        input.start_byte = 0; // 需根据实际偏移计算
        input.old_end_byte = 0;
        input.new_end_byte = change.text.as_bytes().len();
        input.start_point = tree_sitter::Point {
            row: range.start.line as usize,
            column: range.start.character as usize,
        };
        input.old_end_point = tree_sitter::Point {
            row: range.end.line as usize,
            column: range.end.character as usize,
        };
        input.new_end_point = tree_sitter::Point {
            row: range.start.line as usize + change.text.lines().count() - 1,
            column: if change.text.lines().count() == 1 {
                range.start.character as usize + change.text.len()
            } else {
                change.text.lines().last().unwrap().len()
            },
        };
    }
    let new_tree = old_tree.edit(&input);
    Ok(parser.parse(new_content, Some(&new_tree))
        .ok_or_else(|| zed::Error::internal("Incremental parse failed"))?)
}
```

3. **限制解析范围**：对于超大型文件（10k+ 行），可仅解析当前可视范围

#### LSP 性能优化
1. **懒加载功能**：非核心功能（如工作区符号搜索）可延迟初始化
2. **请求节流**：对于高频触发的请求（如补全），添加节流机制
```rust
// 示例：补全请求节流
use std::time::Instant;

struct CompletionThrottle {
    last_request_time: Option<Instant>,
    throttle_duration: std::time::Duration,
}

impl CompletionThrottle {
    fn new(throttle_duration: std::time::Duration) -> Self {
        Self {
            last_request_time: None,
            throttle_duration,
        }
    }

    fn can_request(&mut self) -> bool {
        let now = Instant::now();
        match self.last_request_time {
            None => {
                self.last_request_time = Some(now);
                true
            }
            Some(last) => {
                if now.duration_since(last) >= self.throttle_duration {
                    self.last_request_time = Some(now);
                    true
                } else {
                    false
                }
            }
        }
    }
}
```

3. **结果缓存**：缓存频繁访问的结果（如文档符号、悬停文档）
4. **异步处理**：耗时操作（如工作区搜索）使用异步线程处理，避免阻塞主线程

#### Lint 性能优化
1. **增量 lint**：仅对变更的代码块重新运行 lint，而非整个文档
2. **规则分级**：将 lint 规则分为「快速规则」和「慢速规则」，快速规则实时运行，慢速规则后台运行
3. **并行 lint**：多文件 lint 时使用并行处理（需注意线程安全）

### 8. 未来功能规划

以下是 Cangjie 扩展的未来功能 roadmap，欢迎社区贡献：

| 功能 | 状态 | 优先级 |
|------|------|--------|
| 代码重构（Extract Function/Variable） | 未实现 | 中 |
| 类型检查集成 | 未实现 | 高 |
| 代码操作（Code Actions） | 部分实现 | 中 |
| 导入自动补全 | 未实现 | 中 |
| 格式化范围选择优化 | 未实现 | 低 |
| 多文件重构 | 未实现 | 低 |
| 调试器集成 | 未实现 | 低 |
| 测试运行器集成 | 未实现 | 低 |

### 9. 许可证信息

Cangjie 扩展采用 **MIT 许可证**，允许自由使用、修改和分发，具体条款见 `LICENSE` 文件。

第三方依赖许可证信息可通过以下命令查看：
```bash
cargo about generate script/licenses/zed-licenses.toml
```

### 10. 联系与反馈

- **GitHub 仓库**：https://github.com/your-username/zed-cangjie-extension
- **Issue 提交**：https://github.com/your-username/zed-cangjie-extension/issues
- **Discord 社区**：https://discord.gg/cangjie-lang（示例链接）
- **邮件反馈**：contact@cangjie-lang.org（示例邮箱）

欢迎提交 bug 报告、功能建议和代码贡献，我们会在 24-48 小时内响应。

---

至此，**Cangjie 编程语言 Zed 编辑器扩展**的完整文档和代码实现已全部完成。扩展涵盖了语法支持、LSP 核心功能、代码 linting、可配置化等关键特性，同时提供了详细的开发、部署和扩展指南，便于用户使用和社区贡献。
### 附录：补充工具与资源

#### A. 开发辅助工具

##### Tree-sitter 语法调试工具
1. **Tree-sitter Playground**（在线调试）
   - 地址：https://tree-sitter.github.io/tree-sitter-playground/
   - 用法：
     - 选择 `cangjie` 语言（需本地加载语法库）
     - 输入测试代码，实时查看语法树结构
     - 调试查询语句匹配效果

2. **本地调试脚本**：`scripts/debug_syntax.sh`
```bash
#!/bin/bash
# 语法解析调试脚本
set -e

# 依赖：tree-sitter-cli、jq
if ! command -v tree-sitter &> /dev/null; then
  echo "Error: tree-sitter-cli is not installed. Install with: npm install -g tree-sitter-cli"
  exit 1
fi

if ! command -v jq &> /dev/null; then
  echo "Error: jq is not installed. Install with: brew install jq / apt install jq"
  exit 1
fi

# 参数：测试文件路径
if [ $# -eq 0 ]; then
  echo "Usage: $0 <test-file.cang>"
  exit 1
fi

TEST_FILE=$1
if [ ! -f "$TEST_FILE" ]; then
  echo "Error: File $TEST_FILE not found"
  exit 1
fi

# 1. 解析文件并输出语法树（JSON格式）
echo "=== Parsing $TEST_FILE ==="
tree-sitter parse "$TEST_FILE" --json > /tmp/cangjie_parse.json

# 2. 格式化输出语法树
echo -e "\n=== Syntax Tree ==="
jq '.tree' /tmp/cangjie_parse.json

# 3. 检查是否有语法错误
ERRORS=$(jq '.errors | length' /tmp/cangjie_parse.json)
if [ "$ERRORS" -gt 0 ]; then
  echo -e "\n=== Syntax Errors ==="
  jq '.errors' /tmp/cangjie_parse.json
  exit 1
else
  echo -e "\n✅ No syntax errors"
fi

# 4. 运行高亮查询测试
echo -e "\n=== Highlight Test ==="
tree-sitter highlight "$TEST_FILE" --query src/syntax/highlights.scm --output-format json | jq
```

##### 扩展测试脚本
`scripts/test_extension.sh`
```bash
#!/bin/bash
# 扩展测试脚本（自动化测试、打包、部署验证）
set -e

# 1. 运行单元测试
echo "=== Running Unit Tests ==="
cargo test -- --test-threads=1

# 2. 运行 Clippy 代码检查
echo -e "\n=== Running Clippy ==="
cargo clippy --all-targets --all-features -- -D warnings

# 3. 格式化检查
echo -e "\n=== Checking Formatting ==="
cargo fmt -- --check

# 4. 构建扩展
echo -e "\n=== Building Extension ==="
cargo build --release --target wasm32-unknown-unknown

# 5. 打包扩展
echo -e "\n=== Packaging Extension ==="
zed extensions package --output ./cangjie-extension.zed-extension

# 6. 验证包完整性
echo -e "\n=== Verifying Package ==="
unzip -l ./cangjie-extension.zed-extension | grep -E "wasm|package.json|README.md"
if [ $? -ne 0 ]; then
  echo "Error: Package is missing required files"
  exit 1
fi

echo -e "\n✅ Extension test and package completed successfully"
echo "Package path: ./cangjie-extension.zed-extension"
```

#### B. 示例 Cangjie 代码

##### 1. 基础语法示例（`examples/basic.cang`）
```cangjie
// 变量声明
let message = "Hello, Cangjie!";
let count = 42;
let is_active = true;

// 常量声明
const PI = 3.14159;
const MAX_RETRIES = 5;

// 函数定义
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// 异步函数
async fn fetch_data(url: String) -> Result<String, String> {
    // 模拟异步请求
    await sleep(1000);
    if url.starts_with("https") {
        Ok("Data received: ...".to_string())
    } else {
        Err("Invalid URL: must use HTTPS".to_string())
    }
}

// 结构体定义
struct User {
    id: u64,
    name: String,
    email: Option<String>,
    created_at: DateTime,
}

// 结构体实现
impl User {
    // 构造函数
    fn new(id: u64, name: String) -> Self {
        User {
            id,
            name,
            email: None,
            created_at: now(),
        }
    }

    // 方法
    fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }

    fn get_display_name(&self) -> String {
        match &self.email {
            Some(email) => format!("{} ({})", self.name, email),
            None => self.name.clone(),
        }
    }
}

// 枚举定义
enum Status {
    Pending,
    Completed(String),
    Failed { reason: String, code: i32 },
}

// 主函数
fn main() {
    // 使用结构体
    let mut user = User::new(1, "Alice".to_string());
    user.set_email("alice@example.com".to_string());
    println!("User: {}", user.get_display_name());

    // 使用函数
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);

    // 使用枚举
    let status = Status::Completed("Task done".to_string());
    match status {
        Status::Pending => println!("Task is pending"),
        Status::Completed(msg) => println!("Task completed: {}", msg),
        Status::Failed { reason, code } => println!("Task failed ({}): {}", code, reason),
    }
}
```

##### 2. 模块化示例（`examples/module.cang`）
```cangjie
// 导入模块
import math from "./math.cang";
import { User, Status } from "./models.cang";

// 使用导入的模块
fn calculate_circle_area(radius: f64) -> f64 {
    return math::pi() * radius * radius;
}

fn main() {
    let radius = 5.0;
    let area = calculate_circle_area(radius);
    println!("Circle area (r={}): {:.2f}", radius, area);

    let user = User::new(2, "Bob".to_string());
    println!("User: {}", user.get_display_name());
}
```

##### 3. 错误处理示例（`examples/error_handling.cang`）
```cangjie
// 自定义错误类型
enum AppError {
    FileNotFound(String),
    InvalidData(String),
    NetworkError(String),
}

// 实现错误显示
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::FileNotFound(path) => write!(f, "File not found: {}", path),
            AppError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

// 读取文件函数
fn read_config(path: String) -> Result<String, AppError> {
    if !file::exists(&path) {
        return Err(AppError::FileNotFound(path));
    }

    let content = file::read_to_string(&path)
        .map_err(|_| AppError::InvalidData("Failed to read file".to_string()))?;

    if content.is_empty() {
        return Err(AppError::InvalidData("Empty config file".to_string()));
    }

    Ok(content)
}

// 使用错误处理
fn main() {
    let config_path = "config.cang";
    match read_config(config_path.to_string()) {
        Ok(content) => println!("Config loaded: {}", content),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
```

#### C. 扩展本地化指南

Cangjie 扩展支持多语言本地化，以下是添加新语言的步骤：

1. **创建本地化文件**：在 `src/locale` 目录下创建语言文件（如 `zh-CN.toml`）
```toml
# src/locale/zh-CN.toml
[error]
file_not_found = "文件未找到: {}"
invalid_syntax = "语法错误: {} (行 {}:{})"
unsupported_feature = "不支持的功能: {}"

[lint]
unused_variable = "未使用的变量: {}"
line_too_long = "行长度超过限制 (最大 {} 字符)"
invalid_naming = "命名不符合规范: {} (应为 {})"

[completion]
function_snippet = "函数声明"
struct_snippet = "结构体声明"
enum_snippet = "枚举声明"
```

2. **添加本地化加载逻辑**：`src/locale/mod.rs`
```rust
//! 本地化支持
use std::collections::HashMap;
use toml::Value;
use crate::utils::file::read_toml_file;

/// 支持的语言列表
pub const SUPPORTED_LANGUAGES: &[&str] = &["en-US", "zh-CN", "ja-JP", "ko-KR"];

/// 本地化数据结构
#[derive(Debug, Clone)]
pub struct Locale {
    data: HashMap<String, String>,
    language: String,
}

impl Locale {
    /// 加载指定语言的本地化文件
    pub fn load(language: &str) -> Result<Self, crate::utils::error::CangjieError> {
        //  fallback 到 en-US
        let lang = if SUPPORTED_LANGUAGES.contains(&language) {
            language
        } else {
            "en-US"
        };

        let path = format!("src/locale/{}.toml", lang);
        let toml_data: Value = read_toml_file(&std::path::Path::new(&path))?;

        let mut data = HashMap::new();
        flatten_toml(&toml_data, "", &mut data);

        Ok(Self {
            data,
            language: lang.to_string(),
        })
    }

    /// 获取本地化字符串
    pub fn get(&self, key: &str) -> String {
        self.data.get(key).cloned().unwrap_or_else(|| {
            crate::utils::log::warn!("Missing locale key: {}", key);
            key.to_string()
        })
    }

    /// 获取本地化字符串并格式化
    pub fn format(&self, key: &str, args: &[&str]) -> String {
        let template = self.get(key);
        self.format_template(&template, args)
    }

    /// 格式化模板字符串
    fn format_template(&self, template: &str, args: &[&str]) -> String {
        let mut result = template.to_string();
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
        result
    }
}

/// 扁平化 TOML 数据为 key-value 结构
fn flatten_toml(value: &Value, prefix: &str, data: &mut HashMap<String, String>) {
    match value {
        Value::Table(table) => {
            for (key, val) in table {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                flatten_toml(val, &new_prefix, data);
            }
        }
        Value::String(s) => {
            if !prefix.is_empty() {
                data.insert(prefix.to_string(), s.clone());
            }
        }
        _ => {}
    }
}

/// 全局本地化实例
static LOCALE: std::sync::Mutex<Option<Locale>> = std::sync::Mutex::new(None);

/// 初始化本地化
pub fn init_locale(language: &str) -> Result<(), crate::utils::error::CangjieError> {
    let locale = Locale::load(language)?;
    *LOCALE.lock()? = Some(locale);
    Ok(())
}

/// 获取本地化字符串
pub fn t(key: &str) -> String {
    LOCALE.lock().ok()
        .and_then(|l| l.as_ref().map(|loc| loc.get(key)))
        .unwrap_or_else(|| key.to_string())
}

/// 获取本地化字符串并格式化
pub fn tf(key: &str, args: &[&str]) -> String {
    LOCALE.lock().ok()
        .and_then(|l| l.as_ref().map(|loc| loc.format(key, args)))
        .unwrap_or_else(|| {
            let mut result = key.to_string();
            for (i, arg) in args.iter().enumerate() {
                result = result.replace(&format!("{{{}}}", i), arg);
            }
            result
        })
}
```

3. **在扩展中使用本地化**
```rust
// 初始化本地化（在 LSP 初始化时）
fn initialize(&mut self, params: InitializeParams) -> Result<InitializeResult> {
    // 获取客户端语言设置
    let language = params.client_info
        .as_ref()
        .and_then(|info| info.locale.as_ref())
        .unwrap_or("en-US");

    // 初始化本地化
    crate::locale::init_locale(language)?;

    // ... 其他初始化逻辑
}

// 使用本地化字符串
fn validate_variable_name(name: &str) -> Result<(), CangjieError> {
    if !is_snake_case(name) {
        return Err(user_error(&tf(
            "lint.invalid_naming",
            &[name, "snake_case"]
        )));
    }
    Ok(())
}
```

4. **添加语言选择配置**：在 `src/config/mod.rs` 中添加配置项
```rust
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CangjieConfig {
    // ... 其他配置项
    /// 本地化语言（支持：en-US, zh-CN, ja-JP, ko-KR）
    pub locale: Option<String>,
}
```

#### D. 常见问题排查工具

##### 1. 扩展日志查看
- Zed 内置日志：`View > Extensions > Cangjie > Logs`
- 文件日志（需启用）：默认路径 `~/.config/zed/extensions/cangjie/logs/cangjie.log`

##### 2. 性能分析工具
使用 `cargo flamegraph` 分析性能瓶颈：
```bash
# 安装依赖
cargo install flamegraph

# 运行性能分析（需要测试用例）
flamegraph -- cargo test --release -- --test-threads=1 --nocapture
```

##### 3. 内存泄漏检测
```bash
# 安装依赖
cargo install cargo-valgrind

# 运行内存检测
cargo valgrind test --release
```

#### E. 社区贡献模板

##### Pull Request 模板（`.github/PULL_REQUEST_TEMPLATE.md`）
```markdown
## 描述
<!-- 请描述此 PR 的目的和修改内容 -->

## 类型
- [ ] 新功能（feat）
- [ ] Bug 修复（fix）
- [ ] 代码优化（refactor）
- [ ] 文档更新（docs）
- [ ] 测试补充（test）
- [ ] 其他（请说明）

## 相关 Issue
<!-- 关联的 GitHub Issue 编号，如 #123 -->
Closes #

## 测试
- [ ] 新增测试用例
- [ ] 所有现有测试通过
- [ ] 手动测试验证（请说明测试场景）

## 检查清单
- [ ] 代码符合 Rust 代码规范（`cargo fmt` 格式化）
- [ ] 无 Clippy 警告（`cargo clippy -- -D warnings`）
- [ ] 文档已更新（如需）
- [ ] 本地化字符串已添加（如需）
- [ ] 性能无明显下降（如需）

## 截图（如需）
<!-- 如涉及 UI 变更，请添加截图 -->

## 附加说明
<!-- 其他需要说明的内容 -->
```

##### Issue 模板（`.github/ISSUE_TEMPLATE/`）

**Bug 报告模板（`bug_report.md`）**
```markdown
---
name: Bug 报告
about: 创建 Bug 报告以帮助我们改进
title: '[BUG] '
labels: bug
assignees: ''
---

## 描述 Bug
<!-- 清晰简洁地描述 Bug 内容 -->

## 复现步骤
1. 打开文件 '...'
2. 执行操作 '....'
3. 查看结果 '....'
4. 发现错误

## 预期行为
<!-- 描述你期望发生的事情 -->

## 实际行为
<!-- 描述实际发生的事情 -->

## 截图
<!-- 如适用，添加截图以帮助解释你的问题 -->

## 环境信息
- Zed 版本: [e.g. v0.130.0]
- 操作系统: [e.g. macOS 14.0, Ubuntu 22.04, Windows 11]
- 扩展版本: [e.g. v0.1.0]

## 附加信息
<!-- 任何其他相关信息 -->
```

**功能请求模板（`feature_request.md`）**
```markdown
---
name: 功能请求
about: 为这个项目提出一个想法
title: '[FEATURE] '
labels: enhancement
assignees: ''
---

## 功能描述
<!-- 清晰简洁地描述你想要的功能 -->

## 使用场景
<!-- 描述这个功能会如何帮助你 -->

## 替代方案
<!-- 描述你考虑过的任何替代解决方案或功能 -->

## 附加信息
<!-- 在此处添加有关功能请求的任何其他上下文或截图 -->
```

---

### 最终说明

Cangjie 扩展是一个开源项目，欢迎所有开发者参与贡献。无论是代码提交、文档完善、bug 报告还是功能建议，都能帮助我们不断改进扩展质量。

如果你在使用过程中遇到任何问题，或有任何疑问，欢迎通过 GitHub Issue、Discord 社区或邮件联系我们。我们致力于打造一个强大、易用、高性能的 Cangjie 语言开发环境，感谢你的支持！

**最后更新时间**：2025-11-09
**扩展版本**：v1.0.0
**兼容 Zed 版本**：v0.130.0+
**兼容 Cangjie 版本**：v1.0.0+
