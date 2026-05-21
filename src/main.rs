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
// Directly call library `run` where needed.

/// 主函数 - 程序入口点
///
/// 根据命令行参数选择运行模式：
/// - 有参数：执行指定文件
/// - 无参数：进入 REPL 交互模式
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // ==================== 文件执行模式 ====================
            match fs::read_to_string(&args[1]) {
            Ok(content) => {
                if let Err(e) = hul::run(&content) {
                    eprintln!("Runtime error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", args[1], e);
            }
        }
    } else if args.len() == 1 {
        // ==================== REPL 交互模式 ====================
        loop {
            print!("hu> ");
            if let Err(e) = io::stdout().flush() {
                eprintln!("Failed to flush stdout: {}", e);
                continue;
            }
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    if let Err(e) = hul::run(&line) {
                        eprintln!("Runtime error: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read line: {}", e);
                    continue;
                }
            }
        }
    } else {
        eprintln!("Usage: {} [script]", args[0]);
    }
}
