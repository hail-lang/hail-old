//! Automatic semicolon insertion for Hail.

mod raw {
    pub use crate::raw_lexer::Lexer;
    pub use crate::raw_token::*;
}

use std::vec::IntoIter;
use std::{ops::Range};

use crate::token::*;

use hail_diagnostic::Diag;
use hail_shared::iter::{IntoBetterIterator};

/// Converts an object into an [`Asi`].
pub trait Tokenize<'a> {
    /// Tokenizes this object.
    /// 
    /// Converts this object into a lexer for Hail source.
    fn tokenize(self) -> Result<Asi<'a>, Diag>;
}

impl<'a, IntoStr: Into<&'a str>> Tokenize<'a> for IntoStr {
    fn tokenize(self) -> Result<Asi<'a>, Diag> {
        Asi::from_source(self.into())
    }
}

/// A context for semicolon insertion.
pub struct AsiCtx<'a> {
    /// The lexer that this semicolon inserter wraps.
    pub tokens: IntoIter<raw::TokNode<'a>>,

    /// The location to insert a semicolon at.
    pub insert_semicolon: Option<Range<usize>>,

    /// Whether or not the last token can possibly end the expression.
    pub last_tok_endable: bool,

    /// The comments before the next token.
    pub comments: Vec<Comment<'a>>,
}

impl<'a> AsiCtx<'a> {
    pub fn new(tokens: Vec<raw::TokNode<'a>>) -> Self {
        Self {
            tokens: tokens.into_iter(),
            insert_semicolon: None,
            last_tok_endable: false,
            comments: vec![],
        }
    }
}

/// Automatic semicolon inserter.
/// 
/// This struct filters out all unnecessary tokens (whitespace, comments, etc.), and inserts semicolons in the appropriate locations.
pub struct Asi<'a> {
    ctx: AsiCtx<'a>,
}

impl<'a> Asi<'a> {
    /// Creates a lexer from the provided source string.
    pub fn from_source(source: &'a str) -> Result<Self, Diag> {
        let mut tokens = vec![];

        let lexer = raw::Lexer::new(source).better_iter();
        for token in lexer {
            tokens.push(token?);
        }

        Ok(Self {
            ctx: AsiCtx::new(tokens),
        })
    }

    /// Inserts a semicolon instead of the next token, with the provided location.
    fn insert_semicolon(ctx: &mut AsiCtx<'a>, loc: Range<usize>) {
        ctx.insert_semicolon = Some(loc);
    }

    /// Whether or not a semicolon can be inserted safely.
    fn can_insert(ctx: &AsiCtx<'a>) -> bool {
        ctx.last_tok_endable
    }

    /// Skips a single skippable token in the lexer, if any.
    fn skip_tokens(ctx: &mut AsiCtx<'a>) -> Result<Space, Diag> {
        let mut spacing = Space::None;

        while let Some(token) = ctx.tokens.clone().next() {
            match &token.node {
                raw::Tok::Comment(raw_com) => {
                    if raw_com.commentary {
                        // The comment was a doc comment.
                        ctx.tokens.next();
                        let value = raw_com.value;
                        ctx.comments.push(Comment {
                            value,
                        });
                    }
                },
                raw::Tok::Whitespace() => {
                    ctx.tokens.next();
                    if spacing == Space::None {
                        spacing = Space::Whitespace;
                    }
                },
                raw::Tok::LineBreak() => {
                    // Whether or not the lexer should insert a semicolon.
                    if Self::can_insert(ctx) {
                        Self::insert_semicolon(ctx, token.loc.clone());
                        break;
                    }

                    ctx.tokens.next();
                    spacing = Space::LineBreak;
                },
                _ => break,
            }
        }

        Ok(spacing)
    }

