mod error;
mod span;
mod token;

pub use error::LexicalError;
pub use span::Span;
pub use token::{Token, TokenType, TypeName};

pub type Result<T> = std::result::Result<T, LexicalError>;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    start_column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            start_column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn peek_ahead(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.position += 1;
        if let Some(c) = ch {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
        self.start_column = self.column;
    }

    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
        self.start_column = self.column;
    }

    fn skip_block_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '*' {
                if let Some(ch) = self.peek_ahead() {
                    if ch == '/' {
                        break;
                    }
                }
            }
            self.advance();
        }
        self.advance();
        self.advance();
        self.start_column = self.column;
    }

    fn read_string(&mut self) -> Result<Token> {
        let start_span = self.current_location();
        let mut string = String::new();
        self.advance(); // skip opening quote

        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    self.advance(); // skip closing quote
                    return Ok(Token::new(TokenType::StringLiteral(string), start_span));
                }
                '\\' => {
                    self.advance();
                    if let Some(next) = self.peek() {
                        let escaped = match next {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '"' => '"',
                            '\\' => '\\',
                            _ => {
                                return Err(LexicalError::InvalidEscapeSequence {
                                    escape: next,
                                    span: start_span,
                                })
                            }
                        };
                        string.push(escaped);
                        self.advance();
                    } else {
                        return Err(LexicalError::UnexpectedEOF { span: start_span });
                    }
                }
                _ => {
                    string.push(ch);
                    self.advance();
                }
            }
        }
        Err(LexicalError::UnterminatedString { span: start_span })
    }

    fn current_location(&self) -> Span {
        Span::new(self.line, self.start_column)
    }

    fn read_number(&mut self) -> Result<Token> {
        let start_span = self.current_location();
        let mut number = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                number.push(ch);
                self.advance();
            } else if ch.is_alphabetic() {
                return Err(LexicalError::InvalidNumber {
                    value: format!("{}{}", number, ch),
                    span: start_span,
                });
            } else {
                break;
            }
        }

        match number.parse::<i64>() {
            Ok(n) => Ok(Token::new(TokenType::IntegerLiteral(n), start_span)),
            Err(_) => Err(LexicalError::InvalidNumber {
                value: number,
                span: start_span,
            }),
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        identifier
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();
        let start_span = self.current_location();
        let mut current_token = Token::new_with_span(start_span);

        match self.peek() {
            None => Ok(Token::new(TokenType::EOF, start_span)),
            Some(ch) => match ch {
                '+' => {
                    self.advance();
                    current_token.set_type(TokenType::Plus);
                    Ok(current_token)
                }
                '-' => {
                    if self.peek_ahead() == Some('>') {
                        self.advance();
                        self.advance();
                        current_token.set_type(TokenType::Arrow);
                        Ok(current_token)
                    } else {
                        self.advance();
                        current_token.set_type(TokenType::Minus);
                        Ok(current_token)
                    }
                }
                '*' => {
                    self.advance();
                    current_token.set_type(TokenType::Star);
                    Ok(current_token)
                }
                '/' => {
                    if self.peek_ahead() == Some('/') {
                        self.skip_line_comment();
                        return self.next_token();
                    } else if self.peek_ahead() == Some('*') {
                        self.skip_block_comment();
                        return self.next_token();
                    } else {
                        self.advance();
                        current_token.set_type(TokenType::Slash);
                        Ok(current_token)
                    }
                }
                '=' => {
                    if self.peek_ahead() == Some('=') {
                        self.advance();
                        self.advance();
                        current_token.set_type(TokenType::Equal);
                        Ok(current_token)
                    } else {
                        self.advance();
                        current_token.set_type(TokenType::Assign);
                        Ok(current_token)
                    }
                }
                '>' => {
                    if self.peek_ahead() == Some('=') {
                        self.advance();
                        self.advance();
                        current_token.set_type(TokenType::GreaterEqual);
                        Ok(current_token)
                    } else {
                        self.advance();
                        current_token.set_type(TokenType::Greater);
                        Ok(current_token)
                    }
                }
                '<' => {
                    if self.peek_ahead() == Some('=') {
                        self.advance();
                        self.advance();
                        current_token.set_type(TokenType::LessEqual);
                        Ok(current_token)
                    } else {
                        self.advance();
                        current_token.set_type(TokenType::Less);
                        Ok(current_token)
                    }
                }
                '!' => {
                    self.advance();
                    current_token.set_type(TokenType::Not);
                    Ok(current_token)
                }
                '(' => {
                    self.advance();
                    current_token.set_type(TokenType::LeftParen);
                    Ok(current_token)
                }
                ')' => {
                    self.advance();
                    current_token.set_type(TokenType::RightParen);
                    Ok(current_token)
                }
                '{' => {
                    self.advance();
                    current_token.set_type(TokenType::LeftBrace);
                    Ok(current_token)
                }
                '}' => {
                    self.advance();
                    current_token.set_type(TokenType::RightBrace);
                    Ok(current_token)
                }
                ';' => {
                    self.advance();
                    current_token.set_type(TokenType::Semicolon);
                    Ok(current_token)
                }
                ':' => {
                    self.advance();
                    current_token.set_type(TokenType::Colon);
                    Ok(current_token)
                }
                ',' => {
                    self.advance();
                    current_token.set_type(TokenType::Comma);
                    Ok(current_token)
                }
                '.' => {
                    self.advance();
                    current_token.set_type(TokenType::Dot);
                    Ok(current_token)
                }
                '[' => {
                    self.advance();
                    current_token.set_type(TokenType::LeftBracket);
                    Ok(current_token)
                }
                ']' => {
                    self.advance();
                    current_token.set_type(TokenType::RightBracket);
                    Ok(current_token)
                }
                '&' => {
                    if self.peek_ahead() == Some('&') {
                        self.advance();
                        self.advance();
                        current_token.set_type(TokenType::And);
                        Ok(current_token)
                    } else {
                        return Err(LexicalError::InvalidChar {
                            character: ch,
                            span: start_span,
                        });
                    }
                }
                '|' => {
                    if self.peek_ahead() == Some('|') {
                        self.advance();
                        self.advance();
                        current_token.set_type(TokenType::Or);
                        Ok(current_token)
                    } else {
                        return Err(LexicalError::InvalidChar {
                            character: ch,
                            span: start_span,
                        });
                    }
                }
                '"' => self.read_string(),

                // numbers
                ch if ch.is_digit(10) => self.read_number(),

                // identifiers and keywords
                ch if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();
                    match identifier.as_str() {
                        "class" => {
                            current_token.set_type(TokenType::Class);
                            Ok(current_token)
                        }
                        "meth" => {
                            current_token.set_type(TokenType::Meth);
                            Ok(current_token)
                        }
                        "init" => {
                            current_token.set_type(TokenType::Init);
                            Ok(current_token)
                        }
                        "extends" => {
                            current_token.set_type(TokenType::Extends);
                            Ok(current_token)
                        }
                        "this" => {
                            current_token.set_type(TokenType::This);
                            Ok(current_token)
                        }
                        "super" => {
                            current_token.set_type(TokenType::Super);
                            Ok(current_token)
                        }
                        "while" => {
                            current_token.set_type(TokenType::While);
                            Ok(current_token)
                        }
                        "break" => {
                            current_token.set_type(TokenType::Break);
                            Ok(current_token)
                        }
                        "return" => {
                            current_token.set_type(TokenType::Return);
                            Ok(current_token)
                        }
                        "if" => {
                            current_token.set_type(TokenType::If);
                            Ok(current_token)
                        }
                        "else" => {
                            current_token.set_type(TokenType::Else);
                            Ok(current_token)
                        }
                        "new" => {
                            current_token.set_type(TokenType::New);
                            Ok(current_token)
                        }
                        "true" => {
                            current_token.set_type(TokenType::True);
                            Ok(current_token)
                        }
                        "false" => {
                            current_token.set_type(TokenType::False);
                            Ok(current_token)
                        }
                        "print" => {
                            current_token.set_type(TokenType::Print);
                            Ok(current_token)
                        }
                        "println" => {
                            current_token.set_type(TokenType::Println);
                            Ok(current_token)
                        }
                        "const" => {
                            current_token.set_type(TokenType::Const);
                            Ok(current_token)
                        }
                        "Int" => {
                            current_token.set_type(TokenType::Type(TypeName::Int));
                            Ok(current_token)
                        }
                        "Boolean" => {
                            current_token.set_type(TokenType::Type(TypeName::Boolean));
                            Ok(current_token)
                        }
                        "Void" => {
                            current_token.set_type(TokenType::Type(TypeName::Void));
                            Ok(current_token)
                        }
                        "Str" => {
                            current_token.set_type(TokenType::Type(TypeName::Str));
                            Ok(current_token)
                        }
                        "let" => {
                            current_token.set_type(TokenType::Let);
                            Ok(current_token)
                        }
                        "fun" => {
                            current_token.set_type(TokenType::Fun);
                            Ok(current_token)
                        }
                        _ => {
                            current_token.set_type(TokenType::Identifier(identifier));
                            Ok(current_token)
                        }
                    }
                }

                // error if no match
                ch => Err(LexicalError::InvalidChar {
                    character: ch,
                    span: start_span,
                }),
            },
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = token.token_type == TokenType::EOF;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub use error::LexicalError;
    use token::{Token, TokenType, TypeName};

    fn create_token(token_type: TokenType, line: usize, column: usize) -> Token {
        Token::new(token_type, Span::new(line, column))
    }

    #[test]
    fn tokenize_punctuation() {
        let mut lexer = Lexer::new("() {}; , .");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::RightParen);
        assert_eq!(tokens[2].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[3].token_type, TokenType::RightBrace);
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
        assert_eq!(tokens[5].token_type, TokenType::Comma);
        assert_eq!(tokens[6].token_type, TokenType::Dot);
        assert_eq!(tokens[7].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_empty() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_operators() {
        let mut lexer = Lexer::new("+ - * / =");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].token_type, TokenType::Star);
        assert_eq!(tokens[3].token_type, TokenType::Slash);
        assert_eq!(tokens[4].token_type, TokenType::Assign);
        assert_eq!(tokens[5].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_boolean_ops() {
        let mut lexer = Lexer::new("< <= == > >= !");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Less);
        assert_eq!(tokens[1].token_type, TokenType::LessEqual);
        assert_eq!(tokens[2].token_type, TokenType::Equal);
        assert_eq!(tokens[3].token_type, TokenType::Greater);
        assert_eq!(tokens[4].token_type, TokenType::GreaterEqual);
        assert_eq!(tokens[5].token_type, TokenType::Not);
        assert_eq!(tokens[6].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_keywords() {
        let mut lexer = Lexer::new("class meth init extends this super while break return if else new true false println fun let");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Class);
        assert_eq!(tokens[1].token_type, TokenType::Meth);
        assert_eq!(tokens[2].token_type, TokenType::Init);
        assert_eq!(tokens[3].token_type, TokenType::Extends);
        assert_eq!(tokens[4].token_type, TokenType::This);
        assert_eq!(tokens[5].token_type, TokenType::Super);
        assert_eq!(tokens[6].token_type, TokenType::While);
        assert_eq!(tokens[7].token_type, TokenType::Break);
        assert_eq!(tokens[8].token_type, TokenType::Return);
        assert_eq!(tokens[9].token_type, TokenType::If);
        assert_eq!(tokens[10].token_type, TokenType::Else);
        assert_eq!(tokens[11].token_type, TokenType::New);
        assert_eq!(tokens[12].token_type, TokenType::True);
        assert_eq!(tokens[13].token_type, TokenType::False);
        assert_eq!(tokens[14].token_type, TokenType::Println);
        assert_eq!(tokens[15].token_type, TokenType::Fun);
        assert_eq!(tokens[16].token_type, TokenType::Let);
        assert_eq!(tokens[17].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_types() {
        let mut lexer = Lexer::new("Int Boolean Void");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Type(TypeName::Int));
        assert_eq!(tokens[1].token_type, TokenType::Type(TypeName::Boolean));
        assert_eq!(tokens[2].token_type, TokenType::Type(TypeName::Void));
        assert_eq!(tokens[3].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_pos_integers() {
        let mut lexer = Lexer::new("1 10 100");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::IntegerLiteral(1));
        assert_eq!(tokens[1].token_type, TokenType::IntegerLiteral(10));
        assert_eq!(tokens[2].token_type, TokenType::IntegerLiteral(100));
        assert_eq!(tokens[3].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_neg_integers() {
        let mut lexer = Lexer::new("-1 -10 -100");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Minus);
        assert_eq!(tokens[1].token_type, TokenType::IntegerLiteral(1));
        assert_eq!(tokens[2].token_type, TokenType::Minus);
        assert_eq!(tokens[3].token_type, TokenType::IntegerLiteral(10));
        assert_eq!(tokens[4].token_type, TokenType::Minus);
        assert_eq!(tokens[5].token_type, TokenType::IntegerLiteral(100));
        assert_eq!(tokens[6].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_return_types() {
        let mut lexer = Lexer::new("fun greet(name: Str) -> Void");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Fun);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("greet".to_string())
        );
        assert_eq!(tokens[2].token_type, TokenType::LeftParen);
        assert_eq!(
            tokens[3].token_type,
            TokenType::Identifier("name".to_string())
        );
        assert_eq!(tokens[4].token_type, TokenType::Colon);
        assert_eq!(tokens[5].token_type, TokenType::Type(TypeName::Str));
        assert_eq!(tokens[6].token_type, TokenType::RightParen);
        assert_eq!(tokens[7].token_type, TokenType::Arrow);
        assert_eq!(tokens[8].token_type, TokenType::Type(TypeName::Void));
        assert_eq!(tokens[9].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_whitespace() {
        let mut lexer = Lexer::new("let    value: Int =    \n      123");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier(String::from("value"))
        );
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::Type(TypeName::Int));
        assert_eq!(tokens[4].token_type, TokenType::Assign);
        assert_eq!(tokens[5].token_type, TokenType::IntegerLiteral(123));
        assert_eq!(tokens[6].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_line_comments() {
        let mut lexer = Lexer::new(
            "Int value = 123; //This line creates an integer variable with value 123 \n
                                            2 + 2",
        );
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Type(TypeName::Int));
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier(String::from("value"))
        );
        assert_eq!(tokens[2].token_type, TokenType::Assign);
        assert_eq!(tokens[3].token_type, TokenType::IntegerLiteral(123));
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
        assert_eq!(tokens[5].token_type, TokenType::IntegerLiteral(2));
        assert_eq!(tokens[6].token_type, TokenType::Plus);
        assert_eq!(tokens[7].token_type, TokenType::IntegerLiteral(2));
        assert_eq!(tokens[8].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_block_comments() {
        let mut lexer = Lexer::new("let value: Int = 123; /*This \nline \ncreates \nan \ninteger \nvariable \nwith \nvalue \n123*/ \n
                                            2 + 2");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier(String::from("value"))
        );
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::Type(TypeName::Int));
        assert_eq!(tokens[4].token_type, TokenType::Assign);
        assert_eq!(tokens[5].token_type, TokenType::IntegerLiteral(123));
        assert_eq!(tokens[6].token_type, TokenType::Semicolon);
        assert_eq!(tokens[7].token_type, TokenType::IntegerLiteral(2));
        assert_eq!(tokens[8].token_type, TokenType::Plus);
        assert_eq!(tokens[9].token_type, TokenType::IntegerLiteral(2));
        assert_eq!(tokens[10].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_string() {
        let mut lexer = Lexer::new("\"hello\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::StringLiteral("hello".to_string())
        );
        assert_eq!(tokens[1].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_string_newline() {
        let mut lexer = Lexer::new("\"hello \\nworld\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::StringLiteral("hello \nworld".to_string())
        );
        assert_eq!(tokens[1].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_string_tab() {
        let mut lexer = Lexer::new("\"hello \\tworld\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::StringLiteral("hello \tworld".to_string())
        );
        assert_eq!(tokens[1].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_string_return() {
        let mut lexer = Lexer::new("\"hello \\rworld\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::StringLiteral("hello \rworld".to_string())
        );
        assert_eq!(tokens[1].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_string_quotes() {
        let mut lexer = Lexer::new("\"hello \\\"world\\\"\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::StringLiteral("hello \"world\"".to_string())
        );
        assert_eq!(tokens[1].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_string_slash() {
        let mut lexer = Lexer::new("\"hello \\\\ world\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::StringLiteral("hello \\ world".to_string())
        );
        assert_eq!(tokens[1].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_unterminated_string() {
        let mut lexer = Lexer::new("String \"Hello World");
        let result = lexer.tokenize();
        assert!(matches!(
            result,
            Err(LexicalError::UnterminatedString { span })
            if span == Span::new(1, 8)
        ));
    }

    #[test]
    fn tokenize_invalid_number() {
        let mut lexer = Lexer::new("13 * 2 \nlet i: Int = 123a");
        let result = lexer.tokenize();
        assert!(matches!(
            result,
            Err(LexicalError::InvalidNumber { value, span })
            if value == "123a" && span == Span::new(2, 14)
        ));
    }

    #[test]
    fn tokenize_invalid_character() {
        let mut lexer = Lexer::new("13 * 2 \nlet $ = 123");
        let result = lexer.tokenize();
        assert!(matches!(
            result,
            Err(LexicalError::InvalidChar { character, span })
            if character == '$' && span == Span::new(2, 5)
        ));
    }

    #[test]
    fn tokenize_invalid_escape() {
        let mut lexer = Lexer::new("string \"Hello \\world\"");
        let result = lexer.tokenize();
        assert!(matches!(
            result,
            Err(LexicalError::InvalidEscapeSequence { escape, span })
            if escape == 'w' && span == Span::new(1, 8)
        ));
    }

    #[test]
    fn tokenize_unexpected_eof() {
        let mut lexer = Lexer::new("string \"Hello \\");
        let result = lexer.tokenize();
        assert!(matches!(
            result,
            Err(LexicalError::UnexpectedEOF { span })
            if span == Span::new(1, 8)
        ));
    }

    #[test]
    fn test_integers() {
        let src = "let myNumber: Int = 123;";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("myNumber".to_string())
        );
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::Type(TypeName::Int));
        assert_eq!(tokens[4].token_type, TokenType::Assign);
        assert_eq!(tokens[5].token_type, TokenType::IntegerLiteral(123));
        assert_eq!(tokens[6].token_type, TokenType::Semicolon);
        assert_eq!(tokens[7].token_type, TokenType::EOF);
    }

    #[test]
    fn test_classes() {
        let src = "class Animal { init() {} meth speak() -> Void { return println(0); } }";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Class);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("Animal".to_string())
        );
        assert_eq!(tokens[2].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[3].token_type, TokenType::Init);
        assert_eq!(tokens[4].token_type, TokenType::LeftParen);
        assert_eq!(tokens[5].token_type, TokenType::RightParen);
        assert_eq!(tokens[6].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[7].token_type, TokenType::RightBrace);
        assert_eq!(tokens[8].token_type, TokenType::Meth);
        assert_eq!(
            tokens[9].token_type,
            TokenType::Identifier("speak".to_string())
        );
        assert_eq!(tokens[10].token_type, TokenType::LeftParen);
        assert_eq!(tokens[11].token_type, TokenType::RightParen);
        assert_eq!(tokens[12].token_type, TokenType::Arrow);
        assert_eq!(tokens[13].token_type, TokenType::Type(TypeName::Void));
        assert_eq!(tokens[14].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[15].token_type, TokenType::Return);
        assert_eq!(tokens[16].token_type, TokenType::Println);
        assert_eq!(tokens[17].token_type, TokenType::LeftParen);
        assert_eq!(tokens[18].token_type, TokenType::IntegerLiteral(0));
        assert_eq!(tokens[19].token_type, TokenType::RightParen);
        assert_eq!(tokens[20].token_type, TokenType::Semicolon);
        assert_eq!(tokens[21].token_type, TokenType::RightBrace);
        assert_eq!(tokens[22].token_type, TokenType::RightBrace);
        assert_eq!(tokens[23].token_type, TokenType::EOF);
    }

    #[test]
    fn test_inheritance() {
        let src = "class Dog extends Animal { init() { super(); } }";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 16);
        assert_eq!(tokens[0].token_type, TokenType::Class);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("Dog".to_string())
        );
        assert_eq!(tokens[2].token_type, TokenType::Extends);
        assert_eq!(
            tokens[3].token_type,
            TokenType::Identifier("Animal".to_string())
        );
        assert_eq!(tokens[4].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[5].token_type, TokenType::Init);
        assert_eq!(tokens[6].token_type, TokenType::LeftParen);
        assert_eq!(tokens[7].token_type, TokenType::RightParen);
        assert_eq!(tokens[8].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[9].token_type, TokenType::Super);
        assert_eq!(tokens[10].token_type, TokenType::LeftParen);
        assert_eq!(tokens[11].token_type, TokenType::RightParen);
        assert_eq!(tokens[12].token_type, TokenType::Semicolon);
        assert_eq!(tokens[13].token_type, TokenType::RightBrace);
        assert_eq!(tokens[14].token_type, TokenType::RightBrace);
        assert_eq!(tokens[15].token_type, TokenType::EOF);
    }

    #[test]
    fn test_strings() {
        let src = r#"console.log("Hello, World!");"#;
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 8);
        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("console".to_string())
        );
        assert_eq!(tokens[1].token_type, TokenType::Dot);
        assert_eq!(
            tokens[2].token_type,
            TokenType::Identifier("log".to_string())
        );
        assert_eq!(tokens[3].token_type, TokenType::LeftParen);
        assert_eq!(
            tokens[4].token_type,
            TokenType::StringLiteral("Hello, World!".to_string())
        );
        assert_eq!(tokens[5].token_type, TokenType::RightParen);
        assert_eq!(tokens[6].token_type, TokenType::Semicolon);
        assert_eq!(tokens[7].token_type, TokenType::EOF);
    }

    #[test]
    fn test_const_array() {
        let src = "const num = [1, 2];";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Const);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("num".to_string())
        );
        assert_eq!(tokens[2].token_type, TokenType::Assign);
        assert_eq!(tokens[3].token_type, TokenType::LeftBracket);
        assert_eq!(tokens[4].token_type, TokenType::IntegerLiteral(1));
        assert_eq!(tokens[5].token_type, TokenType::Comma);
        assert_eq!(tokens[6].token_type, TokenType::IntegerLiteral(2));
        assert_eq!(tokens[7].token_type, TokenType::RightBracket);
        assert_eq!(tokens[8].token_type, TokenType::Semicolon);
        assert_eq!(tokens[9].token_type, TokenType::EOF);
    }
}
