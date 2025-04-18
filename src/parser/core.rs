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

    pub fn peek_ahead_amount(&mut self, amount: usize) -> Option<Token> {
        if self.position + amount >= self.tokens.len() {
            return None;
        }
        match self.tokens.get(self.position + amount) {
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

mod tests {
    use crate::{ast::*, lexer::*};
    use super::Parser;

    fn parse(input: &str) -> Option<Entry> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    fn parser_has_errors(input: &str) -> bool {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        parser.has_errors()
    }


    #[test]
    fn test_parse_empty() {
        let entry = parse("").unwrap();
        assert!(matches!(
            entry,
            Entry {
                statements,
                class_defs,
                fun_defs
            }
            if statements.len() == 0
            && class_defs.len() == 0
            && fun_defs.len() == 0
        ))
    }

    #[test]
    fn test_parse_no_errors_empty() {
        let has_error = parser_has_errors("");
        assert!(has_error == false)
    }

    #[test]
    fn test_parse_only_statements() {
        let entry = parse("let i: Int = 1; let myStr: Str;").unwrap();
        assert!(matches!(
            entry,
            Entry {
                statements,
                class_defs,
                fun_defs
            }
            if statements.len() == 2
            && class_defs.len() == 0
            && fun_defs.len() == 0
        ))
    }

    #[test]
    fn test_parse_statement_no_errors() {
        let has_error = parser_has_errors("let i: Int = 1; let myStr: Str;");
        assert!(has_error == false)
    }

    #[test]
    fn test_parse_only_classes() {
        let entry = parse("class ClassOne {init() {}} class ClassTwo {init() {}}").unwrap();
        assert!(matches!(
            entry,
            Entry {
                statements,
                class_defs,
                fun_defs
            }
            if statements.len() == 0
            && class_defs.len() == 2
            && fun_defs.len() == 0
        ))
    }

    #[test]
    fn test_parse_classes_no_errors() {
        let has_errors = parser_has_errors("class ClassOne {init() {}} class ClassTwo {init() {}}");
        assert!(has_errors == false);
    }

    #[test]
    fn test_parse_only_functions() {
        let entry = parse("fun returnOne() -> Int {return 1;} 
                            fun returnHello() -> Str {return \"Hello\";}").unwrap();
        assert!(matches!(
            entry,
            Entry {
                statements,
                class_defs,
                fun_defs
            }
            if statements.len() == 0
            && class_defs.len() == 0
            && fun_defs.len() == 2
        ))
    }

    #[test]
    fn test_parse_functions_no_errors() {
        let has_errors = parser_has_errors("fun returnOne() -> Int {return 1;} 
                                            fun returnHello() -> Str {return \"Hello\";}");
        assert!(has_errors == false);
    }

    #[test]
    fn test_parse_methods_on_class() {
        let entry = parse("class Operations{init(){}
                meth add(x: Int, y: Int) -> Int {
                    return x + y;
                }
            }
            let adder: Operations = new Operations();
            let two: Int = adder.add(1, 1);").unwrap();
        assert!(matches!(
            entry,
            Entry { 
                statements,
                class_defs,
                fun_defs
            }
            if statements.len() == 2
            && class_defs.len() == 1
            && fun_defs.len() == 0
        ))
    }

    #[test]
    fn test_parse_methods_on_class_no_errors() {
        let has_errors = parser_has_errors("class Operations{init(){}
                meth add(x: Int, y: Int) -> Int {
                    return x + y;
                }
            }
            let adder: Operations = new Operations();
            let two: Int = adder.add(1, 1);");
        assert!(has_errors == false)
    }
}