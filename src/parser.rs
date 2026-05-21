use crate::ast::*;
/// 语法分析器模块 - 将 Token 流转换为抽象语法树（AST）
///
/// 该模块包含：
/// - Parser 结构体：实现递归下降解析器
/// - Pratt 解析算法：处理运算符优先级和结合性
///
/// 支持的语法结构：
/// - 变量声明和赋值
/// - 算术、比较、逻辑运算
/// - 条件语句（if-else）
/// - 循环语句（while）
/// - 代码块和作用域
/// - 打印输出语句
use crate::lexer::{Lexer, Token, TokenType};
use crate::value::Value;

/// 语法分析器结构体
///
/// 采用递归下降解析策略，通过 Pratt 解析算法处理运算符优先级。
/// 维护一个 Token 列表和当前扫描位置。
pub struct Parser {
    /// 词法分析产生的 Token 列表
    tokens: Vec<Token>,
    /// 当前处理的 Token 索引
    current: usize,
}

impl Parser {
    /// 创建新的语法分析器
    ///
    /// # 参数
    /// - `source`: 源代码字符串
    ///
    /// # 返回
    /// 初始化后的 Parser 实例，已完成词法分析
    pub fn new(source: &str) -> Result<Self, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.collect_tokens()?;
        Ok(Parser { tokens, current: 0 })
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// 查看当前 Token 的类型（不移动位置）
    ///
    /// # 返回
    /// 当前 Token 的类型，如果已到达末尾则返回 Eof
    fn peek(&self) -> TokenType {
        self.current_token()
            .map(|t| t.ty.clone())
            .unwrap_or(TokenType::Eof)
    }

