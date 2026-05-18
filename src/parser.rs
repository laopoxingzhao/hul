use crate::lexer::{Lexer, Token, TokenType};
use crate::ast::*;
use crate::value::Value;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> TokenType {
        self.tokens.get(self.current).map(|t| t.ty.clone()).unwrap_or(TokenType::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.current].clone();
        self.current += 1;
        tok
    }

    fn check(&self, ty: TokenType) -> bool {
        self.peek() == ty
    }

    fn expect(&mut self, expected: TokenType, msg: &str) -> Result<Token, String> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            let tok = self.tokens.get(self.current).cloned();
            Err(format!("{} at {:?}", msg, tok))
        }
    }

    // ---------- 语句 ----------
    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        while !self.check(TokenType::Eof) {
            stmts.push(self.statement()?);
        }
        Ok(stmts)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            TokenType::Let => self.let_statement(),
            TokenType::If => self.if_statement(),
            TokenType::While => self.while_statement(),
            TokenType::Print => self.print_statement(),
            TokenType::LBrace => self.block_statement(),
            _ => self.expression_statement(),
        }
    }

    fn block_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::LBrace, "Expected '{'")?;
        let mut stmts = Vec::new();
        while !self.check(TokenType::RBrace) && !self.check(TokenType::Eof) {
            stmts.push(self.statement()?);
        }
        self.expect(TokenType::RBrace, "Expected '}'")?;
        Ok(Stmt::Block(stmts))
    }

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

    fn print_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::Print, "Expected 'print'")?;
        let expr = self.expression()?;
        self.expect(TokenType::Semicolon, "Expected ';' after print")?;
        Ok(Stmt::Print(expr))
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::If, "Expected 'if'")?;
        self.expect(TokenType::LParen, "Expected '('")?;
        let condition = self.expression()?;
        self.expect(TokenType::RParen, "Expected ')'")?;
        let then_branch = self.block()?;
        let else_branch = if self.check(TokenType::Else) {
            self.advance();
            if self.check(TokenType::If) {
                // else if
                vec![self.if_statement()?]
            } else {
                self.block()?
            }
        } else {
            vec![]
        };
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch: if else_branch.is_empty() { None } else { Some(else_branch) },
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::While, "Expected 'while'")?;
        self.expect(TokenType::LParen, "Expected '('")?;
        let condition = self.expression()?;
        self.expect(TokenType::RParen, "Expected ')'")?;
        let body = self.block()?;
        Ok(Stmt::While { condition, body })
    }

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

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        // 允许赋值表达式
        if let Expr::Binary { .. } = &expr {} // 赋值由 expression 内的逻辑处理
        self.expect(TokenType::Semicolon, "Expected ';'")?;
        Ok(Stmt::Expression(expr))
    }

    // ---------- Pratt 解析表达式 ----------
    fn expression(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_or()?;
        if self.check(TokenType::Assign) {
            self.advance();
            let value = self.parse_assignment()?;
            if let Expr::Variable(name) = expr {
                // 正确保留变量名，让解释器能够识别并执行赋值
                return Ok(Expr::Binary {
                    left: Box::new(Expr::Variable(name)),
                    operator: BinaryOp::Add, // dummy operator, 解释器会特殊处理
                    right: Box::new(value),
                });
            } else {
                return Err("Invalid assignment target".to_string());
            }
        }
        Ok(expr)
    }

    // 各优先级层
    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.check(TokenType::Or) {
            let op = match self.advance().ty {
                TokenType::Or => LogicalOp::Or,
                _ => unreachable!(),
            };
            let right = self.parse_and()?;
            left = Expr::Logical { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        while self.check(TokenType::And) {
            let op = match self.advance().ty {
                TokenType::And => LogicalOp::And,
                _ => unreachable!(),
            };
            let right = self.parse_equality()?;
            left = Expr::Logical { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        while matches!(self.peek(), TokenType::EqualEqual | TokenType::BangEqual) {
            let op = match self.advance().ty {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            left = Expr::Binary { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        while matches!(self.peek(), TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual) {
            let op = match self.advance().ty {
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            left = Expr::Binary { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
        while matches!(self.peek(), TokenType::Plus | TokenType::Minus) {
            let op = match self.advance().ty {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            left = Expr::Binary { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        while matches!(self.peek(), TokenType::Star | TokenType::Slash | TokenType::Percent) {
            let op = match self.advance().ty {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                TokenType::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expr::Binary { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), TokenType::Minus | TokenType::Not) {
            let op = match self.advance().ty {
                TokenType::Minus => UnaryOp::Negate,
                TokenType::Not => UnaryOp::Not,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            Ok(Expr::Unary { operator: op, right: Box::new(right) })
        } else {
            self.parse_primary()
        }
    }

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
            TokenType::True => { self.advance(); Ok(Expr::Literal(Value::Boolean(true))) }
            TokenType::False => { self.advance(); Ok(Expr::Literal(Value::Boolean(false))) }
            TokenType::Nil => { self.advance(); Ok(Expr::Literal(Value::Nil)) }
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
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }

    fn expect_identifier(&mut self, msg: &str) -> Result<String, String> {
        if let TokenType::Identifier(name) = self.peek() {
            self.advance();
            Ok(name)
        } else {
            Err(msg.to_string())
        }
    }
}