# Hul 语言语法参考手册

本文档详细描述 `hul` 语言的每一条语法规则。配合 `PRINCIPLES.md`（原理详解）一起阅读效果更好。

---

## 目录

- [1. 基本元素](#1-基本元素)
- [2. 关键字与保留字](#2-关键字与保留字)
- [3. 表达式](#3-表达式)
- [4. 语句](#4-语句)
- [5. 作用域规则](#5-作用域规则)
- [6. 真值规则](#6-真值规则)
- [7. 错误处理](#7-错误处理)
- [8. 完整示例](#8-完整示例)

---

## 1. 基本元素

### 1.1 注释

注释是写给人看的说明文字，解释器会完全忽略它们。

**单行注释**：从 `//` 开始到行尾

```hu
// 这是一行注释
let x = 10;  // 这是行尾注释
```

**多行注释**：从 `/*` 开始到 `*/` 结束，可以跨越多行

```hu
/* 这是多行注释
   可以写很多行 */
let y = 20;

let z = 30; /* 行内多行注释 */
```

**注意**：注释不能嵌套。`/* /* 嵌套 */ */` 中，内层 `/*` 被当作注释内容，
外层第一个 `*/` 结束注释，后面的 `*/` 会导致语法错误。

### 1.2 标识符

标识符（Identifier）是给变量、函数等起的名字。

**命名规则**：
- 以字母或下划线 `_` 开头
- 后续字符可以是字母、数字或下划线
- 不能与关键字同名

```hu
// 合法的标识符
let x = 1;
let _value = 2;
let myVar = 3;
let count2 = 4;
let _ = 5;

// 非法的标识符
// let 123abc = 1;    ← 错误：不能以数字开头
// let my-var = 1;    ← 错误：不能包含连字符
// let let = 1;       ← 错误：let 是关键字
```

### 1.3 字面量

字面量（Literal）是在代码中直接写出来的值。

#### 数字

只有一种数字类型：64 位浮点数（`f64`）。整数和小数都是同一种类型。

```hu
let a = 42;       // 整数
let b = 3.14;     // 小数
let c = 0.5;      // 前导零
let d = -7;       // 负数（实际上是 unary - 运算）
```

**注意**：没有单独的整数类型。`10 / 4` 的结果是 `2.5`，不是 `2`。

#### 字符串

用双引号 `"` 包裹的文本。单引号不支持。

```hu
let s1 = "hello";
let s2 = "";       // 空字符串
let s3 = "hello world!";
```

**转义序列**：在字符串中，反斜杠 `\` 有特殊含义，用于表示无法直接输入的字符：

| 转义 | 含义 | 示例 |
|------|------|------|
| `\n` | 换行符 | `"line1\nline2"` → 两行 |
| `\t` | 制表符 | `"name\tage"` → name 和 age 之间有 Tab |
| `\\` | 反斜杠本身 | `"C:\\Users"` → C:\Users |
| `\"` | 双引号本身 | `"say \"hi\""` → say "hi" |
| `\r` | 回车符 | `"hello\rworld"` |

```hu
let msg = "第一行\n第二行";
print msg;
// 输出:
// 第一行
// 第二行

let path = "C:\\Users\\test";
print path;  // C:\Users\test
```

如果写了不认识的转义（如 `\q`），会报错：`Unknown escape '\q'`。

#### 布尔值

只有两个值：`true` 和 `false`。

```hu
let ok = true;
let no = false;
```

#### nil

`nil` 表示"没有值"，类似于其他语言的 `null`。

```hu
let x;       // x 的值为 nil
let y = nil; // 显式赋值为 nil
```

### 1.4 运算符一览

| 优先级 | 类别 | 运算符 | 说明 |
|--------|------|--------|------|
| 1（最低）| 赋值 | `=` | 右结合 |
| 2 | 逻辑 | `or` | 短路求值 |
| 3 | 逻辑 | `and` | 短路求值 |
| 4 | 相等 | `==` `!=` | |
| 5 | 比较 | `<` `<=` `>` `>=` | |
| 6 | 加减 | `+` `-` | 左结合 |
| 7 | 乘除 | `*` `/` `%` | 左结合 |
| 8 | 一元 | `-` `not` | 右结合 |
| 9（最高）| 括号 | `()` | |

**左结合**：`a - b - c` 等价于 `(a - b) - c`

**右结合**：`a = b = c` 等价于 `a = (b = c)`

---

## 2. 关键字与保留字

以下关键字不能用作变量名或函数名：

| 关键字 | 用途 | 示例 |
|--------|------|------|
| `let` | 声明变量 | `let x = 1;` |
| `fn` | 声明函数 | `fn add(a, b) { ... }` |
| `return` | 函数返回 | `return 42;` |
| `if` | 条件判断 | `if (x > 0) { ... }` |
| `else` | 条件分支 | `else { ... }` |
| `while` | while 循环 | `while (x < 10) { ... }` |
| `for` | for 循环 | `for (let i = 0; i < 10; i = i + 1) { ... }` |
| `break` | 跳出循环 | `break;` |
| `continue` | 跳过本次迭代 | `continue;` |
| `print` | 输出值 | `print x;` |
| `and` | 逻辑与 | `a and b` |
| `or` | 逻辑或 | `a or b` |
| `not` | 逻辑非 | `not x` |
| `true` | 布尔真 | `let ok = true;` |
| `false` | 布尔假 | `let no = false;` |
| `nil` | 空值 | `let x = nil;` |

---

## 3. 表达式

表达式（Expression）是"计算后会产生一个值"的代码片段。

### 3.1 字面量表达式

最简单的表达式，直接写出值：

```hu
42           // 值为 42.0
3.14         // 值为 3.14
"hello"      // 值为 "hello"
true         // 值为 true
nil          // 值为 nil
```

### 3.2 变量引用

用标识符引用已声明的变量：

```hu
let x = 10;
print x;     // x 是一个表达式，值为 10
print x + 5; // x + 5 是一个表达式，值为 15
```

如果引用了未定义的变量，会报错：`Undefined variable 'xxx'`。

### 3.3 算术运算

```hu
print 10 + 3;    // 13 — 加法
print 10 - 3;    // 7  — 减法
print 10 * 3;    // 30 — 乘法
print 10 / 3;    // 3.333... — 除法
print 10 % 3;    // 1  — 取模（求余数）
print -5;        // -5 — 一元取负
```

**字符串拼接**：`+` 也可以拼接字符串。如果一边是字符串，另一边会被自动转为字符串：

```hu
print "hello" + " " + "world";  // hello world
print "result=" + 42;           // result=42
print 100 + "%";                // 100%
```

**类型限制**：`-`、`*`、`/`、`%` 只能用于数字。`"hello" - 1` 会报错。

### 3.4 比较运算

比较运算的结果是布尔值 `true` 或 `false`。

```hu
print 1 == 1;       // true  — 等于
print 1 != 2;       // true  — 不等于
print 3 < 5;        // true  — 小于
print 3 <= 3;       // true  — 小于等于
print 5 > 3;        // true  — 大于
print 5 >= 5;       // true  — 大于等于
```

比较运算符可以比较数字和布尔值：

```hu
print true == true;   // true
print nil == nil;     // true
print nil == false;   // false
```

### 3.5 逻辑运算

```hu
// and：两边都为真才为真
print true and true;    // true
print true and false;   // false
print false and true;   // false

// or：有一边为真就为真
print true or false;    // true
print false or false;   // false

// not：取反
print not true;         // false
print not false;        // true
```

**短路求值**：`and` 遇到左边为假时不再算右边，`or` 遇到左边为真时不再算右边：

```hu
// and 的短路：左边是 false，右边不会执行
false and print "不会输出";

// or 的短路：左边是 true，右边不会执行
true or print "不会输出";
```

这对避免错误很有用：

```hu
if (x != nil and x > 0) {
    // 当 x 为 nil 时，and 短路，x > 0 不会执行，不会报错
}
```

### 3.6 分组表达式

用圆括号 `()` 改变运算优先级：

```hu
print 1 + 2 * 3;     // 7  — 先算 2*3=6，再算 1+6=7
print (1 + 2) * 3;   // 9  — 先算 1+2=3，再算 3*3=9
```

### 3.7 赋值表达式

赋值 `=` 本身也是一个表达式（值为被赋的值）：

```hu
let x;
print x = 10;        // 10 — 赋值表达式的值就是被赋的值

// 链式赋值
let a, b;
a = b = 5;           // b=5，然后 a=5
print a;  // 5
print b;  // 5
```

### 3.8 函数调用

```hu
fn add(a, b) { return a + b; }

print add(3, 4);              // 7
print add(add(1, 2), 3);      // 6 — 嵌套调用
```

调用规则：
- 参数个数必须和函数定义一致，否则报错 `Expected N arguments but got M`
- 如果函数没有 `return` 语句，返回值为 `nil`

---

## 4. 语句

语句（Statement）是"执行后产生副作用（如打印、赋值）但不产生值"的代码片段。
每条语句以分号 `;` 结尾（代码块 `{}` 除外）。

### 4.1 变量声明

```hu
let x = 10;          // 声明并初始化
let y;               // 声明，值为 nil
let a = 1 + 2 * 3;  // 初始化表达式可以是任意表达式
```

`let` 声明的变量存在于当前作用域。如果同名变量在外层已存在，内层的 `let` 会**遮蔽**（shadow）外层的：

```hu
let x = 1;
{
    let x = 2;       // 这是一个新变量，遮蔽了外层的 x
    print x;         // 2
}
print x;             // 1 — 外层的 x 没变
```

### 4.2 赋值语句

```hu
let x = 10;
x = 20;              // 修改已存在的变量
x = x + 5;           // 用变量的旧值计算新值
```

赋值目标**必须是已存在的变量**，否则报错：
```hu
// 未声明就赋值 → 错误
// y = 10;  // Undefined variable 'y'

// 应该先 let
let y;
y = 10;    // OK
```

### 4.3 打印语句

```hu
print 42;            // 42
print "hello";       // hello
print x + 1;         // 计算 x+1 后打印
print true;          // true
print nil;           // nil
```

`print` 不是函数调用（没有括号），而是一个关键字语句。

### 4.4 条件语句

```hu
// 基本 if
if (x > 0) {
    print "positive";
}

// if-else
if (x > 0) {
    print "positive";
} else {
    print "non-positive";
}

// if - else if - else
if (x > 0) {
    print "positive";
} else if (x < 0) {
    print "negative";
} else {
    print "zero";
}
```

**`else if` 的原理**：没有单独的 `else if` 语法。`else if` 是 `else` 后面跟一个嵌套的 `if`：

```hu
// 这两段代码完全等价：

// 写法1：else if
if (a) { ... } else if (b) { ... } else { ... }

// 写法2：嵌套 if（解释器内部就是这样处理的）
if (a) {
    ...
} else {
    if (b) {
        ...
    } else {
        ...
    }
}
```

**省略大括号**：如果分支只有一条语句，可以省略大括号：

```hu
if (x > 0)
    print x;       // 单条语句，不需要大括号
```

但建议始终加上大括号，代码更清晰。

### 4.5 while 循环

```hu
let i = 0;
while (i < 5) {
    print i;
    i = i + 1;
}
// 输出: 0 1 2 3 4
```

执行流程：

```
1. 计算条件 i < 5
2. 如果条件为假 → 跳出循环
3. 如果条件为真 → 执行循环体
4. 回到第1步
```

**无限循环**：

```hu
while (true) {
    // 用 break 跳出
}
```

### 4.6 for 循环

```hu
for (初始化; 条件; 更新) {
    循环体
}
```

三部分都可以省略：

```hu
// 标准 for
for (let i = 0; i < 10; i = i + 1) {
    print i;
}

// 省略初始化（变量在外面声明）
let i = 0;
for (; i < 10; i = i + 1) {
    print i;
}

// 省略更新（在循环体内更新）
for (let i = 0; i < 10;) {
    print i;
    i = i + 1;
}

// 省略条件（无限循环，用 break 跳出）
for (let i = 0;; i = i + 1) {
    if (i >= 5) { break; }
    print i;
}

// 全部省略（无限循环）
let running = true;
for (;;) {
    if (not running) { break; }
}
```

**for 循环的执行流程**：

```
1. 执行初始化（只执行一次）
2. 计算条件
3. 如果条件为假 → 跳出循环
4. 执行循环体
5. 执行更新
6. 回到第2步
```

**作用域**：`for` 循环的初始化中声明的变量，在整个 for 循环中可见（类似 C 语言）：

```hu
for (let i = 0; i < 5; i = i + 1) {
    print i;     // OK，i 在循环体内可见
}
// print i;     // 错误：i 在循环外不可见
```

### 4.7 break 和 continue

**`break`**：立即跳出当前循环

```hu
for (let i = 0; i < 100; i = i + 1) {
    if (i == 5) { break; }  // 到 5 就跳出
    print i;
}
// 输出: 0 1 2 3 4
```

**`continue`**：跳过本次迭代的剩余部分，直接进入下一次迭代

```hu
for (let i = 0; i < 6; i = i + 1) {
    if (i == 3) { continue; }  // 跳过 3
    print i;
}
// 输出: 0 1 2 4 5
```

**注意**：`break` 和 `continue` 只影响最内层循环：

```hu
for (let i = 0; i < 3; i = i + 1) {
    for (let j = 0; j < 3; j = j + 1) {
        if (j == 1) { break; }  // 只跳出内层 for
        print i * 10 + j;
    }
}
// 输出: 0  10  20
// i=0: j=0 输出0, j=1 break
// i=1: j=0 输出10, j=1 break
// i=2: j=0 输出20, j=1 break
```

**在循环外使用会报错**：

```hu
// break;  ← 'break' outside of loop
// continue;  ← 'continue' outside of loop
```

### 4.8 函数声明

```hu
fn 函数名(参数1, 参数2, ...) {
    语句1;
    语句2;
    return 返回值;
}
```

示例：

```hu
// 无参数函数
fn say_hello() {
    print "hello!";
}

// 带参数函数
fn add(a, b) {
    return a + b;
}

// 无返回值（隐式返回 nil）
fn greet(name) {
    print "hello " + name;
}
print greet("hul");  // 先打印 "hello hul"，然后打印 nil
```

**递归函数**：

```hu
fn factorial(n) {
    if (n <= 1) { return 1; }
    return n * factorial(n - 1);
}
print factorial(5);  // 120
```

递归的工作方式：每次调用创建新的作用域，`n` 在每次调用中是独立的：

```
factorial(5)
  → 5 * factorial(4)
       → 4 * factorial(3)
            → 3 * factorial(2)
                 → 2 * factorial(1)
                      → return 1
                 → return 2 * 1 = 2
            → return 3 * 2 = 6
       → return 4 * 6 = 24
  → return 5 * 24 = 120
```

**函数是一等公民**：函数可以赋值给变量、作为参数传递：

```hu
fn add(a, b) { return a + b; }
let f = add;          // 把函数赋给变量
print f(3, 4);        // 7
```

### 4.9 return 语句

```hu
fn check(x) {
    if (x > 0) { return "positive"; }
    if (x < 0) { return "negative"; }
    return "zero";
}
```

`return` 会立即结束函数执行，把值返回给调用者。

**没有 return 的函数**：执行完最后一条语句后返回 `nil`。

**在循环外使用 return 会报错**：`"Unexpected return outside function"`

### 4.10 代码块

用 `{}` 包裹的语句序列，创建一个新的作用域：

```hu
let x = 1;
{
    let y = 2;
    print x + y;  // 3
}
// print y;  // 错误：y 在块外不可见
print x;      // 1
```

代码块可以嵌套：

```hu
{
    let a = 1;
    {
        let b = 2;
        {
            let c = 3;
            print a + b + c;  // 6
        }
    }
}
```

### 4.11 表达式语句

任何表达式后面跟 `;` 都可以作为语句，值被丢弃：

```hu
1 + 2;           // 计算了 3，但没有用
add(3, 4);       // 函数返回 7，但没有用
```

这在调用不关心返回值的函数时有用：

```hu
fn log(msg) {
    print msg;
}
log("started");  // 不需要返回值
```

---

## 5. 作用域规则

### 5.1 什么是作用域

作用域（Scope）决定了变量在哪里可见。可以把它想象成"变量的可见范围"。

```
全局作用域（最外层）
├── let x = 1;        ← x 在全局可见
│
└── {                  ← 块作用域
    ├── let y = 2;    ← y 在这个块内可见
    │
    └── {             ← 嵌套块作用域
        └── let z = 3; ← z 在这个块内可见
    }
}
```

### 5.2 作用域链查找

当使用一个变量时，解释器从当前作用域开始，逐层向外查找：

```hu
let x = 1;
{
    let y = 2;
    {
        print x;     // 当前层没有 x → 外层没有 x → 最外层有 x = 1
        print y;     // 当前层没有 y → 外层有 y = 2
    }
}
```

如果查找链的所有层都没有，报错：`Undefined variable 'xxx'`。

### 5.3 变量遮蔽

内层作用域可以用 `let` 声明同名变量，遮蔽外层的：

```hu
let x = 1;
{
    let x = 2;       // 新变量，遮蔽外层 x
    print x;         // 2
}
print x;             // 1 — 外层 x 没变
```

### 5.4 赋值 vs 声明

- `let x = ...`：在当前作用域**创建**新变量
- `x = ...`：**修改**已存在的变量（沿作用域链查找）

```hu
let x = 1;
{
    x = 2;        // 修改外层的 x（不是创建新变量）
}
print x;          // 2 — 外层 x 被修改了
```

```hu
let x = 1;
{
    let x = 2;   // 创建新变量（遮蔽外层）
}
print x;          // 1 — 外层 x 没变
```

---

## 6. 真值规则

### 6.1 哪些值为真，哪些为假

| 值 | 用于条件时 |
|---|-----------|
| `nil` | 假 |
| `false` | 假 |
| `0` | 假 |
| `""`（空字符串） | 假 |
| `true` | 真 |
| 非零数字 | 真 |
| 非空字符串 | 真 |

### 6.2 在条件语句中使用

```hu
if (0) { print "不会执行"; }
if (1) { print "会执行"; }
if ("") { print "不会执行"; }
if ("hello") { print "会执行"; }
if (nil) { print "不会执行"; }
if (false) { print "不会执行"; }
```

### 6.3 在逻辑运算中使用

`and` 和 `or` 返回操作数本身，而不是强制转为布尔值：

```hu
let a = 0 or 42;       // a = 42（0 为假，返回右边）
let b = 1 or 42;       // b = 1（1 为真，短路返回左边）
let c = 0 and 42;      // c = 0（0 为假，短路返回左边）
let d = 1 and 42;      // d = 42（1 为真，返回右边）
```

---

## 7. 错误处理

### 7.1 编译时错误（语法分析阶段）

这些错误在代码执行前就会被发现：

| 错误信息 | 含义 | 示例 |
|----------|------|------|
| `Expected variable name` | `let` 后面缺少变量名 | `let = 1;` |
| `Expected ';' after ...` | 语句末尾缺少分号 | `let x = 1` |
| `Expected '('` | 缺少左括号 | `if x > 0 {` |
| `Expected ')'` | 缺少右括号 | `if (x > 0 {` |
| `Expected '{'` | 缺少左大括号 | `if (x > 0) print x;`（多语句时） |
| `Unexpected token ...` | 出现了意外的 Token | `let x = + + 1;` |
| `Invalid assignment target` | 赋值号左边不是变量 | `1 + 2 = 3;` |

### 7.2 运行时错误

这些错误在代码执行时才被发现：

| 错误信息 | 含义 | 示例 |
|----------|------|------|
| `Undefined variable 'x'` | 使用了未定义的变量 | `print x;`（x 未声明）|
| `Operands for '+' must be numbers or strings` | 运算符类型不匹配 | `true + false` |
| `Operands for '<' must be numbers` | 比较运算只支持数字 | `"a" < "b"` |
| `'break' outside of loop` | 循环外使用 break | `break;` |
| `'continue' outside of loop` | 循环外使用 continue | `continue;` |
| `Unexpected return outside function` | 函数外使用 return | `return 1;` |
| `Expected N arguments but got M` | 函数参数个数不匹配 | `add(1)` 但 add 期望 2 个参数 |
| `Can only call functions` | 调用了非函数值 | `let x = 1; x();` |
| `Unknown escape '\q'` | 字符串中出现未知转义 | `"\q"` |

### 7.3 错误恢复

当代码中有多个语法错误时，解释器会尝试报告所有错误：

```hu
let a = 1;
let = ;         // 错误1: Expected variable name
let b = 2;
+ 3;            // 错误2: Unexpected token Plus
```

输出：
```
Expected variable name
Unexpected token Plus at 4:1
```

**工作原理**：遇到语法错误后，解释器跳过 Token 直到找到 `;` 或关键字，然后继续解析下一条语句。

---

## 8. 完整示例

### 8.1 计算阶乘

```hu
fn factorial(n) {
    if (n <= 1) { return 1; }
    return n * factorial(n - 1);
}

for (let i = 1; i <= 10; i = i + 1) {
    print i + "! = " + factorial(i);
}
```

输出：
```
1! = 1
2! = 2
3! = 6
4! = 24
5! = 120
6! = 720
7! = 5040
8! = 40320
9! = 362880
10! = 3628800
```

### 8.2 斐波那契数列

```hu
fn fib(n) {
    if (n <= 1) { return n; }
    return fib(n - 1) + fib(n - 2);
}

for (let i = 0; i < 10; i = i + 1) {
    print "fib(" + i + ") = " + fib(i);
}
```

### 8.3 找素数

```hu
fn is_prime(n) {
    if (n < 2) { return false; }
    let i = 2;
    while (i * i <= n) {
        if (n % i == 0) { return false; }
        i = i + 1;
    }
    return true;
}

print "素数：";
for (let n = 2; n < 30; n = n + 1) {
    if (is_prime(n)) { print n; }
}
```

输出：
```
素数：
2
3
5
7
11
13
17
19
23
29
```

### 8.4 字符串处理

```hu
// 字符串拼接
let name = "hul";
let version = 1.0;
print "欢迎使用 " + name + " v" + version;

// 转义符
let path = "C:\\Users\\test\\file.txt";
let multi = "第一行\n第二行\t缩进";
print path;
print multi;
```

### 8.5 综合示例：FizzBuzz

```hu
for (let i = 1; i <= 20; i = i + 1) {
    if (i % 15 == 0) {
        print "FizzBuzz";
    } else if (i % 3 == 0) {
        print "Fizz";
    } else if (i % 5 == 0) {
        print "Buzz";
    } else {
        print i;
    }
}
```

---

## 附录：语法规则速查表

```
程序        → 语句*
语句        → let 声明 | 赋值 | print | if | while | for | break | continue
            | return | fn 声明 | 块语句 | 表达式语句
let 声明    → 'let' 标识符 ('=' 表达式)? ';'
赋值        → 标识符 '=' 表达式 ';'
print       → 'print' 表达式 ';'
if          → 'if' '(' 表达式 ')' 块 ('else' (if | 块))?
while       → 'while' '(' 表达式 ')' 块
for         → 'for' '(' (let 声明 | 表达式 ';')? 表达式? ';' 表达式? ')' 块
break       → 'break' ';'
continue    → 'continue' ';'
return      → 'return' 表达式 ';'
fn 声明     → 'fn' 标识符 '(' 参数列表 ')' 块
块          → '{' 语句* '}'
表达式语句  → 表达式 ';'

表达式      → 赋值表达式
赋值表达式  → 逻辑或 ('=' 赋值表达式)?
逻辑或      → 逻辑与 ('or' 逻辑与)*
逻辑与      → 相等 ('and' 相等)*
相等        → 比较 (('==' | '!=') 比较)*
比较        → 加减 (('<' | '<=' | '>' | '>=') 加减)*
加减        → 乘除 (('+' | '-') 乘除)*
乘除        → 一元 (('*' | '/' | '%') 一元)*
一元        → ('-' | 'not') 一元 | 调用
调用        → 基本项 ('(' 参数列表 ')')*
基本项      → 数字 | 字符串 | true | false | nil | 标识符 | '(' 表达式 ')'
```
