/// 解释器模块 - 执行抽象语法树（AST）
///
/// 该模块包含：
/// - Interpreter 结构体：负责遍历 AST 并执行语义操作
/// - 环境管理：通过 Environment 维护变量作用域链
/// - 表达式求值：支持算术、比较、逻辑运算
/// - 语句执行：支持变量声明、赋值、条件、循环、打印等
use crate::ast::*;
use crate::value::*;
use std::cell::RefCell;
use std::rc::Rc;

/// 解释器结构体
///
/// 维护当前的执行环境（作用域链），负责：
/// - 遍历和执行语句
/// - 求值表达式
/// - 管理变量绑定和作用域
pub struct Interpreter {
    /// 当前执行环境的引用（支持嵌套作用域）
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    /// 创建新的解释器实例
    ///
    /// # 返回
    /// 初始化后的解释器，带有空的顶层环境
    pub fn new() -> Self {
        Interpreter {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    /// 执行程序
    ///
    /// 依次执行所有顶层语句
    ///
    /// # 参数
    /// - `stmts`: 语句向量（AST 根节点）
    ///
    /// # 返回
    /// - `Ok(())`: 执行成功
    /// - `Err(String)`: 运行时错误信息
    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

    /// 执行单个语句
    ///
    /// 根据语句类型分发到不同的执行函数
    ///
    /// # 参数
    /// - `stmt`: 要执行的语句节点
    ///
    /// # 返回
    /// - `Ok(())`: 执行成功
    /// - `Err(String)`: 运行时错误
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            // 变量声明：求值初始化表达式并在当前作用域定义变量
            Stmt::Let { name, initializer } => {
                let value = self.eval_expr(initializer)?;
                self.env.borrow_mut().define(name.clone(), value);
                Ok(())
            }
            // 变量赋值：求值右侧表达式并更新已有变量
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.borrow_mut().assign(name, val)
            }
            // 打印语句：求值表达式并输出结果
            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                println!("{}", val.borrow());
                Ok(())
            }
            // 条件语句：根据条件真值选择执行分支
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = self.eval_expr(condition)?;
                if is_truthy(&cond.borrow()) {
                    self.exec_block(then_branch)
                } else if let Some(else_stmts) = else_branch {
                    self.exec_block(else_stmts)
                } else {
                    Ok(())
                }
            }
            // 循环语句：当条件为真时重复执行循环体
            Stmt::While { condition, body } => {
                loop {
                    let cond = self.eval_expr(condition)?;
                    if !is_truthy(&cond.borrow()) {
                        break;
                    }
                    self.exec_block(body)?;
                }
                Ok(())
            }
            // 代码块：创建新的作用域，执行完毕后恢复原环境
            Stmt::Block(stmts) => {
                // 保存当前环境
                let old_env = self.env.clone();
                // 创建新环境，父环境指向旧环境
                self.env = Rc::new(RefCell::new(Environment::new_with_parent(old_env.clone())));
                // 执行块内语句
                let result = self.exec_block(stmts);
                // 恢复原环境（退出作用域）
                self.env = old_env;
                result
            }
            // 表达式语句：通常用于赋值表达式
            Stmt::Expression(expr) => {
                // 处理赋值表达式，如 a = 5;
                if let Expr::Binary {
                    left,
                    operator: _,
                    right,
                } = expr
                {
                    if let Expr::Variable(name) = left.as_ref() {
                        let value = self.eval_expr(right)?;
                        return self.env.borrow_mut().assign(name, value);
                    }
                }
                // 其他表达式求值并丢弃结果
                self.eval_expr(expr)?;
                Ok(())
            }
        }
    }

    /// 执行语句块
    ///
    /// 依次执行块中的所有语句
    ///
    /// # 参数
    /// - `stmts`: 语句向量
    ///
    /// # 返回
    /// - `Ok(())`: 全部执行成功
    /// - `Err(String)`: 执行错误
    fn exec_block(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

    /// 求值表达式
    ///
    /// 递归地计算表达式的值，返回 ValueRef
    ///
    /// # 参数
    /// - `expr`: 要计算的表达式节点
    ///
    /// # 返回
    /// - `Ok(ValueRef)`: 计算结果的值引用
    /// - `Err(String)`: 求值错误（如类型不匹配、未定义变量）
    fn eval_expr(&self, expr: &Expr) -> Result<ValueRef, String> {
        match expr {
            // 字面量：直接包装为 ValueRef
            Expr::Literal(val) => Ok(new_value_ref(val.clone())),
            // 变量：从环境中查找
            Expr::Variable(name) => self.env.borrow().get(name),
            // 分组表达式：递归求值内部表达式
            Expr::Grouping(e) => self.eval_expr(e),
            // 一元运算：先求值操作数，再应用运算符
            Expr::Unary { operator, right } => {
                let r = self.eval_expr(right)?;
                let r_val = r.borrow();
                let result = match operator {
                    // 取负：仅支持数值
                    UnaryOp::Negate => {
                        if let Value::Number(n) = *r_val {
                            Value::Number(-n)
                        } else {
                            return Err("Negation requires number".into());
                        }
                    }
                    // 逻辑非：对真值取反
                    UnaryOp::Not => Value::Boolean(!is_truthy(&r_val)),
                };
                Ok(new_value_ref(result))
            }
            // 二元运算：先求值左右操作数，再应用运算符
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                // 注意：赋值已经在 Expression 语句中处理，这里处理其他二元运算
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                let l_val = l.borrow();
                let r_val = r.borrow();
                let result = match operator {
                    // 算术运算：加减乘除取模
                    BinaryOp::Add => self.arith_binop(&l_val, &r_val, |a, b| a + b, "+")?,
                    BinaryOp::Sub => self.arith_binop(&l_val, &r_val, |a, b| a - b, "-")?,
                    BinaryOp::Mul => self.arith_binop(&l_val, &r_val, |a, b| a * b, "*")?,
                    BinaryOp::Div => self.arith_binop(&l_val, &r_val, |a, b| a / b, "/")?,
                    BinaryOp::Mod => self.arith_binop(&l_val, &r_val, |a, b| a % b, "%")?,
                    // 相等性比较
                    BinaryOp::Equal => Value::Boolean(*l_val == *r_val),
                    BinaryOp::NotEqual => Value::Boolean(*l_val != *r_val),
                    // 大小比较
                    BinaryOp::Less => self.cmp_binop(&l_val, &r_val, |a, b| a < b, "<")?,
                    BinaryOp::LessEqual => self.cmp_binop(&l_val, &r_val, |a, b| a <= b, "<=")?,
                    BinaryOp::Greater => self.cmp_binop(&l_val, &r_val, |a, b| a > b, ">")?,
                    BinaryOp::GreaterEqual => {
                        self.cmp_binop(&l_val, &r_val, |a, b| a >= b, ">=")?
                    }
                };
                Ok(new_value_ref(result))
            }
            // 逻辑运算：支持短路求值
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let l = self.eval_expr(left)?;
                let l_is_truthy = is_truthy(&l.borrow());
                match operator {
                    // and：如果左操作数为假，直接返回左操作数（短路）
                    LogicalOp::And => {
                        if !l_is_truthy {
                            Ok(l)
                        } else {
                            self.eval_expr(right)
                        }
                    }
                    // or：如果左操作数为真，直接返回左操作数（短路）
                    LogicalOp::Or => {
                        if l_is_truthy {
                            Ok(l)
                        } else {
                            self.eval_expr(right)
                        }
                    }
                }
            }
        }
    }

    /// 执行算术二元运算
    ///
    /// 确保两个操作数都是数值类型，然后应用给定的运算函数
    ///
    /// # 参数
    /// - `a`: 左操作数
    /// - `b`: 右操作数
    /// - `op`: 运算函数（如加法、减法等）
    /// - `name`: 运算符名称（用于错误提示）
    ///
    /// # 返回
    /// - `Ok(Value)`: 运算结果
    /// - `Err(String)`: 类型错误
    fn arith_binop<F>(&self, a: &Value, b: &Value, op: F, name: &str) -> Result<Value, String>
    where
        F: Fn(f64, f64) -> f64,
    {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(op(*x, *y))),
            _ => Err(format!("Operands for '{}' must be numbers", name)),
        }
    }

    /// 执行比较二元运算
    ///
    /// 确保两个操作数都是数值类型，然后应用给定的比较函数
    ///
    /// # 参数
    /// - `a`: 左操作数
    /// - `b`: 右操作数
    /// - `op`: 比较函数（如小于、大于等）
    /// - `name`: 运算符名称（用于错误提示）
    ///
    /// # 返回
    /// - `Ok(Value)`: 布尔结果
    /// - `Err(String)`: 类型错误
    fn cmp_binop<F>(&self, a: &Value, b: &Value, op: F, name: &str) -> Result<Value, String>
    where
        F: Fn(f64, f64) -> bool,
    {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(op(*x, *y))),
            _ => Err(format!("Operands for '{}' must be numbers", name)),
        }
    }
}
