# Hul 语言解释器 — 原理详解

本文档从零开始，详细解释 `hul` 解释器是如何工作的。即使你完全不懂编译原理，也能看懂。

---

## 目录

- [第一章：解释器是什么](#第一章解释器是什么)
- [第二章：词法分析（Lexer）](#第二章词法分析lexer)
- [第三章：语法分析（Parser）](#第三章语法分析parser)
- [第四章：抽象语法树（AST）](#第四章抽象语法树ast)
- [第五章：解释执行（Interpreter）](#第五章解释执行interpreter)
- [第六章：值类型系统（Value）](#第六章值类型系统value)
- [第七章：变量与作用域（Environment）](#第七章变量与作用域environment)
- [第八章：函数与闭包](#第八章函数与闭包)
- [第九章：控制流](#第九章控制流)
- [第十章：错误处理](#第十章错误处理)
- [附录：完整执行流程图](#附录完整执行流程图)

---

## 第一章：解释器是什么

### 1.1 人类语言 vs 编程语言

假设你写了一封中文信给朋友，朋友需要：

1. **认字** — 把一个个汉字分开识别
2. **理解语法** — 知道"我 吃 饭"是"主语+谓语+宾语"
3. **执行含义** — 知道这句话要表达的意思

编程语言也一样。当解释器看到 `let x = 1 + 2;` 时：

```
源代码:  let x = 1 + 2;
         ↓
第一步:  识别出 [let] [x] [=] [1] [+] [2] [;]   ← 词法分析 (Lexer)
         ↓
第二步:  理解为 "声明变量x，值为1+2的结果"       ← 语法分析 (Parser)
         ↓
第三步:  真正计算 1+2，把结果3存到变量x里        ← 解释执行 (Interpreter)
```

### 1.2 三阶段流水线

整个解释器像一条流水线，有三个车间：

```
┌─────────┐    Token流    ┌─────────┐     AST     ┌─────────────┐
│  源代码  │ ──────────→ │  Lexer  │ ─────────→ │   Parser    │
└─────────┘              └─────────┘            └──────┬──────┘
                                                       │
                                                  语法树(AST)
                                                       │
                                                       ↓
                                              ┌─────────────────┐
                                              │   Interpreter   │
                                              │   (解释执行)     │
                                              └─────────────────┘
```

每个阶段只做一件事：

- **Lexer（词法分析器）**：把连续的字符切成一个个有意义的"单词"（Token）
- **Parser（语法分析器）**：把 Token 流组装成一棵"语法树"（AST）
- **Interpreter（解释器）**：遍历语法树，执行每个节点的含义

---

## 第二章：词法分析（Lexer）

### 2.1 什么是词法分析

你读这句话时，大脑其实自动把字切成了词：

```
"我喜欢吃苹果"  →  [我] [喜欢] [吃] [苹果]
```

Lexer 做的就是同样的事。它把源代码字符串切成一个个 **Token**（词法单元）。

### 2.2 Token 是什么

Token 是一个"有类型的单词"：

```
源代码:  let x = 42;

Token序列:
┌────────┬──────────┬────────┬──────────┬───────┐
│  let   │    x     │   =    │    42    │   ;   │
├────────┼──────────┼────────┼──────────┼───────┤
│ Let    │Ident(x)  │Assign  │Number(42)│Semi-  │
│ 关键字  │ 标识符    │ 赋值号  │ 数字字面量 │colon  │
└────────┴──────────┴────────┴──────────┴───────┘
```

每个 Token 有两部分信息：

- **类型**（是什么）：关键字、标识符、数字、运算符...
- **值**（内容是什么）：`x`、`42`、`+`...

### 2.3 Lexer 的工作方式

Lexer 用一个指针从左到右扫描源代码，逐字符判断：

```rust
// 简化的流程（伪代码）
pos = 0  // 指针，指向当前字符
while pos < 代码长度:
    ch = 当前字符
    if ch 是空格或换行:  跳过
    if ch 是数字:        读取整个数字 → Number token
    if ch 是字母:        读取整个单词 → 关键字 或 标识符 token
    if ch 是 '"':        读取到下一个 '"' → String token
    if ch 是 '+':        → Plus token
    if ch 是 '=':        看下一个字符
        if 下一个是 '=': → EqualEqual token (==)
        else:            → Assign token (=)
    ...
```

### 2.4 具体例子

以 `let x = 1 + 2;` 为例，Lexer 的扫描过程：

```
位置:  l  e  t     x     =     1     +     2     ;
      0  1  2  3  4  5  6  7  8  9  10 11 12 13

步骤:
  pos=0:  看到 'l'，是字母 → 继续读 'e','t' → 得到 "let" → 这是关键字 Let
  pos=3:  看到 ' '，是空白 → 跳过
  pos=4:  看到 'x'，是字母 → 读完得到 "x" → 这是标识符 Identifier("x")
  pos=5:  看到 ' '，是空白 → 跳过
  pos=6:  看到 '='，后面不是 '=' → 赋值号 Assign
  pos=7:  看到 ' '，是空白 → 跳过
  pos=8:  看到 '1'，是数字 → 读完得到 1.0 → Number(1.0)
  pos=9:  看到 ' '，是空白 → 跳过
  pos=10: 看到 '+' → Plus
  pos=11: 看到 ' '，是空白 → 跳过
  pos=12: 看到 '2'，是数字 → Number(2.0)
  pos=13: 看到 ';' → Semicolon
```

### 2.5 关键词 vs 标识符

`let` 和 `myVar` 都是字母组成的字符串，怎么区分？

Lexer 先把整个单词读出来，然后查表判断：

```rust
// 读取到一个单词后
match 单词 {
    "let"    → TokenType::Let       // 关键字
    "if"     → TokenType::If
    "while"  → TokenType::While
    "true"   → TokenType::True
    _        → TokenType::Identifier(单词)  // 普通标识符
}
```

所以 `letter` 不会被误认为 `let` + `tter`，因为 Lexer 会把 `letter` 作为一个整体读出来。

### 2.6 双字符运算符

像 `==`、`!=`、`<=`、`>=` 这类双字符运算符，Lexer 需要"向前看一个字符"：

```rust
// 看到 '=' 时
if 下一个字符也是 '=':
    消费掉两个字符 → Token::EqualEqual (==)
else:
    消费掉一个字符 → Token::Assign (=)
```

### 2.7 注释处理

```rust
// 看到 '/' 时
if 下一个字符也是 '/':
    跳过所有字符直到换行 → 这是单行注释，不产生 Token
else if 下一个字符是 '*':
    跳过所有字符直到 '*/' → 这是多行注释，不产生 Token
else:
    Token::Slash (除号)
```

注释不会产生 Token —— 它们被直接丢弃，后续阶段完全看不到。

### 2.8 位置信息

每个 Token 都记录了它在源代码中的**行号和列号**：

```rust
pub struct Token {
    pub ty: TokenType,   // Token 类型
    pub line: usize,     // 第几行（从1开始）
    pub col: usize,      // 第几列（Token起始位置）
}
```

这样报错时就能告诉你：`Unexpected token '+' at 3:15`（第3行第15列有意外的 `+`）。

位置记录的是 Token 的**起始**位置。比如：

```
  let x = 42;
  ^    ^   ^
  1:1  1:5 1:9
```

---

## 第三章：语法分析（Parser）

### 3.1 什么是语法分析

Lexer 只切词，不管语法对不对。比如 `let = 5;` 切出来是：

```
[let] [=] [5] [;]
```

四个合法的 Token，但语法是错的 —— `let` 后面应该跟变量名。

Parser 的工作就是检查 Token 序列是否**符合语法规则**，并构建**语法树**。

### 3.2 递归下降解析

Parser 使用**递归下降（Recursive Descent）**方法。核心思想是：

> 每种语句/表达式都有一个专门的解析函数。

```
parse_program()    → 循环调用 parse_statement()
parse_statement()  → 根据当前 Token 分发：
    Token::Let     → parse_let_statement()
    Token::If      → parse_if_statement()
    Token::While   → parse_while_statement()
    Token::Print   → parse_print_statement()
    其他           → parse_expression_statement()
```

### 3.3 表达式与运算符优先级

这是 Parser 最难的部分。考虑这个表达式：

```
1 + 2 * 3
```

按照数学规则，应该先算 `2 * 3`，再算 `1 + 6`，结果是 `7`。

Parser 是怎么知道"先算乘法再算加法"的？靠**优先级层次**。

表达式解析从最低优先级开始，每层调用上一层：

```
表达式
  ↓ 调用
赋值 (=)          ← 最低优先级
  ↓ 调用
逻辑或 (or)
  ↓ 调用
逻辑与 (and)
  ↓ 调用
相等 (==, !=)
  ↓ 调用
比较 (<, <=, >, >=)
  ↓ 调用
加减 (+, -)
  ↓ 调用
乘除 (*, /, %)    ← 较高优先级
  ↓ 调用
一元 (-, not)     ← 更高优先级
  ↓ 调用
基本项 (数字, 变量, 括号)  ← 最高优先级
```

### 3.4 优先级如何工作

以 `1 + 2 * 3` 为例：

```
parse_expression()
  → parse_assignment()
    → parse_or()
      → parse_and()
        → parse_equality()
          → parse_comparison()
            → parse_term()  ← 加减优先级
              → parse_factor()  ← 乘除优先级
                → parse_unary()
                  → parse_primary()
                    看到 1，返回 Literal(1)

              ← 回到 parse_term
              看到 +，继续调用 parse_factor()
                → parse_primary()  看到 2，返回 Literal(2)
                ← 回到 parse_factor
                看到 *，调用 parse_unary()
                  → parse_primary()  看到 3，返回 Literal(3)
                返回 Binary(2, *, 3)

              返回 Binary(1, +, Binary(2, *, 3))
```

关键：`parse_term`（处理加减）在递归调用 `parse_factor`（处理乘除）时，
`parse_factor` 先把 `2 * 3` 吃掉了，所以加法看到的右操作数是 `2*3` 的结果。

这就是为什么 `+` 的 AST 节点是：

```
    +
   / \
  1   *
     / \
    2   3
```

而不是：

```
      *
     / \
    +   3
   / \
  1   2
```

### 3.5 结合性

`a - b - c` 应该算 `(a - b) - c` 还是 `a - (b - c)`？

左结合：`(a - b) - c`，所以 `10 - 3 - 2 = 5`

实现方式：在每层用 `while` 循环，左结合地处理相同优先级的运算符：

```rust
// parse_term 中的 while 循环（左结合）
let mut left = parse_factor()?;  // 解析左操作数
while matches!(peek(), Plus | Minus) {
    let op = advance();          // 吃掉运算符
    let right = parse_factor()?; // 解析右操作数
    left = Binary(left, op, right);  // 把左操作数和右操作数组合
}
return left;  // 最终的 left 是左结合的结果
```

### 3.6 错误恢复

如果代码有多处语法错误，Parser 不能在第一个错误就停止。

**错误恢复（Synchronize）** 的做法：

```
遇到错误
  → 记录错误信息
  → 跳过 Token 直到找到一个"安全点"
  → 继续解析
```

"安全点"是指：分号 `;` 或者关键字（`let`、`if`、`while` 等），这些通常是新语句的开始。

```rust
fn synchronize(&mut self) {
    while not EOF {
        if 当前 Token 是 ';' → 跳过它，返回（准备解析下一条语句）
        if 当前 Token 是关键字 → 返回（准备解析下一条语句）
        else → 跳过这个 Token（它可能是错误的残留）
    }
}
```

---

## 第四章：抽象语法树（AST）

### 4.1 什么是 AST

AST（Abstract Syntax Tree，抽象语法树）是用**树形结构**表示代码含义的数据结构。

```
源代码:  let x = 1 + 2;

AST:
    ┌──────────────────┐
    │ Let {            │
    │   name: "x",     │
    │   initializer:   │
    │     Binary {     │
    │       left:  1,  │
    │       op:    +,  │
    │       right: 2   │
    │     }            │
    │ }                │
    └──────────────────┘
```

为什么叫"抽象"？因为 Token 中的 `let`、空格、`=`、`;` 这些**语法细节**都被丢弃了，
只剩下**语义信息**：声明一个叫 `x` 的变量，初始值为 `1 + 2`。

### 4.2 两种节点类型

Hul 的 AST 有两种节点：

**Stmt（语句）** — 不产生值，产生副作用

```
Let       → let x = expr;      // 声明变量
Assign    → x = expr;           // 给变量赋值
Print     → print expr;         // 打印值
If        → if (cond) { } else { }
While     → while (cond) { }
For       → for (init; cond; update) { }
Block     → { stmt1; stmt2; ... }
Return    → return expr;
Break     → break;
Continue  → continue;
Function  → fn name(params) { }
Expression → expr;  // 表达式语句（值被丢弃）
```

**Expr（表达式）** — 计算并返回一个值

```
Literal   → 42, "hello", true, nil   // 字面量
Variable  → x                         // 变量引用
Assign    → x = expr                  // 赋值也返回值
Binary    → a + b, a * b, a == b     // 二元运算
Unary     → -x, not x                 // 一元运算
Logical   → a and b, a or b          // 逻辑运算（支持短路）
Grouping  → (expr)                    // 括号分组
Call      → fn(a, b)                 // 函数调用
```

### 4.3 语句和表达式的区别

一个简单的判断方法：

- 语句不关心"值是多少"，只关心"做了什么"
- 表达式必须有"值"的概念

```
let x = 1;     ← 这是语句（做了一件事：声明变量）
1 + 2           ← 这是表达式（有值：3）
print x;        ← 这是语句（做了一件事：打印）
x = 5           ← 这既是语句（赋值）又是表达式（值为5）
```

### 4.4 AST 节点定义示例

```rust
pub enum Stmt {
    // let x = expr;
    Let {
        name: String,           // 变量名
        initializer: Expr,      // 初始化表达式
    },
    // if (cond) { ... } else { ... }
    If {
        condition: Expr,            // 条件表达式
        then_branch: Vec<Stmt>,     // if 分支的语句列表
        else_branch: Option<Vec<Stmt>>,  // else 分支（可选）
    },
    // while (cond) { ... }
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    // ... 其他语句类型
}

pub enum Expr {
    // 42, "hello", true, nil
    Literal(Value),
    // x, myVar
    Variable(String),
    // a + b, a * b, x == y
    Binary {
        left: Box<Expr>,        // 左操作数（用 Box 因为表达式大小不确定）
        operator: BinaryOp,     // 运算符
        right: Box<Expr>,       // 右操作数
    },
    // -x, not x
    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
    // (expr)
    Grouping(Box<Expr>),
    // fn(arg1, arg2)
    Call {
        callee: Box<Expr>,      // 被调用的表达式
        arguments: Vec<Expr>,   // 参数列表
    },
    // ...
}
```

### 4.5 Box 的作用

Rust 中 `Box<Expr>` 表示"在堆上分配的表达式"。

为什么需要 `Box`？因为 `Expr` 是递归的：

```rust
// 如果不加 Box，编译器无法确定 Expr 的大小
pub enum Expr {
    Binary {
        left: Expr,  // Expr 里面套 Expr...
        // 编译器: 这个结构要多大？不知道！
    },
}

// 加 Box 后，编译器知道 left 只是一个指针（8字节）
pub enum Expr {
    Binary {
        left: Box<Expr>,  // 指针大小是固定的
    },
}
```

### 4.6 完整 AST 示例

对于 `let x = 1 + 2 * 3;`：

```
Stmt::Let {
    name: "x",
    initializer: Expr::Binary {
        left: Expr::Literal(Number(1.0)),
        operator: Add,
        right: Expr::Binary {
            left: Expr::Literal(Number(2.0)),
            operator: Mul,
            right: Expr::Literal(Number(3.0)),
        },
    },
}
```

---

## 第五章：解释执行（Interpreter）

### 5.1 什么是解释执行

有了 AST，解释器就遍历这棵树，对每个节点执行对应的操作。

```
AST 节点              执行操作
─────────────────────────────────────
Literal(42)           直接返回 42
Binary(1, +, 2)       先算左=1，再算右=2，然后 1+2=3
Let {x, 1+2}          先算 1+2=3，然后把 3 存到变量 x
If {cond, then, else}  先算条件，根据真假选择执行 then 还是 else
While {cond, body}     循环：算条件，真则执行 body，再算条件...
```

### 5.2 递归遍历

Interpreter 用递归方式遍历 AST：

```rust
fn eval_expr(&mut self, expr: &Expr) -> Result<ValueRef, String> {
    match expr {
        // 字面量：直接返回值
        Expr::Literal(val) => Ok(new_value_ref(val.clone())),

        // 变量：从环境中查找
        Expr::Variable(name) => self.env.get(name),

        // 二元运算：递归求值左右操作数，然后运算
        Expr::Binary { left, operator, right } => {
            let l = self.eval_expr(left)?;   // 递归！
            let r = self.eval_expr(right)?;  // 递归！
            apply_operator(l, operator, r)    // 运算
        },
        // ...
    }
}
```

### 5.3 短路求值

`and` 和 `or` 运算符支持**短路求值**：

```
false and print "never"   ← 不会执行 print，因为 and 遇到 false 直接短路
true  or  print "never"   ← 不会执行 print，因为 or 遇到 true 直接短路
```

实现方式：

```rust
Expr::Logical { left, operator, right } => {
    let l = self.eval_expr(left)?;
    match operator {
        LogicalOp::And => {
            if !is_truthy(&l) {
                return Ok(l);  // 左边为假，直接返回左边（短路！）
            }
            self.eval_expr(right)  // 左边为真，继续算右边
        }
        LogicalOp::Or => {
            if is_truthy(&l) {
                return Ok(l);  // 左边为真，直接返回左边（短路！）
            }
            self.eval_expr(right)  // 左边为假，继续算右边
        }
    }
}
```

### 5.4 控制流信号

执行语句时可能产生三种"控制流信号"：

```rust
enum ControlFlow {
    Normal,            // 正常执行，继续下一条语句
    Break,             // break 语句，跳出当前循环
    Continue,          // continue 语句，跳过本次循环
    Return(ValueRef),  // return 语句，从函数返回值
}
```

这些信号需要在调用链中向上传播：

```
exec_block → exec_stmt → exec_block → exec_stmt → ...
                      ↗                              ↘
                  break!                            break语句
                  (向上传播)                         (产生信号)
```

`exec_block` 中遇到非 Normal 的信号时停止当前块，把信号返回给外层：

```rust
fn exec_block(&mut self, stmts: &[Stmt]) -> Result<ControlFlow, String> {
    for stmt in stmts {
        let flow = self.exec_stmt(stmt)?;
        match flow {
            ControlFlow::Normal => continue,  // 正常，继续
            _ => return Ok(flow),             // 有信号，向上传播
        }
    }
    Ok(ControlFlow::Normal)
}
```

### 5.5 循环如何处理 break/continue

```
while (cond) {
    if (...) { break; }     ← 产生 Break 信号
    if (...) { continue; }  ← 产生 Continue 信号
    正常语句...
}
```

While 处理器的逻辑：

```rust
loop {
    if !condition_is_true() { break; }  // Rust 的 break（退出循环）

    match exec_block(body)? {
        ControlFlow::Normal   => {}      // 正常，继续迭代
        ControlFlow::Break    => break,  // hul 的 break → Rust 的 break
        ControlFlow::Continue => continue,// hul 的 continue → Rust 的 continue
        ControlFlow::Return(v) => return Ok(Return(v)), // return 传到函数级
    }
}
```

---

## 第六章：值类型系统（Value）

### 6.1 用枚举表示所有类型

Hul 的值用 Rust 的枚举来表示：

```rust
pub enum Value {
    Number(f64),       // 数字：42, 3.14
    String(String),    // 字符串："hello"
    Boolean(bool),     // 布尔：true, false
    Function(Function),// 函数对象
    Nil,               // 空值：nil
}
```

为什么用 `f64` 而不是整数类型？因为脚本语言通常不区分整数和浮点数：

- `1 + 2` 的结果是 `3.0`
- `10 / 4` 的结果是 `2.5`

### 6.2 真值判断

不同语言对"真"和"假"的定义不同。Hul 的规则：


| 值               | 真/假 |
| ---------------- | ----- |
| `nil`            | 假    |
| `false`          | 假    |
| `0`              | 假    |
| `""`（空字符串） | 假    |
| 其他所有值       | 真    |

```rust
pub fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil         => false,
        Value::Boolean(b)  => *b,
        Value::Number(n)   => *n != 0.0,
        Value::String(s)   => !s.is_empty(),
        _ => true,
    }
}
```

### 6.3 字符串拼接

`+` 运算符对数字做加法，对字符串做拼接：

```rust
match (left, right) {
    (Number(a), Number(b)) => Number(a + b),       // 数字加法
    (String(a), String(b)) => String(a + b),       // 字符串拼接
    (String(a), Number(b)) => String(format!("{}{}", a, b)), // 自动转字符串
    (Number(a), String(b)) => String(format!("{}{}", a, b)), // 自动转字符串
    _ => Err("类型错误"),
}
```

所以 `"age=" + 25` 结果是 `"age=25"`。

---

## 第七章：变量与作用域（Environment）

### 7.1 变量存在哪里

解释器用一个叫 **Environment**（环境）的结构来存变量：

```rust
pub struct Environment {
    bindings: HashMap<String, ValueRef>,   // 变量名 → 值的映射
    parent: Option<Rc<RefCell<Environment>>>,  // 指向外层环境的指针
}
```

可以把 Environment 想象成一个字典：

```
┌───────────────────────────┐
│ Environment               │
│  bindings: {              │
│    "x" → Number(10.0)    │
│    "y" → String("hi")    │
│    "sum" → Number(30.0)  │
│  }                        │
│  parent: → (外层环境)      │
└───────────────────────────┘
```

### 7.2 作用域链

当代码块嵌套时，内层环境的 `parent` 指向外层环境：

```hu
let x = 1;           // 存在外层环境
{
    let y = 2;       // 存在内层环境
    print x + y;     // 查找 x：内层没有 → 查外层 → 找到
}
```

对应的环境链：

```
内层 Environment          外层 Environment
┌──────────────┐         ┌──────────────┐
│ y → 2        │ ──────→ │ x → 1        │
│ parent: ──────┘         │ parent: None  │
└──────────────┘         └──────────────┘
```

查找变量 `x` 时：先在内层找 → 没有 → 去 parent（外层）找 → 找到了！

### 7.3 查找 vs 定义 vs 赋值

三种操作的含义不同：

**定义（define）**：在当前作用域创建新变量

```rust
fn define(&mut self, name: String, value: ValueRef) {
    self.bindings.insert(name, value);  // 直接插入当前环境
}
```

**查找（get）**：沿作用域链向上查找

```rust
fn get(&self, name: &str) -> Result<ValueRef, String> {
    if let Some(val) = self.bindings.get(name) {
        return Ok(val.clone());  // 当前层找到了
    }
    if let Some(parent) = &self.parent {
        return parent.borrow().get(name);  // 递归查外层
    }
    Err(format!("Undefined variable '{}'", name))  // 找遍了都没找到
}
```

**赋值（assign）**：沿作用域链查找并修改已存在的变量

```rust
fn assign(&mut self, name: &str, value: ValueRef) -> Result<(), String> {
    if self.bindings.contains_key(name) {
        self.bindings.insert(name.to_string(), value);  // 当前层找到了，修改
        Ok(())
    } else if let Some(parent) = &self.parent {
        parent.borrow_mut().assign(name, value)  // 递归查外层
    } else {
        Err(format!("Undefined variable '{}'", name))
    }
}
```

关键区别：`define` 只在当前作用域操作，`assign` 沿作用域链查找。

### 7.4 作用域何时创建/销毁

```
代码块 { stmt1; stmt2; }:
    进入块 → 创建新环境（parent=旧环境）
    执行语句 → 使用新环境
    离开块 → 恢复旧环境（新环境被丢弃）

函数调用 fn(a, b):
    进入函数 → 创建新环境（parent=闭包环境）
    绑定参数 → a=arg1, b=arg2
    执行函数体
    离开函数 → 恢复旧环境
```

### 7.5 ValueRef —— 引用计数

变量的值用 `Rc<RefCell<Value>>` 包装：

```rust
pub type ValueRef = Rc<RefCell<Value>>;
```

- **`Rc`**（Reference Counted）：允许多个变量共享同一个值。当引用计数归零时自动释放。
- **`RefCell`**：允许在运行时修改值（内部可变性）。

为什么要这样？考虑这种情况：

```hu
let a = 42;
let b = a;       // b 和 a 指向同一个值
a = 100;         // 修改 a
print b;         // b 还是 42（独立的）
```

`Rc` 让我们能安全地在多处共享值，`RefCell` 让我们能在不可变引用时修改值。

---

## 第八章：函数与闭包

### 8.1 函数是什么

在 Hul 中，函数是一种值，就像数字和字符串一样：

```rust
pub struct Function {
    pub name: String,              // 函数名
    pub params: Vec<String>,       // 参数名列表
    pub body: Vec<Stmt>,           // 函数体语句列表
    pub closure: Rc<RefCell<Environment>>,  // 定义时的环境（闭包）
}
```

函数在内部被存为 `Value::Function(Function)`，所以它可以像普通值一样传递：

```hu
fn add(a, b) { return a + b; }
let f = add;           // 把函数赋值给变量
print f(3, 4);         // 7
```

### 8.2 函数调用过程

```
调用 add(3, 4)：

1. 找到 add 的函数对象
2. 检查参数个数：期望 2 个，得到 2 个 ✓
3. 保存当前环境（old_env）
4. 创建新环境，parent 指向函数定义时的环境（闭包）
5. 绑定参数：a = 3, b = 4
6. 执行函数体
7. 遇到 return → 恢复旧环境，返回值
```

### 8.3 闭包是什么

闭包（Closure）是指函数"记住"了它被定义时的环境。

```hu
fn make_counter() {
    let count = 0;
    fn increment() {
        count = count + 1;   // 这里的 count 是 make_counter 里的 count！
        return count;
    }
    return increment;
}

let counter = make_counter();
print counter();  // 1
print counter();  // 2
```

`increment` 函数被定义在 `make_counter` 内部。当 `make_counter` 执行完毕后，
它的局部变量 `count` 本应该被销毁，但 `increment` 函数的 `closure` 字段
保存了对那个环境的引用，所以 `count` 活了下来。

### 8.4 Return 信号如何传播

```
fn factorial(n) {
    if (n <= 1) { return 1; }          ← 产生 Return 信号
    return n * factorial(n - 1);       ← 递归调用，也可能产生 Return 信号
}
```

`return` 产生 `ControlFlow::Return(value)` 信号。

这个信号必须**穿过所有中间层**，一直传到**函数调用处**才被消费：

```
函数体 → exec_block → exec_stmt → ... → call_function
                                     ↑
                              消费 Return 信号，返回值
```

如果 `return` 穿过了所有语句到达 `interpret()`（顶层），就会报错：
`"Unexpected return outside function"`

---

## 第九章：控制流

### 9.1 if-else

```
if (condition) {
    then_branch
} else {
    else_branch
}
```

实现：条件为真执行 then，否则执行 else（如果有的话）。

`else if` 不需要特殊处理 —— 它是 `else` 后面跟一个嵌套的 `if`：

```hu
if (a > 0) { ... } else if (a < 0) { ... } else { ... }
// 等价于：
if (a > 0) {
    ...
} else {
    if (a < 0) {     // else 后面直接跟 if 语句
        ...
    } else {
        ...
    }
}
```

### 9.2 while 循环

```
while (condition) {
    body
}
```

Rust 实现（无限循环直到条件为假）：

```rust
loop {
    if !is_truthy(condition) { break; }  // Rust 的 break
    exec_block(body)?;                    // 执行循环体
}
```

### 9.3 for 循环

```
for (init; condition; update) {
    body
}
```

for 循环在执行时被"脱糖"（desugar）为等价的 while 结构：

```hu
// 你写的：
for (let i = 0; i < 10; i = i + 1) { print i; }

// 解释器内部等价于：
{
    let i = 0;
    while (i < 10) {
        print i;
        i = i + 1;
    }
}
```

### 9.4 break 和 continue

`break` 和 `continue` 通过 `ControlFlow` 信号传播：

- **`break`** → 产生 `ControlFlow::Break` → 循环处理器收到后退出循环
- **`continue`** → 产生 `ControlFlow::Continue` → 循环处理器收到后跳到下一次迭代

关键：**只跳出最内层循环**。嵌套循环中，内层的 break 不会影响外层：

```hu
for (let i = 0; i < 3; i = i + 1) {
    for (let j = 0; j < 3; j = j + 1) {
        if (j == 1) { break; }  // 只跳出内层 for
    }
}
```

---

## 第十章：错误处理

### 10.1 错误的两种来源

**编译时错误**（词法/语法分析阶段）：

- 非法字符：`Unexpected character '@'`
- 未结束的字符串：`Unterminated string literal`
- 语法错误：`Expected ';' after expression`

**运行时错误**（解释执行阶段）：

- 未定义变量：`Undefined variable 'x'`
- 类型错误：`Operands for '+' must be numbers or strings`
- 循环外的 break：`'break' outside of loop`

### 10.2 错误信息的格式

```
错误描述 at 行号:列号
```

例如：

```
Unexpected token Plus at 3:15
```

行号和列号来自 Token 的位置信息（Lexer 阶段记录）。

### 10.3 错误恢复（Parser）

Parser 遇到语法错误时，不立即停止。而是：

1. 记录错误
2. 跳过 Token 直到 `;` 或关键字
3. 继续解析下一条语句
4. 最后把所有错误一起报告

```
代码：
let a = 1;
let = ;        ← 错误1：let 后面没有变量名
let b = 2;
+ 3;           ← 错误2：+ 不应该出现在语句开头
let c = a + b;

输出：
Expected variable name
Unexpected token Plus at 4:1
```

注意：即使有错误，已成功解析的部分（`let a = 1;`、`let b = 2;`）仍然被保留在 AST 中。

---

## 附录：完整执行流程图

以 `let x = 1 + 2 * 3;` 为例：

```
【1. 源代码】
let x = 1 + 2 * 3;

【2. Lexer → Token 流】
┌──────┬─────┬────────┬───────┬─────┬───────┬─────┬───────┬──┐
│ Let  │ Id(x)│ Assign │ Num(1)│ Plus│ Num(2)│ Star│ Num(3)│; │
└──────┴─────┴────────┴───────┴─────┴───────┴─────┴───────┴──┘

【3. Parser → AST】
Stmt::Let {
    name: "x",
    initializer: Expr::Binary {
        op: Add,
        left: Literal(1),
        right: Expr::Binary {
            op: Mul,
            left: Literal(2),
            right: Literal(3),
        },
    },
}

【4. Interpreter → 执行】
eval_expr(Binary(1, +, Binary(2, *, 3)))
  → eval_expr(Literal(1))           = 1
  → eval_expr(Binary(2, *, 3))
      → eval_expr(Literal(2))       = 2
      → eval_expr(Literal(3))       = 3
      → 2 * 3                       = 6
  → 1 + 6                           = 7
env.define("x", 7)

【5. 结果】
变量 x 的值为 7
```

---

## 关键数据结构一览

```
源代码 String
    ↓ Lexer
Vec<Token>  (Token 流)
    ↓ Parser
Vec<Stmt>   (AST 根节点)
    ↓ Interpreter
执行副作用 (print 输出、变量赋值等)
```

```rust
Token    { ty: TokenType, line: usize, col: usize }
Stmt     Let | Assign | Print | If | While | For | Block | Function | Return | Break | Continue | Expression
Expr     Literal | Variable | Assign | Binary | Unary | Logical | Grouping | Call
Value    Number(f64) | String(String) | Boolean(bool) | Function(Function) | Nil
Environment { bindings: HashMap<String, ValueRef>, parent: Option<...> }
ControlFlow Normal | Break | Continue | Return(ValueRef)
```
