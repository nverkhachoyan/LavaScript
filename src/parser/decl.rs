use super::*;
use crate::ast::{ClassDef, MethDef, ParamDecl};
use crate::lexer::{Span, TokenType};

pub trait ParserDecl {
    fn parse_class(&mut self) -> Option<ClassDef>;
    fn parse_constructor(&mut self);
    fn parse_method(&mut self, parent_span: Span) -> Option<MethDef>;
    fn parse_comma_param_decl(
        &mut self,
        parent_name: &str,
        parent_span: Span,
    ) -> Option<Vec<ParamDecl>>;
    fn parse_param(&mut self, parent_name: &str) -> Option<ParamDecl>;
}

impl ParserDecl for Parser {
    fn parse_class(&mut self) -> Option<ClassDef> {
        let mut class = ClassDef::default();

        // skip class keyword, parse name
        self.advance();
        if let Some(token) = self.peek() {
            let span = token.span.clone();
            match token.token_type {
                TokenType::Identifier(ident) => {
                    class.name = ident;
                    self.advance();
                }
                _ => {
                    self.errors.push(ParseError::MissingClassName { span });
                    self.synchronize(SyncPoint::ClassBody);
                    return None;
                }
            }
        }

        // skip extends, get class name for extend
        if let Some(token) = self.peek() {
            if matches!(token.token_type, TokenType::Extends) {
                let span = token.span.clone();
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
                                span,
                            });
                            self.synchronize(SyncPoint::ClassBody);
                            return None;
                        }
                    }
                    None => {
                        self.errors.push(ParseError::MissingClassExtendIdent {
                            symbol: class.name.clone(),
                            span,
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
            let span = token.span.clone();
            if matches!(token.token_type, TokenType::LeftBrace) {
                self.advance();
            } else {
                self.errors.push(ParseError::ExpectedLeftCurlyBrace {
                    symbol: class.name,
                    span,
                });
                self.synchronize(SyncPoint::ClassBody);
                return None;
            }
        }

        // class constructor
        if let Some(token) = self.peek() {
            let span = token.span.clone();
            match token.token_type {
                TokenType::Init => {
                    self.advance();
                    if let Some(params) = self.parse_comma_param_decl(&class.name, span) {
                        class.constructor.params = params;
                    }
                }
                _ => {
                    self.errors.push(ParseError::MissingClassInit {
                        symbol: class.name,
                        span: span,
                    });
                    self.synchronize(SyncPoint::ClassBody);
                    return None;
                }
            }
        }

        // TODO: parse methods
        let mut methods: Vec<MethDef> = vec![];
        while let Some(token) = self.peek() {
            let span = token.span.clone();
            if let TokenType::Meth = token.token_type {
                match self.parse_method(span) {
                    Some(meth) => {
                        methods.push(meth);
                    }
                    None => {
                        self.errors.push(ParseError::ExpectedMethName {
                            symbol: class.name,
                            span,
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
    fn parse_method(&mut self, parent_span: Span) -> Option<MethDef> {
        let mut method = MethDef::default();

        // method name
        if let Some(token) = self.peek() {
            let span = token.span.clone();
            match token.token_type {
                TokenType::Identifier(meth_name) => {
                    method.name = meth_name;
                }
                _ => {
                    self.errors.push(ParseError::ExpectedMethName {
                        symbol: "scope".to_string(),
                        span,
                    });
                }
            }
        } else {
            self.errors.push(ParseError::ExpectedMethName {
                symbol: "scope".to_string(),
                span: parent_span,
            });
            self.synchronize(SyncPoint::MethodBody);
            return None;
        }

        // param declaration, comma separated params
        match (self.peek(), self.peek_ahead()) {
            (Some(token), Some(next_token)) => {
                let left_span = token.span.clone();
                if token.token_type == TokenType::LeftParen
                    && next_token.token_type == TokenType::RightParen
                {
                    self.advance();
                    self.advance();
                } else if token.token_type == TokenType::LeftParen {
                    while let Some(inner_token) = self.peek() {
                        if inner_token.token_type != TokenType::RightParen {
                            if let Some(param) = self.parse_param(&method.name) {
                                method.params.push(param);
                            }
                        }
                    }
                } else {
                    self.errors.push(ParseError::ExpectedLeftParen {
                        symbol: method.name.clone(),
                        span: left_span,
                    });
                    self.synchronize(SyncPoint::MethodBody);
                    return None;
                }
            }
            _ => {
                self.errors
                    .push(ParseError::UnexpectedEOF { span: parent_span });
                self.synchronize(SyncPoint::MethodBody);
                return None;
            }
        }

        // get return type
        match self.peek() {
            Some(token) => {
                if token.token_type == TokenType::Arrow {
                    self.advance();
                    match self.peek() {
                        Some(inner_token) => {
                            let span = token.span.clone();
                            if let TokenType::Type(return_type) = inner_token.token_type {
                                method.return_type = return_type;
                            } else {
                                self.errors.push(ParseError::ExpectedReturnType {
                                    symbol: method.name,
                                    span,
                                });
                                self.synchronize(SyncPoint::MethodBody);
                                return None;
                            }
                        }
                        None => {
                            self.errors
                                .push(ParseError::UnexpectedEOF { span: parent_span });
                            self.synchronize(SyncPoint::MethodBody);
                            return None;
                        }
                    }
                }
            }
            None => {
                self.errors
                    .push(ParseError::UnexpectedEOF { span: parent_span });
                self.synchronize(SyncPoint::MethodBody);
                return None;
            }
        }

        // parse method body inside curly braces
        match (self.peek(), self.peek_ahead()) {
            (Some(token), Some(next_token)) => {
                let span = token.span.clone();
                if token.token_type == TokenType::LeftBrace
                    && next_token.token_type == TokenType::RightBrace
                {
                    self.advance();
                    self.advance();
                    return Some(method);
                } else if token.token_type == TokenType::LeftBrace {
                    match self.parse_block(parent_span, BlockContext::Meth) {
                        Some(block) => method.statements.push(block),
                        None => {
                            self.errors
                                .push(ParseError::UnexpectedEOF { span: parent_span }); // TODO: Add more useful error
                            self.synchronize(SyncPoint::MethodBody);
                            return None;
                        }
                    }
                } else {
                    self.errors.push(ParseError::ExpectedLeftCurlyBrace {
                        symbol: method.name.clone(),
                        span,
                    });
                }
            }
            _ => {
                self.errors
                    .push(ParseError::UnexpectedEOF { span: parent_span });
                self.synchronize(SyncPoint::MethodBody);
                return None;
            }
        }

        Some(method)
    }

    fn parse_comma_param_decl(
        &mut self,
        parent_name: &str,
        parent_span: Span,
    ) -> Option<Vec<ParamDecl>> {
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
            _ => {
                self.errors
                    .push(ParseError::UnexpectedEOF { span: parent_span });
                self.synchronize(SyncPoint::ClassBody); // TODO: choose a better sync point
                return None;
            }
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
}
