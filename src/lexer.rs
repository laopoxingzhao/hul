/// Hul 语言的词法分析器（Lexer）
///
/// 将源代码字符串转换为 Token 流，为语法分析器提供输入。
/// 支持的语法特性：
/// - 字面量：数字、字符串、布尔值、nil
/// - 标识符：变量名、关键字
/// - 运算符：算术、比较、逻辑
/// - 注释：单行注释 `//`

#[derive(Debug,Clone, PartialEq)]
pub enum TokenType {
    /// 数字字面量，如 `42`、`3.14`
    Number(f64),
    /// 字符串字面量，如 `"hello"`
    String(String),
    /// 布尔值 true
    True,
    /// 布尔值 false
    False,
    /// 空值/无值
    Nil,
    /// 标识符（变量名、函数名等）
    Identifier(String),

    /// 关键字 `let`，用于变量声明
    Let,
    /// 关键字 `if`，用于条件判断
    If,
    /// 关键字 `else`，用于条件分支
    Else,
    /// 关键字 `while`，用于循环
    While,
    /// 关键字 `for`，用于循环
    For,
    /// 关键字 `break`，跳出循环
    Break,
    /// 关键字 `continue`，跳过本次循环
    Continue,
    /// 关键字 `print`，用于输出
    Print,
    /// 关键字 `fn`，用于函数声明
    Fn,
    /// 关键字 `return`，用于函数返回
    Return,
    /// 关键字 `and`，逻辑与运算
    And,
    /// 关键字 `or`，逻辑或运算
    Or,
    /// 关键字 `not`，逻辑非运算
    Not,

    // 运算符与分隔符
    /// 加法运算符 `+`
    Plus,
    /// 减法运算符 `-`
    Minus,
    /// 乘法运算符 `*`
    Star,
    /// 除法运算符 `/`
    Slash,
    /// 取模运算符 `%`
    Percent,
    /// 相等比较 `==`
    EqualEqual,
    /// 不相等比较 `!=`
    BangEqual,
    /// 小于比较 `<`
    Less,
    /// 小于等于比较 `<=`
    LessEqual,
    /// 大于比较 `>`
    Greater,
    /// 大于等于比较 `>=`
    GreaterEqual,
    /// 赋值运算符 `=`
    Assign,
    /// 语句结束符 `;`
    Semicolon,
    /// 逗号 `,`
    Comma,
    /// 左圆括号 `(`
    LParen,
    /// 右圆括号 `)`
    RParen,
    /// 左大括号 `{`
    LBrace,
    /// 右大括号 `}`
    RBrace,
    /// 源代码结束标记
    Eof,
}

/// 表示源代码中的一个 Token（词法单元）
///
/// 包含：
/// - `ty`: Token 的类型
/// - `line`: 所在的行号（从 1 开始）
/// - `col`: 所在的列号（从 1 开始）
#[derive(Debug, Clone)]
pub struct Token {
    /// Token 类型
    pub ty: TokenType,
    /// 所在行号
    pub line: usize,
    /// 所在列号
    pub col: usize,
}

/// 词法分析器
///
/// 将源代码字符串按字符顺序扫描，识别出各个 Token。
/// 采用状态机模式，支持：
/// - 空白字符跳过
/// - 单行注释跳过
/// - 数字字面量识别
/// - 标识符/关键字识别
/// - 字符串字面量识别
pub struct Lexer {
    /// 源代码字符列表
    chars: Vec<char>,
    /// 当前扫描位置
    pos: usize,
    /// 当前行号
    line: usize,
    /// 当前列号
    col: usize,
}

