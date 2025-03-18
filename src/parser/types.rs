use crate::lexer::Span;
use std::fmt;

pub mod expected {
    pub const EXPRESSION: &str = "expression";
    pub const STATEMENT: &str = "statement";
    pub const IDENTIFIER: &str = "identifier";
    pub const SEMICOLON: &str = "semicolon";
    pub const COLON: &str = ":";
    pub const LEFT_PAREN: &str = "(";
    pub const RIGHT_PAREN: &str = ")";
    pub const LEFT_BRACE: &str = "{";
    pub const RIGHT_BRACE: &str = "}";
    pub const LEFT_BRACKET: &str = "[";
    pub const RIGHT_BRACKET: &str = "]";
    pub const VARIABLE_TYPE: &str = "variable type";
    pub const CLASS_NAME: &str = "class name";
    pub const METHOD_NAME: &str = "method name";
    pub const PARAMETER_NAME: &str = "parameter name";
    pub const RETURN_TYPE: &str = "return type";
}

pub enum SyncPoint {
    ClassBody,
    MethodBody,
    Statement,
    Expression,
}

#[derive(Clone)]
pub enum BlockContext {
    Meth,
    Fun,
    ControlFlow,
    TopLevel,
    Nested(Box<BlockContext>),
}

impl BlockContext {
    pub fn allows_return(&self) -> bool {
        match self {
            BlockContext::Meth | BlockContext::Fun => true,
            BlockContext::Nested(parent) => parent.allows_return(),
            _ => false,
        }
    }

    pub fn allows_break(&self) -> bool {
        match self {
            BlockContext::ControlFlow => true,
            BlockContext::Nested(parent) => parent.allows_break(),
            _ => false,
        }
    }

    pub fn allows_this(&self) -> bool {
        match self {
            BlockContext::Meth => true,
            BlockContext::Nested(parent) => parent.allows_this(),
            _ => false,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum DelimiterType {
    Brace,
    Paren,
    Bracket,
}

impl fmt::Display for DelimiterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DelimiterType::Brace => write!(f, "brace"),
            DelimiterType::Paren => write!(f, "paren"),
            DelimiterType::Bracket => write!(f, "bracket"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DelimiterContext {
    pub typ: DelimiterType,
    pub span: Span,
}
