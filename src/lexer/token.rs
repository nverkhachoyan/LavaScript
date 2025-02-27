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

    // types
    Int,
    Boolean,
    Void,

    // identifiers and literals
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),

    // operators and punctuation
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dot,
    LeftBracket,
    RightBracket,

    // special
    EOF,
}
