//! A hand-written parser for Hail source.

pub mod stream;

use stream::TokenStream;

use hailc_ast::AstUnit;
use hailc_ctx::Ctx;
use hailc_diag::driver::DiagDriver;
use hailc_lexer::Tok;

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

    /// Parses the entire source file, from the roots.
    /// 
    /// 
    pub fn parse(&mut self, toks: Vec<Tok>) -> Result<AstUnit, ()> {
        let _stream = TokenStream::from_vec(toks);

        let unit = AstUnit::new();

        Ok(unit)
    }
}