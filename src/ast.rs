/// 抽象语法树（AST）模块 - 定义 Hul 语言的语法结构
///
/// 该模块包含：
/// - Stmt 枚举：表示各种语句类型（变量声明、条件、循环等）
/// - Expr 枚举：表示各种表达式类型（字面量、运算、变量等）
/// - BinaryOp/UnaryOp/LogicalOp 枚举：表示运算符类型
use crate::value::Value;

/// 语句枚举 - 表示 Hul 语言中的所有语句类型
///
/// # 变体
/// - `Let`: 变量声明语句，如 `let x = 10;`
/// - `Assign`: 变量赋值语句，如 `x = 20;`
/// - `Print`: 打印输出语句，如 `print(x);`
/// - `If`: 条件分支语句，支持 if-else 和 else-if
/// - `While`: 循环语句，当条件为真时重复执行
/// - `Block`: 代码块语句，由大括号包围的多个语句
/// - `Expression`: 表达式语句，单独的表达式（通常用于赋值）
#[derive(Debug, Clone)]
pub enum Stmt {
    /// 变量声明语句
    /// - `name`: 变量名称
    /// - `initializer`: 初始化表达式
    Let { name: String, initializer: Expr },
    /// 变量赋值语句
    /// - `name`: 要赋值的变量名
    /// - `value`: 赋值的表达式
    Assign { name: String, value: Expr },
    /// 打印输出语句
    /// - 内部包含要打印的表达式
    Print(Expr),
    /// 条件分支语句
    /// - `condition`: 条件表达式
    /// - `then_branch`: 条件为真时执行的语句块
    /// - `else_branch`: 条件为假时执行的语句块（可选）
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    /// 循环语句
    /// - `condition`: 循环条件表达式
    /// - `body`: 循环体语句块
    While { condition: Expr, body: Vec<Stmt> },
    /// 代码块语句
    /// - 包含一系列语句，形成新的作用域
    Block(Vec<Stmt>),
    /// 表达式语句
    /// - 将表达式作为独立语句执行（常用于赋值表达式）
    Expression(Expr),
}

/// 表达式枚举 - 表示 Hul 语言中的所有表达式类型
///
/// # 变体
/// - `Literal`: 字面量表达式（数字、字符串、布尔值、nil）
/// - `Variable`: 变量引用表达式
/// - `Binary`: 二元运算表达式（算术、比较运算）
/// - `Unary`: 一元运算表达式（取负、逻辑非）
/// - `Logical`: 逻辑运算表达式（and、or，支持短路求值）
/// - `Grouping`: 分组表达式（圆括号括起来的子表达式）
#[derive(Debug, Clone)]
pub enum Expr {
    /// 字面量表达式
    /// - 直接包含一个 Value 值
    Literal(Value),
    /// 变量引用表达式
    /// - 包含变量名称字符串
    Variable(String),
    /// 二元运算表达式
    /// - `left`: 左操作数
    /// - `operator`: 二元运算符
    /// - `right`: 右操作数
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    /// 一元运算表达式
    /// - `operator`: 一元运算符
    /// - `right`: 操作数
    Unary { operator: UnaryOp, right: Box<Expr> },
    /// 逻辑运算表达式（支持短路求值）
    /// - `left`: 左操作数
    /// - `operator`: 逻辑运算符（And/Or）
    /// - `right`: 右操作数
    Logical {
        left: Box<Expr>,
        operator: LogicalOp,
        right: Box<Expr>,
    },
    /// 分组表达式
    /// - 用于改变运算优先级，如 `(a + b) * c`
    Grouping(Box<Expr>),
}

/// 二元运算符枚举
///
/// # 算术运算符
/// - `Add`: 加法 `+`
/// - `Sub`: 减法 `-`
/// - `Mul`: 乘法 `*`
/// - `Div`: 除法 `/`
/// - `Mod`: 取模 `%`
///
/// # 比较运算符
/// - `Equal`: 相等 `==`
/// - `NotEqual`: 不等 `!=`
/// - `Less`: 小于 `<`
/// - `LessEqual`: 小于等于 `<=`
/// - `Greater`: 大于 `>`
/// - `GreaterEqual`: 大于等于 `>=`
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

/// 一元运算符枚举
///
/// # 变体
/// - `Negate`: 数值取负，如 `-x`
/// - `Not`: 逻辑取反，如 `not x`
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

/// 逻辑运算符枚举
///
/// # 变体
/// - `And`: 逻辑与，支持短路求值
/// - `Or`: 逻辑或，支持短路求值
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
}
