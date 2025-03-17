use super::*;
use crate::ast::{
    BlockStmt, BooleanLiteral, BreakStmt, ReturnStmt, Stmt, StringLiteral, VarDeclStmt,
    VarDeclWithAssign,
};
use crate::lexer::{Span, Token, TokenType};

pub trait ParserStmt {
    fn parse_block(&mut self, parent_span: Span, context: BlockContext) -> Option<Stmt>;
    fn parse_var_decl(&mut self) -> Option<Stmt>;
    fn parse_stmt(&mut self) -> Option<Stmt>;
    fn parse_break(&mut self) -> Option<Stmt>;
    fn parse_return(&mut self) -> Option<Stmt>;
}

impl ParserStmt for Parser {
    fn parse_stmt(&mut self) -> Option<Stmt> {
        let token = self.peek()?;

        match token.token_type {
            TokenType::Let => self.parse_var_decl(),
            TokenType::Break => self.parse_break(),
            TokenType::Return => self.parse_return(),
            _ => {
                println!("NO MATCH. PANIC!");
                None
            }
        }
    }

    fn parse_return(&mut self) -> Option<Stmt> {
        let span = self.current_span()?;
        self.advance(); // 'return'

        // if semicolon
        if let Some(token) = self.peek() {
            if token.token_type == TokenType::Semicolon {
                self.advance();
                return Some(Stmt::Return(ReturnStmt {
                    value: None,
                    span: token.span,
                }));
            }
        }

        // if not immediate semicolon, try to parse an expression
        let expr = self.parse_expr()?;
        self.advance_expected(TokenType::Semicolon)?;

        Some(Stmt::Return(ReturnStmt {
            value: Some(Box::new(expr)),
            span,
        }))
    }

    fn parse_break(&mut self) -> Option<Stmt> {
        let span = self.current_span()?;
        self.advance();
        Some(Stmt::Break(BreakStmt { span }))
    }

    fn parse_var_decl(&mut self) -> Option<Stmt> {
        self.advance(); // 'let'

        let token = self.peek()?;
        let span = token.span.clone();

        let var_name = match &token.token_type {
            TokenType::Identifier(name) => {
                self.advance();
                name.clone()
            }
            _ => {
                self.errors.push(ParseError::ExpectedButFound {
                    expected: TokenType::Identifier("var_name".to_string()).to_string(),
                    found: token.token_type.to_string(),
                    span,
                });
                return None;
            }
        };

        self.advance_expected(TokenType::Colon)?;

        let token = self.peek()?;
        let var_type = match &token.token_type {
            TokenType::Type(t) => {
                self.advance();
                t.clone()
            }
            _ => {
                self.errors.push(ParseError::ExpectedButFound {
                    expected: "variable type".to_string(),
                    found: token.token_type.to_string(),
                    span: token.span.clone(),
                });
                return None;
            }
        };

        // check for semicolon or assignment
        let token = self.peek()?;
        let current_span = token.span.clone();

        match token.token_type {
            TokenType::Semicolon => {
                self.advance();
                Some(Stmt::VarDecl(VarDeclStmt {
                    name: var_name,
                    var_type,
                    span: current_span,
                }))
            }
            TokenType::Assign => {
                self.advance();

                // peek next token to check if it's immediately followed by semicolon
                let next = self.peek()?;
                if next.token_type == TokenType::Semicolon {
                    self.errors.push(ParseError::ExpectedButFound {
                        expected: "expression".to_string(),
                        found: "semicolon".to_string(),
                        span: next.span.clone(),
                    });
                    return None;
                }

                let expr = self.parse_expr()?;

                // check for semicolon
                match self.peek() {
                    Some(token) if token.token_type == TokenType::Semicolon => {
                        self.advance();
                        Some(Stmt::VarDeclWithAssign(VarDeclWithAssign {
                            name: var_name,
                            var_type,
                            expr: Box::new(expr),
                            span: current_span,
                        }))
                    }
                    Some(token) => {
                        self.errors.push(ParseError::ExpectedButFound {
                            expected: TokenType::Semicolon.to_string(),
                            found: token.token_type.to_string(),
                            span,
                        });
                        None
                    }
                    None => {
                        self.errors.push(ParseError::UnexpectedEOF {
                            span: Some(current_span),
                        });
                        None
                    }
                }
            }
            _ => {
                self.errors.push(ParseError::UnexpectedToken {
                    symbol: token.token_type.to_string(),
                    span: current_span,
                });
                None
            }
        }
    }

