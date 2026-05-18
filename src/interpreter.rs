use crate::ast::*;
use crate::value::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: Rc::new(RefCell::new(Environment::new())) }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, initializer } => {
                let value = self.eval_expr(initializer)?;
                self.env.borrow_mut().define(name.clone(), value);
                Ok(())
            }
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.borrow_mut().assign(name, val)
            }
            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                println!("{}", val.borrow());
                Ok(())
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let cond = self.eval_expr(condition)?;
                if is_truthy(&cond.borrow()) {
                    self.exec_block(then_branch)
                } else if let Some(else_stmts) = else_branch {
                    self.exec_block(else_stmts)
                } else {
                    Ok(())
                }
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond = self.eval_expr(condition)?;
                    if !is_truthy(&cond.borrow()) { break; }
                    self.exec_block(body)?;
                }
                Ok(())
            }
            Stmt::Block(stmts) => {
                // 创建新作用域
                let old_env = self.env.clone();
                self.env = Rc::new(RefCell::new(Environment::new_with_parent(old_env.clone())));
                let result = self.exec_block(stmts);
                self.env = old_env;
                result
            }
            Stmt::Expression(expr) => {
                // 处理赋值表达式，如 a = 5;
                if let Expr::Binary { left, operator: _, right } = expr {
                    if let Expr::Variable(name) = left.as_ref() {
                        let value = self.eval_expr(right)?;
                        return self.env.borrow_mut().assign(name, value);
                    }
                }
                // 其他表达式求值并丢弃
                self.eval_expr(expr)?;
                Ok(())
            }
        }
    }

    fn exec_block(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

    fn eval_expr(&self, expr: &Expr) -> Result<ValueRef, String> {
        match expr {
            Expr::Literal(val) => Ok(new_value_ref(val.clone())),
            Expr::Variable(name) => self.env.borrow().get(name),
            Expr::Grouping(e) => self.eval_expr(e),
            Expr::Unary { operator, right } => {
                let r = self.eval_expr(right)?;
                let r_val = r.borrow();
                let result = match operator {
                    UnaryOp::Negate => {
                        if let Value::Number(n) = *r_val {
                            Value::Number(-n)
                        } else {
                            return Err("Negation requires number".into());
                        }
                    }
                    UnaryOp::Not => Value::Boolean(!is_truthy(&r_val)),
                };
                Ok(new_value_ref(result))
            }
            Expr::Binary { left, operator, right } => {
                // 注意：赋值已经在 Expression 语句中处理，这里处理其他二元运算
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                let l_val = l.borrow();
                let r_val = r.borrow();
                let result = match operator {
                    BinaryOp::Add => self.arith_binop(&l_val, &r_val, |a, b| a + b, "+")?,
                    BinaryOp::Sub => self.arith_binop(&l_val, &r_val, |a, b| a - b, "-")?,
                    BinaryOp::Mul => self.arith_binop(&l_val, &r_val, |a, b| a * b, "*")?,
                    BinaryOp::Div => self.arith_binop(&l_val, &r_val, |a, b| a / b, "/")?,
                    BinaryOp::Mod => self.arith_binop(&l_val, &r_val, |a, b| a % b, "%")?,
                    BinaryOp::Equal => Value::Boolean(*l_val == *r_val),
                    BinaryOp::NotEqual => Value::Boolean(*l_val != *r_val),
                    BinaryOp::Less => self.cmp_binop(&l_val, &r_val, |a, b| a < b, "<")?,
                    BinaryOp::LessEqual => self.cmp_binop(&l_val, &r_val, |a, b| a <= b, "<=")?,
                    BinaryOp::Greater => self.cmp_binop(&l_val, &r_val, |a, b| a > b, ">")?,
                    BinaryOp::GreaterEqual => self.cmp_binop(&l_val, &r_val, |a, b| a >= b, ">=")?,
                };
                Ok(new_value_ref(result))
            }
            Expr::Logical { left, operator, right } => {
                let l = self.eval_expr(left)?;
                let l_is_truthy = is_truthy(&l.borrow());
                match operator {
                    LogicalOp::And => {
                        if !l_is_truthy { Ok(l) }
                        else { self.eval_expr(right) }
                    }
                    LogicalOp::Or => {
                        if l_is_truthy { Ok(l) }
                        else { self.eval_expr(right) }
                    }
                }
            }
        }
    }

    fn arith_binop<F>(&self, a: &Value, b: &Value, op: F, name: &str) -> Result<Value, String>
        where F: Fn(f64, f64) -> f64
    {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(op(*x, *y))),
            _ => Err(format!("Operands for '{}' must be numbers", name)),
        }
    }

    fn cmp_binop<F>(&self, a: &Value, b: &Value, op: F, name: &str) -> Result<Value, String>
        where F: Fn(f64, f64) -> bool
    {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(op(*x, *y))),
            _ => Err(format!("Operands for '{}' must be numbers", name)),
        }
    }
}