use crate::lexer::Span;
use colored::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
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

    #[error("Expected left paren for '{symbol}' at {span}")]
    ExpectedLeftParen { symbol: String, span: Span },

    #[error("Expected return type for '{symbol}' at {span}")]
    ExpectedReturnType { symbol: String, span: Span },

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

    #[error("Unexpected end of file at {span}")]
    UnexpectedEOF { span: Span },
}

impl ParseError {
    pub fn get_span(&self) -> &Span {
        match self {
            Self::UnexpectedEOF { span }
            | Self::MissingClassName { span }
            | Self::MissingTypeAnnotation { span, .. }
            | Self::MissingClassExtendIdent { span, .. }
            | Self::ExpectedLeftCurlyBrace { span, .. }
            | Self::ExpectedRightCurlyBrace { span, .. }
            | Self::MissingClassInit { span, .. }
            | Self::ExpectedParamName { span, .. }
            | Self::ExpectedColonParamDecl { span, .. }
            | Self::ExpectedParamType { span, .. }
            | Self::ExpectedMethName { span, .. }
            | Self::ExpectedLeftParen { span, .. }
            | Self::ExpectedReturnType { span, .. }
            | Self::MismatchedDelimiter { span, .. }
            | Self::UnclosedDelimiter { span, .. }
            | Self::InvalidReturnLocation { span, .. }
            | Self::InvalidBreakLocation { span, .. }
            | Self::UnexpectedToken { span, .. }
            | Self::ExpectedIdentifier { span, .. }
            | Self::ExpectedColon { span, .. }
            | Self::UnexpectedClosingDelimiter { span, .. } => span,
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
            Self::ExpectedLeftParen { .. } => "E012",
            Self::ExpectedReturnType { .. } => "E013",
            Self::MismatchedDelimiter { .. } => "E014",
            Self::UnclosedDelimiter { .. } => "E015",
            Self::UnexpectedClosingDelimiter { .. } => "E016",
            Self::InvalidReturnLocation { .. } => "E017",
            Self::InvalidBreakLocation { .. } => "E018",
            Self::UnexpectedToken { .. } => "E019",
            Self::ExpectedIdentifier { .. } => "E020",
            Self::ExpectedColon { .. } => "E021",
        }
    }

    pub fn print_with_context(&self, source: &str) {
        let span = self.get_span();
        let error_code = self.get_code();
        let lines: Vec<&str> = source.lines().collect();

        eprintln!(
            "{}: {} {}",
            "error".red().bold(),
            error_code.yellow(),
            self.to_string().white().bold()
        );

        // print source context if the span points to a valid line
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

        eprintln!();
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
