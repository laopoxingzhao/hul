# Hul 语言解释器设计文档

## 1. 项目概述

`hul` 是一个用 Rust 实现的轻量脚本语言解释器，目标是提供简单的编程语言核心功能，包括：

- 变量声明与赋值
- 算术、比较、逻辑运算
- 条件语句 `if-else`
- 循环语句 `while`
- 打印输出
- REPL 与文件执行双模式

代码结构清晰，模块化设计便于后续扩展与维护。

---

## 2. 架构概览

整体执行流程如下：

1. `src/main.rs`
   - 负责命令行入口
   - 支持文件执行和 REPL
2. `src/lexer.rs`
   - 将源代码转换为 `Token` 流
3. `src/parser.rs`
   - 将 `Token` 序列解析成 AST
4. `src/ast.rs`
   - 定义 AST 节点类型
5. `src/interpreter.rs`
   - 遍历 AST 并执行程序
6. `src/value.rs`
   - 定义运行时值与作用域环境

7. `src/lib.rs` (新增)
  - 将内部模块封装为库，导出公共 API（`Interpreter`, `Parser`, `Value`）
  - 提供 `run(source: &str) -> Result<(), String>` 的公共入口，供二进制 `src/main.rs` 与集成测试使用

---

## 3. 模块职责

### `src/main.rs`

- 解析命令行参数
- 调用 `run(source)`
- 文件模式：读取整个文件，然后执行
- REPL 模式：逐行读取、执行

### `src/lexer.rs`

- 基于字符扫描器实现词法分析
- 跳过空白和单行注释
- 识别关键字、标识符、数字、字符串与运算符
- 生成带位置信息（行/列）的 `Token`

### `src/parser.rs`

- 递归下降 + Pratt 表达式解析
- 先将整个源代码词法化
- 解析语句与表达式
- 处理赋值优先级
- 支持语句：
  - `let`
  - `print`
  - `if/else`
  - `while`
  - `{}` 块
  - 表达式语句

### `src/ast.rs`

- 定义语法树节点
- 语句类型 `Stmt`
- 表达式类型 `Expr`
- 运算符枚举：`BinaryOp`、`UnaryOp`、`LogicalOp`

### `src/interpreter.rs`

- 解释执行 AST
- 对每种 `Stmt` 和 `Expr` 进行分发求值
- 管理作用域链
- 实现运行时错误报告

新增职责（2026-05-21 重构）:
- 支持函数对象的创建与调用语义
- 在函数声明时记录闭包环境（用于实现简单闭包/词法作用域）

### `src/value.rs`

- 定义运行时值模型 `Value`
- 新增 `Value::Function` 存储函数对象（名称、参数、体、闭包环境）
- 值引用类型 `ValueRef = Rc<RefCell<Value>>`
- 作用域环境 `Environment`
  - 变量定义 `define`
  - 变量赋值 `assign`
  - 变量读取 `get`
  - 链式嵌套作用域

---

## 4. 语言语法与表达式

### 支持的数据类型

- `Number(f64)`
- `String(String)`
- `Boolean(bool)`
- `Nil`

### 支持运算

- 算术：`+ - * / %`
- 比较：`== != < <= > >=`
- 逻辑：`and`, `or`, `not`

### 语句

- 变量声明：`let x = expr;`
- 赋值语句：`x = expr;`
- 打印语句：`print expr;`
- 条件语句：
  - `if (cond) { ... }`
  - `if (cond) { ... } else { ... }`
  - `else if`
- 循环语句：`while (cond) { ... }`
- 代码块：`{ ... }`

---

## 5. 词法分析设计

### 核心类型

- `TokenType`
  - 关键字：`Let`, `If`, `Else`, `While`, `Print`, `And`, `Or`, `Not`
  - 符号：`Plus`, `Minus`, `Star`, `Slash`, `Percent`, `Assign`, `Semicolon`, `LParen`, `RParen`, `LBrace`, `RBrace`
  - 文字值：`Number`, `String`, `True`, `False`, `Nil`, `Identifier`
  - 结束：`Eof`

