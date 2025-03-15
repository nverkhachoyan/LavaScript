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

    #[error("Missing opening curly brace for class '{symbol}' at {span}")]
    MissingOpeningCurlyBrace { symbol: String, span: Span },

    #[error("Missing closing curly brace for class '{symbol}' at {span}")]
    MissingClosingCurlyBrace { symbol: String, span: Span },

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

    #[error("Unexpected end of file at {span}")]
    UnexpectedEOF { span: Span },
}

impl ParseError {
    pub fn get_span(&self) -> &Span {
        match self {
            Self::MissingClassName { span } => span,
            Self::MissingClassExtendIdent { span, .. } => span,
            Self::MissingOpeningCurlyBrace { span, .. } => span,
            Self::MissingClosingCurlyBrace { span, .. } => span,
            Self::MissingClassInit { span, .. } => span,
            Self::ExpectedParamName { span, .. } => span,
            Self::ExpectedColonParamDecl { span, .. } => span,
            Self::ExpectedParamType { span, .. } => span,
            Self::UnexpectedEOF { span } => span,
            Self::MissingTypeAnnotation { span, .. } => span,
            Self::ExpectedMethName { span, .. } => span,
        }
    }

    pub fn get_code(&self) -> &str {
        match self {
            Self::MissingClassName { .. } => "E001",
            Self::MissingClassExtendIdent { .. } => "E002",
            Self::MissingOpeningCurlyBrace { .. } => "E003",
            Self::MissingClosingCurlyBrace { .. } => "E004",
            Self::MissingClassInit { .. } => "E005",
            Self::ExpectedParamName { .. } => "E006",
            Self::ExpectedColonParamDecl { .. } => "E007",
            Self::ExpectedParamType { .. } => "E008",
            Self::UnexpectedEOF { .. } => "E009",
            Self::MissingTypeAnnotation { .. } => "E010",
            Self::ExpectedMethName { .. } => "E011",
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
                eprintln!(
                    "{} {}",
                    format!("{:3} |", start_line).blue().bold(),
                    lines[start_line - 1]
                );
            }

            eprintln!(
                "{} {}",
                format!("{:3} |", span.line).blue().bold(),
                lines[span.line - 1]
            );

            let indicator = " ".repeat(span.column.saturating_sub(1)) + "^";
            let label = match self {
                Self::MissingClassName { .. } => " expected class name",
                Self::MissingClassExtendIdent { .. } => " expected identifier after 'extends'",
                Self::MissingOpeningCurlyBrace { .. } => " expected '{'",
                Self::MissingClosingCurlyBrace { .. } => " expected '}'",
                _ => "",
            };

            eprintln!(
                "{} {}{}",
                "    |".blue().bold(),
                indicator.red().bold(),
                label.red()
            );

            if end_line > span.line {
                eprintln!(
                    "{} {}",
                    format!("{:3} |", end_line).blue().bold(),
                    lines[end_line - 1]
                );
            }
        }

        eprintln!();
    }
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
