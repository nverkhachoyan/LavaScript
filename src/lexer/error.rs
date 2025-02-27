use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum LexicalError {
    #[error("Invalid character '{character}' at {location}")]
    InvalidChar {
        character: char,
        location: SourceLocation,
    },

    #[error("Unterminated string literal at {location}")]
    UnterminatedString { location: SourceLocation },

    #[error("Invalid number format '{value}' at {location}")]
    InvalidNumber {
        value: String,
        location: SourceLocation,
    },

    #[error("Invalid escape sequence '\\{escape}' at {location}")]
    InvalidEscapeSequence {
        escape: char,
        location: SourceLocation,
    },

    #[error("Unexpected end of file at {location}")]
    UnexpectedEOF { location: SourceLocation },
}
