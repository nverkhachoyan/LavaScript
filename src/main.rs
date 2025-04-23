#![allow(dead_code)]
mod ast;
mod lexer;
mod parser;
mod codegen;

use codegen::CodeGenerator;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <source_file> <optional_output_file>", args[0]);
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

    let output =if args.len() == 3 {
        &args[2]
    }
    else {
        let charlength = &args[1].len();
        &[&source_path[0..(charlength-4)],"js"].join("")
    };

    println!("{}",output);

    compile(&source, &output);
}

fn compile(source: &str, output: &str) {
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

    let generator = CodeGenerator::new(ast);
    let code = generator.generate();
    println!();
    println!("{}",code);
    match fs::write(output, code) {
        Ok(_) => println!("Code file outputted to {}", output),
        Err(_) => println!("Error! code not successfully compiled")
    }
}
