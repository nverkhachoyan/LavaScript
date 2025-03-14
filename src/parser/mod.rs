mod error;

use crate::ast::{self, ClassDef};
use crate::lexer::Token;
use error::{ParseError, SourceLocation};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            errors: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<ast::Entry, ParseError> {
        let mut program = ast::Entry::default();

        while let Some(token) = self.peek() {
            match token {
                Token::Class => {
                    program.class_defs.push(self.parse_class()?);
                }
                Token::Fun => {}
                _ => {
                    break;
                }
            }
        }

        Ok(program)
    }

    fn parse_class(&mut self) -> Result<ClassDef, ParseError> {
        let mut class = ClassDef::default();

        self.advance();
        match self.peek() {
            Some(Token::Identifier(ident)) => {
                class.name = ident;
                self.advance();
            }
            _ => {
                return Err(ParseError::MissingClassName {
                    location: SourceLocation::new(1, 1),
                })
            }
        }

        match self.peek() {
            Some(Token::Extends) => {
                self.advance();
                match self.peek() {
                    Some(Token::Identifier(ident)) => {
                        class.extends = Some(ident);
                        self.advance();
                    }
                    _ => {
                        return Err(ParseError::MissingClassExtendIdent {
                            symbol: class.name.clone(),
                            location: SourceLocation::new(1, 1),
                        })
                    }
                }
            }
            _ => class.extends = None,
        }

        match self.peek() {
            Some(Token::LeftBrace) => {
                self.advance();
            }
            _ => {
                return Err(ParseError::MissingOpeningCurlyBrace {
                    symbol: class.name,
                    location: SourceLocation::new(1, 1),
                })
            }
        }

        while self.peek() != Some(Token::RightBrace) {
            self.advance();
        }

        match self.peek() {
            Some(Token::RightBrace) => {
                self.advance();
            }
            _ => {
                return Err(ParseError::MissingClosingCurlyBrace {
                    symbol: class.name,
                    location: SourceLocation::new(1, 1),
                })
            }
        }

        Ok(class)
    }

    fn parse_meth(&mut self) {}

    fn parse_constructor(&mut self) {}

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&mut self) -> Option<Token> {
        if self.position > self.tokens.len() {
            return None;
        }
        self.tokens.get(self.position).cloned()
    }

    fn peek_ahead(&mut self) -> Option<Token> {
        self.tokens.get(self.position).cloned()
    }

    fn is_eof(&self) -> bool {
        return self.tokens[self.position] == Token::EOF;
    }
}

// pub struct ClassDef {
//     pub name: String,
//     pub extends: Option<String>,
//     pub vars: Vec<VarDeclStmt>,
//     pub constructor: Constructor,
//     pub methods: Vec<MethDef>,
// }
