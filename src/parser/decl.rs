use super::*;
use crate::ast::{ClassDef, Constructor, MethDef, ParamDecl};
use crate::lexer::{Span, TokenType};

pub trait ParserDecl {
    fn parse_class(&mut self, parent_span: Span) -> Option<ClassDef>;
    fn parse_constructor(&mut self, class_nam: &str, parent_span: Span) -> Option<Constructor>;
    fn parse_method(&mut self, class_name: &str, parent_span: Span) -> Option<MethDef>;
    fn parse_comma_param_decl(
        &mut self,
        parent_name: &str,
        parent_span: Span,
    ) -> Option<Vec<ParamDecl>>;
    fn parse_param(&mut self, parent_name: &str, parent_span: Span) -> Option<ParamDecl>;
}

impl ParserDecl for Parser {
    fn parse_class(&mut self, parent_span: Span) -> Option<ClassDef> {
        let mut class = ClassDef::default();
        self.consume(TokenType::Class)?;

        class.name = self.consume_identifier("Expected class name")?;

        if let Some(token) = self.peek() {
            if token.token_type == TokenType::Extends {
                self.consume(TokenType::Extends)?;
                class.extends = Some(self.consume_identifier("Expected parent class name")?);
            }
        }

        self.consume(TokenType::LeftBrace)?;

        // Parse class constructor
        if let Some(token) = self.peek() {
            if token.token_type == TokenType::Init {
                let span = token.span.clone();
                class.constructor = self.parse_constructor(&class.name, span)?;
            } else {
                self.errors.push(ParseError::MissingClassInit {
                    symbol: class.name.clone(),
                    span: token.span.clone(),
                });
                self.synchronize(SyncPoint::ClassBody);
                return None;
            }
        } else {
            self.errors.push(ParseError::UnexpectedEOF {
                span: Some(parent_span),
            });
            return None;
        }

        // Parse methods
        while let Some(token) = self.peek() {
            if token.token_type == TokenType::Meth {
                let span = token.span.clone();
                match self.parse_method(&class.name, span) {
                    Some(meth) => {
                        class.methods.push(meth);
                    }
                    None => {
                        self.errors.push(ParseError::ExpectedMethName {
                            symbol: class.name.clone(),
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

        // Consume closing brace
        self.consume(TokenType::RightBrace)?;

        Some(class)
    }

    // TODO: finish implement
    fn parse_constructor(&mut self, class_name: &str, parent_span: Span) -> Option<Constructor> {
        let mut constructor = Constructor::default();
        self.advance(); // skip Init keyword

        // params
        if let Some(params) = self.parse_comma_param_decl(class_name, parent_span) {
            constructor.params = params;
        }

        // opening curly
        if let Some(token) = self.peek() {
            if token.token_type == TokenType::LeftBrace {
                self.advance();
            } else {
                self.errors.push(ParseError::ExpectedLeftCurlyBrace {
                    symbol: class_name.to_string(),
                    span: parent_span,
                });
            }
        } else {
            self.errors.push(ParseError::UnexpectedEOF {
                span: Some(parent_span),
            });
        }

        // super
        if let Some(token) = self.peek() {
            if token.token_type == TokenType::Super {
                self.advance();
                match (self.peek(), self.peek_ahead()) {
                    (Some(token), Some(next_token)) => {
                        let span = token.span.clone();
                        if token.token_type == TokenType::LeftParen
                            && next_token.token_type == TokenType::RightParen
                        {
                            self.advance();
                            self.advance();

                            // semicolon
                            if let Some(token) = self.peek() {
                                if token.token_type == TokenType::Semicolon {
                                    self.advance();
                                } else {
                                    self.errors.push(ParseError::ExpectedSemicolon { span });
                                }
                            } else {
                                self.errors
                                    .push(ParseError::UnexpectedEOF { span: Some(span) });
                            }
                        } else if token.token_type == TokenType::LeftParen {
                            // expressions in super
                            let super_expressions = self.parse_comma_expr();

                            if let Some(token) = self.peek() {
                                if token.token_type == TokenType::RightParen {
                                    self.advance();

                                    // if semicolon, set super
                                    if let Some(token) = self.peek() {
                                        if token.token_type == TokenType::Semicolon {
                                            self.advance();
                                            constructor.super_call = Some(super_expressions);
                                        } else {
                                            self.errors
                                                .push(ParseError::ExpectedSemicolon { span });
                                            return None;
                                        }
                                    }
                                } else {
                                    self.errors.push(ParseError::ExpectedButFound {
                                        expected: TokenType::RightParen.to_string(),
                                        found: token.token_type.to_string(),
                                        span: Some(span),
                                    });
                                    return None;
                                }
                            }
                        } else {
                            self.errors.push(ParseError::ExpectedButFound {
                                expected: TokenType::LeftParen.to_string(),
                                found: token.token_type.to_string(),
                                span: Some(span),
                            });
                            return None;
                        }
                    }
                    _ => {
                        self.errors.push(ParseError::UnexpectedEOF {
                            span: Some(parent_span),
                        });
                        return None;
                    }
                }
            }
        } else {
            self.errors.push(ParseError::UnexpectedEOF {
                span: Some(parent_span),
            });
            return None;
        }

        // statements
        // TODO: parse statements

        // // closing curly
        // if let Some(token) = self.peek() {
        //     if token.token_type == TokenType::RightBrace {
        //         self.advance();
        //     } else {
        //         self.errors.push(ParseError::ExpectedRightCurlyBrace {
        //             symbol: class_name.to_string(),
        //             span: parent_span,
        //         });
        //     }
        // } else {
        //     self.errors
        //         .push(ParseError::UnexpectedEOF { span: parent_span });
        // }

        Some(constructor)
    }

    // TODO: implement
    fn parse_method(&mut self, class_name: &str, parent_span: Span) -> Option<MethDef> {
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
                        symbol: class_name.to_string(),
                        span,
                    });
                }
            }
        } else {
            self.errors.push(ParseError::ExpectedMethName {
                symbol: class_name.to_string(),
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
                    self.advance();
                    while let Some(inner_token) = self.peek() {
                        let inner_span = inner_token.span.clone();
                        if inner_token.token_type != TokenType::RightParen {
                            if let Some(param) = self.parse_param(&method.name, inner_span) {
                                method.params.push(param);
                            }
                        } else if inner_token.token_type == TokenType::RightParen
                            || inner_token.token_type == TokenType::Comma
                        {
                            self.advance();
                            break;
                        } else {
                            self.errors.push(ParseError::UnexpectedToken {
                                symbol: inner_token.token_type.to_string(),
                                span: inner_span,
                            });
                        }
                    }
                } else {
                    self.errors.push(ParseError::ExpectedButFound {
                        expected: TokenType::LeftParen.to_string(),
                        found: token.token_type.to_string(),
                        span: Some(left_span),
                    });
                    self.synchronize(SyncPoint::MethodBody);
                    return None;
                }
            }
            _ => {
                self.errors.push(ParseError::UnexpectedEOF {
                    span: Some(parent_span),
                });
                self.synchronize(SyncPoint::MethodBody);
                return None;
            }
        }

        // return type
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
                            self.errors.push(ParseError::UnexpectedEOF {
                                span: Some(parent_span),
                            });
                            self.synchronize(SyncPoint::MethodBody);
                            return None;
                        }
                    }
                }
            }
            None => {
                self.errors.push(ParseError::UnexpectedEOF {
                    span: Some(parent_span),
                });
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
                    match self.parse_block() {
                        Some(block) => method.statements.push(block),
                        None => {
                            self.errors.push(ParseError::UnexpectedEOF {
                                span: Some(parent_span),
                            }); // TODO: Add more useful error
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
                self.errors.push(ParseError::UnexpectedEOF {
                    span: Some(parent_span),
                });
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
                        let span = token.span.clone();
                        if token.token_type == TokenType::RightParen {
                            self.advance();
                            break;
                        }

                        if token.token_type == TokenType::Comma {
                            self.advance();
                        }

                        if let Some(param) = self.parse_param(parent_name, span) {
                            params.push(param);
                        } else {
                            self.advance();
                        }
                    }
                }
            }
            _ => {
                self.errors.push(ParseError::UnexpectedEOF {
                    span: Some(parent_span),
                });
                self.synchronize(SyncPoint::ClassBody); // TODO: choose a better sync point
                return None;
            }
        }

        Some(params)
    }

    fn parse_param(&mut self, _parent_name: &str, _parent_span: Span) -> Option<ParamDecl> {
        let mut current_param = ParamDecl::default();

        // Parse parameter name using our helper
        let param_name = self.consume_identifier("Expected parameter name")?;
        current_param.name = param_name;

        // Consume colon
        self.consume(TokenType::Colon)?;

        // Parse parameter type using our helper
        let param_type = self.consume_type()?;
        current_param.param_type = param_type;

        Some(current_param)
    }
}

