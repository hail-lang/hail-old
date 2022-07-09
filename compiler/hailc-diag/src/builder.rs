use crate::{Diag, ErrLevel, driver::DiagDriver};

use hailc_loc::files::{FileRegistry};

/// A builder for diagnostics.
pub struct DiagBuilder<'a> {
    /// The diagnostics thrown to this [`DiagBuilder`].
    diags: Vec<Diag<'a>>,

    /// The driver that emits diagnostics.
    driver: Box<dyn DiagDriver>,

    /// Whether or not any diagnostics with an error or higher error level have been thrown.
    err: bool,
}

impl<'a> DiagBuilder<'a> {
    /// Creates a new, empty diagnostic builder.
    pub fn new<Driver: 'static + DiagDriver>(driver: Driver) -> Self {
        Self { diags: Vec::new(), driver: Box::new(driver), err: false }
    }

    /// Creates a new [`ErrLevel::Bug`] diagnostic.
    pub fn new_bug(&self) -> Diag<'a> {
        Diag::new(ErrLevel::Bug)
    }

    /// Creates a new [`ErrLevel::Err`] diagnostic.
    pub fn new_err(&self) -> Diag<'a> {
        Diag::new(ErrLevel::Err)
    }

    /// Creates a new [`ErrLevel::Warn`] diagnostic.
    pub fn new_warn(&self) -> Diag<'a> {
        Diag::new(ErrLevel::Warn)
    }

    /// Creates a new [`ErrLevel::Help`] diagnostic.
    pub fn new_help(&self) -> Diag<'a> {
        Diag::new(ErrLevel::Help)
    }

    /// Creates a new [`ErrLevel::Note`] diagnostic.
    pub fn new_note(&self) -> Diag<'a> {
        Diag::new(ErrLevel::Note)
    }

    /// Throws a diagnostic to this diagnostic emitter.
    /// 
    /// The diagnostic thrown will be emitted to the console when [`DiagBuilder::emit`] is called.
    pub fn throw(&mut self, diag: Diag<'a>) {
        match diag.level() {
            ErrLevel::Bug | ErrLevel::Err => self.err = true,
            _ => {},
        }

        self.diags.push(diag);
    }

    /// Emits the diagnostics in this diagnostic emitter to the terminal.
    pub fn emit(&mut self, files: &FileRegistry) {
        for diag in &self.diags {
            self.driver.emit(diag, files)
        }
    }
}

/// Unwraps a result, or emits the diagnostics from a diagnostic builder.
pub trait UnwrapOrEmit<T> {
    fn unwrap_or_emit(self, files: &FileRegistry) -> T;
}

impl<'a, T> UnwrapOrEmit<T> for Result<T, DiagBuilder<'a>> {
    fn unwrap_or_emit(self, files: &FileRegistry) -> T {
        match self {
            Ok(value) => value,
            Err(mut e) => {
                e.emit(files);
                std::process::exit(1);
            }
        }
    }
}