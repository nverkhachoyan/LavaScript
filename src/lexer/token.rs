use crate::{ast::BinaryOp, lexer::Span};
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
    Print,
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
    Not,

    // logical ops
    Or,
    And,

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

impl TokenType {
    pub fn which_binary_op(&self) -> BinaryOp {
        match self {
            TokenType::Plus => BinaryOp::Add,
            TokenType::Minus => BinaryOp::Subtract,
            TokenType::Star => BinaryOp::Multiply,
            TokenType::Slash => BinaryOp::Divide,
            TokenType::Equal => BinaryOp::Equal,
            TokenType::NotEqual => BinaryOp::NotEqual,
            TokenType::Greater => BinaryOp::Greater,
            TokenType::Less => BinaryOp::Less,
            TokenType::GreaterEqual => BinaryOp::GreaterEqual,
            TokenType::LessEqual => BinaryOp::LessEqual,
            TokenType::And => BinaryOp::And,
            TokenType::Or => BinaryOp::Or,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // keywords
            TokenType::Let => write!(f, "let"),
            TokenType::Arrow => write!(f, "->"),
            TokenType::Class => write!(f, "class"),
            TokenType::Meth => write!(f, "meth"),
            TokenType::Init => write!(f, "init"),
            TokenType::Extends => write!(f, "extends"),
            TokenType::This => write!(f, "this"),
            TokenType::Super => write!(f, "super"),
            TokenType::While => write!(f, "while"),
            TokenType::Break => write!(f, "break"),
            TokenType::Return => write!(f, "return"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::New => write!(f, "new"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::Print => write!(f, "print"),
            TokenType::Println => write!(f, "println"),
            TokenType::Const => write!(f, "const"),
            TokenType::Fun => write!(f, "fun"),

            // types
            TokenType::Type(t) => write!(f, "{}", t),

            // identifiers and literals
            TokenType::Identifier(s) => write!(f, "Identifier({})", s),
            TokenType::IntegerLiteral(i) => write!(f, "IntegerLiteral({})", i),
            TokenType::StringLiteral(s) => write!(f, "StringLiteral({})", s),

            // operators
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Star => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Assign => write!(f, "="),

            // boolean operators
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Equal => write!(f, "=="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::Not => write!(f, "!"),

            // logical operators
            TokenType::Or => write!(f, "||"),
            TokenType::And => write!(f, "&&"),

            // punctuation
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),

            // special
            TokenType::EOF => write!(f, "EOF"),
        }
    }
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
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
