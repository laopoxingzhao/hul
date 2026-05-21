# hul

`hul` 是一个用 Rust 实现的轻量脚本语言解释器，旨在提供一个清晰、可扩展的语言核心。它支持变量、表达式、流程控制、函数定义与调用，以及交互式 REPL。

## 主要特性

- 词法分析、语法分析和解释执行全部用 Rust 编写
- 变量声明与赋值
- 算术、比较、逻辑运算
- 条件语句：`if` / `else`
- 循环语句：`while`
- 打印输出：`print`
- 函数定义与调用：`fn` / `return`
- 代码块作用域
- 交互式 REPL 与文件执行模式

## 目录结构

- `src/main.rs` - 命令行入口，支持文件执行和 REPL
- `src/lib.rs` - 库接口，导出 `run(source)`, `Interpreter`, `Parser`, `Value`
- `src/lexer.rs` - 词法分析器，将源代码转换为 `Token` 流
- `src/parser.rs` - 语法分析器，构建 AST
- `src/ast.rs` - 抽象语法树节点定义
- `src/interpreter.rs` - 解释器，执行 AST 语义
- `src/value.rs` - 运行时值和环境管理
- `tests/` - 集成测试示例

## 快速开始

### 构建

```bash
cargo build
```

### 运行 REPL

```bash
cargo run
```

然后输入 Hul 代码，例如：

```text
hu> let x = 10;
hu> let y = 20;
hu> print x + y;
30
hu> fn add(a, b) { return a + b; }
hu> print add(3, 4);
7
```

### 执行脚本文件

创建一个文件 `script.hul`：

```text
let a = 3;
let b = 5;
print a * b;
```

然后运行：

```bash
cargo run -- script.hul
```

## 示例语言语法

### 变量

```hu
let x = 42;
x = x + 1;
```

### 打印

```hu
print x;
```

### 条件

```hu
if (x > 0) {
    print "positive";
} else {
    print "non-positive";
}
```

### 循环

```hu
let i = 0;
while (i < 5) {
    print i;
    i = i + 1;
}
```

### 函数

```hu
fn add(a, b) {
    return a + b;
}
let result = add(2, 3);
print result;
```

## 测试

```bash
cargo test
```
