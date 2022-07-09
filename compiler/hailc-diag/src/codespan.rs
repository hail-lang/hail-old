use crate::{driver::DiagDriver, ErrLevel};

use codespan_reporting::{diagnostic::{Diagnostic, Label, Severity}, files::SimpleFile};
pub use codespan_reporting::term::{Chars, ColorArg, Config, DisplayStyle, Styles};
pub use codespan_reporting::term::termcolor::{self, Color, ColorChoice, ColorSpec};
use hailc_loc::files::FileRegistry;
use termcolor::StandardStream;

/// A driver for emitting diagnostics with `codespan-reporting`.
#[derive(Clone, Debug)]
pub struct CodespanDriver {
    /// The config for codespan.
    config: Config,

    /// Whether or not to emit diagnostics using colors.
    choice: ColorChoice,
}

impl CodespanDriver {
    /// Creates a codespan driver from the provided configuration.
    pub fn from_config(config: Config, choice: ColorChoice) -> Self {
        Self { config, choice }
    }
}

impl Default for CodespanDriver {
    fn default() -> Self {
        Self { config: Config::default(), choice: ColorChoice::Auto }
    }
}

impl DiagDriver for CodespanDriver {
    fn emit(&mut self, diag: &crate::Diag, files: &FileRegistry) {
        let mut codespan_diag: Diagnostic<()> = Diagnostic::new(match diag.level() {
            ErrLevel::Bug => Severity::Bug,
            ErrLevel::Err => Severity::Error,
            ErrLevel::Warn => Severity::Warning,
            ErrLevel::Help => Severity::Help,
            ErrLevel::Note => Severity::Note,
        });

        if let Some(code) = diag.code() {
            codespan_diag.code = Some(code.into());
        }

        if let Some(msg) = diag.msg() {
            codespan_diag.message = msg.into();
        }

        let files = if let Some(highlight) = diag.highlight() {
            let label = Label::primary((), highlight.start() as usize..highlight.end() as usize);
            codespan_diag.labels.push(label);
            let source = highlight.source();
            SimpleFile::new(files.get_file_path(source), files.get_file_source(source))
        } else {
            SimpleFile::new("", "")
        };

        for note in diag.notes() {
            codespan_diag.notes.push(note.to_string());
        }

        let mut output = match diag.level() {
            ErrLevel::Bug | ErrLevel::Err => StandardStream::stderr(self.choice),
            _ => StandardStream::stdout(self.choice),
        };

        codespan_reporting::term::emit(&mut output, &self.config, &files, &codespan_diag).unwrap();
    }
}