use crate::ast::Stmt;
use super::*;

pub trait StatementGenerator {
    fn generate_statements(&self, statements:Vec<Stmt>) -> String;
}

impl StatementGenerator for CodeGenerator {
    fn generate_statements(&self, statements: Vec<Stmt>) -> String {
        todo!()
    }
}