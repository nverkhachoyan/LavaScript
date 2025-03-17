use super::*;
use crate::{
    ast::{
        BinaryExpr, BooleanLiteral, Expr, FunCall, IntegerLiteral, MethCall, NewExpr, PrintExpr,
        PrintlnExpr, StringLiteral, ThisExpr, Variable,
    },
    lexer::TokenType,
};

pub trait ParserExpr {
    fn parse_expr(&mut self) -> Option<Expr>;
    fn parse_comma_expr(&mut self) -> Vec<Expr>;
    fn parse_mult_expr(&mut self) -> Option<Expr>;
    fn parse_add_expr(&mut self) -> Option<Expr>;
    fn parse_call_expr(&mut self) -> Option<Expr>;
    fn parse_primary_expr(&mut self) -> Option<Expr>;
}

impl ParserExpr for Parser {
    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_add_expr()
    }

    fn parse_comma_expr(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::<Expr>::new();

        if let Some(token) = self.peek() {
            if token.token_type == TokenType::RightParen {
                return exprs;
            }
        }

        if let Some(expr) = self.parse_expr() {
            exprs.push(expr);
        }

        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::RightParen => {
                    return exprs;
                }
                TokenType::Comma => {
                    self.advance();
                    if let Some(expr) = self.parse_expr() {
                        exprs.push(expr);
                    } else {
                        self.errors.push(ParseError::ExpectedExpressionAfterComma {
                            symbol: token.token_type.to_string(), // TODO: better error handling
                            span: token.span,
                        });
                    }
                }
                _ => {
                    self.errors.push(ParseError::UnexpectedToken {
                        symbol: token.token_type.to_string(),
                        span: token.span.clone(),
                    });
                    break;
                }
            }
        }
        exprs
    }

    fn parse_add_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_mult_expr()?;

        while let Some(token) = self.peek() {
            let span = token.span.clone();
            match token.token_type {
                TokenType::Plus | TokenType::Minus => {
                    self.advance();

                    if let Some(right) = self.parse_mult_expr() {
                        left = Expr::Binary(BinaryExpr {
                            left: Box::new(left),
                            operator: token.token_type.which_binary_op(),
                            right: Box::new(right),
                            span,
                        })
                    } else {
                        self.errors
                            .push(ParseError::UnexpectedEOF { span: Some(span) });
                        return None;
                    }
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_mult_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_call_expr()?;

        while let Some(token) = self.peek() {
            let span = token.span.clone();
            match token.token_type {
                TokenType::Star | TokenType::Slash => {
                    self.advance();

                    if let Some(right) = self.parse_call_expr() {
                        left = Expr::Binary(BinaryExpr {
                            left: Box::new(left),
                            operator: token.token_type.which_binary_op(),
                            right: Box::new(right),
                            span,
                        })
                    } else {
                        self.errors
                            .push(ParseError::UnexpectedEOF { span: Some(span) });
                        return None;
                    }
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_call_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary_expr()?;

        while let Some(token) = self.peek() {
            let span = token.span.clone();
            match token.token_type {
                TokenType::Dot => {
                    self.advance();
                    //method call
                    if let Some(meth_token) = self.peek() {
                        if let TokenType::Identifier(meth_name) = meth_token.token_type.clone() {
                            self.advance();

                            // expect left paren
                            if let Some(token) = self.peek() {
                                if token.token_type == TokenType::LeftParen {
                                    self.advance();
                                    let args = self.parse_comma_expr();

                                    // expect right paren
                                    if let Some(token) = self.peek() {
                                        if token.token_type == TokenType::RightParen {
                                            self.advance();
                                            expr = Expr::MethCall(MethCall {
                                                object: Box::new(expr),
                                                meth: meth_name,
                                                args,
                                                span,
                                            });
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self.errors.push(ParseError::UnexpectedToken {
                        symbol: token.token_type.to_string(),
                        span,
                    });
                    return None;
                }
                _ => break,
            }
        }
        Some(expr)
    }

    fn parse_primary_expr(&mut self) -> Option<Expr> {
        if let Some(token) = self.peek() {
            let span = token.span.clone();

            match token.token_type {
                TokenType::StringLiteral(str_literal) => {
                    self.advance();
                    return Some(Expr::StringLiteral(StringLiteral {
                        value: str_literal,
                        span,
                    }));
                }
                TokenType::IntegerLiteral(int_literal) => {
                    self.advance();
                    return Some(Expr::IntegerLiteral(IntegerLiteral {
                        value: int_literal,
                        span,
                    }));
                }
                TokenType::This => {
                    self.advance();
                    return Some(Expr::This(ThisExpr { span }));
                }
                TokenType::True => {
                    self.advance();
                    return Some(Expr::BooleanLiteral(BooleanLiteral { value: true, span }));
                }
                TokenType::False => {
                    self.advance();
                    return Some(Expr::BooleanLiteral(BooleanLiteral { value: false, span }));
                }
                TokenType::Println => {
                    self.advance();
                    // expect left paren
                    if let Some(token) = self.peek() {
                        if token.token_type == TokenType::LeftParen {
                            self.advance();
                        } else {
                            self.errors.push(ParseError::ExpectedButFound {
                                expected: TokenType::LeftParen.to_string(),
                                found: token.token_type.to_string(),
                                span,
                            });
                            return None;
                        }
                    } else {
                        self.errors
                            .push(ParseError::UnexpectedEOF { span: Some(span) });
                        return None;
                    }

                    let expr = self.parse_expr()?;

                    // expect right paren
                    if let Some(token) = self.peek() {
                        if token.token_type == TokenType::RightParen {
                            self.advance();
                        } else {
                            self.errors.push(ParseError::ExpectedButFound {
                                expected: TokenType::LeftParen.to_string(),
                                found: token.token_type.to_string(),
                                span,
                            });
                            return None;
                        }
                    } else {
                        self.errors
                            .push(ParseError::UnexpectedEOF { span: Some(span) });
                        return None;
                    }

                    // return println with expr
                    return Some(Expr::Println(PrintlnExpr {
                        arg: Box::new(expr),
                        span,
                    }));
                }
                TokenType::Print => {
                    self.advance();
                    // expect left paren
                    if let Some(token) = self.peek() {
                        if token.token_type == TokenType::LeftParen {
                            self.advance();
                        } else {
                            self.errors.push(ParseError::ExpectedButFound {
                                expected: TokenType::LeftParen.to_string(),
                                found: token.token_type.to_string(),
                                span,
                            });
                            return None;
                        }
                    } else {
                        self.errors
                            .push(ParseError::UnexpectedEOF { span: Some(span) });
                        return None;
                    }

                    let expr = self.parse_expr()?;

                    // expect right paren
                    if let Some(token) = self.peek() {
                        if token.token_type == TokenType::RightParen {
                            self.advance();
                        } else {
                            self.errors.push(ParseError::ExpectedButFound {
                                expected: TokenType::LeftParen.to_string(),
                                found: token.token_type.to_string(),
                                span,
                            });
                            return None;
                        }
                    } else {
                        self.errors
                            .push(ParseError::UnexpectedEOF { span: Some(span) });
                        return None;
                    }

                    // return print with expr
                    return Some(Expr::Print(PrintExpr {
                        arg: Box::new(expr),
                        span,
                    }));
                }
                TokenType::New => {
                    self.advance();

                    if let Some(token) = self.peek() {
                        if let TokenType::Identifier(class_name) = token.token_type.clone() {
                            self.advance(); // Consume the identifier

                            // expect left paren
                            if let Some(token) = self.peek() {
                                if token.token_type == TokenType::LeftParen {
                                    self.advance();
                                    let args = self.parse_comma_expr();

                                    // expect right paren
                                    if let Some(token) = self.peek() {
                                        if token.token_type == TokenType::RightParen {
                                            self.advance();
                                            return Some(Expr::New(NewExpr {
                                                class_name,
                                                args,
                                                span,
                                            }));
                                        }
                                    }
                                    self.errors.push(ParseError::ExpectedButFound {
                                        expected: TokenType::RightParen.to_string(),
                                        found: token.token_type.to_string(),
                                        span: token.span,
                                    });
                                    return None;
                                }
                                self.errors.push(ParseError::ExpectedButFound {
                                    expected: TokenType::LeftParen.to_string(),
                                    found: token.token_type.to_string(),
                                    span: token.span,
                                });
                                return None;
                            }
                            self.errors
                                .push(ParseError::UnexpectedEOF { span: Some(span) });
                            return None;
                        } else {
                            self.errors.push(ParseError::ExpectedButFound {
                                expected: TokenType::Identifier("class name".to_string())
                                    .to_string(),
                                found: token.token_type.to_string(),
                                span: token.span,
                            });
                            return None;
                        }
                    }
                    self.errors
                        .push(ParseError::UnexpectedEOF { span: Some(span) });
                    return None;
                }
                TokenType::Identifier(name) => {
                    self.advance();

                    // check if a function call
                    if let Some(token) = self.peek() {
                        if token.token_type == TokenType::LeftParen {
                            self.advance();
                            let args = self.parse_comma_expr();
                            let span = token.span.clone();

                            // expect right paren
                            if let Some(token) = self.peek() {
                                if token.token_type == TokenType::RightParen {
                                    self.advance();
                                    return Some(Expr::FunCall(FunCall {
                                        callee: name,
                                        args,
                                        span,
                                    }));
                                } else {
                                    self.errors.push(ParseError::ExpectedButFound {
                                        expected: TokenType::RightParen.to_string(),
                                        found: token.token_type.to_string(),
                                        span,
                                    });
                                    return None;
                                }
                            } else {
                                self.errors
                                    .push(ParseError::UnexpectedEOF { span: Some(span) });
                            }
                        }
                    }

                    // if no left paren, then it's a variable
                    return Some(Expr::Variable(Variable { name, span }));
                }
                TokenType::LeftParen => {
                    self.advance();
                    let expr = self.parse_expr()?;
                    let span = token.span.clone();

                    if let Some(token) = self.peek() {
                        if token.token_type == TokenType::RightParen {
                            self.advance();
                            return Some(Expr::Grouped(Box::new(expr), span));
                        }
                    }
                    self.errors.push(ParseError::ExpectedButFound {
                        expected: "expression".to_string(),
                        found: token.token_type.to_string(),
                        span,
                    });
                }
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::lexer::*;

    fn parse_expr(input: &str) -> Option<Expr> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_expr()
    }

    #[test]
    fn test_primary_expressions() {
        // Test literals
        assert!(matches!(
            parse_expr("42").unwrap(),
            Expr::IntegerLiteral(IntegerLiteral { value: 42, .. })
        ));

        assert!(matches!(
            parse_expr("\"hello\"").unwrap(),
            Expr::StringLiteral(StringLiteral { value, .. }) if value == "hello"
        ));

        assert!(matches!(
            parse_expr("true").unwrap(),
            Expr::BooleanLiteral(BooleanLiteral { value: true, .. })
        ));

        assert!(matches!(
            parse_expr("false").unwrap(),
            Expr::BooleanLiteral(BooleanLiteral { value: false, .. })
        ));

        // Test this
        assert!(matches!(
            parse_expr("this").unwrap(),
            Expr::This(ThisExpr { .. })
        ));

        // Test variable
        assert!(matches!(
            parse_expr("someVar").unwrap(),
            Expr::Variable(Variable { name, .. }) if name == "someVar"
        ));
    }

    #[test]
    fn test_function_calls() {
        // Test simple function call
        let expr = parse_expr("foo()").unwrap();
        assert!(matches!(
            expr,
            Expr::FunCall(FunCall { callee, args, .. })
            if callee == "foo" && args.is_empty()
        ));

        // Test function call with arguments
        let expr = parse_expr("bar(1, 2)").unwrap();
        if let Expr::FunCall(FunCall { callee, args, .. }) = expr {
            assert_eq!(callee, "bar");
            assert_eq!(args.len(), 2);
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_method_calls() {
        // Test simple method call
        let expr = parse_expr("obj.method()").unwrap();
        assert!(matches!(
            expr,
            Expr::MethCall(MethCall { meth, args, .. })
            if meth == "method" && args.is_empty()
        ));

        // Test chained method calls
        let expr = parse_expr("obj.foo().bar()").unwrap();
        assert!(matches!(
            expr,
            Expr::MethCall(MethCall { meth, .. })
            if meth == "bar"
        ));
    }

    #[test]
    fn test_binary_expressions() {
        // Test addition
        let expr = parse_expr("1 + 2").unwrap();
        assert!(matches!(
            expr,
            Expr::Binary(BinaryExpr {
                operator: BinaryOp::Add,
                ..
            })
        ));

        // Test multiplication with precedence
        let expr = parse_expr("1 + 2 * 3").unwrap();
        if let Expr::Binary(BinaryExpr {
            operator: BinaryOp::Add,
            left,
            right,
            ..
        }) = expr
        {
            assert!(matches!(
                *left,
                Expr::IntegerLiteral(IntegerLiteral { value: 1, .. })
            ));
            assert!(matches!(
                *right,
                Expr::Binary(BinaryExpr {
                    operator: BinaryOp::Multiply,
                    ..
                })
            ));
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_print_expressions() {
        // Test println
        let expr = parse_expr("println(42)").unwrap();
        assert!(matches!(
            expr,
            Expr::Println(PrintlnExpr { arg, .. })
            if matches!(&*arg, Expr::IntegerLiteral(IntegerLiteral { value: 42, .. }))
        ));

        // Test print
        let expr = parse_expr("print(\"hello\")").unwrap();
        assert!(matches!(
            expr,
            Expr::Print(PrintExpr { arg, .. })
            if matches!(&*arg, Expr::StringLiteral(StringLiteral { value, .. }) if value == "hello")
        ));
    }

    #[test]
    fn test_new_expressions() {
        // Test new without arguments
        let expr = parse_expr("new MyClass()").unwrap();
        assert!(matches!(
            expr,
            Expr::New(NewExpr { class_name, args, .. })
            if class_name == "MyClass" && args.is_empty()
        ));

        // Test new with arguments
        let expr = parse_expr("new MyClass(1, true)").unwrap();
        if let Expr::New(NewExpr {
            class_name, args, ..
        }) = expr
        {
            assert_eq!(class_name, "MyClass");
            assert_eq!(args.len(), 2);
        } else {
            panic!("Expected new expression");
        }
    }

    #[test]
    fn test_complex_expressions() {
        // Test complex nested expression
        let expr = parse_expr("new MyClass().method(1 + 2 * 3).other()").unwrap();
        assert!(matches!(expr, Expr::MethCall(MethCall { meth, .. }) if meth == "other"));

        // Test function call with complex arguments
        let expr = parse_expr("foo(1 + 2, obj.method(), new MyClass())").unwrap();
        if let Expr::FunCall(FunCall { callee, args, .. }) = expr {
            assert_eq!(callee, "foo");
            assert_eq!(args.len(), 3);
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_error_cases() {
        // Test missing right parenthesis
        assert!(parse_expr("foo(").is_none());

        // Test invalid method call
        assert!(parse_expr("obj.").is_none());

        // Test invalid binary operation
        assert!(parse_expr("1 + ").is_none());

        // Test invalid new expression
        assert!(parse_expr("new ").is_none());
    }
}
