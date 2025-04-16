#![allow(dead_code)]
mod ast;
mod lexer;
mod parser;
mod codegen;

use ast::PrettyPrint;
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
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("Lexical error: {}", error);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Some(ast) => {
            if parser.has_errors() {
                parser.print_errors(source);
            }
            ast
        }
        None => {
            println!("epic failure:");
            return;
        }
    };

    println!();
    ast.print();
}
