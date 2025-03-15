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
                // single char tokens
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
                    current_token.set_type(TokenType::Negate);
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     pub use error::LexicalError;
//     use token::Token;

//     //testing proper tokenization
//     #[test]
//     fn tokenize_punctuation() {
//         let mut lexer = Lexer::new("() {}; , .");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::LeftParen,
//             Token::RightParen,
//             Token::LeftBrace,
//             Token::RightBrace,
//             Token::Semicolon,
//             Token::Comma,
//             Token::Dot,
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_empty() {
//         let mut lexer = Lexer::new("");
//         let expected: Result<Vec<Token>> = Ok(vec![Token::EOF]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_operators() {
//         let mut lexer = Lexer::new("+ - * / =");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::Plus,
//             Token::Minus,
//             Token::Star,
//             Token::Slash,
//             Token::Assign,
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_boolean_ops() {
//         let mut lexer = Lexer::new("< <= == > >= !");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::Less,
//             Token::LessEqual,
//             Token::Equal,
//             Token::Greater,
//             Token::GreaterEqual,
//             Token::Negate,
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_keywords() {
//         let mut lexer = Lexer::new("class meth init extends this super while break return if else new true false println fun let");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::Class,
//             Token::Method,
//             Token::Init,
//             Token::Extends,
//             Token::This,
//             Token::Super,
//             Token::While,
//             Token::Break,
//             Token::Return,
//             Token::If,
//             Token::Else,
//             Token::New,
//             Token::True,
//             Token::False,
//             Token::Println,
//             Token::Fun,
//             Token::Let,
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_types() {
//         let mut lexer = Lexer::new("Int Boolean Void");
//         let expected: Result<Vec<Token>> =
//             Ok(vec![Token::Int, Token::Boolean, Token::Void, Token::EOF]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_pos_integers() {
//         let mut lexer = Lexer::new("1 10 100");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::IntegerLiteral(1),
//             Token::IntegerLiteral(10),
//             Token::IntegerLiteral(100),
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_neg_integers() {
//         let mut lexer = Lexer::new("-1 -10 -100");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::Minus,
//             Token::IntegerLiteral(1),
//             Token::Minus,
//             Token::IntegerLiteral(10),
//             Token::Minus,
//             Token::IntegerLiteral(100),
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_return_types() {
//         let mut lexer = Lexer::new("fun greet(name: Str) -> Void");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::Fun,
//             Token::Identifier("greet".to_string()),
//             Token::LeftParen,
//             Token::Identifier("name".to_string()),
//             Token::Colon,
//             Token::String,
//             Token::RightParen,
//             Token::Arrow,
//             Token::Void,
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap())
//     }

//     #[test]
//     fn tokenize_whitespace() {
//         let mut lexer = Lexer::new("let    value: Int =    \n      123");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::Let,
//             Token::Identifier(String::from("value")),
//             Token::Colon,
//             Token::Int,
//             Token::Assign,
//             Token::IntegerLiteral(123),
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_line_comments() {
//         let mut lexer = Lexer::new(
//             "Int value = 123; //This line creates an integer variable with value 123 \n
//                                             2 + 2",
//         );
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::Int,
//             Token::Identifier(String::from("value")),
//             Token::Assign,
//             Token::IntegerLiteral(123),
//             Token::Semicolon,
//             Token::IntegerLiteral(2),
//             Token::Plus,
//             Token::IntegerLiteral(2),
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_block_comments() {
//         let mut lexer = Lexer::new("let value: Int = 123; /*This \nline \ncreates \nan \ninteger \nvariable \nwith \nvalue \n123*/ \n
//                                             2 + 2");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::Let,
//             Token::Identifier(String::from("value")),
//             Token::Colon,
//             Token::Int,
//             Token::Assign,
//             Token::IntegerLiteral(123),
//             Token::Semicolon,
//             Token::IntegerLiteral(2),
//             Token::Plus,
//             Token::IntegerLiteral(2),
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_string() {
//         let mut lexer = Lexer::new("\"hello\"");
//         let expected: Result<Vec<Token>> =
//             Ok(vec![Token::StringLiteral("hello".to_string()), Token::EOF]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_string_newline() {
//         let mut lexer = Lexer::new("\"hello \\nworld\"");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::StringLiteral("hello \nworld".to_string()),
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_string_tab() {
//         let mut lexer = Lexer::new("\"hello \\tworld\"");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::StringLiteral("hello \tworld".to_string()),
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_string_return() {
//         let mut lexer = Lexer::new("\"hello \\rworld\"");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::StringLiteral("hello \rworld".to_string()),
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_string_quotes() {
//         let mut lexer = Lexer::new("\"hello \\\"world\\\"\"");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::StringLiteral("hello \"world\"".to_string()),
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     #[test]
//     fn tokenize_string_slash() {
//         let mut lexer = Lexer::new("\"hello \\\\ world\"");
//         let expected: Result<Vec<Token>> = Ok(vec![
//             Token::StringLiteral("hello \\ world".to_string()),
//             Token::EOF,
//         ]);
//         assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
//     }

//     //testing errors
//     #[test]
//     fn tokenize_unterminated_string() {
//         let mut lexer = Lexer::new("String \"Hello World");
//         let expected = LexicalError::UnterminatedString {
//             location: (SourceLocation {
//                 line: (1),
//                 column: (8),
//             }),
//         };
//         assert_eq!(lexer.tokenize().unwrap_err(), expected);
//     }

//     #[test]
//     fn tokenize_invalid_number() {
//         let mut lexer = Lexer::new("13 * 2 \nlet i: Int = 123a");
//         let expected = LexicalError::InvalidNumber {
//             value: "123a".to_string(),
//             location: (SourceLocation {
//                 line: (2),
//                 column: (14),
//             }),
//         };
//         assert_eq!(lexer.tokenize().unwrap_err(), expected);
//     }

//     #[test]
//     fn tokenize_invalid_character() {
//         let mut lexer = Lexer::new("13 * 2 \nlet $ = 123");
//         let expected = LexicalError::InvalidChar {
//             character: ('$'),
//             location: (SourceLocation {
//                 line: (2),
//                 column: (5),
//             }),
//         };
//         assert_eq!(lexer.tokenize().unwrap_err(), expected);
//     }

//     #[test]
//     fn tokenize_invalid_escape() {
//         let mut lexer = Lexer::new("string \"Hello \\world\"");
//         let expected = LexicalError::InvalidEscapeSequence {
//             escape: ('w'),
//             location: (SourceLocation {
//                 line: (1),
//                 column: (8),
//             }),
//         };
//         assert_eq!(lexer.tokenize().unwrap_err(), expected);
//     }

//     #[test]
//     fn tokenize_unexpected_eof() {
//         let mut lexer = Lexer::new("string \"Hello \\");
//         let expected = LexicalError::UnexpectedEOF {
//             location: (SourceLocation { line: 1, column: 8 }),
//         };
//         assert_eq!(lexer.tokenize().unwrap_err(), expected);
//     }

//     #[test]
//     fn test_integers() {
//         let src = "let myNumber: Int = 123;";
//         let mut lexer = Lexer::new(src);
//         let mut tokens: Vec<Token> = Vec::new();

//         loop {
//             let token = lexer.next_token().unwrap();
//             tokens.push(token.clone());
//             let is_eof = token == Token::EOF;
//             if is_eof {
//                 break;
//             }
//         }

//         print!("{:?}", tokens);

//         assert_eq!(tokens[0], Token::Let);
//         assert_eq!(tokens[1], Token::Identifier("myNumber".to_string()));
//         assert_eq!(tokens[2], Token::Colon);
//         assert_eq!(tokens[3], Token::Int);
//         assert_eq!(tokens[4], Token::Assign);
//         assert_eq!(tokens[5], Token::IntegerLiteral(123));
//         assert_eq!(tokens[6], Token::Semicolon);
//         assert_eq!(tokens[7], Token::EOF);
//     }

//     #[test]
//     fn test_classes() {
//         let src = "class Animal { init() {} meth speak() -> Void { return println(0); } }";
//         let mut lexer = Lexer::new(src);
//         let mut tokens = Vec::new();

//         loop {
//             let token = lexer.next_token().unwrap();
//             tokens.push(token.clone());
//             let is_eof = token == Token::EOF;
//             if is_eof {
//                 break;
//             }
//         }
//         assert_eq!(tokens[0], Token::Class);
//         assert_eq!(tokens[1], Token::Identifier("Animal".to_string()));
//         assert_eq!(tokens[2], Token::LeftBrace);
//         assert_eq!(tokens[3], Token::Init);
//         assert_eq!(tokens[4], Token::LeftParen);
//         assert_eq!(tokens[5], Token::RightParen);
//         assert_eq!(tokens[6], Token::LeftBrace);
//         assert_eq!(tokens[7], Token::RightBrace);
//         assert_eq!(tokens[8], Token::Method);
//         assert_eq!(tokens[9], Token::Identifier("speak".to_string()));
//         assert_eq!(tokens[10], Token::LeftParen);
//         assert_eq!(tokens[11], Token::RightParen);
//         assert_eq!(tokens[12], Token::Arrow);
//         assert_eq!(tokens[13], Token::Void);
//         assert_eq!(tokens[14], Token::LeftBrace);
//         assert_eq!(tokens[15], Token::Return);
//         assert_eq!(tokens[16], Token::Println);
//         assert_eq!(tokens[17], Token::LeftParen);
//         assert_eq!(tokens[18], Token::IntegerLiteral(0));
//         assert_eq!(tokens[19], Token::RightParen);
//         assert_eq!(tokens[20], Token::Semicolon);
//         assert_eq!(tokens[21], Token::RightBrace);
//         assert_eq!(tokens[22], Token::RightBrace);
//         assert_eq!(tokens[23], Token::EOF);
//     }

//     #[test]
//     fn test_inheritance() {
//         let src = "class Dog extends Animal { init() { super(); } }";
//         let mut lexer = Lexer::new(src);
//         let mut tokens = Vec::new();

//         loop {
//             let token = lexer.next_token().unwrap();
//             tokens.push(token.clone());
//             let is_eof = token == Token::EOF;
//             if is_eof {
//                 break;
//             }
//         }
//         assert_eq!(tokens.len(), 16);
//         assert_eq!(tokens[0], Token::Class);
//         assert_eq!(tokens[1], Token::Identifier("Dog".to_string()));
//         assert_eq!(tokens[2], Token::Extends);
//         assert_eq!(tokens[3], Token::Identifier("Animal".to_string()));
//         assert_eq!(tokens[4], Token::LeftBrace);
//         assert_eq!(tokens[5], Token::Init);
//         assert_eq!(tokens[6], Token::LeftParen);
//         assert_eq!(tokens[7], Token::RightParen);
//         assert_eq!(tokens[8], Token::LeftBrace);
//         assert_eq!(tokens[9], Token::Super);
//         assert_eq!(tokens[10], Token::LeftParen);
//         assert_eq!(tokens[11], Token::RightParen);
//         assert_eq!(tokens[12], Token::Semicolon);
//         assert_eq!(tokens[13], Token::RightBrace);
//         assert_eq!(tokens[14], Token::RightBrace);
//         assert_eq!(tokens[15], Token::EOF);
//     }

//     #[test]
//     fn test_strings() {
//         let src = r#"console.log("Hello, World!");"#;
//         let mut lexer = Lexer::new(src);
//         let mut tokens = Vec::new();

//         loop {
//             let token = lexer.next_token().unwrap();
//             tokens.push(token.clone());
//             let is_eof = token == Token::EOF;
//             if is_eof {
//                 break;
//             }
//         }
//         assert_eq!(tokens.len(), 8);
//         assert_eq!(tokens[0], Token::Identifier("console".to_string()));
//         assert_eq!(tokens[1], Token::Dot);
//         assert_eq!(tokens[2], Token::Identifier("log".to_string()));
//         assert_eq!(tokens[3], Token::LeftParen);
//         assert_eq!(tokens[4], Token::StringLiteral("Hello, World!".to_string()));
//         assert_eq!(tokens[5], Token::RightParen);
//         assert_eq!(tokens[6], Token::Semicolon);
//         assert_eq!(tokens[7], Token::EOF);
//     }

//     #[test]
//     fn test_const_array() {
//         let src = "const num = [1, 2];";
//         let mut lexer = Lexer::new(src);
//         let mut tokens = Vec::new();

//         loop {
//             let token = lexer.next_token().unwrap();
//             tokens.push(token.clone());
//             let is_eof = token == Token::EOF;
//             if is_eof {
//                 break;
//             }
//         }
//         assert_eq!(tokens[0], Token::Const);
//         assert_eq!(tokens[1], Token::Identifier("num".to_string()));
//         assert_eq!(tokens[2], Token::Assign);
//         assert_eq!(tokens[3], Token::LeftBracket);
//         assert_eq!(tokens[4], Token::IntegerLiteral(1));
//         assert_eq!(tokens[5], Token::Comma);
//         assert_eq!(tokens[6], Token::IntegerLiteral(2));
//         assert_eq!(tokens[7], Token::RightBracket);
//         assert_eq!(tokens[8], Token::Semicolon);
//         assert_eq!(tokens[9], Token::EOF);
//     }
// }
