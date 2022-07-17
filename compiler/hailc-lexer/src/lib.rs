//! The Hail lexer, powered by `logos`.

mod raw;

use raw::RawTok;

use hailc_ctx::Ctx;
use hailc_loc::Loc;
use hailc_diag::{driver::DiagDriver, DiagBuilder};
use logos::{Lexer, Logos};

/// A boolean token.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bool<'a> {
    /// The location of the token.
    pub loc: Loc<'a>,

    /// The raw value of the boolean.
    pub value: &'a str,
}

/// A punctuator token.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Punct<'a> {
    /// The location of the token.
    pub loc: Loc<'a>,

    /// The value of the punctuator.
    pub value: &'a str,

    /// Whether or not the punctuator was automatically inserted by the lexer.
    pub inserted: bool,
}

/// A float token.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Float<'a> {
    /// The location of the token.
    pub loc: Loc<'a>,

    /// The raw value of the float.
    pub value: &'a str,
}

/// The syntax used to declare an integer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntKind {
    Plain,
    Hex,
    Bin,
}

/// An integer token.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Int<'a> {
    /// The location of the token.
    pub loc: Loc<'a>,

    /// The syntax used for the token.
    pub kind: IntKind,

    /// The raw value of the int.
    pub value: &'a str,
}

/// An identifier token.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Id<'a> {
    /// Teh location of the token.
    pub loc: Loc<'a>,

    /// The kind of this token.
    pub value: &'a str,
}

/// A string token.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Str<'a> {
    /// The location of the token.
    pub loc: Loc<'a>,

    /// The value of the string.
    pub value: &'a str,
}

/// The kind of a group.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GroupKind {
    /// `()`
    Paren,

    /// `[]`
    Square,

    /// `{}`
    Curly,
}

/// A group token.
#[derive(Clone, Debug, PartialEq)]
pub struct Group<'a> {
    /// The location of the group.
    pub loc: Loc<'a>,

    /// The kind of a group.
    pub kind: GroupKind,

    /// The tokens in the group.
    pub tokens: Vec<Tok<'a>>,
}

