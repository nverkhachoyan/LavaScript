use crate::lexer::Span;
use colored::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ParseError {
    #[error("Missing type annotation for '{symbol}' at {span}")]
    MissingTypeAnnotation { symbol: String, span: Span },

    #[error("Missing class name at {span}")]
    MissingClassName { span: Span },

    #[error("Missing class name which '{symbol}' extends at {span}")]
    MissingClassExtendIdent { symbol: String, span: Span },

    #[error("Expected opening curly brace for class '{symbol}' at {span}")]
    ExpectedLeftCurlyBrace { symbol: String, span: Span },

    #[error("Expected closing curly brace for class '{symbol}' at {span}")]
    ExpectedRightCurlyBrace { symbol: String, span: Span },

    #[error("Constructor missing for class '{symbol}' at {span}")]
    MissingClassInit { symbol: String, span: Span },

    #[error("Expected parameter name for '{symbol}' at {span}")]
    ExpectedParamName { symbol: String, span: Span },

    #[error("Expected colon after param name '{symbol}' at {span}")]
    ExpectedColonParamDecl { symbol: String, span: Span },

    #[error("Expected type for parameter '{symbol}' at {span}")]
    ExpectedParamType { symbol: String, span: Span },

    #[error("Expected method name '{symbol}' at {span}")]
    ExpectedMethName { symbol: String, span: Span },

    #[error("Expected '{expected}' but found '{found}' at {}",  
    .span.map_or("unknown location".to_string(), |s| s.to_string()))]
    ExpectedButFound {
        expected: String,
        found: String,
        span: Option<Span>,
    },

    #[error("Expected return type for '{symbol}' at {span}")]
    ExpectedReturnType { symbol: String, span: Span },

    #[error("Expected a semicolon at {span}")]
    ExpectedSemicolon { span: Span },

    #[error("Mismatched delimiter, '{expected}' at {span}")]
    MismatchedDelimiter {
        expected: String,
        found: String,
        span: Span,
    },

    #[error("Unclosed delimiter '{delimiter}' at {span}")]
    UnclosedDelimiter { delimiter: String, span: Span },

    #[error("Unclosed delimiter '{delimiter}' at {span}")]
    UnexpectedClosingDelimiter { delimiter: String, span: Span },

    #[error("Invalid return location at {span}")]
    InvalidReturnLocation { span: Span },

    #[error("Invalid break location at {span}")]
    InvalidBreakLocation { span: Span },

    #[error("Expected identifier, found '{found}' at {span}")]
    ExpectedIdentifier { found: String, span: Span },

    #[error("Unexpected token at {symbol} at {span}")]
    UnexpectedToken { symbol: String, span: Span },

    #[error("Expected colon after {symbol} at {span}")]
    ExpectedColon { symbol: String, span: Span },

    #[error("Expected expression after comma {symbol} at {span}")]
    ExpectedExpressionAfterComma { symbol: String, span: Span },

    #[error("Unexpected EOF at {span:?}")]
    UnexpectedEOF { span: Option<Span> },
}

impl ParseError {
    pub fn get_span(&self) -> Option<&Span> {
        match self {
            Self::MissingClassName { span }
            | Self::MissingTypeAnnotation { span, .. }
            | Self::MissingClassExtendIdent { span, .. }
            | Self::ExpectedLeftCurlyBrace { span, .. }
            | Self::ExpectedRightCurlyBrace { span, .. }
            | Self::MissingClassInit { span, .. }
            | Self::ExpectedParamName { span, .. }
            | Self::ExpectedColonParamDecl { span, .. }
            | Self::ExpectedParamType { span, .. }
            | Self::ExpectedMethName { span, .. }
            | Self::ExpectedReturnType { span, .. }
            | Self::MismatchedDelimiter { span, .. }
            | Self::UnclosedDelimiter { span, .. }
            | Self::InvalidReturnLocation { span, .. }
            | Self::InvalidBreakLocation { span, .. }
            | Self::UnexpectedToken { span, .. }
            | Self::ExpectedIdentifier { span, .. }
            | Self::ExpectedColon { span, .. }
            | Self::ExpectedSemicolon { span, .. }
            | Self::ExpectedExpressionAfterComma { span, .. }
            | Self::UnexpectedClosingDelimiter { span, .. } => Some(span),

            Self::ExpectedButFound { span, .. } => span.as_ref(),

            Self::UnexpectedEOF { span } => span.as_ref(),
        }
    }

