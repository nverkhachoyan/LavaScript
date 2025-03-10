#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // keywords
    Class,
    Method,
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
    Function,
    ReturnType,

    // types
    Int,
    String,
    Boolean,
    Void,

    // identifiers and literals
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),

    // operators
    Plus,
    Minus,
    Star,
    Slash,
    Equals,

    // boolean_operators
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equality,
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
    EOF,
}
