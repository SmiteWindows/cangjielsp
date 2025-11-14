# Tree-sitter 仓颉语言扩展项目

## 项目概述
该项目为仓颉编程语言（Cangjie）提供 Tree-sitter 语法解析支持，可用于代码高亮、语法分析、自动补全、重构等 IDE/编辑器功能。遵循 Tree-sitter 官方规范开发，支持跨平台使用。

## 项目文件目录
```
tree-sitter-cangjie/
├── .github/                    # GitHub 配置（可选）
│   ├── workflows/
│   │   └── test.yml            # CI 测试工作流
│   └── FUNDING.yml             # 赞助配置（可选）
├── bindings/                   # 语言绑定（供不同编辑器/工具使用）
│   ├── node/                   # Node.js 绑定
│   │   ├── index.js
│   │   ├── package.json
│   │   └── binding.gyp
│   ├── rust/                   # Rust 绑定
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── python/                 # Python 绑定（可选）
│       ├── setup.py
│       └── tree_sitter_cangjie/
│           └── __init__.py
├── corpus/                     # 测试用例 corpus
│   ├── comments.txt            # 注释语法测试
│   ├── expressions.txt         # 表达式语法测试
│   ├── functions.txt           # 函数定义测试
│   ├── types.txt               # 类型定义测试
│   └── statements.txt          # 语句语法测试
├── examples/                   # 示例代码（用于验证解析）
│   ├── hello_world.cangjie
│   ├── complex_program.cangjie
│   └── module_import.cangjie
├── src/                        # 核心语法解析代码
│   ├── grammar.js              # 仓颉语法定义（核心文件）
│   ├── node-types.json         # AST 节点类型定义
│   ├── parser.c                # 自动生成的解析器（勿手动修改）
│   └── scanner.c               # 自定义扫描器（处理复杂语法，如字符串/注释）
├── test/                       # 单元测试
│   └── test.js                 # 使用 tree-sitter-cli 测试语法
├── .gitignore                  # Git 忽略文件
├── Cargo.toml                  # Rust 项目配置（Tree-sitter 基础依赖）
├── package.json                # Node.js 项目配置（开发依赖/脚本）
├── README.md                   # 项目说明文档
└── tree-sitter.json            # Tree-sitter 插件元信息
```

## 核心文件详细说明

### 1. 元信息文件
#### `tree-sitter.json`（必需）
Tree-sitter 插件的元数据，用于编辑器识别语言
```json
{
  "name": "tree-sitter-cangjie",
  "version": "0.1.0",
  "description": "Tree-sitter grammar for the Cangjie programming language",
  "author": "Your Name",
  "license": "MIT",
  "repository": "https://github.com/your-username/tree-sitter-cangjie",
  "main": "bindings/node/index.js",
  "tree-sitter": {
    "file-types": ["cangjie"],
    "scope": "source.cangjie",
    "highlights": "queries/highlights.scm",
    "injections": "queries/injections.scm",
    "locals": "queries/locals.scm"
  }
}
```

#### `package.json`（Node.js 开发依赖）
```json
{
  "name": "tree-sitter-cangjie",
  "version": "0.1.0",
  "main": "bindings/node/index.js",
  "keywords": ["tree-sitter", "parser", "cangjie"],
  "author": "Your Name",
  "license": "MIT",
  "dependencies": {
    "nan": "^2.18.0"
  },
  "devDependencies": {
    "tree-sitter-cli": "^0.22.0",
    "mocha": "^10.2.0"
  },
  "scripts": {
    "test": "mocha",
    "generate": "tree-sitter generate",
    "parse": "tree-sitter parse",
    "test-corpus": "tree-sitter test"
  }
}
```

#### `Cargo.toml`（Rust 绑定依赖）
```toml
[package]
name = "tree-sitter-cangjie"
version = "0.1.0"
authors = ["Your Name <your-email@example.com>"]
edition = "2021"
description = "Tree-sitter grammar for Cangjie"
license = "MIT"
repository = "https://github.com/your-username/tree-sitter-cangjie"

[lib]
path = "bindings/rust/src/lib.rs"

[dependencies]
tree-sitter = "0.22"

[build-dependencies]
tree-sitter-cli = "0.22"
```