    /// 前进到下一个 Token
    ///
    /// # 返回
    /// 当前 Token 的副本，并将内部指针前移
    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.current].clone();
        self.current += 1;
        tok
    }

    /// 检查当前 Token 是否为指定类型
    ///
    /// # 参数
    /// - `ty`: 要检查的 Token 类型
    ///
    /// # 返回
    /// 如果当前 Token 类型匹配则返回 true
    fn check(&self, ty: TokenType) -> bool {
        self.peek() == ty
    }

    /// 期望当前 Token 为指定类型，否则报错
    ///
    /// # 参数
    /// - `expected`: 期望的 Token 类型
    /// - `msg`: 错误消息
    ///
    /// # 返回
    /// - `Ok(Token)`: 匹配成功，返回当前 Token 并前进
    /// - `Err(String)`: 匹配失败，返回错误信息
    fn expect(&mut self, expected: TokenType, msg: &str) -> Result<Token, String> {
        if self.check(expected) {
            Ok(self.advance())
        } else if let Some(tok) = self.current_token() {
            Err(format!("{} at {:?} ({}:{})", msg, tok.ty, tok.line, tok.col))
        } else {
            Err(format!("{} at end of input", msg))
        }
    }

    // ==================== 语句解析 ====================

    /// 解析整个程序
    ///
    /// 持续解析语句直到遇到 EOF
    ///
    /// # 返回
    /// 包含所有语句的向量
    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        while !self.check(TokenType::Eof) {
            stmts.push(self.statement()?);
        }
        Ok(stmts)
    }

    /// 解析单个语句
    ///
    /// 根据当前 Token 类型分发到不同的语句解析函数
    ///
    /// # 返回
    /// 解析后的语句节点
    fn statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            TokenType::Let => self.let_statement(),
            TokenType::Fn => self.fn_statement(),
            TokenType::Return => self.return_statement(),
            TokenType::If => self.if_statement(),
            TokenType::While => self.while_statement(),
            TokenType::Print => self.print_statement(),
            TokenType::LBrace => self.block_statement(),
            _ => self.expression_statement(),
        }
    }

    /// 解析代码块语句 `{ ... }`
    ///
    /// 解析由大括号包围的多个语句序列
    ///
    /// # 返回
    /// Block 语句节点
    fn block_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::LBrace, "Expected '{'")?;
        let mut stmts = Vec::new();
        while !self.check(TokenType::RBrace) && !self.check(TokenType::Eof) {
            stmts.push(self.statement()?);
        }
        self.expect(TokenType::RBrace, "Expected '}'")?;
        Ok(Stmt::Block(stmts))
    }

    /// 解析变量声明语句 `let name = expr;`
    ///
    /// 支持可选的初始化表达式，如果没有初始化则默认为 nil
    ///
    /// # 返回
    /// Let 语句节点
    fn let_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::Let, "Expected 'let'")?;
        let name = self.expect_identifier("Expected variable name")?;
        let initializer = if self.check(TokenType::Assign) {
            self.advance();
            self.expression()?
        } else {
            Expr::Literal(Value::Nil)
        };
        self.expect(TokenType::Semicolon, "Expected ';' after let")?;
        Ok(Stmt::Let { name, initializer })
    }

    /// 解析打印语句 `print expr;`
    ///
    /// # 返回
    /// Print 语句节点
    fn print_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::Print, "Expected 'print'")?;
        let expr = self.expression()?;
        self.expect(TokenType::Semicolon, "Expected ';' after print")?;
        Ok(Stmt::Print(expr))
    }

    /// 解析条件语句 `if (cond) { ... } else { ... }`
    ///
    /// 支持：
    /// - 基本 if 语句
    /// - if-else 语句
    // / - else-if 链式结构
    ///
    /// # 返回
    /// If 语句节点
    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::If, "Expected 'if'")?;
        self.expect(TokenType::LParen, "Expected '('")?;
        let condition = self.expression()?;
        self.expect(TokenType::RParen, "Expected ')'")?;
        let then_branch = self.block()?;
        let else_branch = if self.check(TokenType::Else) {
            self.advance();
            if self.check(TokenType::If) {
                // else if 情况：递归解析 if 语句
                vec![self.if_statement()?]
            } else {
                // else 情况：解析代码块
                self.block()?
            }
        } else {
            vec![]
        };
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch: if else_branch.is_empty() {
                None
            } else {
                Some(else_branch)
            },
        })
    }

    /// 解析循环语句 `while (cond) { ... }`
    ///
    /// # 返回
    /// While 语句节点
    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::While, "Expected 'while'")?;
        self.expect(TokenType::LParen, "Expected '('")?;
        let condition = self.expression()?;
        self.expect(TokenType::RParen, "Expected ')'")?;
        let body = self.block()?;
        Ok(Stmt::While { condition, body })
    }

    /// 解析代码块（可以是显式的 `{...}` 或单条语句）
    ///
    /// 如果遇到左大括号，解析为多语句块；否则解析为单条语句
    ///
    /// # 返回
    /// 语句向量
    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        if self.check(TokenType::LBrace) {
            match self.block_statement()? {
                Stmt::Block(stmts) => Ok(stmts),
                _ => unreachable!(),
            }
        } else {
            // 单条语句当块处理
            let stmt = self.statement()?;
            Ok(vec![stmt])
        }
    }

    /// 解析表达式语句 `expr;`
    ///
    /// 表达式后必须跟分号，常用于赋值表达式
    ///
    /// # 返回
    /// 语句节点
    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.expect(TokenType::Semicolon, "Expected ';'")?;

        if let Expr::Assign { name, value } = expr {
            return Ok(Stmt::Assign { name, value: *value });
        }

        Ok(Stmt::Expression(expr))
    }

    // ==================== Pratt 解析表达式 ====================

    /// 解析表达式的入口函数
    ///
    /// 从最低优先级的赋值表达式开始解析
    ///
    /// # 返回
    /// 解析后的表达式节点
    fn expression(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    /// 解析函数调用或者基础表达式
    fn parse_call(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        while self.check(TokenType::LParen) {
            expr = self.finish_call(expr)?;
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        self.expect(TokenType::LParen, "Expected '(' after function name")?;
        let mut arguments = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                arguments.push(self.expression()?);
                if self.check(TokenType::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenType::RParen, "Expected ')' after arguments")?;
        Ok(Expr::Call {
            callee: Box::new(callee),
            arguments,
        })
    }

    /// 解析赋值表达式 `var = expr`
    ///
    /// 赋值是右结合的，支持链式赋值
    /// 注意：这里使用 Binary 节点临时存储，解释器会特殊处理
    ///
    /// # 返回
    /// 表达式节点（可能是赋值或其他低优先级表达式）
    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_or()?;
        if self.check(TokenType::Assign) {
            self.advance();
            let value = self.parse_assignment()?;
            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }
            return Err("Invalid assignment target".to_string());
        }
        Ok(expr)
    }

    // ==================== 各优先级层（从低到高）====================

    /// 解析逻辑或表达式 `expr or expr`
    ///
    /// 优先级低于 and，支持短路求值
    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.check(TokenType::Or) {
            let op = match self.advance().ty {
                TokenType::Or => LogicalOp::Or,
                _ => unreachable!(),
            };
            let right = self.parse_and()?;
            left = Expr::Logical {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// 解析逻辑与表达式 `expr and expr`
    ///
    /// 优先级低于比较运算，支持短路求值
    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        while self.check(TokenType::And) {
            let op = match self.advance().ty {
                TokenType::And => LogicalOp::And,
                _ => unreachable!(),
            };
            let right = self.parse_equality()?;
            left = Expr::Logical {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// 解析相等性比较 `==` 和 `!=`
    ///
    /// 优先级低于其他比较运算符
    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        while matches!(self.peek(), TokenType::EqualEqual | TokenType::BangEqual) {
            let op = match self.advance().ty {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// 解析比较运算符 `<`, `<=`, `>`, `>=`
    ///
    /// 优先级低于加减运算
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        while matches!(
            self.peek(),
            TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual
        ) {
            let op = match self.advance().ty {
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// 解析加减运算 `+` 和 `-`
    ///
    /// 优先级低于乘除取模运算
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
        while matches!(self.peek(), TokenType::Plus | TokenType::Minus) {
            let op = match self.advance().ty {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// 解析乘除取模运算 `*`, `/`, `%`
    ///
    /// 优先级高于一元运算符
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        while matches!(
            self.peek(),
            TokenType::Star | TokenType::Slash | TokenType::Percent
        ) {
            let op = match self.advance().ty {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                TokenType::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// 解析一元运算符 `-` 和 `not`
    ///
    /// 右结合，可以嵌套使用，如 `-(-x)` 或 `not not x`
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), TokenType::Minus | TokenType::Not) {
            let op = match self.advance().ty {
                TokenType::Minus => UnaryOp::Negate,
                TokenType::Not => UnaryOp::Not,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            Ok(Expr::Unary {
                operator: op,
                right: Box::new(right),
            })
        } else {
            self.parse_call()
        }
    }

    /// 解析初级表达式（最高优先级）
    ///
    /// 包括：
    /// - 字面量（数字、字符串、布尔值、nil）
    /// - 变量引用
    /// - 分组表达式（圆括号）
    ///
    /// # 返回
    /// 基础表达式节点
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            TokenType::Number(n) => {
                self.advance();
                Ok(Expr::Literal(Value::Number(n)))
            }
            TokenType::String(s) => {
                self.advance();
                Ok(Expr::Literal(Value::String(s)))
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal(Value::Boolean(true)))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal(Value::Boolean(false)))
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal(Value::Nil))
            }
            TokenType::Identifier(name) => {
                self.advance();
                Ok(Expr::Variable(name))
            }
            TokenType::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.expect(TokenType::RParen, "Expected ')'")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => {
                if let Some(tok) = self.current_token() {
                    Err(format!("Unexpected token {:?} at {}:{}", tok.ty, tok.line, tok.col))
                } else {
                    Err("Unexpected end of input".to_string())
                }
            }
        }
    }

    fn fn_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::Fn, "Expected 'fn'")?;
        let name = self.expect_identifier("Expected function name")?;
        self.expect(TokenType::LParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                params.push(self.expect_identifier("Expected parameter name")?);
                if self.check(TokenType::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenType::RParen, "Expected ')' after parameters")?;
        let body = match self.block_statement()? {
            Stmt::Block(stmts) => stmts,
            _ => unreachable!(),
        };
        Ok(Stmt::Function { name, params, body })
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::Return, "Expected 'return'")?;
        let value = self.expression()?;
        self.expect(TokenType::Semicolon, "Expected ';' after return")?;
        Ok(Stmt::Return(value))
    }

    /// 期望当前 Token 为标识符
    ///
    /// # 参数
    /// - `msg`: 错误消息
    ///
    /// # 返回
    /// - `Ok(String)`: 标识符名称
    /// - `Err(String)`: 错误信息
    fn expect_identifier(&mut self, msg: &str) -> Result<String, String> {
        if let TokenType::Identifier(name) = self.peek() {
            self.advance();
            Ok(name)
        } else {
            Err(msg.to_string())
        }
    }
}
