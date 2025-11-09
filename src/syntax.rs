//! 仓颉语言语法片段定义
use std::collections::HashMap;

/// 代码片段结构体
#[derive(Debug, Clone)]
pub struct Snippet {
    /// 片段名称
    pub name: String,
    /// 片段描述
    pub description: String,
    /// 片段内容（支持 Snippet 语法）
    pub body: String,
}

/// 获取仓颉语言代码片段
pub fn get_snippets() -> HashMap<String, Vec<Snippet>> {
    let mut snippets = HashMap::new();

    let cangjie_snippets = vec![
        // 函数定义
        Snippet {
            name: "fn",
            description: "函数定义",
            body: "fn ${1:function_name}(${2:params})${3: -> ${4:return_type}} {\\n    ${0:// 函数体}\\n}".to_string(),
        },
        // 结构体定义
        Snippet {
            name: "struct",
            description: "结构体定义",
            body: "struct ${1:StructName} {\\n    ${0:// 字段定义}\\n}".to_string(),
        },
        // 枚举定义
        Snippet {
            name: "enum",
            description: "枚举定义",
            body: "enum ${1:EnumName} {\\n    ${0:// 变体定义}\\n}".to_string(),
        },
        // if 语句
        Snippet {
            name: "if",
            description: "if 条件语句",
            body: "if ${1:condition} {\\n    ${0:// 条件成立时执行}\\n}".to_string(),
        },
        // if-else 语句
        Snippet {
            name: "ifelse",
            description: "if-else 条件语句",
            body: "if ${1:condition} {\\n    ${0:// 条件成立时执行}\\n} else {\\n    // 条件不成立时执行\\n}".to_string(),
        },
        // for 循环
        Snippet {
            name: "for",
            description: "for 循环",
            body: "for ${1:item} in ${2:iterable} {\\n    ${0:// 循环体}\\n}".to_string(),
        },
        // while 循环
        Snippet {
            name: "while",
            description: "while 循环",
            body: "while ${1:condition} {\\n    ${0:// 循环体}\\n}".to_string(),
        },
        // 导入语句
        Snippet {
            name: "import",
            description: "导入模块",
            body: "import ${1:module_path}".to_string(),
        },
        // 变量定义
        Snippet {
            name: "let",
            description: "变量定义",
            body: "let ${1:variable_name}: ${2:type} = ${3:value};".to_string(),
        },
        // 常量定义
        Snippet {
            name: "const",
            description: "常量定义",
            body: "const ${1:constant_name}: ${2:type} = ${3:value};".to_string(),
        },
    ];

    snippets.insert("Cangjie".to_string(), cangjie_snippets);
    snippets
}
