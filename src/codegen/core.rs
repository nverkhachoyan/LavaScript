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

}