### 2. 核心语法定义
#### `src/grammar.js`（核心文件，定义仓颉语法）
```javascript
// 仓颉语言语法规则（需根据实际仓颉语法调整）
module.exports = grammar({
  name: 'cangjie',

  // 基础符号定义
  word: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
  number: $ => /\d+(\.\d+)?([eE][+-]?\d+)?/,
  string: $ => seq(
    '"',
    repeat(choice(
      /[^"\\]+/,
      seq('\\', /["\\nrt]/)
    )),
    '"'
  ),

  // AST 节点类型
  rules: {
    // 程序入口
    source_file: $ => repeat(choice(
      $.comment,
      $.function_definition,
      $.variable_declaration,
      $.import_statement,
      $.export_statement,
      $.type_definition
    )),

    // 注释（单行// 和 多行/* */）
    comment: $ => choice(
      seq('//', /.*/),
      seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/')
    ),
//import package1.foo
// import {package1.foo, package2.bar}
    // 导入语句：import "module" 或 import { a, b } from "module"
    import_statement: $ => seq(
      'import',
      choice(
        seq(
          '{',
          commaSep($.identifier),
          '}',
          // 'from'
        ),
        $.identifier
      ),
      $.string,
      // ';'
    ),

    // 导出语句：export func foo() {}
    export_statement: $ => seq(
      'export',
      choice($.function_definition, $.type_definition, $.variable_declaration)
    ),

    // 变量声明：let x = 10; 或 const y = "hello";
    variable_declaration: $ => seq(
      choice('let', 'const'),
      $.identifier,
      optional(seq(':', $.type_annotation)),
      '=',
      $.expression,
      ';'
    ),

    // 类型注解：Int, String, [Int], { x: Bool }
    type_annotation: $ => choice(
      $.primitive_type,
      $.array_type,
      $.struct_type,
      $.identifier // 自定义类型
    ),

    primitive_type: $ => choice('Int', 'Float', 'Bool', 'String', 'Void'),
    array_type: $ => seq('[', $.type_annotation, ']'),
    struct_type: $ => seq(
      '{',
      commaSep(seq($.identifier, ':', $.type_annotation)),
      '}'
    ),

    // 函数定义：func add(a: Int, b: Int): Int { return a + b; }
    function_definition: $ => seq(
      'func',
      $.identifier,
      '(',
      commaSep(seq($.identifier, ':', $.type_annotation)),
      ')',
      optional(seq(':', $.type_annotation)),
      '{',
      repeat(choice($.statement, $.comment)),
      '}'
    ),

    // 语句：表达式语句、return语句、if语句
    statement: $ => choice(
      $.expression_statement,
      $.return_statement,
      $.if_statement,
      $.variable_declaration
    ),

    expression_statement: $ => seq($.expression, ';'),
    return_statement: $ => seq('return', optional($.expression), ';'),

    if_statement: $ => seq(
      'if',
      '(',
      $.expression,
      ')',
      $.block,
      optional(seq('else', choice($.block, $.if_statement)))
    ),

    block: $ => seq('{', repeat(choice($.statement, $.comment)), '}'),

    // 表达式：二元运算、标识符、数字、字符串、函数调用
    expression: $ => choice(
      $.binary_expression,
      $.identifier,
      $.number_literal,
      $.string_literal,
      $.function_call,
      seq('(', $.expression, ')')
    ),

    binary_expression: $ => prec.left(
      choice(
        seq($.expression, '+', $.expression),
        seq($.expression, '-', $.expression),
        seq($.expression, '*', $.expression),
        seq($.expression, '/', $.expression),
        seq($.expression, '==', $.expression),
        seq($.expression, '!=', $.expression),
        seq($.expression, '<', $.expression),
        seq($.expression, '>', $.expression)
      )
    ),

    number_literal: $ => $.number,
    string_literal: $ => $.string,
    identifier: $ => $.word,

    // 函数调用：foo(1, "a")
    function_call: $ => seq(
      $.identifier,
      '(',
      commaSep($.expression),
      ')'
    ),

    // 类型定义：type User = { name: String, age: Int }
    type_definition: $ => seq(
      'type',
      $.identifier,
      '=',
      $.type_annotation,
      ';'
    )
  }
});

// 辅助函数：处理逗号分隔的列表
function commaSep(rule) {
  return optional(seq(rule, repeat(seq(',', rule)), optional(',')));
}
```

