mod error;

use crate::ast::{self, ClassDef, MethDef, ParamDecl};
use crate::lexer::{Token, TokenType};
use error::ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    errors: Vec<ParseError>,
}

enum SyncPoint {
    ClassBody,
    MethodBody,
    Statement,
    Expression,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            errors: vec![],
        }
    }

    pub fn parse(&mut self) -> Option<ast::Entry> {
        let mut program = ast::Entry::default();

        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Class => {
                    if let Some(class) = self.parse_class() {
                        program.class_defs.push(class);
                    }
                }
                TokenType::Fun => {}
                _ => {
                    break;
                }
            }
        }

        Some(program)
    }

    fn parse_class(&mut self) -> Option<ClassDef> {
        let mut class = ClassDef::default();

        // skip class keyword, parse name
        self.advance();
        if let Some(token) = self.peek() {
            let class_name_span = token.span.clone();
            match token.token_type {
                TokenType::Identifier(ident) => {
                    class.name = ident;
                    self.advance();
                }
                _ => {
                    self.errors.push(ParseError::MissingClassName {
                        span: class_name_span,
                    });
                    self.synchronize(SyncPoint::ClassBody);
                    return None;
                }
            }
        }

        // skip extends, get class name for extend
        if let Some(token) = self.peek() {
            if matches!(token.token_type, TokenType::Extends) {
                let extends_span = token.span.clone();
                self.advance();

                match self.peek() {
                    Some(next_token) => {
                        if let TokenType::Identifier(ident) = &next_token.token_type {
                            let identifier = ident.clone();
                            self.advance();
                            class.extends = Some(identifier);
                        } else {
                            self.errors.push(ParseError::MissingClassExtendIdent {
                                symbol: class.name.clone(),
                                span: extends_span,
                            });
                            self.synchronize(SyncPoint::ClassBody);
                            return None;
                        }
                    }
                    None => {
                        self.errors.push(ParseError::MissingClassExtendIdent {
                            symbol: class.name.clone(),
                            span: extends_span,
                        });
                        return None;
                    }
                }
            } else {
                class.extends = None;
            }
        }

        // opening curly brace
        if let Some(token) = self.peek() {
            let left_brace_span = token.span.clone();
            if matches!(token.token_type, TokenType::LeftBrace) {
                self.advance();
            } else {
                self.errors.push(ParseError::MissingOpeningCurlyBrace {
                    symbol: class.name,
                    span: left_brace_span,
                });
                self.synchronize(SyncPoint::ClassBody);
                return None;
            }
        }

        // class constructor
        if let Some(token) = self.peek() {
            let constructor_span = token.span.clone();
            match token.token_type {
                TokenType::Init => {
                    self.advance();
                    if let Some(params) = self.parse_comma_param_decl(&class.name) {
                        class.constructor.params = params;
                    }
                }
                _ => {
                    self.errors.push(ParseError::MissingClassInit {
                        symbol: class.name,
                        span: constructor_span,
                    });
                    self.synchronize(SyncPoint::ClassBody);
                    return None;
                }
            }
        }

        // TODO: parse methods
        let mut methods: Vec<MethDef> = vec![];
        while let Some(token) = self.peek() {
            let meth_span = token.span.clone();
            if let TokenType::Meth = token.token_type {
                match self.parse_method() {
                    Some(meth) => {
                        methods.push(meth);
                    }
                    None => {
                        self.errors.push(ParseError::ExpectedMethName {
                            symbol: class.name,
                            span: meth_span,
                        });
                        self.synchronize(SyncPoint::ClassBody);
                        return None;
                    }
                }
            } else {
                break;
            }
        }
        class.methods = methods;

        // TODO: uncomment once class body fully parsed
        // closing curly brace
        // if let Some(token) = self.peek() {
        //     let right_brace_span = token.span.clone();
        //     if matches!(token.token_type, TokenType::RightBrace) {
        //         self.advance();
        //     } else {
        //         self.errors.push(ParseError::MissingClosingCurlyBrace {
        //             symbol: class.name,
        //             span: right_brace_span,
        //         });
        //         self.synchronize(SyncPoint::ClassBody);
        //         return None;
        //     }
        // }

        Some(class)
    }

    // TODO: implement
    fn parse_constructor(&mut self) {}

    // TODO: implement
    fn parse_method(&mut self) -> Option<MethDef> {
        None
    }

    fn parse_comma_param_decl(&mut self, parent_name: &str) -> Option<Vec<ParamDecl>> {
        let mut params: Vec<ParamDecl> = vec![];

        match (self.peek(), self.peek_ahead()) {
            (Some(token), Some(next_token)) => {
                if token.token_type == TokenType::LeftParen
                    && next_token.token_type == TokenType::RightParen
                {
                    self.advance();
                    self.advance();
                    return Some(params);
                }

                if token.token_type == TokenType::LeftParen {
                    self.advance();
                    while let Some(token) = self.peek() {
                        if token.token_type == TokenType::RightParen {
                            self.advance();
                            break;
                        }

                        if token.token_type == TokenType::Comma {
                            self.advance();
                        }

                        if let Some(param) = self.parse_param(parent_name) {
                            params.push(param);
                        } else {
                            self.advance();
                        }
                    }
                }
            }
            _ => return None,
        }

        Some(params)
    }

    fn parse_param(&mut self, parent_name: &str) -> Option<ParamDecl> {
        let mut current_param = ParamDecl::default();

        if let Some(token) = self.peek() {
            let param_span = token.span.clone();
            match token.token_type {
                TokenType::Identifier(param_name) => {
                    current_param.name = param_name;
                    self.advance();
                }
                _ => {
                    self.errors.push(ParseError::ExpectedParamName {
                        symbol: parent_name.to_string(),
                        span: param_span,
                    });
                    return None;
                }
            }
        }

        if let Some(token) = self.peek() {
            let param_span = token.span.clone();
            match token.token_type {
                TokenType::Colon => {
                    self.advance();
                }
                _ => {
                    self.errors.push(ParseError::ExpectedColonParamDecl {
                        symbol: current_param.name,
                        span: param_span,
                    });
                    return None;
                }
            }
        }

        if let Some(token) = self.peek() {
            let param_span = token.span.clone();
            match token.token_type {
                TokenType::Type(param_type) => {
                    current_param.param_type = param_type;
                }
                _ => {
                    self.errors.push(ParseError::ExpectedParamType {
                        symbol: current_param.name,
                        span: param_span,
                    });
                    return None;
                }
            }
        }

        Some(current_param)
    }

    // ***** Helper Methods *****
    fn synchronize(&mut self, sync_point: SyncPoint) {
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

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&mut self) -> Option<Token> {
        if self.position >= self.tokens.len() {
            return None;
        }
        match self.tokens.get(self.position) {
            Some(token) => Some(token.clone()),
            None => None,
        }
    }

    fn peek_ahead(&mut self) -> Option<Token> {
        if self.position + 1 >= self.tokens.len() {
            return None;
        }
        match self.tokens.get(self.position + 1) {
            Some(token) => Some(token.clone()),
            None => None,
        }
    }

    fn is_eof(&mut self) -> bool {
        match self.peek() {
            Some(token) => token.token_type == TokenType::EOF,
            None => true,
        }
    }
}