/// A token in the lexer.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok<'a> {
    /// A boolean token.
    Bool(Bool<'a>),

    /// A punctuator token.
    Punct(Punct<'a>),

    /// A float token.
    Float(Float<'a>),

    /// An integer token.
    Int(Int<'a>),

    /// An identifier token.
    Id(Id<'a>),

    /// A string token.
    Str(Str<'a>),

    /// A group token.
    Group(Group<'a>),
}

/// An automatic semicolon inserter, which wraps a lexer written with `logos`.
pub struct Asi<'a, Driver: DiagDriver> {
    /// The lexer this automatic semicolon inserter wraps.
    lexer: Lexer<'a, RawTok>,

    /// The context for the lexer.
    ctx: Ctx<'a, Driver>,

    /// If we are currently tokenizing a group (`()` or `[]`).
    /// 
    /// Blocks (`{}`) don't count as token groups.
    is_group: bool,

    /// If we are currently tokenizing a block.
    is_block: bool,

    /// Whether or not it is possible to insert a semicolon after the last token.
    /// 
    /// `can_insert` is true if we can end the statement at the last token, such as an identifier or number.
    /// 
    /// ```hail
    /// let my_variable = 0   // can end here, insert semicolon
    /// let my_variable = 1 + // can't end here, don't insert semicolon
    ///     2                 // can end here, insert semicolon
    /// ```
    can_insert: bool,

    /// Whether or not the next token should be replaced with a semicolon.
    insert: bool,
}

impl<'a, Driver: DiagDriver> Asi<'a, Driver> {
    /// Creates a new semicolon inserter.
    pub fn new(file: &'a str, ctx: Ctx<'a, Driver>) -> Self {
        Self { lexer: RawTok::lexer(file), ctx, is_group: false, is_block: false, can_insert: false, insert: false }
    }

    /// Skips any skippable tokens.
    fn skip_tokens(&mut self) {
        loop {
            if let Some(tok) = self.lexer.clone().next() {
                if tok == RawTok::LineBreak {
                    if self.can_insert && !self.is_group && self.is_block {
                        self.insert = true;
                        return;
                    }

                    self.lexer.next();

                    let mut peek = self.lexer.clone();

                    continue;
                }

                return;
            }

            break;
        }
    }

    /// Tokenizes a single group of tokens.
    fn lex_group(&mut self, close: &str) -> Result<Vec<Tok<'a>>, DiagBuilder<'a, Driver>> {
        let mut tokens = vec![];
        let start = self.lexer.span().start;
        let old_is_group = self.is_group;
        self.can_insert = false;

        if close != "}" { self.is_group = true } else { self.is_block = true }

        loop {
            self.skip_tokens();
            let mut peek = self.lexer.clone();
            if let Some(_) = peek.next() {
                if peek.slice() == close {
                    self.lexer.next();
                    break;
                }

                if let Some(tok) = self.next_token()? {
                    tokens.push(tok);
                }
            } else {
                let source = self.ctx.source();
                let builder = self.ctx.builder();

                let diag = builder.new_err()
                    .with_code("E0001")
                    .with_highlight(Loc::from_usize_range(self.lexer.span(), source))
                    .with_msg("group never ends");
                
                let diag1 = builder.new_help()
                    .with_highlight(Loc::from_usize_range(start..start + 1, source))
                    .with_note("i found that the group starts here, but never ends.");
                
                builder.throw(diag);
                builder.throw(diag1);
                return Err(self.ctx.builder().clone())
            }
        }

        self.is_group = old_is_group;
        self.can_insert = true;

        Ok(tokens)
    }

    /// Finds the next token in the lexer.
    fn next_token(&mut self) -> Result<Option<Tok<'a>>, DiagBuilder<'a, Driver>> {
        self.skip_tokens();

        if self.insert {
            self.insert = false;
            self.can_insert = false;
            self.lexer.next();

            return Ok(Some(Tok::Punct(Punct {
                loc: Loc::from_usize_range(self.lexer.span(), self.ctx.source()),
                value: ";",
                inserted: true,
            })));
        }

        if let Some(tok) = self.lexer.next() {
            match tok {
                RawTok::Float => {
                    self.can_insert = true;
                    return Ok(Some(Tok::Float(Float {
                        loc: Loc::from_usize_range(self.lexer.span(), self.ctx.source()),
                        value: self.lexer.slice(),
                    })))
                },
                RawTok::Int => {
                    self.can_insert = true;
                    return Ok(Some(Tok::Int(Int {
                        loc: Loc::from_usize_range(self.lexer.span(), self.ctx.source()),
                        kind: IntKind::Plain,
                        value: self.lexer.slice()
                    })))
                },
                RawTok::HexInt => {
                    self.can_insert = true;
                    return Ok(Some(Tok::Int(Int {
                        loc: Loc::from_usize_range(self.lexer.span(), self.ctx.source()),
                        kind: IntKind::Hex,
                        value: self.lexer.slice()
                    })))
                },
                RawTok::BinInt => {
                    self.can_insert = true;
                    return Ok(Some(Tok::Int(Int {
                        loc: Loc::from_usize_range(self.lexer.span(), self.ctx.source()),
                        kind: IntKind::Bin,
                        value: self.lexer.slice(),
                    })))
                },
                RawTok::Id => {
                    self.can_insert = true;
                    let slice = self.lexer.slice();

                    return match self.lexer.slice() {
                        "true" => Ok(Some(Tok::Bool(Bool {
                            loc: Loc::from_usize_range(self.lexer.span(), self.ctx.source()),
                            value: slice,
                        }))),
                        "false" => Ok(Some(Tok::Bool(Bool {
                            loc: Loc::from_usize_range(self.lexer.span(), self.ctx.source()),
                            value: slice,
                        }))),
                        _ => Ok(Some(Tok::Id(Id {
                            loc: Loc::from_usize_range(self.lexer.span(), self.ctx.source()),
                            value: slice,
                        }))),
                    };
                },
                RawTok::Str => {
                    self.can_insert = true;
                    return Ok(Some(Tok::Str(Str {
                        loc: Loc::from_usize_range(self.lexer.span(), self.ctx.source()),
                        value: self.lexer.slice(),
                    })))
                },
                RawTok::Punct => {
                    let slice = self.lexer.slice();

                    match slice {
                        "[" => {
                            let start = self.lexer.span();
                            let tokens = self.lex_group("]")?;
                            self.can_insert = true;
                            return Ok(Some(Tok::Group(Group {
                                loc: Loc::from_usize_range(start.start..self.lexer.span().end, self.ctx.source()),
                                kind: GroupKind::Square,
                                tokens,
                            })))
                        },
                        "{" => {
                            let start = self.lexer.span();
                            let tokens = self.lex_group("}")?;
                            self.can_insert = true;
                            return Ok(Some(Tok::Group(Group {
                                loc: Loc::from_usize_range(start.start..self.lexer.span().end, self.ctx.source()),
                                kind: GroupKind::Curly,
                                tokens,
                            })))
                        },
                        "(" => {
                            let start = self.lexer.span();
                            let tokens = self.lex_group(")")?;
                            self.can_insert = true;
                            return Ok(Some(Tok::Group(Group {
                                loc: Loc::from_usize_range(start.start..self.lexer.span().end, self.ctx.source()),
                                kind: GroupKind::Paren,
                                tokens,
                            })))
                        },
                        "]" | "}" | ")" => {
                            let source = self.ctx.source();
                            let builder = self.ctx.builder();
                            
                            let diag = builder.new_err()
                                .with_code("E0003")
                                .with_highlight(Loc::from_usize_range(self.lexer.span(), source))
                                .with_msg("token group never opened")
                                .with_note("']', '}' and ')' are used for closing groups (that start with '[', '{' and '(' respectively).")
                                .with_note("those characters assume that a group has already been opened, but one hasn't.");
                            
                            builder.throw(diag);

                            return Err(self.ctx.builder().clone());
                        },
                        "?" => {
                            self.can_insert = true;
                        },
                        _ => {
                            self.can_insert = false;
                        },
                    }

                    return Ok(Some(Tok::Punct(Punct {
                        loc: Loc::from_usize_range(self.lexer.span(), self.ctx.source()),
                        value: self.lexer.slice(),
                        inserted: false,
                    })))
                },
                _ => {
                    let source = self.ctx.source();
                    let builder = self.ctx.builder();
                    
                    let diag = builder.new_err()
                        .with_code("E0002")
                        .with_highlight(Loc::from_usize_range(self.lexer.span(), source))
                        .with_msg("invalid token")
                        .with_note("i don't know how to process this! perhaps it's in a different language?");

                    builder.throw(diag);

                    return Err(self.ctx.builder().clone())
                }
            }
        }

        Ok(None)
    }

    /// Returns all tokens in the source string.
    pub fn lex(&mut self) -> Result<Vec<Tok<'a>>, DiagBuilder<'a, Driver>> {
        let mut toks = vec![];

        while let Some(tok) = self.next_token()? {
            toks.push(tok);
        }

        Ok(toks)
    }
}