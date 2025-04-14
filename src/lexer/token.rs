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

#[test]
fn test_all_bin_ops() {
    use TokenType::*;
    assert_eq!(Plus.which_binary_op(), BinaryOp::Add);
    assert_eq!(Minus.which_binary_op(), BinaryOp::Subtract);
    assert_eq!(Star.which_binary_op(), BinaryOp::Multiply);
    assert_eq!(Slash.which_binary_op(), BinaryOp::Divide);
    assert_eq!(Equal.which_binary_op(), BinaryOp::Equal);
    assert_eq!(NotEqual.which_binary_op(), BinaryOp::NotEqual);
    assert_eq!(Greater.which_binary_op(), BinaryOp::Greater);
    assert_eq!(GreaterEqual.which_binary_op(), BinaryOp::GreaterEqual);
    assert_eq!(Less.which_binary_op(), BinaryOp::Less);
    assert_eq!(LessEqual.which_binary_op(), BinaryOp::LessEqual);
    assert_eq!(And.which_binary_op(), BinaryOp::And);
    assert_eq!(Or.which_binary_op(), BinaryOp::Or);
}


#[test]
fn test_display_token_type_literals() {
    let tokens = vec![
        TokenType::Let, TokenType::Arrow, TokenType::Class, TokenType::Meth,
        TokenType::Init, TokenType::This, TokenType::While,
        TokenType::Identifier("foo".to_string()),
        TokenType::IntegerLiteral(123),
        TokenType::Star,
        TokenType::Plus,
        TokenType::LeftParen,
        TokenType::EOF,
    ];

    for token in tokens {
        println!("{}", token); 
    }
}

#[test]
fn test_type_name_display() {
    assert_eq!(TypeName::Int.to_string(), "Int");
    assert_eq!(TypeName::Str.to_string(), "Str");
    assert_eq!(TypeName::Boolean.to_string(), "Boolean");
    assert_eq!(TypeName::Void.to_string(), "Void");
    assert_eq!(TypeName::Class("MyClass".to_string()).to_string(), "MyClass");
}

#[test]
fn test_all_token_types() {
    let _ = vec![
        TokenType::Let,
        TokenType::Arrow,
        TokenType::Class,
        TokenType::Meth,
        TokenType::Init,
        TokenType::Extends,
        TokenType::This,
        TokenType::Super,
        TokenType::While,
        TokenType::Break,
        TokenType::Return,
        TokenType::If,
        TokenType::Else,
        TokenType::New,
        TokenType::True,
        TokenType::False,
        TokenType::Print,
        TokenType::Println,
        TokenType::Const,
        TokenType::Fun,
        TokenType::Type(TypeName::Int),
        TokenType::Identifier("id".into()),
        TokenType::IntegerLiteral(42),
        TokenType::StringLiteral("string".into()),
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Star,
        TokenType::Slash,
        TokenType::Assign,
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::Less,
        TokenType::LessEqual,
        TokenType::Equal,
        TokenType::NotEqual,
        TokenType::Not,
        TokenType::Or,
        TokenType::And,
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::RightBrace,
        TokenType::Semicolon,
        TokenType::Colon,
        TokenType::Comma,
        TokenType::Dot,
        TokenType::LeftBracket,
        TokenType::RightBracket,
        TokenType::EOF,
    ]
    .into_iter()
    .map(|t| t.to_string())
    .collect::<Vec<_>>();
}