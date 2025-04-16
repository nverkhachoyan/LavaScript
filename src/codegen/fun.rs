use crate::ast::FunDef;

use super::*;

pub trait FunctionGenerator {
    fn generate_functions(&self, functions: Vec<FunDef>) -> String;
}

impl FunctionGenerator for CodeGenerator {
    fn generate_functions(&self, functions: Vec<FunDef>) -> String {
        todo!()
    }
}