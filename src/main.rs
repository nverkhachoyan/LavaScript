mod lexer;

use lexer::Lexer;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        process::exit(1);
    }

    let source_path = &args[1];
    let source = match fs::read_to_string(source_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", source_path, e);
            process::exit(1);
        }
    };

    compile(&source);
}

fn compile(source: &str) {
    // 1. Lexical Analysis
    let mut lexer = Lexer::new(source);

    match lexer.tokenize() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(error) => {
            eprintln!("Lexical error: {}", error);
        }
    }
}
