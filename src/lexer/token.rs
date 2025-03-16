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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // keywords
            TokenType::Let => write!(f, "Let"),
            TokenType::Arrow => write!(f, "Arrow"),
            TokenType::Class => write!(f, "Class"),
            TokenType::Meth => write!(f, "Meth"),
            TokenType::Init => write!(f, "Init"),
            TokenType::Extends => write!(f, "Extends"),
            TokenType::This => write!(f, "This"),
            TokenType::Super => write!(f, "Super"),
            TokenType::While => write!(f, "While"),
            TokenType::Break => write!(f, "Break"),
            TokenType::Return => write!(f, "Return"),
            TokenType::If => write!(f, "If"),
            TokenType::Else => write!(f, "Else"),
            TokenType::New => write!(f, "New"),
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),
            TokenType::Println => write!(f, "Println"),
            TokenType::Const => write!(f, "Const"),
            TokenType::Fun => write!(f, "Fun"),

            // types
            TokenType::Type(t) => write!(f, "{}", t),

            // identifiers and literals
            TokenType::Identifier(s) => write!(f, "Identifier({})", s),
            TokenType::IntegerLiteral(i) => write!(f, "IntegerLiteral({})", i),
            TokenType::StringLiteral(s) => write!(f, "StringLiteral({})", s),

            // operators
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Assign => write!(f, "Assign"),

            // boolean operators
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::NotEqual => write!(f, "NotEqual"),
            TokenType::Negate => write!(f, "Negate"),

            // punctuation
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::LeftBracket => write!(f, "LeftBracket"),
            TokenType::RightBracket => write!(f, "RightBracket"),

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