    pub fn get_code(&self) -> &str {
        match self {
            Self::MissingClassName { .. } => "E001",
            Self::MissingClassExtendIdent { .. } => "E002",
            Self::ExpectedLeftCurlyBrace { .. } => "E003",
            Self::ExpectedRightCurlyBrace { .. } => "E004",
            Self::MissingClassInit { .. } => "E005",
            Self::ExpectedParamName { .. } => "E006",
            Self::ExpectedColonParamDecl { .. } => "E007",
            Self::ExpectedParamType { .. } => "E008",
            Self::UnexpectedEOF { .. } => "E009",
            Self::MissingTypeAnnotation { .. } => "E010",
            Self::ExpectedMethName { .. } => "E011",
            Self::ExpectedButFound { .. } => "E012",
            Self::ExpectedReturnType { .. } => "E014",
            Self::MismatchedDelimiter { .. } => "E015",
            Self::UnclosedDelimiter { .. } => "E016",
            Self::UnexpectedClosingDelimiter { .. } => "E017",
            Self::InvalidReturnLocation { .. } => "E018",
            Self::InvalidBreakLocation { .. } => "E019",
            Self::UnexpectedToken { .. } => "E020",
            Self::ExpectedIdentifier { .. } => "E021",
            Self::ExpectedColon { .. } => "E022",
            Self::ExpectedSemicolon { .. } => "E023",
            Self::ExpectedExpressionAfterComma { .. } => "E024",
        }
    }

    pub fn print_with_context(&self, source: &str) {
        let error_code = self.get_code();

        eprintln!(
            "{}: {} {}",
            "error".red().bold(),
            error_code.yellow(),
            self.to_string().white().bold()
        );

        // print source context if we have a valid span
        if let Some(span) = self.get_span() {
            let lines: Vec<&str> = source.lines().collect();

            if span.line > 0 && span.line <= lines.len() {
                let start_line = span.line.saturating_sub(1);
                let end_line = std::cmp::min(span.line + 1, lines.len());

                // print file location
                eprintln!(
                    "{} {}:{}:{}",
                    "-->".blue().bold(),
                    "input".cyan(),
                    span.line,
                    span.column
                );

                eprintln!("{}", "    |".blue().bold());

                if start_line < span.line {
                    print_context_line(start_line, lines[start_line - 1]);
                }

                print_context_line(span.line, lines[span.line - 1]);

                let indicator = " ".repeat(span.column.saturating_sub(1)) + "^";
                let label = match self {
                    Self::MissingClassName { .. } => " expected class name",
                    Self::MissingClassExtendIdent { .. } => " expected identifier after 'extends'",
                    Self::ExpectedLeftCurlyBrace { .. } => " expected '{'",
                    Self::ExpectedRightCurlyBrace { .. } => " expected '}'",
                    _ => "",
                };

                eprintln!(
                    "{} {}{}",
                    "    |".blue().bold(),
                    indicator.red().bold(),
                    label.red()
                );

                if end_line > span.line {
                    print_context_line(end_line, lines[end_line - 1]);
                }
            }
        }

        eprintln!();
    }

    pub fn expected_but_found(expected: String, found: Option<String>, span: Option<Span>) -> Self {
        Self::ExpectedButFound {
            expected,
            found: found.unwrap_or_else(|| "unknown".to_string()),
            span,
        }
    }
}

fn print_context_line(line_num: usize, content: &str) {
    eprintln!("{} {}", format!("{:3} |", line_num).blue().bold(), content);
}

pub fn print_errors(errors: &[ParseError], source: &str) {
    if errors.is_empty() {
        return;
    }

    let error_count = errors.len();
    let error_text = if error_count == 1 { "error" } else { "errors" };

    eprintln!(
        "\n{}",
        format!("Found {} {}", error_count, error_text).red().bold()
    );

    for err in errors {
        err.print_with_context(source);
    }

    eprintln!(
        "{}: {} {}",
        "error".red().bold(),
        format!("aborting due to {} {}", error_count, error_text).bold(),
        "😞".yellow()
    );

    eprintln!();
}

