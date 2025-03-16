use super::*;
use crate::ast::{BlockStmt, Stmt, VarDeclStmt};
use crate::lexer::{Span, TokenType};

pub trait ParserStmt {
    fn parse_block(&mut self, parent_span: Span, context: BlockContext) -> Option<Stmt>;
    fn parse_statement(&mut self) -> Option<Stmt>;
}

impl ParserStmt for Parser {
    fn parse_block(&mut self, parent_span: Span, context: BlockContext) -> Option<Stmt> {
        let mut block = BlockStmt::default();
        let mut delimiter_stack: Vec<DelimiterContext> = Vec::new();

        // opening brace
        if let Some(token) = self.peek() {
            let span = token.span.clone();
            if token.token_type == TokenType::LeftBrace {
                delimiter_stack.push(DelimiterContext {
                    typ: DelimiterType::Brace,
                    span,
                });
                self.advance();

                // if closed immediately, then return empty block stmt
                if let Some(next) = self.peek() {
                    if next.token_type == TokenType::RightBrace {
                        self.advance();
                        return Some(Stmt::Block(block));
                    }
                }
            } else {
                self.errors.push(ParseError::ExpectedLeftCurlyBrace {
                    symbol: "method".to_string(),
                    span,
                });
                return None;
            }
        } else {
            self.errors
                .push(ParseError::UnexpectedEOF { span: parent_span });
        }

        while let Some(token) = self.peek() {
            match &token.token_type {
                TokenType::LeftBrace => {
                    let nested_context = BlockContext::Nested(Box::new(context.clone()));
                    if let Some(stmt) = self.parse_block(token.span.clone(), nested_context) {
                        block.statements.push(stmt);
                        continue;
                    }
                }
                TokenType::RightBrace => {
                    if let Some(ctx) = delimiter_stack.pop() {
                        if ctx.typ == DelimiterType::Brace {
                            self.advance();
                            if delimiter_stack.is_empty() {
                                return Some(Stmt::Block(block));
                            }
                        } else {
                            self.errors.push(ParseError::MismatchedDelimiter {
                                expected: ctx.typ.to_string(),
                                found: DelimiterType::Brace.to_string(),
                                span: token.span.clone(),
                            });
                            return None;
                        }
                    }
                }
                TokenType::LeftParen | TokenType::LeftBracket => {
                    delimiter_stack.push(DelimiterContext {
                        typ: match token.token_type {
                            TokenType::LeftParen => DelimiterType::Paren,
                            TokenType::LeftBracket => DelimiterType::Bracket,
                            _ => unreachable!(),
                        },
                        span: token.span.clone(),
                    });
                    self.advance();
                }
                TokenType::RightParen | TokenType::RightBracket => {
                    let expected_type = match token.token_type {
                        TokenType::RightParen => DelimiterType::Paren,
                        TokenType::RightBracket => DelimiterType::Bracket,
                        _ => unreachable!(),
                    };

                    match delimiter_stack.pop() {
                        Some(ctx) if ctx.typ == expected_type => {
                            self.advance();
                        }
                        Some(ctx) => {
                            self.errors.push(ParseError::MismatchedDelimiter {
                                expected: ctx.typ.to_string(),
                                found: expected_type.to_string(),
                                span: token.span.clone(),
                            });
                            return None;
                        }
                        None => {
                            self.errors.push(ParseError::UnexpectedClosingDelimiter {
                                delimiter: token.token_type.to_string(),
                                span: token.span.clone(),
                            });
                            return None;
                        }
                    }
                }
                // parse statements
                _ => {
                    if let Some(stmt) = self.parse_statement() {
                        match &stmt {
                            Stmt::Return(_) if !context.allows_return() => {
                                self.errors.push(ParseError::InvalidReturnLocation {
                                    span: token.span.clone(),
                                });
                                return None;
                            }
                            Stmt::Block(_) if !context.allows_break() => {
                                self.errors.push(ParseError::InvalidBreakLocation {
                                    span: token.span.clone(),
                                });
                                return None;
                            }
                            _ => block.statements.push(stmt),
                        }
                    }
                }
            }
        }

        if let Some(ctx) = delimiter_stack.last() {
            self.errors.push(ParseError::UnclosedDelimiter {
                delimiter: ctx.typ.to_string(),
                span: ctx.span.clone(),
            });
        }
        None
    }

    // TODO: Finish impl
    fn parse_statement(&mut self) -> Option<Stmt> {
        if let Some(token) = self.peek() {
            let span = token.span.clone();
            match token.token_type {
                TokenType::Let => {
                    if let Some(ident_tok) = self.peek() {
                        match ident_tok.token_type {
                            TokenType::Identifier(ident) => {
                                // consume colon
                                if let Some(token) = self.peek() {
                                    let colon_span = token.span.clone();
                                    if let TokenType::Colon = token.token_type {
                                        self.advance();
                                    } else {
                                        self.errors.push(ParseError::ExpectedColon {
                                            symbol: ident,
                                            span: colon_span,
                                        });
                                        return None;
                                    }
                                } else {
                                    self.errors.push(ParseError::UnexpectedEOF { span });
                                    return None;
                                }

                                // consume type and check assignment
                                if let Some(token) = self.peek() {
                                    if let TokenType::Type(typ) = token.token_type {
                                        if let Some(token) = self.peek() {
                                            let span = token.span.clone();
                                            if token.token_type == TokenType::Semicolon {
                                                return Some(Stmt::VarDecl(VarDeclStmt {
                                                    name: ident,
                                                    var_type: typ,
                                                    span,
                                                }));
                                            } else if token.token_type == TokenType::Assign {
                                                let _expr = self.parse_expr();
                                                // TODO: Finish impl
                                            }
                                        }
                                    }
                                } else {
                                    self.errors.push(ParseError::UnexpectedEOF { span });
                                    return None;
                                }
                            }
                            _ => {
                                self.errors.push(ParseError::ExpectedIdentifier {
                                    found: ident_tok.token_type.to_string(),
                                    span,
                                });
                            }
                        }
                    }
                }
                _ => {
                    self.errors.push(ParseError::UnexpectedToken {
                        symbol: token.token_type.to_string(),
                        span,
                    });
                    return None;
                }
            }
        }
        None
    }
}
