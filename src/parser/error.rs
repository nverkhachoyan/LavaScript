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
pub enum ParseError {
    #[error("Missing type annotation for '{symbol}' at {location}")]
    MissingTypeAnnotation {
        symbol: String,
        location: SourceLocation,
    },

    #[error("Missing class name at {location}")]
    MissingClassName { location: SourceLocation },

    #[error("Missing class name which '{symbol}' extends at {location}")]
    MissingClassExtendIdent {
        symbol: String,
        location: SourceLocation,
    },

    #[error("Missing opening curly brace for class '{symbol}' at {location}")]
    MissingOpeningCurlyBrace {
        symbol: String,
        location: SourceLocation,
    },

    #[error("Missing closing curly brace for class '{symbol}' at {location}")]
    MissingClosingCurlyBrace {
        symbol: String,
        location: SourceLocation,
    },

    #[error("Unexpected end of file at {location}")]
    UnexpectedEOF { location: SourceLocation },
}
