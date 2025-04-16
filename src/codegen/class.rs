use crate::ast::ClassDef;

use super::*;

pub trait ClassGenerator {
    fn generate_classes(&self, classes: Vec<ClassDef>) -> String;
}

impl ClassGenerator for CodeGenerator {
    fn generate_classes(&self, classes: Vec<ClassDef>) -> String {
        todo!()
    }
}