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
                    if let Some(class) = self.parse_class() {
                        program.class_defs.push(class);
                    }
                }
                TokenType::Fun => {
                    if let Some(fun) = self.parse_fun() {
                        program.fun_defs.push(fun);
                    }
                }
                TokenType::EOF => {
                    break;
                }
                _ => {
                    let stmt = self.parse_stmt();
                    match stmt {
                        Some(stmt) => {
                            program.statements.push(stmt);
                        }
                        None => {
                            self.advance();
                        }
                    }
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

    pub fn current_span(&mut self) -> Option<Span> {
        self.peek().map(|token| token.span.clone())
    }

    pub fn consume_identifier(&mut self, ident: &str) -> Option<String> {
        let current_span = self.current_span();

        if let Some(token) = self.peek() {
            match &token.token_type {
                TokenType::Identifier(name) => {
                    let name = name.clone();
                    self.advance();
                    return Some(name);
                }
                _ => {
                    self.errors.push(ParseError::expected_but_found(
                        TokenType::Identifier(ident.to_string()).to_string(),
                        Some(token.token_type.to_string()),
                        Some(token.span.clone()),
                    ));
                    return None;
                }
            }
        }

        self.errors
            .push(ParseError::UnexpectedEOF { span: current_span });
        None
    }

    pub fn consume(&mut self, expected: TokenType) -> Option<Token> {
        let span = self.current_span();

        if let Some(token) = self.peek() {
            if token.token_type == expected {
                self.advance();
                return Some(token);
            }
            self.errors.push(ParseError::ExpectedButFound {
                expected: expected.to_string(),
                found: token.token_type.to_string(),
                span,
            });
            return None;
        }

        self.errors.push(ParseError::UnexpectedEOF { span });
        None
    }

    pub fn consume_optional(&mut self, expected: TokenType) -> Option<Token> {
        let span = self.current_span();

        if let Some(token) = self.peek() {
            if token.token_type == expected {
                self.advance();
                return Some(token);
            }
            return None;
        }

        self.errors.push(ParseError::UnexpectedEOF { span });
        None
    }

    pub fn consume_two_optionals(
        &mut self,
        first: TokenType,
        second: TokenType,
    ) -> Option<(Token, Token)> {
        match (self.peek(), self.peek_ahead()) {
            (Some(token), Some(next_token)) => {
                if token.token_type == first && next_token.token_type == second {
                    self.advance();
                    self.advance();
                    return Some((token, next_token));
                }
            }
            _ => return None,
        }

        None
    }

    pub fn consume_type(&mut self) -> Option<crate::lexer::TypeName> {
        let current_span = self.current_span();

        if let Some(token) = self.peek() {
            let span = token.span.clone();
            match &token.token_type {
                TokenType::Type(typ) => {
                    let typ = typ.clone();
                    self.advance();
                    Some(typ)
                }
                TokenType::Identifier(ident) => {
                    self.advance();
                    Some(crate::lexer::TypeName::Class(ident.to_string()))
                }
                _ => {
                    self.errors.push(ParseError::expected_but_found(
                        "variable type".to_string(),
                        Some(token.token_type.to_string()),
                        Some(span),
                    ));
                    None
                }
            }
        } else {
            self.errors
                .push(ParseError::UnexpectedEOF { span: current_span });
            None
        }
    }
}
