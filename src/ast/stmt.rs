use crate::ast::*;
use crate::lexer::Span;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
    #[default]
    Empty,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExprStmt {
    pub expr: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct VarDeclStmt {
    pub name: String,
    pub var_type: TypeName,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AssignStmt {
    pub name: String,
    pub expr: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct VarDeclWithAssign {
    pub name: String,
    pub var_type: TypeName,
    pub expr: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WhileStmt {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BreakStmt {
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ReturnStmt {
    pub value: Option<Box<Expr>>,
    pub span: Span,
}
