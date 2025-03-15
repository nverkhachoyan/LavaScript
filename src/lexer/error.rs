use crate::lexer::Span;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LexicalError {
    #[error("Invalid character '{character}' at {span}")]
    InvalidChar { character: char, span: Span },

    #[error("Unterminated string literal at {span}")]
    UnterminatedString { span: Span },

    #[error("Invalid number format '{value}' at {span}")]
    InvalidNumber { value: String, span: Span },

    #[error("Invalid escape sequence '\\{escape}' at {span}")]
    InvalidEscapeSequence { escape: char, span: Span },

    #[error("Unexpected end of file at {span}")]
    UnexpectedEOF { span: Span },
}
