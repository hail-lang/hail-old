use hailc_diag::DiagBuilder;
use hailc_diag::driver::DiagDriver;
use hailc_loc::{Idx, files::File};

/// A context for compiling Hail source.
pub struct Ctx<'a, Driver: DiagDriver> {
    /// The diagnostic builder this context wraps.
    builder: DiagBuilder<'a, Driver>,

    /// The source file that the context wraps.
    source: Idx<File<'a>>,
}

impl<'a, Driver: DiagDriver> Ctx<'a, Driver> {
    /// Creates a new context.
    pub fn new(source: Idx<File<'a>>, driver: Driver) -> Self {
        Self { builder: DiagBuilder::new(driver), source }
    }

    /// Returns the diagnostic builder this context wraps.
    pub fn builder(&mut self) -> &mut DiagBuilder<'a, Driver> {
        &mut self.builder
    }

    pub fn get_builder(self) -> DiagBuilder<'a, Driver> {
        self.builder
    }

    /// Returns the source file of the context.
    pub fn source(&self) -> Idx<File<'a>> {
        self.source
    }
}