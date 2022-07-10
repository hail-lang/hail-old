//! The Hail compiler interface.
//! 
//! This crate provides an easy interface to access the Hail compiler, without calling a command line application.

use hailc_ctx::Ctx;
use hailc_diag::DiagBuilder;
use hailc_diag::driver::DiagDriver;
use hailc_lexer::{Tok, Asi};
use hailc_loc::Idx;
use hailc_loc::files::FileRegistry;

/// An easy-to-use interface for accessing the Hail compiler.
/// 
/// 
pub struct Compiler<'a, Driver: DiagDriver> {
    /// The files in the compiler.
    files: FileRegistry<'a>,

    /// The diagnostic driver of the compiler.
    driver: Driver,
}

impl<'a, Driver: DiagDriver> Compiler<'a, Driver> {
    /// Creates a new compiler for the given file.
    pub fn for_file(name: &'a str, source: &'a str, driver: Driver) -> Self {
        let mut files = FileRegistry::new();
        files.register_file(name, source);

        Self { files, driver }
    }

    /// Tokenizes the source string of the compiler.
    pub fn lex(&'a mut self) -> Result<Vec<Tok>, DiagBuilder<'a, Driver>> {
        let ctx = Ctx::new(Idx::from_u32(0), self.driver.clone());
        let mut lexer = Asi::new(self.files.get_file_source(Idx::from_u32(0)), ctx);
        lexer.lex()
    }
}