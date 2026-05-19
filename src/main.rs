/// Hul 语言解释器 - 主程序入口
///
/// 该程序实现了一个简单的脚本语言解释器，支持：
/// - 变量声明和赋值
/// - 算术、比较、逻辑运算
/// - 条件语句（if-else）
/// - 循环语句（while）
/// - 打印输出
///
/// 运行模式：
/// - 文件模式：`cargo run -- script.hul`
/// - REPL 模式：`cargo run`（交互式命令行）
use std::env;
mod ast;
mod interpreter;
mod lexer;
mod parser;
mod value;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use std::fs;
use std::io::{self, Write};

/// 执行源代码
///
/// 完整的编译和执行流程：
/// 1. 词法分析：将源代码转换为 Token 流
/// 2. 语法分析：将 Token 流转换为 AST
/// 3. 解释执行：遍历 AST 并执行语义操作
///
/// # 参数
/// - `source`: 源代码字符串
///
/// # 返回
/// - `Ok(())`: 执行成功
/// - `Err(String)`: 编译或运行时错误信息
fn run(source: &str) -> Result<(), String> {
    // 步骤1：创建解析器（内部完成词法分析）
    let mut parser = Parser::new(source);
    // 步骤2：解析程序得到 AST
    let stmts = parser.parse_program()?;
    // 步骤3：创建解释器并执行
    let mut interp = Interpreter::new();
    interp.interpret(&stmts)
}

/// 主函数 - 程序入口点
///
/// 根据命令行参数选择运行模式：
/// - 有参数：执行指定文件
/// - 无参数：进入 REPL 交互模式
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // ==================== 文件执行模式 ====================
        // 读取文件内容并执行
        let content = fs::read_to_string(&args[1]).expect("Unable to read file");
        if let Err(e) = run(&content) {
            eprintln!("Error: {}", e);
        }
    } else {
        // ==================== REPL 交互模式 ====================
        // 循环读取用户输入并立即执行
        loop {
            print!("hu> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            if line.trim().is_empty() {
                continue;
            }
            if let Err(e) = run(&line) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
