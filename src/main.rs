use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => run_file(&args[1]),
        1 => run_repl(),
        _ => eprintln!("Usage: {} [script]", args[0]),
    }
}

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(content) => {
            if let Err(e) = hul::run(&content) {
                eprintln!("Runtime error: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to read file '{}': {}", path, e),
    }
}

fn run_repl() {
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
                if !line.trim().is_empty()
                    && let Err(e) = hul::run(&line) {
                        eprintln!("Runtime error: {}", e);
                    }
            }
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                continue;
            }
        }
    }
}