    fn parse_block(&mut self, parent_span: Span, context: BlockContext) -> Option<Stmt> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            BooleanLiteral, BreakStmt, Expr, IntegerLiteral, ReturnStmt, Stmt, StringLiteral,
            VarDeclStmt, VarDeclWithAssign,
        },
        lexer::{Lexer, TypeName},
        parser::{ParseError, Parser},
    };

    use super::ParserStmt;

    fn parse_stmt(input: &str) -> Option<Stmt> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_stmt()
    }

    fn get_parse_errors(input: &str) -> Vec<ParseError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_stmt();
        parser.get_errors().to_vec()
    }

    #[test]
    fn test_var_decl_with_assign() {
        let stmt = parse_stmt("let myNum: Int = 5;").unwrap();
        assert!(matches!(
            stmt,
            Stmt::VarDeclWithAssign(VarDeclWithAssign {
                name,
                var_type,
                expr,
                ..
            })
            if name == "myNum"
                && var_type == TypeName::Int
                && matches!(&*expr, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if *value == 5)
        ));
    }

    #[test]
    fn test_var_decl_without_assign() {
        let stmt = parse_stmt("let myStr: Str;").unwrap();
        assert!(matches!(
            stmt,
            Stmt::VarDecl(VarDeclStmt { name, var_type, .. })
            if name == "myStr" && var_type == TypeName::Str
        ));
    }

    #[test]
    fn test_var_decl_missing_type() {
        let errors = get_parse_errors("let myVar;");
        assert!(errors.iter().any(|e| matches!(
            e,
            ParseError::ExpectedButFound { expected, found, .. }
            if expected == ":" && found == ";"
        )));
    }

    #[test]
    fn test_var_decl_missing_semicolon() {
        let errors = get_parse_errors("let myNum: Int = 5");
        assert!(errors
            .iter()
            .any(|e| matches!(e, ParseError::ExpectedButFound { .. })));
    }

    #[test]
    fn test_var_decl_missing_identifier() {
        let errors = get_parse_errors("let : Int;");
        assert!(errors.iter().any(|e| matches!(
            e,
            ParseError::ExpectedButFound { expected,  .. }
            if expected.contains("var_name")
        )));
    }

    #[test]
    fn test_var_decl_invalid_type() {
        let errors = get_parse_errors("let myVar: InvalidType;");
        assert!(errors.iter().any(|e| matches!(
            e,
            ParseError::ExpectedButFound { expected, .. }
            if expected == "variable type"
        )));
    }

    #[test]
    fn test_var_decl_missing_expression() {
        let errors = get_parse_errors("let myNum: Int =;");
        assert!(errors
            .iter()
            .any(|e| matches!(e, ParseError::ExpectedButFound { .. })));
    }

    #[test]
    fn test_var_decl_with_boolean() {
        let stmt = parse_stmt("let isValid: Boolean = true;").unwrap();
        assert!(matches!(
            stmt,
            Stmt::VarDeclWithAssign(VarDeclWithAssign {
                name,
                var_type,
                expr,
                ..
            })
            if name == "isValid"
                && var_type == TypeName::Boolean
                && matches!(&*expr, Expr::BooleanLiteral(BooleanLiteral { value, .. }) if *value == true)
        ));
    }

    #[test]
    fn test_var_decl_with_string() {
        let stmt = parse_stmt("let message: Str = \"Hello\";").unwrap();
        assert!(matches!(
            stmt,
            Stmt::VarDeclWithAssign(VarDeclWithAssign {
                name,
                var_type,
                expr,
                ..
            })
            if name == "message"
                && var_type == TypeName::Str
                && matches!(&*expr, Expr::StringLiteral(StringLiteral { value, .. }) if value == "Hello")
        ));
    }

    #[test]
    fn test_break_stmt() {
        let stmt = parse_stmt("break;").unwrap();
        assert!(matches!(stmt, Stmt::Break(BreakStmt { .. })))
    }

    #[test]
    fn test_return_stmt() {
        let stmt = parse_stmt("return;").unwrap();
        assert!(matches!(stmt, Stmt::Return(ReturnStmt { value: None, .. })));

        let stmt = parse_stmt("return 42;").unwrap();
        assert!(matches!(
            stmt,
            Stmt::Return(ReturnStmt {
                value: Some(expr),
                ..
            }) if matches!(&*expr, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if *value == 42)
        ));
    }
}
