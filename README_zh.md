# hul

一个使用 Rust 实现的轻量级脚本语言解释器，零外部依赖。

```
hu> let sum = 0;
hu> for (let i = 1; i <= 100; i = i + 1) { sum = sum + i; }
hu> print sum;
5050
```

## 特性

- 变量、算术运算、比较运算、逻辑运算符
- 字符串字面量，支持转义序列和字符串拼接（`"age=" + 25`）
- 控制流：`if` / `else if` / `else`、`while`、`for`
- 循环控制：`break`、`continue`
- 函数支持递归和闭包
- 词法作用域，支持块级作用域
- 单行注释（`//`）和多行注释（`/* */`）
- REPL 交互模式和文件执行模式
- 错误恢复：一次解析过程中报告多个解析错误

## 快速开始

```bash
# 构建项目
cargo build

# 运行 REPL 交互模式
cargo run

# 运行脚本文件
cargo run -- examples/test.hu

# 运行测试
cargo test
```

## 文档

| 文档 | 说明 |
|------|------|
| [docs/PRINCIPLES.md](docs/PRINCIPLES.md) | 解释器工作原理 — 词法分析、语法分析、AST、执行流程 |
| [docs/SYNTAX.md](docs/SYNTAX.md) | 语言语法参考及示例 |

## 项目结构

```
src/
├── main.rs          CLI 入口（REPL + 文件模式）
├── lib.rs           公共 API
├── lexer.rs         字符流 → Token 流
├── parser.rs        Token 流 → AST
├── ast.rs           AST 节点定义
├── interpreter.rs   AST 遍历和执行
└── value.rs         运行时值 + 作用域链
```

## 许可证

MIT
