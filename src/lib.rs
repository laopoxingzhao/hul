pub mod ast;
pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod value;

pub use interpreter::Interpreter;
pub use parser::Parser;
pub use value::Value;

/// 运行入口（供二进制和测试使用）
pub fn run(source: &str) -> Result<(), String> {
    let mut parser = parser::Parser::new(source)?;
    let stmts = parser.parse_program()?;
    let mut interp = interpreter::Interpreter::new();
    interp.interpret(&stmts)
}
