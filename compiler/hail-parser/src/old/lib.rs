//! A handwritten parser for Hail.

pub mod asi;
pub mod grammar;
pub mod raw_lexer;
pub mod raw_token;
pub mod token;

/*
use std::iter::Peekable;
use std::vec::IntoIter;

use token::{TokNode, Tok};

pub use hail_ast as ast;
use ast::{Node, ToNode};
use ast::const_expr::{Bool, ConstExpr, Iden, Str};
use ast::decl::Decl;
use hail_diagnostic::{Diag, Label};
use snailquote::{ParseUnicodeError, unescape, UnescapeError};

type Lex<'a> = Peekable<IntoIter<TokNode<'a>>>;

/// Parses a declaration.
pub fn parse_decl<'a>(iter: &mut Lex<'a>) -> Result<Option<Decl>, Diag> {
    if let Some(name) = parse_iden(iter) {
        
    }

    Ok(None)
}

/// Parses a constant token, such as a boolean, integer, string, ..., if any could be found.
pub fn parse_constant<'a>(iter: &mut Lex<'a>) -> Result<Option<Node<ConstExpr>>, Diag> {
    if let Some(iden) = parse_bool(iter) {
        return Ok(Some(ConstExpr::Bool(iden.node()).into_node(iden.loc())));
    } else if let Some(iden) = parse_iden(iter) {
        return Ok(Some(ConstExpr::Iden(iden.node()).into_node(iden.loc())));
    } else if let Some(str) = parse_str(iter)? {
        return Ok(Some(ConstExpr::Str(str.node()).into_node(str.loc())));
    }

    Ok(None)
}

pub fn parse_str<'a>(iter: &mut Lex<'a>) -> Result<Option<Node<Str>>, Diag> {
    if let Some(tok) = iter.peek() {
        match &tok.node {
            Tok::Str(str) => {
                let escaped = match unescape(str.value) {
                    Ok(escaped) => escaped,
                    Err(e) => match e {
                        UnescapeError::InvalidEscape { escape, index, .. } => {
                            let loc = tok.loc.start + index;

                            let label = Label::new(format!("I found an invalid escape code '{}' in this string.", escape))
                                .highlight(loc..loc + 1);
                            
                            let label2 = Label::new("'\\n' (new line) and '\\t' (tab character) are some examples of valid escape codes.");

                            let diag = Diag::error()
                                .with_code("E0008(parser)")
                                .with_labels(vec![label, label2])
                                .with_message("invalid escape code.");
                            
                            return Err(diag);
                        },
                        UnescapeError::InvalidUnicode { source, index, .. } => {
                            let loc = tok.loc.start + index;
                            match source {
                                ParseUnicodeError::BraceNotFound => {
                                    let label = Label::new(format!("I expected a brace here."))
                                        .highlight(loc..loc + 1);
                                    
                                    let label2 = Label::new("Unicode escapes are used like this: '\\u{XX}', \
                                    where 'XX' is a Unicode character code.");

                                    let diag = Diag::error()
                                        .with_code("E0009(parser)")
                                        .with_labels(vec![label, label2])
                                        .with_message("expected a brace.");
                                    
                                    return Err(diag);
                                },
                                ParseUnicodeError::ParseHexFailed { .. } => {
                                    let label = Label::new(format!("That unicode escape is invalid."))
                                        .highlight(loc..loc + 1);
                                    
                                    let label2 = Label::new("The tool I use to evaluate strings doesn't provide with enough data to help much, \
                                    But here are the reasons this error can be thrown:");

                                    let label3 = Label::new("The character code within the braces isn't valid hexadecimal (too large, non-hexadecimal characters, etc.)");

                                    let label4 = Label::new("There was no closing brace found after the opening one: \"\\u{xx\"");

                                    let diag = Diag::error()
                                        .with_code("E0010(parser)")
                                        .with_labels(vec![label, label2, label3, label4])
                                        .with_message("invalid unicode code in unicode excape.");
                                    
                                    return Err(diag);
                                },
                                ParseUnicodeError::ParseUnicodeFailed { value } => {
                                    let label = Label::new("I found this character code, it is apparently an invalid Unicode character.")
                                        .highlight(loc..loc + 1);
                                    
                                    let diag = Diag::error()
                                        .with_code("E0011(parser)")
                                        .with_labels(vec![label])
                                        .with_message("invalid unicode character.");
                                    
                                    return Err(diag);
                                },
                            }
                        }
                    }
                };

                return Ok(Some(Str {
                    value: escaped,
                }.into_node(tok.loc.clone())));
            },
            _ => {},
        }
    }

    Ok(None)
}

/// Parses an identifier literal.
pub fn parse_iden<'a>(iter: &mut Lex<'a>) -> Option<Node<Iden>> {
    if let Some(tok) = iter.peek() {
        match &tok.node {
            Tok::Iden(iden) => {
                if iden.value == "true" || iden.value == "false" {
                    return None;
                }

                return Some(Iden {
                    value: iden.value.to_string(),
                }.into_node(tok.loc.clone()));
            },
            _ => {},
        }
    }

    None
}

/// Parses an identifier literal.
pub fn parse_bool<'a>(iter: &mut Lex<'a>) -> Option<Node<Bool>> {
    if let Some(tok) = iter.peek() {
        match &tok.node {
            Tok::Iden(iden) => {
                if iden.value == "true" || iden.value == "false" {
                    return Some(Bool {
                        value: iden.value == "true",
                    }.into_node(tok.loc.clone()));
                }
            },
            _ => {},
        }
    }

    None
}*/