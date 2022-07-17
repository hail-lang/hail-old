//! A hand-written parser for Hail source.

pub mod stream;

use stream::TokenStream;

use hailc_ast::*;
use hailc_ctx::Ctx;
use hailc_diag::{driver::DiagDriver, DiagBuilder};
use hailc_lexer::{Tok, Id};

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
    fn check_successful(&mut self) -> Result<(), ()> {
        if self.ctx.builder().err() { Err(()) } else { Ok(()) }
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

    /// Parses a single primary expression.
    fn parse_primary(&mut self, stream: &mut TokenStream<'a>) -> Option<Value<'a>> {
        if let Some(bool) = self.parse_bool(stream) {
            Some(Value::Bool(bool))
        } else if let Some(name) = self.parse_name(stream) {
            Some(Value::Name(name))
        } else {
            None
        }
    }

    /// Parses the entire source file, from the roots.
    /// 
    /// 
    pub fn parse(&mut self, toks: Vec<Tok<'a>>) -> Result<AstUnit, DiagBuilder<'a, Driver>> {
        let mut stream = TokenStream::from_vec(toks);

        let unit = AstUnit::new();

        while let Some(primary) = self.parse_primary(&mut stream) {
            dbg!(primary);
        }

        Ok(unit)
    }
}