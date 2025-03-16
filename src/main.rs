#![allow(dead_code)]
mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
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
    let mut lexer = Lexer::new(source);

    match lexer.tokenize() {
        Ok(tokens) => {
            // for tok in tokens.clone() {
            //     println!("{:?}", tok);
            // }

            let mut parser = Parser::new(tokens);
            let ast = parser.parse();

            if parser.has_errors() {
                parser.print_errors(source);
            }

            match ast {
                Some(ast_res) => {
                    // for tok in ast_res.class_defs {
                    //     println!("{:?}", tok);
                    // }
                    println!("{:#?}", ast_res);
                }
                None => println!("epic failure:"),
            }
        }
        Err(error) => {
            eprintln!("Lexical error: {}", error);
        }
    }
}
