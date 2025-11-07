; 缩进起始（@indent.begin）
(init) @indent.begin

; 类/函数定义
(classDefinition
  (classBody) @indent.begin)

(functionDefinition
  (block) @indent.begin)

(operatorFunctionDefinition
  (block) @indent.begin)

(arrowFunction
  (block) @indent.begin)

; 函数调用（多行参数）
(callExpression
  (callSuffix) @indent.begin)

; 集合字面量
(arrayLiteral) @indent.begin
(objectLiteral) @indent.begin
(structLiteral) @indent.begin

; 条件/分支语句
(ifExpression) @indent.begin
(matchExpression) @indent.begin
(matchCase) @indent.branch  ; 仅分支对齐，不额外缩进
(switchStatement) @indent.begin
(switchCase) @indent.branch

; 循环语句
(forInExpression) @indent.begin

; 异常处理语句
(tryStatement) @indent.begin
(catchClause) @indent.begin
(finallyClause) @indent.begin

; 缩进结束+分支（@indent.end @indent.branch）
(arrayLiteral
  "]" @indent.end @indent.branch)

(:or (callExpression) (parenthesizedExpression))
  ")" @indent.end @indent.branch)

(:or (classBody) (block) (objectLiteral) (structLiteral))
  "}" @indent.end @indent.branch)

; 自动适配缩进（@indent.auto）
[
  (lineComment)
  (blockComment)
] @indent.auto

; 忽略缩进（@indent.ignore）
[
  (multiline_string)
  (templateString)
] @indent.ignore

; 单行代码块忽略缩进（无 block 节点时）
(ifExpression
  (expression) @indent.ignore
  (#not-has-child? (block)))

(forInExpression
  (expression) @indent.ignore
  (#not-has-child? (block)))
