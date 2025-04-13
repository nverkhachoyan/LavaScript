use super::*;
use crate::ast::{ClassDef, Constructor, Expr, FunDef, MethDef, ParamDecl};
use crate::lexer::{Span, TokenType};

pub trait ParserDecl {
    fn parse_class(&mut self) -> Option<ClassDef>;
    fn parse_constructor(&mut self, class_nam: &str) -> Option<Constructor>;
    fn parse_method(&mut self) -> Option<MethDef>;
    fn parse_comma_param_decl(&mut self, parent_name: &str) -> Option<Vec<ParamDecl>>;
    fn parse_param(&mut self, parent_name: &str, parent_span: Span) -> Option<ParamDecl>;
    fn parse_fun(&mut self) -> Option<FunDef>;
}

impl ParserDecl for Parser {
    fn parse_class(&mut self) -> Option<ClassDef> {
        let mut class = ClassDef::default();
        self.consume(TokenType::Class)?;

        class.name = self.consume_identifier("class name")?;

        if self.consume_optional(TokenType::Extends).is_some() {
            class.extends = self.consume_identifier("parent class name");
        }

        self.consume(TokenType::LeftBrace)?;

        if let Some(constructor) = self.parse_constructor(&class.name) {
            class.constructor = constructor;
        }

        while self
            .peek()
            .map_or(false, |token| token.token_type == TokenType::Meth)
        {
            let span = self.current_span()?;
            match self.parse_method() {
                Some(meth) => class.methods.push(meth),
                None => {
                    self.errors.push(ParseError::ExpectedMethName {
                        symbol: class.name.clone(),
                        span,
                    });
                    self.synchronize(SyncPoint::ClassBody);
                    return None;
                }
            }
        }

        self.consume(TokenType::RightBrace)?;
        Some(class)
    }

    fn parse_constructor(&mut self, class_name: &str) -> Option<Constructor> {
        let mut constructor = Constructor::default();
        self.consume_optional(TokenType::Init)?;

        if let Some(params) = self.parse_comma_param_decl(class_name) {
            constructor.params = params;
        }

        self.consume(TokenType::LeftBrace)?;

        if let Some(_) = self.consume_optional(TokenType::Super) {
            if let Some((_, _)) =
                self.consume_two_optionals(TokenType::LeftParen, TokenType::RightParen)
            {
                constructor.super_call = Some(vec![Expr::Empty]);
                self.consume(TokenType::Semicolon)?;
            } else {
                let super_expressions = self.parse_comma_expr();
                constructor.super_call = Some(super_expressions);
                self.consume(TokenType::Semicolon)?;
            }
        }

        if let Some(stmt) = self.parse_stmt() {
            constructor.statements = Some(stmt);
        }

        self.consume(TokenType::RightBrace)?;

        Some(constructor)
    }

    fn parse_method(&mut self) -> Option<MethDef> {
        let mut method = MethDef::default();

        self.consume(TokenType::Meth);

        if let Some(ident) = self.consume_identifier("method name") {
            method.name = ident;
        }

        let params = self.parse_comma_param_decl(&method.name)?;
        method.params = params;

        self.consume(TokenType::Arrow)?;
        if let Some(return_type) = self.consume_type() {
            method.return_type = return_type;
        }

        method.statements = self.parse_stmt();

        Some(method)
    }

    fn parse_comma_param_decl(&mut self, parent_name: &str) -> Option<Vec<ParamDecl>> {
        let mut params: Vec<ParamDecl> = vec![];

        if let Some(_) = self.consume_two_optionals(TokenType::LeftParen, TokenType::RightParen) {
            return Some(params);
        }

        self.consume(TokenType::LeftParen)?;

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

        Some(params)
    }

    fn parse_param(&mut self, _parent_name: &str, _parent_span: Span) -> Option<ParamDecl> {
        let mut current_param = ParamDecl::default();
        let param_name = self.consume_identifier("Expected parameter name")?;
        current_param.name = param_name;
        self.consume(TokenType::Colon)?;
        let param_type = self.consume_type()?;
        current_param.param_type = param_type;

        Some(current_param)
    }

    fn parse_fun(&mut self) -> Option<FunDef> {
        let mut fun = FunDef::default();

        self.consume(TokenType::Fun);

        if let Some(ident) = self.consume_identifier("function name") {
            fun.name = ident;
        }

        if let Some(params) = self.parse_comma_param_decl(&fun.name) {
            fun.params = params;
        }

        self.consume(TokenType::Arrow)?;
        if let Some(return_type) = self.consume_type() {
            fun.return_type = return_type;
        }

        fun.statements = self.parse_stmt();

        Some(fun)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;

    #[test]
    fn test_parse_constructor() {
        let source = String::from(
            r#"
            class Gugushik extends Animal {
                init(x: Int, y: Str) { 
                    super(); let myNum: Int = 5; 
                } 
            
                meth speak(z: Int) -> Void { 
                    return z; 
                } 

                meth walk(z: Int) -> Void { 
                    return z; 
                } 
            }
        "#,
        );
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let class = parser.parse_class();
        if let Some(class) = class {
            println!("PARSED CONSTRUCTOR TEST: {:#?}", class)
        } else {
            println!("FAILURE");
            if parser.has_errors() {
                println!(
                    "ERRORS FROM CONSTRUCTOR TEST: {:#?}",
                    parser.print_errors(&source)
                );
            }
        }
    }

    #[test]
    fn test_fun_decl() {
        let source = String::from(
            r#"
            fun fibonacci(n: Int) -> Int {
            if (n <= 0) {
                return 0;
            }
            if (n == 1) {
                return 1;
            }
            
            let a: Int = 0;
            let b: Int = 1;
            let result: Int = 0;
            
            let i: Int = 2;
            while (i <= n) {
                result = a + b;
                a = b;
                b = result;
                i = i + 1;
            }
            
            return result;
            }
        "#,
        );
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let fun = parser.parse_fun();
        if let Some(fun) = fun {
            println!("PARSED FUN TEST: {:#?}", fun)
        } else {
            println!("FAILURE");
            if parser.has_errors() {
                println!("ERRORS FROM FUN TEST: {:#?}", parser.print_errors(&source));
            }
        }
    }
}
