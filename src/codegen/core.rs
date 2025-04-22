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
        println!("{:?}",ast);
        let generator = CodeGenerator::new(ast);
        let program = generator.generate();
        println!("{}",program);
        program
    }


}