use crate::{Diag, ErrLevel, driver::DiagDriver};

use hailc_loc::{Idx, Source};

/// The name and source of a file.
type File<'a> = (&'a str, &'a str);

/// A [`FileRegistry`], which keeps track of files being compiled in a workspace.
pub struct FileRegistry<'a> {
    /// The file names in this [`FileRegistry`].
    private: Vec<File<'a>>,
}

impl<'a> FileRegistry<'a> {
    /// Creates an empty [`FileRegistry`].
    pub fn new() -> Self {
        Self { private: Vec::new() }
    }

    /// Returns a free index for the next registered file.
    pub fn next_idx(&self) -> Idx<Source> {
        Idx::from_usize(self.private.len())
    }

    /// Registers a file in the [`FileRegistry`].
    /// 
    /// Once a file is registered, it cannot be modified.
    pub fn register_file(&mut self, name: &'a str, source: &'a str) -> Idx<Source> {
        let file = (name, source);
        let idx = self.next_idx();
        self.private.push(file);
        idx
    }

    /// Returns the file path of the provided source file.
    pub fn get_file_path(&self, source: Idx<Source>) -> &'a str {
        self.private[source.as_usize()].0
    }

    /// Returns the raw source of the provided source file.
    pub fn get_file_source(&self, source: Idx<Source>) -> &'a str {
        self.private[source.as_usize()].1
    }
}

/// A builder for diagnostics.
pub struct DiagBuilder<'a> {
    /// The diagnostics thrown to this [`DiagBuilder`].
    diags: Vec<Diag<'a>>,

    /// The files used by the compiler.
    files: FileRegistry<'a>,

    /// The driver that emits diagnostics.
    driver: Box<dyn DiagDriver>,

    /// Whether or not any diagnostics with an error or higher error level have been thrown.
    err: bool,
}

impl<'a> DiagBuilder<'a> {
    /// Creates a new, empty diagnostic builder.
    pub fn new<Driver: 'static + DiagDriver>(driver: Driver) -> Self {
        Self { diags: Vec::new(), files: FileRegistry::new(), driver: Box::new(driver), err: false }
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
    pub fn emit(&mut self) {
        for diag in &self.diags {
            self.driver.emit(diag, &self.files)
        }
    }
}