use crate::{Diag};

use hailc_loc::files::FileRegistry;

/// A driver for emitting diagnostics to the terminal.
pub trait DiagDriver {
    /// Emit the diagnostic provided to the terminal.
    fn emit(&mut self, diag: &Diag, files: &FileRegistry);
}