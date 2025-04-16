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
        
        let program: String = [statements, classes, functions].join("\n");
        program
    }
}