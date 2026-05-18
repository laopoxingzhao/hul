
use std::env;
mod ast;
mod lexer;
mod parser;
mod value;
mod interpreter;




use std::fs;
use std::io::{self, Write};
use crate::parser::Parser;
use crate::interpreter::Interpreter;


fn run(source: &str) -> Result<(), String> {
    let mut parser = Parser::new(source);
    let stmts = parser.parse_program()?;
    let mut interp = Interpreter::new();
    interp.interpret(&stmts)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // 执行文件
        let content = fs::read_to_string(&args[1]).expect("Unable to read file");
        if let Err(e) = run(&content) {
            eprintln!("Error: {}", e);
        }
    } else {
        // REPL
        loop {
            print!("hu> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Failed to read line");
            if line.trim().is_empty() { continue; }
            if let Err(e) = run(&line) {
                eprintln!("Error: {}", e);
            }
        }
    }
}