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

mod tests {
    use crate::{ast::*, lexer::*, parser::*};

    use super::ParserDecl;

    fn parse_class(input: &str) -> Option<ClassDef> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_class(Span{line:0,column:0})
    }

    fn get_class_errors(input: &str) -> Vec<ParseError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_class(Span{line:0,column:0});
        parser.get_errors().to_vec()
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
    fn test_class_decl_with_1_method() {
        let class = parse_class("class Animal { init() {} meth speak() -> Void { return println(\"animal noise\"); }}").unwrap();
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
    fn test_class_decl_with_2_methods() {
        let class = parse_class("class Animal { init() {} 
        meth speak() -> Void { return println(\"animal noise\"); }
        meth age() -> Int {return 0;}}").unwrap();
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
                && methods.len() == 2
                && matches!(&constructor, Constructor {params, ..} if params.len() == 0)

        ))
    }

    #[test]
    fn test_class_missing_init() {
        let errors = get_class_errors("class Animal {}");
        assert!(errors.iter().any(|e| matches!(e, ParseError::MissingClassInit { symbol, .. }
        if symbol == "Animal")))
    }

    #[test]
    fn test_class_unnamed_method() {
        let errors = get_class_errors("class Zero { init() {}
        meth -> Int {return 0;}");
        assert!(errors.iter().any(|e| matches!(e, ParseError::ExpectedMethName { symbol, span }
            if symbol == "Zero" && *span == Span{line:2, column:14})))
    }

    fn parse_method(input: &str) -> Option<MethDef> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_method("Dummy",Span{line:0,column:0})
    }

    fn get_method_errors(input: &str) -> Vec<ParseError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_method("Dummy",Span{line:0,column:0});
        parser.get_errors().to_vec()
    }

    #[test]
    fn test_minimal_method_decl() {
        let method = parse_method("methodName() -> Void {}").unwrap();
        assert!(matches!(
            method,
            MethDef {
                name,
                params,
                return_type,
                statements
            }
            if name == "methodName"
                && params.len() == 0
                && return_type == TypeName::Void
                &&statements.len() == 0
        ))
    }

    #[test]
    fn test_minimal_method_1_param() {
        let method = parse_method("methodName(intParam: Int) -> Void {}").unwrap();
        assert!(matches!(
            method,
            MethDef {
                name,
                params,
                return_type,
                statements
            }
            if name == "methodName"
                && params[0].name == "intParam"
                && params[0].param_type == TypeName::Int
                && return_type == TypeName::Void
                &&statements.len() == 0
        ))
    }

    #[test]
    fn test_minimal_method_2_param() {
        let method = parse_method("methodName(intParam: Int, stringParam: Str) -> Void {}").unwrap();
        assert!(matches!(
            method,
            MethDef {
                name,
                params,
                return_type,
                statements
            }
            if name == "methodName"
                && params[0].name == "intParam"
                && params[0].param_type == TypeName::Int
                && params[1].name == "stringParam"
                && params[1].param_type == TypeName::Str
                && return_type == TypeName::Void
                && statements.len() == 0
        ))
    }

    #[test]
    fn test_minimal_method_3_param() {
        let method = parse_method("methodName(intParam: Int, stringParam: Str, boolParam: Boolean) -> Void {}").unwrap();
        assert!(matches!(
            method,
            MethDef {
                name,
                params,
                return_type,
                statements
            }
            if name == "methodName"
                && params[0].name == "intParam"
                && params[0].param_type == TypeName::Int
                && params[1].name == "stringParam"
                && params[1].param_type == TypeName::Str
                && params[2].name == "boolParam"
                && params[2].param_type == TypeName::Boolean
                && return_type == TypeName::Void
                && statements.len() == 0
        ))
    }

    #[test]
    fn test_method_with_body() {
        let method = parse_method("methodName() -> Void {let myNum: Int = 5;}").unwrap();
        assert!(matches!(
            method,
            MethDef {
                name,
                params,
                return_type,
                statements
            }
            if name == "methodName"
                && params.len() == 0
                && return_type == TypeName::Void
                && statements.len() == 1
        ))
    }

    #[test]
    fn test_full_featured_method() {
        let method = parse_method("methodName(myNum: Int) -> Int {let square: Int = myNum * myNum; return myNum * myNum;}").unwrap();
        assert!(matches!(
            method,
            MethDef {
                name,
                params,
                return_type,
                statements
            }
            if name == "methodName"
                && params.len() == 1
                && return_type == TypeName::Int
                && statements.len() > 0
        ))
    }

    #[test]
    fn test_unnamed_method() {
        let errors = get_method_errors("() -> Void {let myNum: Int = 5;}");
        assert!(errors.iter().any(|e| matches!(
            e, ParseError::ExpectedMethName { .. }
        )))
    }
    
    #[test]
    fn test_method_missing_rparen() {
        let errors = get_method_errors("broken ( -> Void {}");
        assert!(errors.iter().any(|e| matches!(
            e, ParseError::UnexpectedToken { .. }
        )))
    }

    #[test]
    fn test_method_missing_lparen() {
        let errors = get_method_errors("broken ) -> Void {}");
        assert!(errors.iter().any(|e| matches!(
            e, ParseError::ExpectedButFound { expected, found, .. }
            if expected == "(" && found == ")"
        )))
    }

    #[test]
    fn test_method_unexpected_eof_params() {
        let errors = get_method_errors("broken (");
        assert!(errors.iter().any(|e| matches!(
            e, ParseError::UnexpectedEOF { .. }
        )))
    }

    #[test]
    fn test_method_unexpected_eof_type() {
        let errors = get_method_errors("broken ()");
        assert!(errors.iter().any(|e| matches!(
            e, ParseError::UnexpectedEOF { .. }
        )))
    }

    #[test]
    fn test_method_unexpected_eof_body() {
        let errors = get_method_errors("broken () -> Void {let myNum:Int;");
        assert!(errors.iter().any(|e| matches!(
            e, ParseError::UnexpectedEOF { .. }
        )))
    }


    #[test]
    fn test_method_missing_type() {
        let errors = get_method_errors("broken () -> {}");
        assert!(errors.iter().any(|e| matches!(
            e, ParseError::ExpectedReturnType { .. }
        )))
    }

    #[test]
    fn test_method_missing_arrow() {
        let errors = get_method_errors("broken () Void {}");
        assert!(errors.iter().any(|e| matches!(
            e, ParseError::ExpectedButFound { expected, found, .. }
            if expected == "->" && found == "Void"
        )))
    }

    fn parse_function(input: &str) -> Option<FunDef> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_function(Span{line:0,column:0})
    }

    #[test]
    fn test_minimal_function() {
        let function = parse_function("functionName() -> Void {}").unwrap();
        assert!(matches!(
            function,
            FunDef {
                name,
                params,
                return_type,
                statements
            }
            if name == "functionName"
                && params.len() == 0
                && return_type == TypeName::Void
                &&statements.len() == 0
        ))
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