mod tests {
    use crate::lexer::Span;

    use super::{print_context_line, print_errors, ParseError};
    
    #[test]
    fn test_error_codes() {
        assert!(ParseError::MissingClassName { span: Span{line:0, column:0} }.get_code() == "E001");
        assert!(ParseError::MissingClassExtendIdent { symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E002");
        assert!(ParseError::ExpectedLeftCurlyBrace { symbol: "".to_string() ,span: Span{line:0, column:0} }.get_code() == "E003");
        assert!(ParseError::ExpectedRightCurlyBrace { symbol: "".to_string() ,span: Span{line:0, column:0} }.get_code() == "E004");
        assert!(ParseError::MissingClassInit {symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E005");
        assert!(ParseError::ExpectedParamName { symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E006");
        assert!(ParseError::ExpectedColonParamDecl { symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E007");
        assert!(ParseError::ExpectedParamType { symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E008");
        assert!(ParseError::UnexpectedEOF  {span: Some(Span{line:0, column:0}) }.get_code() == "E009");
        assert!(ParseError::MissingTypeAnnotation { symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E010");
        assert!(ParseError::ExpectedMethName { symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E011");
        assert!(ParseError::ExpectedButFound { expected: "".to_string(), found: "".to_string(), span: Some(Span{line:0, column:0}) }.get_code() == "E012");
        assert!(ParseError::ExpectedReturnType { symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E014");
        assert!(ParseError::MismatchedDelimiter { expected: "".to_string(), found: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E015");
        assert!(ParseError::UnclosedDelimiter { delimiter: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E016");
        assert!(ParseError::UnexpectedClosingDelimiter { delimiter: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E017");
        assert!(ParseError::InvalidReturnLocation { span: Span{line:0, column:0} }.get_code() == "E018");
        assert!(ParseError::InvalidBreakLocation { span: Span{line:0, column:0} }.get_code() == "E019");
        assert!(ParseError::UnexpectedToken  { symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E020");
        assert!(ParseError::ExpectedIdentifier { found: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E021");
        assert!(ParseError::ExpectedColon { symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E022");
        assert!(ParseError::ExpectedSemicolon { span: Span{line:0, column:0} }.get_code() == "E023");
        assert!(ParseError::ExpectedExpressionAfterComma { symbol: "".to_string(), span: Span{line:0, column:0} }.get_code() == "E024");
    } 

    #[test]
    fn test_get_span() {
        let span = ParseError::MissingClassName { span: Span{line:0, column:0} }.get_span().unwrap();
        assert!(matches!(
            span,
            Span { line:0, column:0 }
        ))
    }

    #[test]
    fn test_get_span_ref() {
        let span = ParseError::UnexpectedEOF  {span: Some(Span{line:0, column:0}) }.get_span().unwrap();
        assert!(matches!(
            span,
            Span { line:0, column:0 }
        ))
    }

    #[test]
    fn test_print_errors_maximum_context() {
        let error = ParseError::ExpectedButFound { 
            expected: "expected".to_string(), 
            found: "found".to_string(), 
            span: Some(Span{line:0, column:0}) };
        error.print_with_context("test");
    }

    #[test]
    fn test_print_errors_unknown_location() {
        let error = ParseError::ExpectedButFound { 
            expected: "expected".to_string(), 
            found: "found".to_string(), 
            span: None };
        error.print_with_context("test");
    }

    #[test]
    fn test_print_context_line() {
        print_context_line(10, "content");
    }

    #[test]
    fn test_print_errors() {
        let error1 = ParseError::ExpectedButFound { 
            expected: "expected".to_string(), 
            found: "found".to_string(), 
            span: Some(Span{line:0, column:0}) };
        let error2 = ParseError::UnexpectedEOF { span: None };

        print_errors(&[error1,error2], "test");
    }
}