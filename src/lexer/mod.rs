mod error;
mod token;

pub use error::{LexicalError, SourceLocation};
use token::Token;

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

    fn read_string(&mut self) -> Result<Token> {
        let start_loc = self.current_location();
        let mut string = String::new();
        self.advance(); // skip opening quote

        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    self.advance(); // skip closing quote
                    return Ok(Token::StringLiteral(string));
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
                                    location: start_loc,
                                })
                            }
                        };
                        string.push(escaped);
                        self.advance();
                    } else {
                        return Err(LexicalError::UnexpectedEOF {
                            location: start_loc,
                        });
                    }
                }
                _ => {
                    string.push(ch);
                    self.advance();
                }
            }
        }
        Err(LexicalError::UnterminatedString {
            location: start_loc,
        })
    }

    fn current_location(&self) -> SourceLocation {
        SourceLocation::new(self.line, self.start_column)
    }

    fn read_number(&mut self) -> Result<Token> {
        let start_loc = self.current_location();
        let mut number = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                number.push(ch);
                self.advance();
            } else if ch.is_alphabetic() {
                return Err(LexicalError::InvalidNumber {
                    value: format!("{}{}", number, ch),
                    location: start_loc,
                });
            } else {
                break;
            }
        }

        match number.parse::<i64>() {
            Ok(n) => Ok(Token::IntegerLiteral(n)),
            Err(_) => Err(LexicalError::InvalidNumber {
                value: number,
                location: start_loc,
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
        let start_loc = self.current_location();
        self.skip_whitespace();

        match self.peek() {
            None => Ok(Token::EOF),
            Some(ch) => match ch {
                // single char tokens
                '+' => { self.advance(); Ok(Token::Plus) }
                '-' => { self.advance(); Ok(Token::Minus) }
                '*' => {
                    self.advance();
                    Ok(Token::Star)
                }
                '/' => {
                    self.advance();
                    Ok(Token::Slash)
                }
                '=' => {
                    self.advance();
                    Ok(Token::Equals)
                }
                '(' => {
                    self.advance();
                    Ok(Token::LeftParen)
                }
                ')' => {
                    self.advance();
                    Ok(Token::RightParen)
                }
                '{' => {
                    self.advance();
                    Ok(Token::LeftBrace)
                }
                '}' => {
                    self.advance();
                    Ok(Token::RightBrace)
                }
                ';' => {
                    self.advance();
                    Ok(Token::Semicolon)
                }
                ',' => {
                    self.advance();
                    Ok(Token::Comma)
                }
                '.' => {
                    self.advance();
                    Ok(Token::Dot)
                }
                '"' => self.read_string(),

                // numbers
                ch if ch.is_digit(10) => self.read_number(),

                // identifiers and keywords
                ch if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();
                    match identifier.as_str() {
                        "class" => Ok(Token::Class),
                        "method" => Ok(Token::Method),
                        "init" => Ok(Token::Init),
                        "extends" => Ok(Token::Extends),
                        "this" => Ok(Token::This),
                        "super" => Ok(Token::Super),
                        "while" => Ok(Token::While),
                        "break" => Ok(Token::Break),
                        "return" => Ok(Token::Return),
                        "if" => Ok(Token::If),
                        "else" => Ok(Token::Else),
                        "new" => Ok(Token::New),
                        "true" => Ok(Token::True),
                        "false" => Ok(Token::False),
                        "println" => Ok(Token::Println),
                        "Int" => Ok(Token::Int),
                        "Boolean" => Ok(Token::Boolean),
                        "Void" => Ok(Token::Void),
                        _ => Ok(Token::Identifier(identifier)),
                    }
                }

                // error if no match
                ch => Err(LexicalError::InvalidChar {
                    character: ch,
                    location: start_loc,
                }),
            },
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = token == Token::EOF;
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

    #[test]
    fn test_integers() {
        let src = "Int myNumber = 123;";
        let mut lexer = Lexer::new(src);
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let token = lexer.next_token().unwrap();
            tokens.push(token.clone());
            let is_eof = token == Token::EOF;
            if is_eof {
                break;
            }
        }

        print!("{:?}", tokens);
        
        assert_eq!(tokens[0], Token::Int);
        assert_eq!(tokens[1], Token::Identifier("myNumber".to_string()));
        assert_eq!(tokens[2], Token::Equals);
        assert_eq!(tokens[4], Token::Semicolon);
        assert_eq!(tokens[5], Token::EOF);
    }

    #[test]
    fn test_classes() {
        let src = "class Animal { init() {} method speak() Void { return println(0); } }";
        let mut lexer = Lexer::new(src);
        let mut tokens = Vec::new();

        loop {
            let token = lexer.next_token().unwrap();
            tokens.push(token.clone());
            let is_eof = token == Token::EOF;
            if is_eof {
                break;
            }
        }
        assert_eq!(tokens[0], Token::Class);
        assert_eq!(tokens[1], Token::Identifier("Animal".to_string()));
        assert_eq!(tokens[2], Token::LeftBrace);
        assert_eq!(tokens[3], Token::Init);
        assert_eq!(tokens[4], Token::LeftParen);
        assert_eq!(tokens[5], Token::RightParen);
        assert_eq!(tokens[6], Token::LeftBrace);
        assert_eq!(tokens[7], Token::RightBrace);
        assert_eq!(tokens[8], Token::Method);
        assert_eq!(tokens[9], Token::Identifier("speak".to_string()));
        assert_eq!(tokens[10], Token::LeftParen);
        assert_eq!(tokens[11], Token::RightParen);
        assert_eq!(tokens[12], Token::Void);
        assert_eq!(tokens[13], Token::LeftBrace);
        assert_eq!(tokens[14], Token::Return);
        assert_eq!(tokens[15], Token::Println);
        assert_eq!(tokens[16], Token::LeftParen);
        assert_eq!(tokens[17], Token::IntegerLiteral(0));
        assert_eq!(tokens[18], Token::RightParen);
        assert_eq!(tokens[19], Token::Semicolon);
        assert_eq!(tokens[20], Token::RightBrace);
        assert_eq!(tokens[21], Token::RightBrace);
        assert_eq!(tokens[22], Token::EOF);
    }
}

#[test]
fn test_inheritance() {
    let src = "class Dog extends Animal { init() { super(); } }";
    let mut lexer = Lexer::new(src);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token().unwrap();
        tokens.push(token.clone());
        let is_eof = token == Token::EOF;
        if is_eof {
            break;
        }
    }
    assert_eq!(tokens.len(), 16);
    assert_eq!(tokens[0], Token::Class);
    assert_eq!(tokens[1], Token::Identifier("Dog".to_string()));
    assert_eq!(tokens[2], Token::Extends);
    assert_eq!(tokens[3], Token::Identifier("Animal".to_string()));
    assert_eq!(tokens[4], Token::LeftBrace);
    assert_eq!(tokens[5], Token::Init);
    assert_eq!(tokens[6], Token::LeftParen);
    assert_eq!(tokens[7], Token::RightParen);
    assert_eq!(tokens[8], Token::LeftBrace);
    assert_eq!(tokens[9], Token::Super);
    assert_eq!(tokens[10], Token::LeftParen);
    assert_eq!(tokens[11], Token::RightParen);
    assert_eq!(tokens[12], Token::Semicolon);
    assert_eq!(tokens[13], Token::RightBrace);
    assert_eq!(tokens[14], Token::RightBrace);
    assert_eq!(tokens[15], Token::EOF);
}

#[test]
fn test_strings() {
    let src = r#""Hello, World!""#;
    let mut lexer = Lexer::new(src);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token().unwrap();
        tokens.push(token.clone());
        let is_eof = token == Token::EOF;
        if is_eof {
            break;
        }
    }
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::StringLiteral("Hello, World!".to_string()));
    assert_eq!(tokens[1], Token::EOF);

}