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

    fn skip_comments(&mut self) {
        if let Some(ch) = self.peek() {
            if ch == '#' {
                while let Some(ch) = self.peek() {
                    if ch == '\n' {
                        self.skip_whitespace();
                        break;
                    }
                    self.advance();
                }
                self.start_column = self.column
            }
        }
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
        self.skip_whitespace();
        self.skip_comments();
        let start_loc = self.current_location();

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
mod tests{
    use super::*;
    use token::Token;
    pub use error::{LexicalError, SourceLocation};

    //testing proper tokenization
    #[test]
    fn tokenize_punctuation() {
        let mut lexer = Lexer::new("() {}; , .");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::LeftParen, Token::RightParen, 
                Token::LeftBrace, Token::RightBrace, Token::Semicolon, 
                Token::Comma, Token::Dot, Token:: EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
        
    }

    #[test]
    fn tokenize_empty() {
        let mut lexer = Lexer::new("");
        let expected: Result<Vec<Token>> = Ok(vec!(Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
        
    }

    #[test]
    fn tokenize_operators() {
        let mut lexer = Lexer::new("+ - * / =");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::Plus, Token::Minus, Token::Star, 
                Token::Slash, Token::Equals, Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
        
    }

    #[test]
    fn tokenize_keywords() {
        let mut lexer = Lexer::new("class method init extends this super while break return if else new true false println");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::Class, Token::Method, Token::Init,
                Token::Extends, Token::This, Token::Super,
                Token::While, Token::Break, Token::Return,
                Token::If, Token::Else, Token::New,Token::True,
                Token::False, Token::Println,Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
        
    }

    #[test]
    fn tokenize_types() {
        let mut lexer = Lexer::new("Int Boolean Void");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::Int,Token::Boolean,Token::Void,Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
        
    }

    #[test]
    fn tokenize_pos_integers() {
        let mut lexer = Lexer::new("1 10 100");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::IntegerLiteral(1), Token::IntegerLiteral(10),
                Token::IntegerLiteral(100), Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap()); 
    }

    #[test]
    fn tokenize_neg_integers() {
        let mut lexer = Lexer::new("-1 -10 -100");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::Minus, Token::IntegerLiteral(1), Token::Minus, 
                Token::IntegerLiteral(10), Token::Minus,
                Token::IntegerLiteral(100), Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());   
    }

    #[test]
    fn tokenize_whitespace() {
        let mut lexer = Lexer::new("Int    value        =    \n      123");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::Int, Token::Identifier(String::from("value")), Token::Equals,
                Token::IntegerLiteral(123), Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
        
    }

    #[test]
    fn tokenize_comments() {
        let mut lexer = Lexer::new("Int value = 123; #This line creates an integer variable with value 123 \n
                                            2 + 2");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::Int, Token::Identifier(String::from("value")), Token::Equals,
                Token::IntegerLiteral(123), Token::Semicolon, Token::IntegerLiteral(2),
                Token::Plus, Token::IntegerLiteral(2), Token::EOF));   
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());
        
    }

    #[test]
    fn tokenize_string() {
        let mut lexer = Lexer::new("\"hello\"");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::StringLiteral("hello".to_string()),Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());  
    }

    #[test]
    fn tokenize_string_newline() {
        let mut lexer = Lexer::new("\"hello \\nworld\"");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::StringLiteral("hello \nworld".to_string()),Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());  
    }

    #[test]
    fn tokenize_string_tab() {
        let mut lexer = Lexer::new("\"hello \\tworld\"");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::StringLiteral("hello \tworld".to_string()),Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());  
    }

    #[test]
    fn tokenize_string_return() {
        let mut lexer = Lexer::new("\"hello \\rworld\"");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::StringLiteral("hello \rworld".to_string()),Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());  
    }

    #[test]
    fn tokenize_string_quotes() {
        let mut lexer = Lexer::new("\"hello \\\"world\\\"\"");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::StringLiteral("hello \"world\"".to_string()),Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());  
    }

    #[test]
    fn tokenize_string_slash() {
        let mut lexer = Lexer::new("\"hello \\\\ world\"");
        let expected: Result<Vec<Token>> = 
            Ok(vec!(Token::StringLiteral("hello \\ world".to_string()),Token::EOF));
        assert_eq!(lexer.tokenize().unwrap(), expected.unwrap());  
    }

    //testing errors
    #[test]
    fn tokenize_unterminated_string() {
        let mut lexer = Lexer::new("String \"Hello World");
        let expected = 
            LexicalError::UnterminatedString { location: (SourceLocation { line: (1), column: (8) }) };
        assert_eq!(lexer.tokenize().unwrap_err(), expected);
    }

    #[test]
    fn tokenize_invalid_number() {
        let mut lexer = Lexer::new("13 * 2 \nInt i = 123a");
        let expected = 
            LexicalError::InvalidNumber { value: "123a".to_string(),location: (SourceLocation { line: (2), column: (9) }) };
        assert_eq!(lexer.tokenize().unwrap_err(), expected);
    }

    #[test]
    fn tokenize_invalid_character() {
        let mut lexer = Lexer::new("13 * 2 \nint $ = 123");
        let expected = 
            LexicalError::InvalidChar { character: ('$'), location: (SourceLocation { line: (2), column: (5) }) };
        assert_eq!(lexer.tokenize().unwrap_err(), expected);
    }

    #[test]
    fn tokenize_invalid_escape() {
        let mut lexer = Lexer::new("string \"Hello \\world\"");
        let expected = 
            LexicalError::InvalidEscapeSequence { escape: ('w'), location: (SourceLocation { line: (1), column: (8) }) };
        assert_eq!(lexer.tokenize().unwrap_err(), expected);
    }


    #[test]
    fn tokenize_unexpected_eof() {
        let mut lexer = Lexer::new("string \"Hello \\");
        let expected = 
            LexicalError::UnexpectedEOF { location: (SourceLocation{line: 1, column:8 }) };
        assert_eq!(lexer.tokenize().unwrap_err(), expected);
    }


    
}
