use super::*;
use crate::ast::*;

pub struct CodeGenerator {
    pub statements: Vec<Stmt>,
    pub classes: Vec<ClassDef>,
    pub functions: Vec<FunDef>
}

impl CodeGenerator {
    pub fn new(ast: Entry) -> Self {
        Self {
            statements: ast.statements,
            classes: ast.class_defs,
            functions: ast.fun_defs
        }
    }

    pub fn generate(&self) -> String {
        let statements = self.generate_statements(self.statements.clone());
        let classes = self.generate_classes(self.classes.clone());
        let functions = self.generate_functions(self.functions.clone());
        
        let program: String = [classes, functions, statements].join("\n");
        program
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::*, parser::*};
    use super::*;

    fn gen_program(input: &str) -> String {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = CodeGenerator::new(ast);
        let program = generator.generate();
        program
    }

    const FIBONACCI_CODE: &str = 
        r"fun fibonacci(n: Int) -> Int {
        if (n <= 0) {
            return 0;
        }
        if (n == 1) {
            return 1;
        }
        
        let a: Int = 0;
        let b: Int = 1;
        let result: Int = 0;
        
        let i: Int = 2;
        while (i <= n) {
            result = a + b;
            a = b;
            b = result;
            i = i + 1;
        }
        
        return result;
        }

        println(fibonacci(10));
        fibonacci(10);";

    #[test]
    fn test_fibonacci_program() {
        let program = gen_program(FIBONACCI_CODE);
        println!("{}", program)
    }

    const RECTANGLE_CODE: &str =
        r"class Rectangle {
            let width: Int;
            let height: Int;
            init(width: Int, height: Int) {{
                this.width = width;
                this.height = height;
            }}
            meth area() -> Int {
                return this.width * this.height;
            }
            meth perimeter() -> Int {
                return 2 * this.width + 2* this.height;
            }
        }
        let rec1: Rectangle = new Rectangle(2,4);
        println(rec1.width);
        println(rec1.height);

        println(rec1.area());
        println(rec1.perimeter());
        ";
    
    #[test]
    fn test_rectangle_program() {
        let program = gen_program(RECTANGLE_CODE);
        println!("{}", program)
    }

    #[test]
    fn test_empty_program() {
        let program = gen_program("");
        println!("{}", program);
    }

    #[test]
    fn test_class_no_methods() {
        let code = r"class Empty {}";
        let program = gen_program(code);
        println!("{}", program);
    }

    #[test]
    fn test_empty_function() {
        let code = r"fun doNothing() {}";
        let program = gen_program(code);
        println!("{}", program);
    }

    #[test]
    fn test_multiple_functions_and_classes() {
        let code = r"
            fun a() -> Int { return 1; }
            fun b() -> Int { return 2; }
            class A {}
            class B {}
        ";
        let program = gen_program(code);
        println!("{}", program);
    }

    #[test]
    fn test_recursive_function() {
        let code = r"
            fun fact(n: Int) -> Int {
                if (n <= 1) return 1;
                return n * fact(n - 1);
            }
            println(fact(5));
        ";
        let program = gen_program(code);
        println!("{}", program);
    }

    #[test]
    fn test_empty_program_again() {
        let code = "";
        let program = gen_program(code);
        assert_eq!(program.trim(), "");
    }

    #[test]
    fn test_class_without_methods() {
        let code = r"class Empty {}";
        let program = gen_program(code);
        assert!(program.contains("class Empty"));
    }

    #[test]
    fn test_generate_statements_empty() {
        let gen = CodeGenerator {
            statements: vec![],
            classes: vec![],
            functions: vec![],
        };
        let output = gen.generate();
        assert_eq!(output.trim(), "");
    }

    #[test]
    fn test_void_like_function() {
        let code = r"
            fun logMessage() {
                println(22);
            }
            logMessage();
        ";
        let program = gen_program(code);
        println!("{}", program);
    }

    #[test]
    fn test_function_with_nested_logic() {
        let code = r"
            fun test(n: Int) -> Int {
                if (n > 10) {
                    if (n > 20) {
                        return 2;
                    } else {
                        return 1;
                    }
                } else {
                    return 0;
                }
            }
            println(test(15));
        ";
        let program = gen_program(code);
        println!("{}", program);
    }

    #[test]
    fn test_only_statements() {
        let code = r"
            let x: Int = 5;
            println(x);
            x = x + 10;
            println(x);
        ";
        let program = gen_program(code);
        println!("{}", program);
    }

    #[test]
    fn test_deep_nesting_in_function() {
        let code = r"
            fun deep() -> Int {
                if (true) {
                    if (true) {
                        if (true) {
                            return 42;
                        }
                    }
                }
                return 0;
            }
            println(deep());
        ";
        let program = gen_program(code);
        println!("{}", program);
    }
}