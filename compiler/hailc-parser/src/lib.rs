//! A hand-written parser for Hail source.

pub mod stream;

use std::num::IntErrorKind;

use stream::TokenStream;

use hailc_ast::*;
use hailc_ctx::Ctx;
use hailc_diag::{driver::DiagDriver, DiagBuilder};
use hailc_lexer::{Tok, IntKind};
use hailc_loc::Loc;
use snailquote::{unescape, UnescapeError};

/// The parser.
pub struct Parser<'a, Driver: DiagDriver> {
    /// The context of the parser, for emitting diagnostics.
    ctx: Ctx<'a, Driver>,
}

impl<'a, Driver: DiagDriver> Parser<'a, Driver> {
    /// Creates a new parser for the provided context.
    pub fn new(ctx: Ctx<'a, Driver>) -> Self {
        Self { ctx }
    }

    /// Checks if an error diagnostic has been thrown.
    /// 
    /// This is used to easily cancel the parsing process.
    ///
    /// TODO: remove this function if it's actually not useful.
    fn check_successful(&mut self) -> Result<(), DiagBuilder<'a, Driver>> {
        if self.ctx.builder().err() {
            println!("ERROR!");
            Err(self.ctx.builder().clone())
        } else {
            Ok(())
        }
    }

    /// Checks if an error diagnostic has been thrown.
    /// 
    /// This is used to easily cancel the parsing process.
    ///
    /// TODO: remove this function if it's actually not useful.
    fn check_successful_opt(&mut self) -> Option<()> {
        if self.ctx.builder().err() { None } else { Some(()) }
    }

    /// Parses a boolean value.
    fn parse_bool(&mut self, stream: &mut TokenStream<'a>) -> Option<value::Bool<'a>> {
        if let Some(tok) = stream.peek() {
            match tok {
                Tok::Bool(id) => {
                    stream.next();
                    Some(value::Bool {
                        loc: id.loc,
                        value: id.value == "true",
                    })
                },
                _ => None
            }
        } else {
            None
        }
    }

    /// Parses a name expression.
    #[inline]
    fn parse_name(&mut self, stream: &mut TokenStream<'a>) -> Option<Name<'a>> {
        if let Some(tok) = stream.peek() {
            match tok {
                Tok::Id(id) => {
                    stream.next();
                    Some(Name {
                        loc: id.loc,
                        value: id.value,
                    })
                },
                _ => None
            }
        } else {
            None
        }
    }

    /// Parses an integer value.
    #[inline]
    fn parse_int(&mut self, stream: &mut TokenStream<'a>) -> Option<value::Int<'a>> {
        if let Some(tok) = stream.peek() {
            match tok {
                Tok::Int(int) => {
                    stream.next();
                    Some(value::Int {
                        loc: int.loc,
                        kind: match int.kind {
                            IntKind::Plain => value::IntKind::Plain,
                            IntKind::Hex => value::IntKind::Hex,
                            IntKind::Bin => value::IntKind::Bin,
                        },
                        value: 
                            match int.kind {
                                IntKind::Plain => match int.value.parse() {
                                    Ok(val) => val,
                                    Err(e) => match e.kind() {
                                        IntErrorKind::PosOverflow => {
                                            let diag = self.ctx.builder().new_err()
                                                .with_code("E0004")
                                                .with_highlight(int.loc)
                                                .with_msg("integer is too large")
                                                .with_note("integers have a maximum size of 18,446,744,073,709,551,615");
                                            
                                            self.ctx.builder().throw(diag);
                                            return None;
                                        },
                                        IntErrorKind::NegOverflow => {
                                            let diag = self.ctx.builder().new_err()
                                                .with_code("E0005")
                                                .with_highlight(int.loc)
                                                .with_msg("integer is too small")
                                                .with_note("integers have a minimum size of -9,223,372,036,854,775,808");
                                            
                                            self.ctx.builder().throw(diag);
                                            return None;
                                        },
                                        _ => unreachable!(),
                                    },
                                },
                                IntKind::Hex => match i64::from_str_radix(&int.value[2..], 16) {
                                    Ok(val) => val,
                                    Err(e) => match e.kind() {
                                        IntErrorKind::PosOverflow => {
                                            let diag = self.ctx.builder().new_err()
                                                .with_code("E0004")
                                                .with_highlight(int.loc)
                                                .with_msg("integer is too large")
                                                .with_note("integers have a maximum size of 18,446,744,073,709,551,615");
                                            
                                            self.ctx.builder().throw(diag);
                                            return None;
                                        },
                                        IntErrorKind::NegOverflow => {
                                            let diag = self.ctx.builder().new_bug()
                                                .with_code("E0006")
                                                .with_highlight(int.loc)
                                                .with_msg("hexadecimal integer is too small")
                                                .with_note("hexadecimal integers have a minimum size of 0")
                                                .with_note("this should be unreachable; as hexadecimal integers cannot be negative.");
                                            
                                            self.ctx.builder().throw(diag);
                                            return None;
                                        },
                                        _ => unreachable!(),
                                    },
                                },
                                IntKind::Bin => match i64::from_str_radix(&int.value[2..], 2) {
                                    Ok(val) => val,
                                    Err(e) => match e.kind() {
                                        IntErrorKind::PosOverflow => {
                                            let diag = self.ctx.builder().new_err()
                                                .with_code("E0004")
                                                .with_highlight(int.loc)
                                                .with_msg("integer is too large")
                                                .with_note("integers have a maximum size of 18,446,744,073,709,551,615");
                                            
                                            self.ctx.builder().throw(diag);
                                            return None;
                                        },
                                        IntErrorKind::NegOverflow => {
                                            let diag = self.ctx.builder().new_bug()
                                                .with_code("E0007")
                                                .with_highlight(int.loc)
                                                .with_msg("binary integer is too small")
                                                .with_note("binary integers have a minimum size of 0")
                                                .with_note("this should be unreachable; as binary integers cannot be negative.");
                                            
                                            self.ctx.builder().throw(diag);
                                            return None;
                                        },
                                        _ => unreachable!(),
                                    },
                                },
                            }
                        },
                    )
                },
                _ => None
            }
        } else {
            None
        }
    }

    /// Parses an integer value.
    #[inline]
    fn parse_float(&mut self, stream: &mut TokenStream<'a>) -> Option<value::Float<'a>> {
        if let Some(tok) = stream.peek() {
            match tok {
                Tok::Float(float) => {
                    stream.next();
                    Some(value::Float {
                        loc: float.loc,
                        value: match float.value.parse() {
                            Ok(val) => val,
                            Err(e) => {
                                let diag = self.ctx.builder().new_bug()
                                    .with_code("E0008")
                                    .with_highlight(float.loc)
                                    .with_msg("invalid float")
                                    .with_note("this should (??) be unreachable");
                                            
                                self.ctx.builder().throw(diag);
                                return None;
                            },
                        },
                    })
                },
                _ => None
            }
        } else {
            None
        }
    }

    /// Parses a single string token, if any.
    fn parse_str(&mut self, stream: &mut TokenStream<'a>) -> Option<value::Str<'a>> {
        if let Some(tok) = stream.peek() {
            if let Tok::Str(str) = tok {
                stream.next();
                match unescape(str.value) {
                    Ok(value) => Some(value::Str {
                        loc: str.loc,
                        value: value,
                    }),
                    Err(err) => {
                        // let idx = match err {
                        //     UnescapeError::InvalidEscape { index, .. } => index,
                        //     UnescapeError::InvalidUnicode { index, .. } => index,
                        // };

                        //let start = str.loc.start();
                        //let range = (start + idx as u32)..(start + idx as u32);

                        // TODO: use `err` to make better diagnostics

                        let diag = self.ctx.builder().new_err()
                            .with_code("E0009")
                            .with_highlight(str.loc)
                            //.with_highlight(Loc::from_u32_range(range, str.loc.source()))
                            .with_msg("this is an invalid string.");
                        
                        self.ctx.builder().throw(diag);
                        
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Parses a single primary expression.
    fn parse_primary(&mut self, stream: &mut TokenStream<'a>) -> Option<Value<'a>> {
        if let Some(bool) = self.parse_bool(stream) {
            return Some(Value::Bool(bool))
        } else if let Some(name) = self.parse_name(stream) {
            return Some(Value::Name(name))
        } else if let Some(int) = self.parse_int(stream) {
            return Some(Value::Int(int))
        } else if let Some(float) = self.parse_float(stream) {
            return Some(Value::Float(float))
        } else if let Some(str) = self.parse_str(stream) {
            return Some(Value::Str(str))
        }

        None
    }

    /// Parses the entire source file, from the roots.
    pub fn parse(&mut self, toks: Vec<Tok<'a>>) -> Result<AstUnit, DiagBuilder<'a, Driver>> {
        let mut stream = TokenStream::from_vec(toks);

        let unit = AstUnit::new();

        while let Some(primary) = self.parse_primary(&mut stream) {
            dbg!(primary);
        }

        self.check_successful()?;

        Ok(unit)
    }
}