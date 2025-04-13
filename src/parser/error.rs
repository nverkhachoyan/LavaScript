use crate::lexer::Span;
use colored::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ParseError {
    #[error("Expected method name for class '{symbol}' at {span}")]
    ExpectedMethName { symbol: String, span: Span },

    #[error("Expected '{expected}' but found '{found}' at {}",  
    .span.map_or("unknown location".to_string(), |s| s.to_string()))]
    ExpectedButFound {
        expected: String,
        found: String,
        span: Option<Span>,
    },

    #[error("Expected expression after comma {symbol} at {span}")]
    ExpectedExpressionAfterComma { symbol: String, span: Span },

    #[error("Expected expression at {span}")]
    ExpectedExpression { span: Span },

    #[error("Unexpected token at {symbol} at {span}")]
    UnexpectedToken { symbol: String, span: Span },

    #[error("Unexpected EOF at {span:?}")]
    UnexpectedEOF { span: Option<Span> },
}

impl ParseError {
    pub fn get_span(&self) -> Option<&Span> {
        match self {
            Self::ExpectedMethName { span, .. }
            | Self::UnexpectedToken { span, .. }
            | Self::ExpectedExpressionAfterComma { span, .. }
            | Self::ExpectedExpression { span, .. } => Some(span),

            Self::ExpectedButFound { span, .. } => span.as_ref(),
            Self::UnexpectedEOF { span } => span.as_ref(),
        }
    }

    pub fn get_code(&self) -> &str {
        match self {
            Self::ExpectedMethName { .. } => "E011",
            Self::ExpectedButFound { .. } => "E012",
            Self::UnexpectedEOF { .. } => "E009",
            Self::UnexpectedToken { .. } => "E020",
            Self::ExpectedExpressionAfterComma { .. } => "E024",
            Self::ExpectedExpression { .. } => "E025",
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

                eprintln!("{} {}", "    |".blue().bold(), indicator.red().bold(),);

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