- `Token { ty, line, col }`

### 设计要点

- 使用 `chars: Vec<char>` 逐字符扫描
- 维护 `pos`, `line`, `col`
- `skip_whitespace` 和 `skip_line_comment`
- `read_number` 支持浮点数
- `read_identifier` 区分关键字与普通标识符
- `make_token` 统一创建带位置的 `Token`

---

## 6. 语法分析设计

### 语法分析器 `Parser`

- 从 `Lexer` 收集所有 `Token`
- 使用 `current` 索引遍历

### 语句解析流程

- `parse_program()`
- `statement()`
- `let_statement()`
- `print_statement()`
- `if_statement()`
- `while_statement()`
- `block_statement()`
- `expression_statement()`

### 表达式解析

采用 Pratt 解析结构，实现运算符优先级：

- `expression()` → `parse_assignment()`
- `parse_assignment()` 处理右结合赋值
- `parse_or()` / `parse_and()`
- `parse_equality()`
- `parse_comparison()`
- `parse_term()`
- `parse_factor()`
- `parse_unary()`
- `parse_primary()`

扩展（函数与调用）:
- `parse_call()` / `finish_call()`：解析函数调用表达式 `callee(arg1, arg2)`，支持 `,` 分隔的参数列表
- `fn_statement()`：解析函数声明 `fn name(params...) { body }`
- `return` 语句解析 `return expr;`

### 赋值规则

- 只有 `Expr::Variable` 可以作为赋值目标
- 赋值使用右递归：
  - `a = b = 3`
- `expression_statement()` 会将赋值表达式提升为 `Stmt::Assign`

---

## 7. AST 设计

### 语句 `Stmt`

- `Let { name, initializer }`
- `Assign { name, value }`
- `Print(Expr)`
- `If { condition, then_branch, else_branch }`
- `While { condition, body }`
- `Block(Vec<Stmt>)`
- `Expression(Expr)`

### 表达式 `Expr`

- `Literal(Value)`
- `Variable(String)`
- `Assign { name, value }`
- `Binary { left, operator, right }`
- `Unary { operator, right }`
- `Logical { left, operator, right }`
- `Grouping(Box<Expr>)`

### 运算符枚举

- `BinaryOp`: `Add`, `Sub`, `Mul`, `Div`, `Mod`, `Equal`, `NotEqual`, `Less`, `LessEqual`, `Greater`, `GreaterEqual`
- `UnaryOp`: `Negate`, `Not`
- `LogicalOp`: `And`, `Or`

---

## 8. 解释器与运行时模型

### 解释器核心

- `Interpreter { env: Rc<RefCell<Environment>> }`
- `Interpreter { env: Rc<RefCell<Environment>> }`（重构后 `env` 暴露为 `pub` 以便集成测试检查运行时状态）
- 解释器状态包含当前环境
- `interpret(&[Stmt])` 逐语句执行

新增函数调用语义（已实现）:
- `Value::Function(Function)`：函数对象包含 `name, params, body, closure`
- 在声明函数时，解释器将函数对象绑定到当前环境
- 调用函数时：
  - 创建新环境并将 `closure` 作为父环境，按参数顺序绑定实参
  - 执行函数体（支持 `return` 提供的早期返回）
  - 恢复调用者环境并返回值（若无 `return` 则返回 `nil`）

控制流扩展：解释器内部使用 `ControlFlow::Return(ValueRef)` 在块/函数间传递返回值

### 语句执行

- `Let`: 计算初始值并在当前环境定义变量
- `Assign`: 计算右值并更新变量
- `Print`: 计算表达式并打印
- `If`: 根据条件执行分支
- `While`: 循环求值直到条件为假
- `Block`: 创建新作用域执行语句块
- `Expression`: 计算表达式结果并丢弃

