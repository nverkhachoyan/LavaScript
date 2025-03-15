use crate::lexer::Span;
use std::fmt;

#[derive(Debug, PartialEq, Default, Clone)]
pub enum TokenType {
    // keywords
    Let,
    Arrow,
    Class,
    Meth,
    Init,
    Extends,
    This,
    Super,
    While,
    Break,
    Return,
    If,
    Else,
    New,
    True,
    False,
    Println,
    Const,
    Fun,

    // types
    Type(TypeName),

    // identifiers and literals
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),

    // operators
    Plus,
    Minus,
    Star,
    Slash,
    Assign,

    // boolean_operators
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    Negate,

    // punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Colon,
    Comma,
    Dot,
    LeftBracket,
    RightBracket,

    // special
    #[default]
    EOF,
}

#[derive(Debug, Default, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
    }

    pub fn new_with_span(span: Span) -> Self {
        let token_type = TokenType::default();
        Self { token_type, span }
    }

    pub fn set_type(&mut self, token_type: TokenType) {
        self.token_type = token_type;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum TypeName {
    Int,
    Str,
    Boolean,
    #[default]
    Void,
    Class(String),
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeName::Int => write!(f, "Int"),
            TypeName::Str => write!(f, "Str"),
            TypeName::Boolean => write!(f, "Boolean"),
            TypeName::Void => write!(f, "Void"),
            TypeName::Class(name) => write!(f, "{}", name),
        }
    }
}
