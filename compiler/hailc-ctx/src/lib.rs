use hailc_diag::DiagBuilder;
use hailc_loc::{Idx, files::File};

/// A context for compiling Hail source.
pub struct Ctx<'a> {
    /// The diagnostic builder this context wraps.
    builder: &'a mut DiagBuilder<'a>,

    /// The source file that the context wraps.
    source: Idx<File<'a>>,
}

impl<'a> Ctx<'a> {
    /// Creates a new context.
    pub fn new(builder: &'a mut DiagBuilder<'a>, source: Idx<File<'a>>) -> Self {
        Self { builder, source }
    }

    /// Returns the diagnostic builder this context wraps.
    pub fn builder(&mut self) -> &mut DiagBuilder<'a> {
        self.builder
    }

    /// Returns the source file of the context.
    pub fn source(&self) -> Idx<File<'a>> {
        self.source
    }
}