mod tests {
    use crate::{ast::*, lexer::*, parser::*};

    use super::ParserDecl;

    fn parse_class(input: &str) -> Option<ClassDef> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_class(Span{line:0,column:0})
    }

    fn parse_method(input: &str) -> Option<MethDef> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        for token in &tokens {
            println!("{:?}", token);
        }
        let mut parser = Parser::new(tokens);
        parser.parse_method("Dummy",Span{line:0,column:0})
    }

    #[test]
    fn test_minimal_class_decl() {
        let class = parse_class("class Animal { init() {} }").unwrap();
        assert!(matches!(
            class,
            ClassDef {
                name,
                extends,
                vars,
                constructor,
                methods
            }
            if name == "Animal"
                && extends == None
                && vars == []
                && methods.len() == 0
                && matches!(&constructor, Constructor {params, ..} if params.len() == 0)

        ))
    }

    #[test]
    fn test_minimal_inherited_class_decl() {
        let class = parse_class("class Cat extends Animal { init() {super();} }").unwrap();
        assert!(matches!(
            class,
            ClassDef {
                name,
                extends,
                vars,
                constructor,
                methods
            }
            if name == "Cat"
                && extends == Some("Animal".to_string())
                && vars == []
                && methods.len() == 0
                && matches!(&constructor, Constructor {params, ..} if params.len() == 0)

        ))
    }

    #[test]
    fn test_class_decl_with_params() {
        let class = parse_class("class Animal { init(voice: Str) {} }").unwrap();
        assert!(matches!(
            class,
            ClassDef {
                name,
                extends,
                vars,
                constructor,
                methods
            }
            if name == "Animal"
                && extends == None
                && vars == []
                && methods.len() == 0
                && matches!(&constructor, Constructor {params, ..} if params.len() == 1)

        ))
    }

   #[test]
    fn test_class_decl_with_method() {
        let class = parse_class("class Animal { init() {} 
        meth speak() -> Void { return println(\"animal noise\"); }}").unwrap();
        assert!(matches!(
            class,
            ClassDef {
                name,
                extends,
                vars,
                constructor,
                methods
            }
            if name == "Animal"
                && extends == None
                && vars == []
                && methods.len() == 1
                && matches!(&constructor, Constructor {params, ..} if params.len() == 0)

        ))
    }

    #[test]
    fn test_minimal_method_decl() {
        let method = parse_method("methodName() -> Void {1 + 1;}").unwrap();
        assert!(matches!(
            method,
            MethDef {
                name,
                params,
                return_type
                ,statements
            }
            if name == "method"
                && params.len() == 0
                && return_type == TypeName::Void
                && statements.len() == 0
        ))
    }
}
