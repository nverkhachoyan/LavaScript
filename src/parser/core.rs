use super::*;
use crate::ast::Entry;
use crate::lexer::{Span, Token, TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
    pub errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            errors: vec![],
        }
    }

    pub fn parse(&mut self) -> Option<Entry> {
        let mut program = Entry::default();

        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Class => {
                    let span = token.span.clone();
                    if let Some(class) = self.parse_class(span) {
                        program.class_defs.push(class);
                    }
                }
                TokenType::Fun => {
                    // TODO: Parse stand-alone funcs
                    self.advance();
                }
                TokenType::EOF => {
                    break;
                }
                // TODO: Parse standalone stmts
                _ => {
                    self.advance();
                }
            }
        }

        Some(program)
    }

    pub fn synchronize(&mut self, sync_point: SyncPoint) {
        match sync_point {
            SyncPoint::ClassBody => {
                while let Some(token) = self.peek() {
                    match token.token_type {
                        TokenType::LeftBrace | TokenType::Class => {
                            break;
                        }
                        _ => self.advance(),
                    }
                }
            }
            _ => {}
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn get_errors(&self) -> &[ParseError] {
        &self.errors
    }

    pub fn print_errors(&self, source: &str) {
        error::print_errors(&self.errors, source);
    }

    pub fn advance_expected(&mut self, expected: TokenType) -> Option<()> {
        if let Some(token) = self.peek() {
            if token.token_type == expected {
                self.advance();
                return Some(());
            }
            self.errors.push(ParseError::ExpectedButFound {
                expected: expected.to_string(),
                found: token.token_type.to_string(),
                span: token.span.clone(),
            });
            return None;
        }
        self.errors.push(ParseError::UnexpectedEOF { span: None });
        None
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn peek(&mut self) -> Option<Token> {
        if self.position >= self.tokens.len() {
            return None;
        }
        match self.tokens.get(self.position) {
            Some(token) => Some(token.clone()),
            None => None,
        }
    }

    pub fn peek_ahead(&mut self) -> Option<Token> {
        if self.position + 1 >= self.tokens.len() {
            return None;
        }
        match self.tokens.get(self.position + 1) {
            Some(token) => Some(token.clone()),
            None => None,
        }
    }

    pub fn is_eof(&mut self) -> bool {
        match self.peek() {
            Some(token) => token.token_type == TokenType::EOF,
            None => true,
        }
    }

    pub fn current_span(&mut self) -> Option<Span> {
        self.peek().map(|token| token.span.clone())
    }
}
