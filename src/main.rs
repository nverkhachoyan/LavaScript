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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile() {
        let source = "let x = 5;";
        let output = "output.js";
        compile(source, output);
        assert!(fs::metadata(output).is_ok());
        fs::remove_file(output).unwrap();
    }

    #[test]
    fn test_compile_lex_error() {
        let source = "let @x = 5;";
        let result = std::panic::catch_unwind(|| compile(source, "lex_fail.js"));
        assert!(result.is_ok()); 
    }

    #[test]
    fn test_compile_parse_failure() {
        let source = "fun {"; 
        let result = std::panic::catch_unwind(|| compile(source, "parse_fail.js"));
        assert!(result.is_ok()); 
    }

    #[test]
    fn test_compile_parser_with_errors() {
        let source = r#"
            fun main() {
                let x = ;
            }
        "#;
        let result = std::panic::catch_unwind(|| compile(source, "errors.js"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_write_fail() {
        let source = "let x = 5;";
        let output = "/root/protected_output.js";
        let result = std::panic::catch_unwind(|| compile(source, output));
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_with_lexical_error() {
        let bad_source = "let @x = 5;"; 
        let output = "lex_fail.js";
    
        let result = std::panic::catch_unwind(|| compile(bad_source, output));
        assert!(result.is_ok());

        assert!(!std::path::Path::new(output).exists());
    }

    #[test]
    fn test_compile_success_case() {
        let source = r#"
            fun main() {
                let x: Int = 10;
                println(x);
            }
        "#;
        let output = "compiled_output.js";

        compile(source, output);
        assert!(std::fs::metadata(output).is_ok());
        std::fs::remove_file(output).unwrap();
    }
}