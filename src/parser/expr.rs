use super::*;
use crate::{
    ast::{
        BinaryExpr, BooleanLiteral, Expr, FunCall, IntegerLiteral, MethCall, NewExpr, PrintExpr,
        PrintlnExpr, StringLiteral, ThisExpr, UnaryExpr, Variable,
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
    fn parse_or_expr(&mut self) -> Option<Expr>;
    fn parse_and_expr(&mut self) -> Option<Expr>;
    fn parse_comparison_expr(&mut self) -> Option<Expr>;
    fn parse_unary_expr(&mut self) -> Option<Expr>;
}

impl ParserExpr for Parser {
    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_or_expr()
    }

    fn parse_unary_expr(&mut self) -> Option<Expr> {
        let token = self.peek()?;
        let span = self.current_span()?;

        match token.token_type {
            TokenType::Not => {
                self.advance();
                let unary_expr = self.parse_unary_expr()?;

                Some(Expr::Unary(UnaryExpr {
                    operator: crate::ast::UnaryOp::Not,
                    expr: Box::new(unary_expr),
                    span,
                }))
            }
            TokenType::Minus => {
                self.advance();
                let unary_expr = self.parse_unary_expr()?;

                Some(Expr::Unary(UnaryExpr {
                    operator: crate::ast::UnaryOp::Negate,
                    expr: Box::new(unary_expr),
                    span,
                }))
            }
            TokenType::Plus => {
                self.advance();
                let unary_expr = self.parse_unary_expr()?;

                Some(Expr::Unary(UnaryExpr {
                    operator: crate::ast::UnaryOp::Plus,
                    expr: Box::new(unary_expr),
                    span,
                }))
            }
            _ => self.parse_call_expr(),
        }
    }

    fn parse_comma_expr(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::<Expr>::new();
        self.consume(TokenType::LeftParen);

        if let Some(expr) = self.parse_expr() {
            exprs.push(expr);
        }

        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::RightParen => {
                    self.advance();
                    return exprs;
                }
                TokenType::Comma => {
                    self.advance();
                    if let Some(expr) = self.parse_expr() {
                        exprs.push(expr);
                    } else {
                        self.errors.push(ParseError::ExpectedExpressionAfterComma {
                            symbol: token.token_type.to_string(),
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
            match token.token_type {
                TokenType::Plus | TokenType::Minus => {
                    self.advance();

                    if let Some(right) = self.parse_mult_expr() {
                        let span = self.current_span()?;
                        left = Expr::Binary(BinaryExpr {
                            left: Box::new(left),
                            operator: token.token_type.which_binary_op(),
                            right: Box::new(right),
                            span,
                        })
                    } else {
                        let span = self.current_span()?;
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

    fn parse_comparison_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_add_expr()?;

        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Less
                | TokenType::LessEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Equal
                | TokenType::NotEqual => {
                    self.advance();
                    if let Some(right) = self.parse_add_expr() {
                        let span = self.current_span()?;
                        left = Expr::Binary(BinaryExpr {
                            left: Box::new(left),
                            operator: token.token_type.which_binary_op(),
                            right: Box::new(right),
                            span,
                        })
                    } else {
                        let span = self.current_span()?;
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

    fn parse_and_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_comparison_expr()?;

        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::And => {
                    self.advance();
                    if let Some(right) = self.parse_comparison_expr() {
                        let span = token.span.clone();
                        left = Expr::Binary(BinaryExpr {
                            left: Box::new(left),
                            operator: token.token_type.which_binary_op(),
                            right: Box::new(right),
                            span,
                        })
                    } else {
                        let span = self.current_span()?;
                        self.errors.push(ParseError::ExpectedExpression { span });
                    }
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn parse_or_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_and_expr()?;

        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Or => {
                    self.advance();
                    if let Some(right) = self.parse_and_expr() {
                        let span = token.span.clone();
                        left = Expr::Binary(BinaryExpr {
                            left: Box::new(left),
                            operator: token.token_type.which_binary_op(),
                            right: Box::new(right),
                            span,
                        })
                    } else {
                        let span = self.current_span()?;
                        self.errors.push(ParseError::ExpectedExpression { span });
                    }
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn parse_mult_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_unary_expr()?;

        while let Some(token) = self.peek() {
            let span = token.span.clone();
            match token.token_type {
                TokenType::Star | TokenType::Slash => {
                    self.advance();

                    if let Some(right) = self.parse_unary_expr() {
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
            let span = self.current_span()?;
            match token.token_type {
                TokenType::Dot => {
                    self.advance();
                    let ident = self.consume_identifier("method name")?;
                    let args = self.parse_comma_expr();
                    expr = Expr::MethCall(MethCall {
                        object: Box::new(expr),
                        meth: ident,
                        args,
                        span,
                    });
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
                    self.consume(TokenType::LeftParen)?;
                    let expr = self.parse_expr()?;
                    self.consume(TokenType::RightParen)?;
                    return Some(Expr::Println(PrintlnExpr {
                        arg: Box::new(expr),
                        span,
                    }));
                }
                TokenType::Print => {
                    self.advance();
                    self.consume(TokenType::LeftParen)?;
                    let expr = self.parse_expr()?;
                    self.consume(TokenType::RightParen)?;

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
                            let args = self.parse_comma_expr();
                            return Some(Expr::New(NewExpr {
                                class_name,
                                args,
                                span,
                            }));
                        } else {
                            self.errors.push(ParseError::ExpectedButFound {
                                expected: TokenType::Identifier("class name".to_string())
                                    .to_string(),
                                found: token.token_type.to_string(),
                                span: Some(token.span),
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
                            let args = self.parse_comma_expr();
                            let span = self.current_span()?;
                            return Some(Expr::FunCall(FunCall {
                                callee: name,
                                args,
                                span,
                            }));
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
                        span: Some(span),
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

    fn get_expression_errors(input: &str) -> Vec<ParseError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_expr();
        parser.get_errors().to_vec()
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
    fn test_complexest_expression() {
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
    fn test_unary_not_expression() {
        let expr = parse_expr("!true").unwrap();
        assert!(matches!(
            expr,
            Expr::Unary(UnaryExpr { operator, .. })
            if operator == UnaryOp::Not
        ))
    }
    #[test]
    fn test_unary_negate_expression() {
        let expr = parse_expr("-x").unwrap();
        assert!(matches!(
            expr,
            Expr::Unary(UnaryExpr { operator, .. })
            if operator == UnaryOp::Negate
        ))
    }
    #[test]
    fn test_unary_plus_expression() {
        let expr = parse_expr("+x").unwrap();
        assert!(matches!(
            expr,
            Expr::Unary(UnaryExpr { operator, .. })
            if operator == UnaryOp::Plus
        ))
    }

    #[test]
    fn test_boolean_and_expression() {
        let expr = parse_expr("true && true").unwrap();
        assert!(matches!(
            expr,
            Expr::Binary(BinaryExpr { operator, ..})
            if operator == BinaryOp::And
        ))
    }

    #[test]
    fn test_boolean_or_expression() {
        let expr = parse_expr("true || false").unwrap();
        assert!(matches!(
            expr,
            Expr::Binary(BinaryExpr { operator, ..})
            if operator == BinaryOp::Or
        ))
    }

    #[test]
    fn test_comma_expr_error() {
        let errors = get_expression_errors("bar(1,)");
        assert!(errors.iter().any(|e| matches!(
            e, ParseError::ExpectedExpressionAfterComma { .. }
        )))
    }

    #[test]
    fn test_new_expr_error() {
        let errors = get_expression_errors("new");
        assert!(errors.iter().any(|e| matches!(
            e, ParseError::ExpectedButFound { .. }
        )))
    }

    #[test]
    fn test_add_expr() {
        let errors = get_expression_errors("5 +");
        assert!(errors.iter().any(|e| matches!(e, ParseError::UnexpectedEOF { .. })));
    }

    #[test]
    fn test_mult_expr() {
        let errors = get_expression_errors("4 *");
        assert!(errors.iter().any(|e| matches!(e, ParseError::UnexpectedEOF { .. })));
    }

    #[test]
    fn test_comparison_expr() {
        let errors = get_expression_errors("5 <");
        assert!(errors.iter().any(|e| matches!(e, ParseError::UnexpectedEOF { .. })));
    }

    #[test]
    fn test_and_expr() {
        let errors = get_expression_errors("true &&");
        assert!(errors.iter().any(|e| matches!(e, ParseError::ExpectedExpression { .. })));
    }

    #[test]
    fn test_or_expr() {
        let errors = get_expression_errors("true ||");
        assert!(errors.iter().any(|e| matches!(e, ParseError::ExpectedExpression { .. })));
    }

    #[test]
    fn test_expr_missing_rparen() {
        let errors = get_expression_errors("(5 + 3");
        assert!(errors.iter().any(|e| matches!(e, ParseError::ExpectedButFound { expected, .. } if expected == "expression")));
    }
}
