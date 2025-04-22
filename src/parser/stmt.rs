use super::*;
use crate::ast::{
    AssignStmt, BlockStmt, BreakStmt, ExprStmt, Field, IfStmt, ReturnStmt, Stmt, VarDeclStmt, VarDeclWithAssign, WhileStmt
};
use crate::lexer::TokenType;
use crate::parser::types::expected;

pub trait ParserStmt {
    fn parse_field_assign(&mut self) -> Option<Stmt>;
    fn parse_var_decl(&mut self) -> Option<Stmt>;
    fn parse_var_assign(&mut self) -> Option<Stmt>;
    fn parse_stmt(&mut self) -> Option<Stmt>;
    fn parse_break(&mut self) -> Option<Stmt>;
    fn parse_return(&mut self) -> Option<Stmt>;
    fn parse_if(&mut self) -> Option<Stmt>;
    fn parse_while(&mut self) -> Option<Stmt>;
    fn parse_expr_stmt(&mut self) -> Option<Stmt>;
    fn parse_block(&mut self) -> Option<Stmt>;
}

impl ParserStmt for Parser {
    fn parse_stmt(&mut self) -> Option<Stmt> {
        let token = self.peek()?;
        let next_token = self.peek_ahead()?;

        match token.token_type {
            TokenType::Let => self.parse_var_decl(),
            TokenType::Break => self.parse_break(),
            TokenType::Return => self.parse_return(),
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::LeftBrace => self.parse_block(),
            TokenType::Identifier(_) => {
                if matches!(next_token.token_type, TokenType::Assign) {
                    return self.parse_var_assign();
                }
                else if matches!(next_token.token_type, TokenType::Dot) {
                    let nexter_token = self.peek_ahead_amount(3)?;
                    if !matches!(nexter_token.token_type, TokenType::LeftParen) {
                        return self.parse_field_assign();
                    }
                }
                return self.parse_expr_stmt();
            }
            TokenType::This => {
                if matches!(next_token.token_type, TokenType::Dot) {
                    let nexter_token = self.peek_ahead_amount(3)?;
                    if !matches!(nexter_token.token_type, TokenType::LeftParen) {
                        return self.parse_field_assign();
                    }
                }
                return self.parse_expr_stmt();
            }
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        let expr = self.parse_expr()?;
        let span = self.current_span()?;
        Some(Stmt::Expr(ExprStmt {
            expr: Box::new(expr),
            span,
        }))
    }

    fn parse_while(&mut self) -> Option<Stmt> {
        self.consume(TokenType::While)?;
        self.consume(TokenType::LeftParen)?;

        let expr = match self.parse_expr() {
            Some(expr) => expr,
            None => {
                let span = self.current_span();
                self.errors.push(ParseError::expected_but_found(
                    expected::EXPRESSION.to_string(),
                    None,
                    span,
                ));
                return None;
            }
        };

        self.consume(TokenType::RightParen)?;

        if let Some(stmt) = self.parse_stmt() {
            let span = self.current_span()?;
            return Some(Stmt::While(WhileStmt {
                condition: Box::new(expr),
                body: Box::new(stmt),
                span,
            }));
        }

        let span = self.current_span();
        self.errors.push(ParseError::expected_but_found(
            expected::EXPRESSION.to_string(),
            None,
            span,
        ));
        None
    }

    fn parse_if(&mut self) -> Option<Stmt> {
        self.consume(TokenType::If)?;
        self.consume(TokenType::LeftParen)?;

        let condition = match self.parse_expr() {
            Some(expr) => expr,
            None => {
                let span = self.current_span();
                self.errors.push(ParseError::expected_but_found(
                    expected::EXPRESSION.to_string(),
                    None,
                    span,
                ));
                return None;
            }
        };

        self.consume(TokenType::RightParen)?;
        self.consume(TokenType::LeftBrace)?;

        let then_stmt = match self.parse_stmt() {
            Some(stmt) => stmt,
            None => {
                let span = self.current_span();
                self.errors.push(ParseError::expected_but_found(
                    expected::STATEMENT.to_string(),
                    None,
                    span,
                ));
                return None;
            }
        };

        self.consume(TokenType::RightBrace)?;

        let mut else_branch = None;

        if let Some(token) = self.peek() {
            if token.token_type == TokenType::Else {
                self.consume(TokenType::Else)?;
                self.consume(TokenType::LeftBrace)?;

                else_branch = match self.parse_stmt() {
                    Some(else_stmt) => Some(Box::new(else_stmt)),
                    None => {
                        let span = self.current_span();
                        self.errors.push(ParseError::expected_but_found(
                            expected::STATEMENT.to_string(),
                            None,
                            span,
                        ));
                        return None;
                    }
                };

                self.consume(TokenType::RightBrace)?;
            }
        }

        let span = self.current_span()?;
        Some(Stmt::If(IfStmt {
            condition: Box::new(condition),
            then_branch: Box::new(then_stmt),
            else_branch,
            span,
        }))
    }

    fn parse_var_assign(&mut self) -> Option<Stmt> {
        let token = self.peek()?;

        let var_name = self.consume_identifier("var_name")?;

        self.consume(TokenType::Assign)?;

        if let Some(expr) = self.parse_expr() {
            let span = self.current_span()?;
            return Some(Stmt::Assign(AssignStmt {
                name: var_name,
                expr: Box::new(expr),
                span,
            }));
        }
        self.errors.push(ParseError::expected_but_found(
            "expression".to_string(),
            None,
            Some(token.span),
        ));
        return None;
    }

    fn parse_field_assign(&mut self) -> Option<Stmt> {
        let token = self.peek()?;

        match self.parse_call_expr(){
            Some(left_expr) => match left_expr{
                crate::ast::Expr::Field(field) => {
                    let full_name= self.parse_full_field_expr_name(crate::ast::Expr::Field(field));

                    self.consume(TokenType::Assign)?;

                    if let Some(expr) = self.parse_expr() {
                        let span = self.current_span()?;
                        return Some(Stmt::Assign(AssignStmt {
                            name: full_name,
                            expr: Box::new(expr),
                            span,
                        }));
                    }
                    self.errors.push(ParseError::expected_but_found(
                        "expression".to_string(),
                        None,
                        Some(token.span),
                    ));
                    return None;

                }
                _ => None,
            }
            None => None,
        }
    }

    fn parse_return(&mut self) -> Option<Stmt> {
        let span = self.current_span()?;
        self.consume(TokenType::Return)?;

        if let Some(token) = self.consume_optional(TokenType::Semicolon) {
            return Some(Stmt::Return(ReturnStmt {
                value: None,
                span: token.span,
            }));
        }

        let expr = self.parse_expr()?;
        self.consume(TokenType::Semicolon)?;

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
        self.consume(TokenType::Let)?;
        let var_name = self.consume_identifier("var_name")?;
        self.consume(TokenType::Colon)?;

        let var_type = self.consume_type()?;

        let token = self.peek()?;
        let current_span = token.span.clone();

        match token.token_type {
            TokenType::Semicolon => {
                self.consume(TokenType::Semicolon);

                Some(Stmt::VarDecl(VarDeclStmt {
                    name: var_name,
                    var_type,
                    span: current_span,
                }))
            }
            TokenType::Assign => {
                self.consume(TokenType::Assign)?;

                let next = self.peek()?;
                if next.token_type == TokenType::Semicolon {
                    self.errors.push(ParseError::expected_but_found(
                        "expression".to_string(),
                        Some("semicolon".to_string()),
                        Some(next.span.clone()),
                    ));
                    return None;
                }

                let expr = self.parse_expr()?;
                self.consume(TokenType::Semicolon)?;

                Some(Stmt::VarDeclWithAssign(VarDeclWithAssign {
                    name: var_name,
                    var_type,
                    expr: Box::new(expr),
                    span: current_span,
                }))
            }
            _ => {
                self.errors.push(ParseError::expected_but_found(
                    "semicolon or assign".to_string(),
                    Some(token.token_type.to_string()),
                    Some(token.span),
                ));
                None
            }
        }
    }

    fn parse_block(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftBrace)?;

        let mut statements = Vec::<Stmt>::new();

        while let Some(token) = self.peek() {
            if token.token_type == TokenType::RightBrace {
                break;
            }

            if let Some(stmt) = self.parse_stmt() {
                statements.push(stmt);
            } else {
                // if parsing stmt fails, advance to avoid infinite loop
                self.advance();
            }
        }

        self.consume(TokenType::RightBrace)?;

        let span = self.current_span()?;
        Some(Stmt::Block(BlockStmt { statements, span }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::*, lexer::*, parser::*};

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
    fn test_var_assign_with_int() {
        let stmt = parse_stmt("myNum = 5;").unwrap();
        println!("{}", stmt);
        assert!(matches!(stmt, Stmt::Assign(AssignStmt { name, expr, .. })
            if name == "myNum" && matches!(&*expr, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if *value == 5)
        ));
    }

    #[test]
    fn test_method_call_statement() {
        let stmt = parse_stmt("object.method()").unwrap();
        println!("{}", stmt);
    }

    #[test]
    fn test_field_assignment() {
        let stmt = parse_stmt("object.field = 5").unwrap();
        println!("{:?}",stmt);
        assert!(matches!(stmt, Stmt::Assign(AssignStmt { name, expr, .. })
            if name=="object.field" && matches!(&*expr, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if *value == 5)
        ));
    }

    #[test]
    fn test_this_assignment() {
        let stmt = parse_stmt("this.field = 5").unwrap();
        println!("{:?}",stmt);
        assert!(matches!(stmt, Stmt::Assign(AssignStmt { name, expr, .. })
            if name=="this.field" && matches!(&*expr, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if *value == 5)
        ));
    }

    #[test]
    fn test_nested_field_assignment() {
        let stmt = parse_stmt("object.subobject.subsubobject.field = 5").unwrap();
        println!("{:?}",stmt);
        assert!(matches!(stmt, Stmt::Assign(AssignStmt { name, expr, .. })
            if name=="object.subobject.subsubobject.field" && matches!(&*expr, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if *value == 5)
        ));
    }

    #[test]
    fn test_var_assign_with_string() {
        let stmt = parse_stmt("myStr = \"hello world\";").unwrap();
        assert!(matches!(stmt, Stmt::Assign(AssignStmt {name, expr, ..})
            if name == "myStr"
            && matches!(&*expr, Expr::StringLiteral(StringLiteral { value, .. }) if *value == "hello world")
        ));
    }

    #[test]
    fn test_var_assign_with_var() {
        let stmt = parse_stmt("myStr = anotherStr;").unwrap();
        assert!(matches!(stmt, Stmt::Assign(AssignStmt {name, expr, ..})
            if name == "myStr"
            && matches!(&*expr, Expr::Variable(Variable {name, ..}) if name == "anotherStr")
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

    #[test]
    fn test_if_then() {
        let stmt = parse_stmt("if (5) {println(\"hello\")}").unwrap();
        assert!(matches!(
            stmt,
            Stmt::If(IfStmt {
                condition,
                then_branch,
                ..
            }) if matches!(&*condition, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if *value == 5)
                && matches!(&*then_branch, Stmt::Expr(ExprStmt { expr, .. })
                    if matches!(&**expr, Expr::Println(PrintlnExpr { arg, .. })
                        if matches!(&**arg, Expr::StringLiteral(StringLiteral { value, .. })
                            if value == "hello")))
        ));
    }

    #[test]
    fn test_if_else_then() {
        let stmt = parse_stmt("if (5) {println(\"hello\")} else {\"bye\"}").unwrap();
        assert!(matches!(
            stmt,
            Stmt::If(IfStmt {
                condition,
                then_branch,
                else_branch,
                ..
            }) if matches!(&*condition, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if *value == 5)
                && matches!(&*then_branch, Stmt::Expr(ExprStmt { expr, .. })
                    if matches!(&**expr, Expr::Println(PrintlnExpr { arg, .. })
                        if matches!(&**arg, Expr::StringLiteral(StringLiteral { value, .. })
                            if value == "hello")))
                && matches!(&else_branch, Some(else_stmt)
                    if matches!(&**else_stmt, Stmt::Expr(ExprStmt { expr, .. })
                        if matches!(&**expr, Expr::StringLiteral(StringLiteral { value, .. })
                            if value == "bye")))
        ));
    }

    #[test]
    fn test_simple_binary_expr_stmt() {
        let stmt = parse_stmt("5 + 3;").unwrap();
        assert!(matches!(
            stmt,
            Stmt::Expr(ExprStmt { expr, .. })
            if matches!(&*expr, Expr::Binary(BinaryExpr {
                left,
                operator,
                right,
                ..
            }) if matches!(*left.as_ref(), Expr::IntegerLiteral(IntegerLiteral { value, .. }) if value == 5)
                && matches!(operator, BinaryOp::Add)
                && matches!(*right.as_ref(), Expr::IntegerLiteral(IntegerLiteral { value, .. }) if value == 3))
        ));
    }

    #[test]
    fn test_multiplicative_binary_expr_stmt() {
        let stmt = parse_stmt("7 * 2;").unwrap();
        assert!(matches!(
            stmt,
            Stmt::Expr(ExprStmt { expr, .. })
            if matches!(&*expr, Expr::Binary(BinaryExpr {
                left,
                operator,
                right,
                ..
            }) if matches!(*left.as_ref(), Expr::IntegerLiteral(IntegerLiteral { value, .. }) if value == 7)
                && matches!(operator, BinaryOp::Multiply)
                && matches!(*right.as_ref(), Expr::IntegerLiteral(IntegerLiteral { value, .. }) if value == 2))
        ));
    }

    #[test]
    fn test_complex_binary_expr_with_precedence() {
        let stmt = parse_stmt("1 + 2 * 3;").unwrap();

        if let Stmt::Expr(ExprStmt { expr, .. }) = stmt {
            if let Expr::Binary(BinaryExpr {
                left,
                operator: BinaryOp::Add,
                right,
                ..
            }) = *expr
            {
                assert!(
                    matches!(*left, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if value == 1)
                );

                if let Expr::Binary(BinaryExpr {
                    left: mult_left,
                    operator: BinaryOp::Multiply,
                    right: mult_right,
                    ..
                }) = *right
                {
                    assert!(
                        matches!(*mult_left, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if value == 2)
                    );
                    assert!(
                        matches!(*mult_right, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if value == 3)
                    );
                } else {
                    panic!("Expected right side to be multiplication");
                }
            } else {
                panic!("Expected binary addition expression");
            }
        } else {
            panic!("Expected expression statement");
        }
    }

    #[test]
    fn test_nested_parenthesized_expr() {
        let stmt = parse_stmt("(2 + 3) * 4;").unwrap();

        assert!(matches!(
            stmt,
            Stmt::Expr(ExprStmt { expr, .. })
            if matches!(&*expr, Expr::Binary(BinaryExpr {
                left,
                operator: BinaryOp::Multiply,
                right,
                ..
            }) if matches!(*left.as_ref(), Expr::Grouped(..))
                && matches!(*right.as_ref(), Expr::IntegerLiteral(IntegerLiteral { value, .. }) if value == 4))
        ));
    }

    #[test]
    fn test_complex_expression_with_multiple_operations() {
        let stmt = parse_stmt("5 * (8 / 2 + 3) - 1;").unwrap();
        stmt.print();

        assert!(matches!(
            stmt,
            Stmt::Expr(ExprStmt { expr, .. })
            if matches!(&*expr, Expr::Binary(BinaryExpr {
                left: _,
                operator: BinaryOp::Subtract,
                right,
                ..
            }) if matches!(*right.as_ref(), Expr::IntegerLiteral(IntegerLiteral { value, .. }) if value == 1))
        ));
    }

    #[test]
    fn test_very_complex_nested_expression() {
        let stmt = parse_stmt("5 * (((8 / 2) + 3 * (4) / 2) - 1) * 7;").unwrap();

        assert!(matches!(
            stmt,
            Stmt::Expr(ExprStmt { expr, .. })
            if matches!(&*expr, Expr::Binary(BinaryExpr { .. }))
        ));
    }

    #[test]
    fn test_function_call_with_complex_args() {
        let stmt = parse_stmt("println(5 + 3 * 2);").unwrap();
        stmt.print();

        assert!(matches!(
            stmt,
            Stmt::Expr(ExprStmt { expr, .. })
            if matches!(&*expr, Expr::Println(PrintlnExpr { .. }))
        ));
    }

    #[test]
    fn test_method_call_with_complex_expr() {
        let stmt = parse_stmt("println(\"hello\");").unwrap();

        assert!(matches!(
            stmt,
            Stmt::Expr(ExprStmt { expr, .. })
            if matches!(&*expr, Expr::Println(PrintlnExpr { arg, .. })
                if matches!(&**arg, Expr::StringLiteral(StringLiteral { value, .. }) if value == "hello"))
        ));
    }

    #[test]
    fn test_var_assign_with_complex_expr() {
        let stmt = parse_stmt("result = 5 * (3 + 2) / (10 - 5);").unwrap();

        assert!(matches!(
            stmt,
            Stmt::Assign(AssignStmt {
                name,
                expr,
                ..
            }) if name == "result" && matches!(&*expr, Expr::Binary(BinaryExpr { .. }))
        ));
    }

    #[test]
    fn test_var_decl_with_complex_expr() {
        let stmt = parse_stmt("let complexCalc: Int = ((5 + 3) * 2) / (7 - 3);").unwrap();

        assert!(matches!(
            stmt,
            Stmt::VarDeclWithAssign(VarDeclWithAssign {
                name,
                var_type,
                expr,
                ..
            }) if name == "complexCalc"
                && var_type == TypeName::Int
                && matches!(&*expr, Expr::Binary(BinaryExpr { .. }))
        ));
    }

    #[test]
    fn test_nested_expressions_with_different_operators() {
        let stmt = parse_stmt("(2 + 3) * 4 / 2 - 1;").unwrap();

        assert!(matches!(
            stmt,
            Stmt::Expr(ExprStmt { expr, .. })
            if matches!(&*expr, Expr::Binary(BinaryExpr { operator: BinaryOp::Subtract, .. }))
        ));
    }

    #[test]
    fn test_parenthesized_expressions_in_var_decl() {
        let stmt = parse_stmt("let result: Int = (10 + 5) * (8 - 3);").unwrap();

        assert!(matches!(
            stmt,
            Stmt::VarDeclWithAssign(VarDeclWithAssign {
                name,
                var_type,
                expr,
                ..
            }) if name == "result"
                && var_type == TypeName::Int
                && matches!(&*expr, Expr::Binary(BinaryExpr { operator: BinaryOp::Multiply, .. }))
        ));
    }

    #[test]
    fn test_complex_expr_in_if_condition() {
        let stmt = parse_stmt("if (5 * 2 + 3) {println(\"true\")}").unwrap();

        if let Stmt::If(IfStmt { condition, .. }) = stmt {
            assert!(matches!(*condition, Expr::Binary(BinaryExpr { .. })));
        } else {
            panic!("Expected if statement");
        }
    }

    #[test]
    fn test_complex_expr_in_while_condition() {
        let stmt = parse_stmt("while (5) {break;}").unwrap();

        if let Stmt::While(WhileStmt { condition, .. }) = stmt {
            assert!(
                matches!(*condition, Expr::IntegerLiteral(IntegerLiteral { value, .. }) if value == 5)
            );
        } else {
            panic!("Expected while statement");
        }
    }

    #[test]
    fn test_operator_precedence() {
        let first_stmt = parse_stmt("1 + 2 * 3").unwrap();
        first_stmt.print();

        let second_stmt = parse_stmt("1 * 2 + 3").unwrap();
        second_stmt.print();
    }

    #[test]
    fn test_invalid_type() {
        let errors = get_parse_errors("let x: Int * 5;");
        assert!(errors.iter().any(|e| matches!(
            e,
            ParseError::ExpectedButFound { expected, .. } if expected.contains("semicolon or assign")
        )));
    }

    #[test]
    fn test_invalid_stmt_in_block() {
        let errors = get_parse_errors("{ let x = ; }");
        assert!(errors.iter().any(|e| matches!(e, ParseError::ExpectedButFound { .. })));
    }

    #[test]
    fn test_missing_var() {
        let errors = get_parse_errors("myVar = ;");
        assert!(errors.iter().any(|e| matches!(
            e,
            ParseError::ExpectedButFound { expected, .. } if expected == "expression"
        )));
    }

    #[test]
    fn test_invalid_condition() {
        let errors = get_parse_errors("if () { 5 }");
        assert!(errors.iter().any(|e| matches!(e, ParseError::ExpectedButFound { expected, .. } if expected == "expression")));
    }

    #[test]
    fn test_while() {
        let errors = get_parse_errors("while 5) { break; }");
        assert!(errors.iter().any(|e| matches!(e, ParseError::ExpectedButFound { expected, .. } if expected == "(")));
    }

    
}