#### `src/node-types.json`（AST 节点类型描述）
```json
[
  {
    "type": "source_file",
    "kind": "document"
  },
  {
    "type": "comment",
    "kind": "comment"
  },
  {
    "type": "import_statement",
    "kind": "statement",
    "fields": {
      "specifiers": { "type": "identifier", "multiple": true },
      "module": { "type": "string_literal" }
    }
  },
  {
    "type": "export_statement",
    "kind": "statement",
    "fields": {
      "declaration": { "type": ["function_definition", "type_definition", "variable_declaration"] }
    }
  },
  {
    "type": "variable_declaration",
    "kind": "declaration",
    "fields": {
      "name": { "type": "identifier" },
      "type": { "type": "type_annotation", "optional": true },
      "value": { "type": "expression" }
    }
  },
  {
    "type": "function_definition",
    "kind": "declaration",
    "fields": {
      "name": { "type": "identifier" },
      "parameters": { "type": "identifier", "multiple": true },
      "return_type": { "type": "type_annotation", "optional": true },
      "body": { "type": "block" }
    }
  },
  {
    "type": "type_definition",
    "kind": "declaration",
    "fields": {
      "name": { "type": "identifier" },
      "type": { "type": "type_annotation" }
    }
  },
  {
    "type": "binary_expression",
    "kind": "expression",
    "fields": {
      "left": { "type": "expression" },
      "operator": { "type": "string" },
      "right": { "type": "expression" }
    }
  },
  {
    "type": "function_call",
    "kind": "expression",
    "fields": {
      "function": { "type": "identifier" },
      "arguments": { "type": "expression", "multiple": true }
    }
  },
  {
    "type": "identifier",
    "kind": "name"
  },
  {
    "type": "number_literal",
    "kind": "literal"
  },
  {
    "type": "string_literal",
    "kind": "literal"
  }
]
```

#### `src/scanner.c`（自定义扫描器，处理复杂语法）
```c
#include <tree_sitter/parser.h>
#include <string.h>
#include <wctype.h>

enum TokenType {
  // 自定义 token（如需扩展语法，如模板字符串、正则等）
  UNKNOWN_TOKEN
};

void *tree_sitter_cangjie_external_scanner_create() {
  return NULL;
}

void tree_sitter_cangjie_external_scanner_destroy(void *payload) {}

void tree_sitter_cangjie_external_scanner_reset(void *payload) {}

unsigned tree_sitter_cangjie_external_scanner_serialize(void *payload, char *buffer) {
  return 0;
}

void tree_sitter_cangjie_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {}

bool tree_sitter_cangjie_external_scanner_scan(
  void *payload, TSLexer *lexer, const bool *valid_symbols
) {
  // 如需处理复杂语法（如多行字符串、嵌套注释等），在此实现
  return false;
}
```

### 3. 测试相关文件
#### `corpus/comments.txt`（注释测试用例）
```
# 单行注释
// 这是单行注释
let x = 10; // 行尾注释

# 多行注释
/* 这是
   多行
   注释 */
let y = 20;

/* 嵌套注释？
/* 内部注释 */
结束 */
```

#### `corpus/functions.txt`（函数测试用例）
```
# 无参数无返回值函数
func hello() {
  println("Hello");
}

# 带参数带返回值函数
func add(a: Int, b: Int): Int {
  return a + b;
}

# 空函数体
func empty(): Void {}

# 函数调用
let result = add(10, 20);
hello();
```

#### `test/test.js`（单元测试）
```javascript
const { expect } = require('chai');
const Parser = require('tree-sitter');
const Cangjie = require('../bindings/node');

describe('Cangjie Parser', () => {
  let parser;

  beforeEach(() => {
    parser = new Parser();
    parser.setLanguage(Cangjie);
  });

  it('parses variable declarations', () => {
    const source = 'let x: Int = 10; const y = "hello";';
    const tree = parser.parse(source);
    const root = tree.rootNode;

    expect(root.type).to.equal('source_file');
    expect(root.children.length).to.equal(2);
    expect(root.children[0].type).to.equal('variable_declaration');
    expect(root.children[1].type).to.equal('variable_declaration');
  });

  it('parses function definitions', () => {
    const source = 'func add(a: Int, b: Int): Int { return a + b; }';
    const tree = parser.parse(source);
    const root = tree.rootNode;

    expect(root.type).to.equal('source_file');
    expect(root.children[0].type).to.equal('function_definition');
    expect(root.children[0].children[1].type).to.equal('identifier');
    expect(root.children[0].children[1].text).to.equal('add');
  });
});
```

### 4. 编辑器语法高亮配置（queries 目录）
需创建 `queries` 目录，存放语法高亮、变量作用域等查询文件：