impl Lexer {
    /// 创建新的词法分析器
    ///
    /// # 参数
    /// - `source`: 源代码字符串
    ///
    /// # 返回
    /// 初始化后的 Lexer 实例，行号和列号从 1 开始
    pub fn new(source: &str) -> Self {
        Lexer {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// 前进到下一个字符，并更新行号和列号
    ///
    /// # 返回
    /// - `Some(char)`: 下一个字符
    /// - `None`: 已到达源代码末尾
    fn advance(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            return None;
        }
        let ch = self.chars[self.pos];
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    /// 查看当前字符（不移动位置）
    ///
    /// # 返回
    /// - `Some(char)`: 当前字符
    /// - `None`: 已到达末尾
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    /// 获取当前读取位置（当前行号、列号）
    fn current_pos(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    /// 创建指定类型的 Token，使用给定的位置信息
    fn make_token(&self, ty: TokenType, line: usize, col: usize) -> Token {
        Token { ty, line, col }
    }

    /// 跳过空白字符（空格、制表符、换行等）
    ///
    /// 注意：换行符会换行，但注释后的换行会被 skip_line_comment 处理
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// 跳过单行注释 `// ...`
    ///
    /// 从 `//` 后开始，跳过所有字符直到遇到换行符或 EOF
    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// 读取数字字面量
    ///
    /// 支持整数和小数（如 `42`、`3.14`）
    ///
    /// # 参数
    /// - `first`: 已经读取的第一个数字字符
    ///
    /// # 返回
    /// Number 类型的 Token
    fn read_number(&mut self, first: char, start_line: usize, start_col: usize) -> Result<Token, String> {
        let mut num_str = String::from(first);
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        let value = num_str.parse::<f64>().map_err(|_| {
            format!("Invalid number literal '{}' at {}:{}", num_str, start_line, start_col)
        })?;
        Ok(self.make_token(TokenType::Number(value), start_line, start_col))
    }

    /// 读取标识符或关键字
    ///
    /// 识别规则：以字母或下划线开头，后续字符可以是字母、数字或下划线
    ///
    /// 如果识别出关键字，返回对应的 TokenType；否则返回标识符本身
    ///
    /// # 参数
    /// - `first`: 已经读取的第一个字符
    ///
    /// # 返回
    /// 关键字或标识符 Token
    fn read_identifier(&mut self, first: char, start_line: usize, start_col: usize) -> Token {
        let mut ident = String::from(first);
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        let ty = match ident.as_str() {
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "print" => TokenType::Print,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "nil" => TokenType::Nil,
            _ => TokenType::Identifier(ident),
        };
        self.make_token(ty, start_line, start_col)
    }

    /// 读取字符串字面量
    ///
    /// 从双引号开始，到下一个双引号结束
    /// 支持转义序列：`\n` `\t` `\r` `\\` `\"`
    ///
    /// # 返回
    /// String 类型的 Token
    fn read_string(&mut self, start_line: usize, start_col: usize) -> Result<Token, String> {
        let mut s = String::new();
        while let Some(ch) = self.peek() {
            self.advance();
            if ch == '"' {
                return Ok(self.make_token(TokenType::String(s), start_line, start_col));
            }
            if ch == '\\' {
                match self.advance() {
                    Some('n')  => s.push('\n'),
                    Some('t')  => s.push('\t'),
                    Some('r')  => s.push('\r'),
                    Some('\\') => s.push('\\'),
                    Some('"')  => s.push('"'),
                    Some(other) => return Err(format!(
                        "Unknown escape '\\{}' at {}:{}", other, self.line, self.col
                    )),
                    None => return Err(format!(
                        "Unexpected EOF in string escape at {}:{}", self.line, self.col
                    )),
                }
            } else {
                s.push(ch);
            }
        }
        Err(format!("Unterminated string literal at {}:{}", start_line, start_col))
    }

    fn skip_block_comment(&mut self, start_line: usize, start_col: usize) -> Result<(), String> {
        while let Some(ch) = self.peek() {
            self.advance();
            if ch == '*' && self.peek() == Some('/') {
                self.advance();
                return Ok(());
            }
        }
        Err(format!("Unterminated block comment starting at {}:{}", start_line, start_col))
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();

        let (start_line, start_col) = self.current_pos();

        let ch = match self.peek() {
            Some(ch) => ch,
            None => return Ok(None),
        };

        if ch == '/' {
            self.advance();
            if self.peek() == Some('/') {
                self.skip_line_comment();
                return self.next_token();
            } else if self.peek() == Some('*') {
                self.advance();
                self.skip_block_comment(start_line, start_col)?;
                return self.next_token();
            }
            return Ok(Some(self.make_token(TokenType::Slash, start_line, start_col)));
        }

        let token = match ch {
            '+' => {
                self.advance();
                self.make_token(TokenType::Plus, start_line, start_col)
            }
            '-' => {
                self.advance();
                self.make_token(TokenType::Minus, start_line, start_col)
            }
            '*' => {
                self.advance();
                self.make_token(TokenType::Star, start_line, start_col)
            }
            '%' => {
                self.advance();
                self.make_token(TokenType::Percent, start_line, start_col)
            }
            '(' => {
                self.advance();
                self.make_token(TokenType::LParen, start_line, start_col)
            }
            ')' => {
                self.advance();
                self.make_token(TokenType::RParen, start_line, start_col)
            }
            '{' => {
                self.advance();
                self.make_token(TokenType::LBrace, start_line, start_col)
            }
            '}' => {
                self.advance();
                self.make_token(TokenType::RBrace, start_line, start_col)
            }
            ';' => {
                self.advance();
                self.make_token(TokenType::Semicolon, start_line, start_col)
            }
            '=' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    self.make_token(TokenType::EqualEqual, start_line, start_col)
                } else {
                    self.make_token(TokenType::Assign, start_line, start_col)
                }
            }
            '!' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    self.make_token(TokenType::BangEqual, start_line, start_col)
                } else {
                    return Err(format!("Unexpected character '!' at {}:{}", start_line, start_col));
                }
            }
            '<' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    self.make_token(TokenType::LessEqual, start_line, start_col)
                } else {
                    self.make_token(TokenType::Less, start_line, start_col)
                }
            }
            '>' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    self.make_token(TokenType::GreaterEqual, start_line, start_col)
                } else {
                    self.make_token(TokenType::Greater, start_line, start_col)
                }
            }
            ',' => {
                self.advance();
                self.make_token(TokenType::Comma, start_line, start_col)
            }
            '"' => {
                self.advance();
                self.read_string(start_line, start_col)?
            }
            c if c.is_ascii_digit() => {
                self.advance();
                self.read_number(c, start_line, start_col)?
            }
            c if c.is_alphabetic() || c == '_' => {
                self.advance();
                self.read_identifier(c, start_line, start_col)
            }
            _ => {
                return Err(format!("Unexpected character '{}' at {}:{}", ch, start_line, start_col));
            }
        };

        Ok(Some(token))
    }

    pub fn collect_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }
}
