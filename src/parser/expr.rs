use super::*;
use crate::ast::Expr;

pub trait ParserExpr {
    fn parse_expr(&mut self) -> Option<Expr>;
}

impl ParserExpr for Parser {
    // TODO: impl
    fn parse_expr(&mut self) -> Option<Expr> {
        None
    }
}
