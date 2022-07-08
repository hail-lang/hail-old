use crate::{Diag, builder::FileRegistry};

/// A driver for emitting diagnostics to the terminal.
pub trait DiagDriver {
    /// Emit the diagnostic provided to the terminal.
    fn emit(&mut self, diag: &Diag, files: &FileRegistry);
}