#### `queries/highlights.scm`（语法高亮规则）
```scheme
; 关键字
[
  "func" "let" "const" "return" "if" "else" "type"
  "import" "export" "from" "Void" "Int" "Float" "Bool" "String"
] @keyword

; 注释
(comment) @comment

; 字符串
(string_literal) @string

; 数字
(number_literal) @number

; 标识符
(identifier) @variable

; 函数定义名称
(function_definition (identifier)) @function

; 函数调用名称
(function_call (identifier)) @function.call

; 类型名称
(type_definition (identifier)) @type
(struct_type (identifier)) @type.property
(array_type (type_annotation)) @type

; 运算符
[
  "+" "-" "*" "/" "==" "!=" "<" ">"
] @operator

; 标点符号
[
  "(" ")" "{" "}" "[" "]" ":" ";" "," "="
] @punctuation
```

#### `queries/locals.scm`（变量作用域规则）
```scheme
; 变量定义
(variable_declaration (identifier) @definition.var)

; 函数定义
(function_definition (identifier) @definition.function)

; 类型定义
(type_definition (identifier) @definition.type)

; 变量引用
(identifier) @reference.var

; 函数调用引用
(function_call (identifier) @reference.function)

; 类型引用
(type_annotation (identifier) @reference.type)
```

### 5. 绑定文件示例
#### `bindings/node/index.js`（Node.js 绑定入口）
```javascript
const fs = require('fs');
const path = require('path');
const bindings = require('bindings');

const modulePath = bindings('tree_sitter_cangjie_binding');
const Parser = require('tree-sitter');

const Cangjie = new Parser();
Cangjie.setLanguage(modulePath);

module.exports = {
  ...modulePath,
  Parser: Cangjie,
  parse: (code) => Cangjie.parse(code),
  parseFile: (filePath) => {
    const code = fs.readFileSync(filePath, 'utf8');
    return Cangjie.parse(code);
  }
};
```

#### `bindings/rust/src/lib.rs`（Rust 绑定入口）
```rust
use tree_sitter::Language;

extern "C" {
    fn tree_sitter_cangjie() -> Language;
}

/// 获取仓颉语言的 Tree-sitter Language
pub fn language() -> Language {
    unsafe { tree_sitter_cangjie() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_parse_variable() {
        let mut parser = Parser::new();
        parser.set_language(language()).unwrap();

        let source = "let x: Int = 10;";
        let tree = parser.parse(source, None).unwrap();
        assert_eq!(tree.root_node().to_sexp(), "(source_file (variable_declaration (identifier) (type_annotation (primitive_type)) (number_literal)))");
    }
}
```

## 项目使用说明
### 1. 开发依赖安装
```bash
# 安装 Node.js 依赖
npm install

# （可选）安装 Rust 依赖
cargo build
```

### 2. 生成解析器
```bash
# 生成 parser.c 和绑定代码
npm run generate
```

### 3. 运行测试
```bash
# 运行单元测试
npm test

# 运行 corpus 测试（语法覆盖测试）
npm run test-corpus

# 解析示例文件
npm run parse examples/hello_world.cangjie
```

### 4. 编辑器集成
- **Neovim**：通过 `nvim-treesitter` 插件，添加 `cangjie` 语法并配置文件类型关联
- **VS Code**：开发对应的语法高亮插件，依赖该 Tree-sitter 解析器
- **Sublime Text**：通过 `Tree-sitter` 插件集成

## 注意事项
1. 语法规则需根据 **实际仓颉语言规范** 调整（上述 `grammar.js` 是基于常见语言特性的示例）
2. 新增语法特性后，需同步更新 `corpus` 测试用例和 `node-types.json`
3. 自定义扫描器 `scanner.c` 仅在需要处理复杂语法（如多行字符串、嵌套注释、正则表达式）时使用
4. 发布前需测试多平台兼容性（Windows/macOS/Linux）

## 扩展方向
1. 支持仓颉语言的最新语法特性（如泛型、接口、错误处理等）
2. 完善编辑器绑定（如 VS Code 插件、JetBrains 插件）
3. 添加代码格式化、重构等高级功能的查询规则
4. 集成到 LSP（语言服务器协议）实现智能提示

该项目结构遵循 Tree-sitter 官方最佳实践，可直接基于此框架扩展，适配实际的仓颉语言语法规范。


缺文件, 语法解析有误
