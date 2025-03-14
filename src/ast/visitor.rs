use crate::ast::*;
pub trait Visitor {
    fn visit_integer_literal(&self, integer_literal: &IntegerLiteral) -> Result<(), String>;
    fn visit_string_literal(&self, string_literal: &StringLiteral) -> Result<(), String>;
    fn visit_boolean_literal(&self, boolean_literal: &BooleanLiteral) -> Result<(), String>;
    fn visit_variable(&self, variable: &Variable) -> Result<(), String>;
    fn visit_binary_expr(&self, binary_expr: &BinaryExpr) -> Result<(), String>;
    fn visit_fun_call(&self, fun_call: &FunCall) -> Result<(), String>;
    fn visit_meth_call(&self, meth_call: &MethCall) -> Result<(), String>;
    fn visit_new(&self, new: &NewExpr) -> Result<(), String>;
    fn visit_this(&self, this: &ThisExpr) -> Result<(), String>;
    fn visit_println(&self, println: &PrintlnExpr) -> Result<(), String>;
}