### 表达式求值

- `Literal` 直接生成值
- `Variable` 从环境读取
- `Grouping` 递归求值
- `Unary` 处理 `-` / `not`
- `Assign` 更新变量并返回赋值结果
- `Binary` 处理算术与比较
- `Logical` 实现短路求值

### 作用域与环境

- `Environment`
  - `bindings: HashMap<String, ValueRef>`
  - `parent: Option<Rc<RefCell<Environment>>>`
- 通过 `new_with_parent()` 实现块级作用域
- 变量查找支持向上查找父环境
- 赋值优先在当前作用域查找，未找到则递归父作用域

---

## 9. 值表示

### `Value` 类型

- `Number(f64)`
- `String(String)`
- `Boolean(bool)`
- `Nil`

### 运行时引用

- `ValueRef = Rc<RefCell<Value>>`
- 支持共享与内部可变性
- 允许变量替换和值传递保持引用语义

### 真值规则

- `Nil` → false
- `Boolean(false)` → false
- 其他值 → true

---

## 10. 错误处理

### 词法 / 语法阶段

- 使用 `Result<..., String>`
- 语法错误带位置信息 `line:col`
- `expect()` 用于断言当前 Token 类型

### 运行时阶段

- 解释器返回详细错误信息
- `main()` 将错误输出到 `stderr`

---

## 11. 运行方式

### 文件执行
```bash
cargo run -- script.hul
```

### REPL
```bash
cargo run
```

### 运行测试

项目包含单元测试（位于 `src/` 内）与集成测试（位于 `tests/` 目录）。运行所有测试：

```bash
cargo test
```

运行单个集成测试文件：

```bash
cargo test --test interpreter
```

说明：为了便于集成测试，项目已重构为库（`src/lib.rs`），并在 `src/main.rs` 中通过 `hul::run()` 调用公共入口。

---

## 12. 设计优点

- 结构清晰，模块职责分离
- 无外部依赖，纯 Rust 实现
- 支持语言核心控制结构
- 解释器实现简单，可扩展性好
- 词法和语法错误定位明确

---

## 13. 未来扩展建议

建议后续可以继续扩展以下能力：

- 函数与调用机制
- 数组/字典类型
- 标准库内置函数
- 异常处理或返回机制
- 模块导入与命名空间
- 词法分析器支持多行注释
- 更完整的类型检查与错误信息

---

## 14. 最近变更摘要（2026-05-21）

简要记录最近一次重构与功能实现，便于回溯与审计：

- 新增函数支持：`fn`、`return`、函数调用 `name(args...)`。
- 词法扩展：新增 `TokenType::Fn`, `TokenType::Return`, `TokenType::Comma`。
- AST 扩展：新增 `Stmt::Function`, `Stmt::Return`, `Expr::Call`。
- 值模型：新增 `Value::Function` 与 `Function` 对象（包含闭包环境）。
- 解释器：添加 `call_function` 实现、闭包式环境管理与 `ControlFlow::Return` 机制。
- 库封装：新增 `src/lib.rs` 并导出 `run`、`Interpreter`、`Parser`、`Value`，`src/main.rs` 现在调用库入口。
- 测试结构：将解释器测试从 `src/` 移到 `tests/interpreter.rs`（集成测试），并清理临时测试文件。

受影响的主要文件：
- [src/lib.rs](src/lib.rs)
- [src/main.rs](src/main.rs)
- [src/ast.rs](src/ast.rs)
- [src/lexer.rs](src/lexer.rs)
- [src/parser.rs](src/parser.rs)
- [src/interpreter.rs](src/interpreter.rs)
- [src/value.rs](src/value.rs)
- [tests/interpreter.rs](tests/interpreter.rs)
- [plans/design.md](plans/design.md) （本文件）

验证：已运行 `cargo test`，所有测试与 doctest 通过。