    /// Converts a token from a raw token to a parser token.
    fn convert_token(token: raw::TokNode<'a>, ctx: &mut AsiCtx<'a>) -> Result<Tok<'a>, Diag> {
        match token.node {
            raw::Tok::Punct(raw) => {
                let kind = match raw.char {
                    '~' => PunctKind::Tilde,
                    '!' => PunctKind::Bang,
                    '@' => PunctKind::At,
                    '#' => PunctKind::Hash,
                    '%' => PunctKind::Perc,
                    '^' => PunctKind::Caret,
                    '&' => PunctKind::Amp,
                    '*' => PunctKind::Star,
                    '-' => PunctKind::Dash,
                    '=' => PunctKind::Eq,
                    '+' => PunctKind::Plus,
                    '|' => PunctKind::Pipe,
                    ';' => PunctKind::Semi(false),
                    ':' => PunctKind::Colon,
                    '/' => PunctKind::Slash,
                    '.' => PunctKind::Dot,
                    ',' => PunctKind::Comma,
                    '>' => PunctKind::Gt,
                    '<' => PunctKind::Lt,
                    '?' => PunctKind::Quest,
                    _ => unreachable!(),
                };
                
                ctx.last_tok_endable = kind == PunctKind::Quest;
                let spacing = Self::skip_tokens(ctx)?;

                Ok(Tok::Punct(Punct {
                    loc: token.loc,
                    spacing,
                    kind,
                }))
            },
            raw::Tok::Num(num) => {
                let kind = match num.kind {
                    raw::NumKind::Int => NumKind::Int,
                    raw::NumKind::Hex => NumKind::Hex,
                    raw::NumKind::Bin => NumKind::Bin,
                    raw::NumKind::Float => NumKind::Float,
                };

                ctx.last_tok_endable = true;
                let spacing = Self::skip_tokens(ctx)?;

                Ok(Tok::Num(Num {
                    loc: token.loc,
                    spacing,
                    kind,
                    value: num.value,
                }))
            },
            raw::Tok::Iden(iden) => {
                
                ctx.last_tok_endable = true;

                let spacing = Self::skip_tokens(ctx)?;

                Ok(Tok::Iden(Iden {
                    loc: token.loc,
                    spacing,
                    value: iden.value,
                }))
            },
            raw::Tok::Str(str) => {
                let node = Tok::Str(Str {
                    value: str.value,
                });
                ctx.last_tok = Some(node.clone());

                let spacing = Self::skip_tokens(ctx)?;

                Ok(TokNode {
                    loc: token.loc,
                    node,
                    spacing,
                })
            },
            raw::Tok::Group(group) => {
                //let group_tokens = group.tokens;

                let len = group.tokens.len();
                let mut group_ctx = AsiCtx::new(group.tokens);
                let mut tokens = vec![];
                

                for _ in 0..len {
                    if let Some(loc) = group_ctx.insert_semicolon {
                        group_ctx.insert_semicolon = None;
                        group_ctx.tokens.next();
            
                        let spacing = Self::skip_tokens(&mut group_ctx)?;
                        
                        tokens.push(TokNode {
                            loc,
                            node: Tok::Punct(Punct::Semi(true)),
                            spacing,
                        });

                        continue;
                    }


                    Self::skip_tokens(&mut group_ctx)?;
                    if let Some(tok) = group_ctx.tokens.next() {
                        let node = Self::convert_token(tok.clone(), &mut group_ctx)?;
                        ctx.last_tok = Some(node.node.clone());
                        tokens.push(node);
                    }
                }

                let node = Tok::Group(Group {
                    tokens,
                });
                
                ctx.last_tok = Some(node.clone());
                let spacing = Self::skip_tokens(ctx)?;

                Ok(TokNode {
                    loc: token.loc,
                    node,
                    spacing,
                })
            }
            _ => {
                unreachable!()
            },
        }
    }

    pub fn collect_tokens(&mut self) -> Result<Vec<TokNode>, Diag> {
        let mut tokens = vec![];

        while let Some(tok) = self.next() {
            tokens.push(tok?);
        }

        Ok(tokens)
    }
}

impl<'a> Iterator for Asi<'a> {
    type Item = Result<TokNode<'a>, Diag>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(loc) = self.ctx.insert_semicolon.clone() {
            self.ctx.insert_semicolon = None;
            self.ctx.tokens.next();

            let spacing = match Self::skip_tokens(&mut self.ctx) {
                Err(e) => return Some(Err(e)),
                Ok(space) => space,
            };
            return Some(Ok(TokNode {
                loc,
                node: Tok::Punct(Punct::Semi(true)),
                spacing,
            }));
        }
        
        match Self::skip_tokens(&mut self.ctx) {
            Err(e) => return Some(Err(e)),
            _ => {},
        };

        let token = self.ctx.tokens.next()?;

        let converted = Self::convert_token(token, &mut self.ctx);
        match converted {
            Ok(tok) => Some(Ok(tok)),
            Err(e) => Some(Err(e)),
        }
    }
}