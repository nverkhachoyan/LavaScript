mod span;
mod types;

use span::Span;
use std::fmt;
use types::TypeName;

#[derive(Debug, Clone)]
pub enum Expr {
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    Variable(Variable),
    Binary(BinaryExpr),
    FunCall(FunCall),
    MethCall(MethCall),
    New(NewExpr),
    This(ThisExpr),
    Println(PrintlnExpr),
    Print(PrintExpr),
    Grouped(Box<Expr>, Span),
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: i64,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub value: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOp,
    pub right: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunCall {
    pub callee: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MethCall {
    pub object: Box<Expr>,
    pub meth: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct NewExpr {
    pub class_name: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ThisExpr {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct PrintExpr {
    pub arg: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct PrintlnExpr {
    pub arg: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(ExprStmt),
    VarDecl(VarDeclStmt),
    Assign(AssignStmt),
    VarDeclWithAssign(VarDeclWithAssign),
    While(WhileStmt),
    If(IfStmt),
    Break(BreakStmt),
    Return(ReturnStmt),
    Block(BlockStmt),
}

#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expr: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VarDeclStmt {
    pub name: String,
    pub var_type: TypeName,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct AssignStmt {
    pub name: String,
    pub expr: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VarDeclWithAssign {
    pub name: String,
    pub var_type: TypeName,
    pub expr: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BreakStmt {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub value: Option<Box<Expr>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ParamDecl {
    pub name: String,
    pub param_type: TypeName,
}

#[derive(Debug, Clone)]
pub struct FunDef {
    pub name: String,
    pub params: Vec<ParamDecl>,
    pub return_type: TypeName,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub params: Vec<ParamDecl>,
    pub is_super: bool,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct MethDef {
    pub name: String,
    pub params: Vec<ParamDecl>,
    pub return_type: TypeName,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub name: String,
    pub extends: String,
    pub vars: Vec<VarDeclStmt>,
    pub constructor: Constructor,
    pub methods: Vec<MethDef>,
}

#[derive(Debug, Default, Clone)]
pub struct Entry {
    pub statements: Vec<Stmt>,
    pub class_defs: Vec<ClassDef>,
    pub fun_defs: Vec<FunDef>,
}

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
