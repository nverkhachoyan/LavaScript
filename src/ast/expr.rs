use crate::ast::*;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
    #[default]
    Empty,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub value: i64,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StringLiteral {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BooleanLiteral {
    pub value: bool,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOp,
    pub right: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    #[default]
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::GreaterEqual => write!(f, ">="),
            BinaryOp::LessEqual => write!(f, "<="),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FunCall {
    pub callee: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MethCall {
    pub object: Box<Expr>,
    pub meth: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NewExpr {
    pub class_name: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ThisExpr {
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrintExpr {
    pub arg: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrintlnExpr {
    pub arg: Box<Expr>,
    pub span: Span,
}
