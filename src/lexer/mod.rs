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
                                return Err(LexicalError::InvalidChar {
                                    character: next,
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

    pub fn next_token(&mut self) -> Result<Token> {
        let start_loc = self.current_location();
        self.skip_whitespace();

        match self.peek() {
            None => Ok(Token::EOF),
            Some(ch) => match ch {
                // single char tokens
                '+' => {
                    self.advance();
                    Ok(Token::Plus)
                }
                '-' => {
                    self.advance();
                    Ok(Token::Minus)
                }
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
