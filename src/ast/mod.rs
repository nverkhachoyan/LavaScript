mod decl;
mod expr;
mod span;
mod stmt;
mod types;
mod visitor;

pub use decl::*;
pub use expr::*;
pub use span::Span;
pub use stmt::*;
pub use types::TypeName;
pub use visitor::Visitor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_var_decl() {
        let mut program = Entry::default();
        let fun_def = FunDef {
            name: "greet".to_string(),
            params: vec![],
            statements: vec![Stmt::Expr(ExprStmt {
                expr: Box::new(Expr::Println(PrintlnExpr {
                    arg: Box::new(Expr::StringLiteral(StringLiteral {
                        value: "hello world".to_string(),
                        span: Span::default(),
                    })),
                    span: Span::default(),
                })),
                span: Span::default(),
            })],
            return_type: TypeName::Void,
        };

        program.fun_defs = vec![fun_def];

        println!("AST: {:#?}", program)
    }
}
