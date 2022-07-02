use std::io::{Write};
use std::ops::Range;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use textwrap::wrap;

/// The severity of a [`Highlight`] or [`Diag`].
/// 
/// Used to determine the color and prefix of a diagnostic/highlight.
#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Bug,
    Error,
    Warning,
    Help,
    Note 
}

/// A highlighted piece of information in a diagnostic.
#[derive(Debug, Clone, PartialEq)]
pub struct Highlight {
    /// The location to highlight.
    pub loc: Range<usize>,
}

/// A label in a diagnostic.
#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub highlight: Option<Highlight>,

    /// The message of this label.
    pub message: String,
}

impl Label {
    /// Creates a new label.
    pub fn new<Str: Into<String>>(message: Str) -> Self {
        Self {
            highlight: None,
            message: message.into()

        }
    }

    /// Highlights a location for this label.
    pub fn highlight(mut self, loc: Range<usize>) -> Self {
        self.highlight = Some(Highlight {
            loc,
        });
        self
    }
}

/// A diagnostic that occurred during running `hail`.
#[derive(Debug, Clone, PartialEq)]
pub struct Diag {
    /// The severity of this diagnostic.
    pub severity: Severity,

    /// The error code of this diagnostic.
    pub code: Option<String>,
    
    /// The message in this diagnostic.
    /// 
    /// Displayed after the severity and error code.
    pub message: Option<String>,

    /// The labels in this diagnostic.
    pub labels: Vec<Label>,
}

impl Diag {
    /// Creates a new diagnostic with the provided severity.
    pub fn new(severity: Severity) -> Self {
        Self {
            severity,
            code: None,
            message: None,
            labels: Vec::new(),
        }
    }

    /// Creates a new diagnostic with [`Severity::Bug`] severity.
    pub fn bug() -> Self {
        Self::new(Severity::Bug)
    }

    /// Creates a new diagnostic with [`Severity::Error`] severity.
    pub fn error() -> Self {
        Self::new(Severity::Error)
    }

    /// Creates a new diagnostic with [`Severity::Warning`] severity.
    pub fn warning() -> Self {
        Self::new(Severity::Warning)
    }

    /// Creates a new diagnostic with [`Severity::Help`] severity.
    pub fn help() -> Self {
        Self::new(Severity::Help)
    }

    /// Creates a new diagnostic with [`Severity::Note`] severity.
    pub fn note() -> Self {
        Self::new(Severity::Note)
    }

    /// Adds a code to this diagnostic.
    pub fn with_code<Str: Into<String>>(mut self, code: Str) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Adds a message to this diagnostic.
    pub fn with_message<Str: Into<String>>(mut self, message: Str) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Adds highlights to this diagnostic.
    pub fn with_labels(mut self, labels: Vec<Label>) -> Self {
        self.labels = labels;
        self
    }
}

/// Emits diagnostics to the terminal.
pub struct DiagEmitter {
    /// Whether or not to use colors while emitting diagnostic.
    no_color: bool,

    /// The name of the file.
    filename: String,

    /// The source of the file.
    source: String,
}

impl DiagEmitter {
    /// Creates a new diagnostic emitter.
    pub fn new<Str: Into<String>>(no_color: bool, filename: Str, source: Str) -> Self {
        Self {
            no_color,
            filename: filename.into(),
            source: source.into(),
        }
    }

    /// Changes the source of this emitter to the source given.
    pub fn set_source<Str: Into<String>>(&mut self, source: Str) {
        self.source = source.into();
    }

    fn get_color_choice(&self) -> ColorChoice {
        if self.no_color {
            ColorChoice::Never
        } else {
            ColorChoice::Auto
        }
    }

    /// Returns the line and column of an index in the source string.
    /// 
    /// Returns `(line, col)` in this order.
    fn line_col(&mut self, idx: usize) -> Option<(usize, usize)> {
        if idx > self.source.len() {
            return None;
        }

        let mut line = 1;
        let mut char = 1;

        let chars = self.source.chars();

        for (i, item) in chars.enumerate() {
            if i == idx {
                break;
            }

            if item == '\n' {
                line += 1;
                char = 1;
            } else {
                char += 1;
            }
        }

        Some((line, char))
    }
    
    pub fn emit(&mut self, diagnostic: Diag) -> std::io::Result<()> {
        let mut out = match diagnostic.severity {
            Severity::Bug | Severity::Error => StandardStream::stderr(self.get_color_choice()),
            _ => StandardStream::stdout(self.get_color_choice()),
        };

        let mut blue = ColorSpec::new();
        blue.set_fg(Some(Color::Blue));
        blue.set_bold(true);
        blue.set_intense(true);

        let mut cyan = ColorSpec::new();
        cyan.set_fg(Some(Color::Cyan));
        cyan.set_bold(true);
        cyan.set_intense(false);

        let mut red = ColorSpec::new();
        red.set_fg(Some(Color::Red));
        red.set_bold(true);
        red.set_intense(true);

        let mut yellow = ColorSpec::new();
        yellow.set_fg(Some(Color::Yellow));
        yellow.set_bold(true);
        yellow.set_intense(true);

        let mut green = ColorSpec::new();
        green.set_fg(Some(Color::Green));
        green.set_bold(true);
        green.set_intense(true);

        let mut bgblue = ColorSpec::new();
        bgblue.set_bg(Some(Color::Blue));
        bgblue.set_bold(true);
        bgblue.set_intense(true);

        let mut bgcyan = ColorSpec::new();
        bgcyan.set_bg(Some(Color::Cyan));
        bgcyan.set_bold(true);
        bgcyan.set_intense(false);

        let mut bgred = ColorSpec::new();
        bgred.set_bg(Some(Color::Red));
        bgred.set_bold(true);
        bgred.set_intense(true);

        let mut bgyellow = ColorSpec::new();
        bgyellow.set_bg(Some(Color::Yellow));
        bgyellow.set_bold(true);
        bgyellow.set_intense(true);

        let mut bggreen = ColorSpec::new();
        bggreen.set_bg(Some(Color::Green));
        bggreen.set_bold(true);
        bggreen.set_intense(true);

        let mut white = ColorSpec::new();
        white.set_fg(Some(Color::White));
        white.set_bold(true);
        white.set_intense(true);

        let mut gray = ColorSpec::new();
        gray.set_fg(Some(Color::White));

        // write name of program
        out.set_color(&blue)?;
        write!(&mut out, "hail")?;
        out.set_color(&cyan)?;
        write!(&mut out, "[v{}] ", env!("CARGO_PKG_VERSION"))?;

        match diagnostic.severity {
            Severity::Bug => {
                out.set_color(&red)?;
                write!(&mut out, "bug")?;
            },
            Severity::Error => {
                out.set_color(&red)?;
                write!(&mut out, "err")?;
            },
            Severity::Warning => {
                out.set_color(&yellow)?;
                write!(&mut out, "warn")?;
            },
            Severity::Help => {
                out.set_color(&green)?;
                write!(&mut out, "help")?;
            },
            Severity::Note => {
                out.set_color(&blue)?;
                write!(&mut out, "note")?;
            },
        }

        match diagnostic.code {
            Some(code) => {
                out.set_color(&cyan)?;
                write!(&mut out, "[{}]", code)?;
            },
            None => {},
        }

        out.reset()?;
        out.set_color(&white)?;
        write!(&mut out, ": ")?;

        match diagnostic.message {
            Some(message) => {
                write!(&mut out, "{}", message)?;
            },
            None => {
                write!(&mut out, "[no message]")?;
            },
        }

        out.reset()?;
        write!(&mut out, "\n")?;

        let mut i = 0;
        while i < diagnostic.labels.len() {
            // TODO: show highlighted code from label.
            let label = &diagnostic.labels[i];

            if let Some(highlight) = &label.highlight {
                out.set_color(&cyan)?;

                let start = self.line_col(highlight.loc.start).unwrap();
                let end = self.line_col(highlight.loc.end - 1).unwrap();
                let longest_linenum_length = end.0.to_string().len();

                out.set_color(&cyan)?;
                write!(&mut out, "{: >width$}--> ", "", width = longest_linenum_length + 2)?;
                out.reset()?;
                writeln!(&mut out, "{}:{}:{}", self.filename, start.0, start.1)?;
                
                out.set_color(&cyan)?;
                writeln!(&mut out, "{: >width$} |", "", width = longest_linenum_length + 1)?;
                for (i, line) in self.source.split("\n").enumerate() {
                    if i < start.0 - 1|| i > end.0 - 1 {
                        continue;
                    }

                    out.set_color(&cyan)?;
                    write!(&mut out, "{: >width$} | ", i + 1, width = longest_linenum_length + 1)?;
                    
                    //writeln!(&mut out, "{}", line);

                    for (i2, char) in line.chars().enumerate() {
                        if i + 1 >= end.0 && i2 + 1 > end.1
                        || i + 1 <= end.0 && i2 + 1 < start.1 {
                            out.reset()?;
                            write!(&mut out, "{}", char)?;
                            continue;
                        }

                        out.set_color(match diagnostic.severity {
                            Severity::Bug | Severity::Error => &bgred,
                            Severity::Warning => &bgyellow,
                            Severity::Help => &bggreen,
                            Severity::Note => &bgcyan,
                        }).unwrap();

                        write!(&mut out, "{}", char)?;
                    }

                    out.reset()?;
                    write!(&mut out, "\n")?;
                    out.reset()?;
                }

                out.set_color(&cyan)?;
                writeln!(&mut out, "{: >width$} |", "", width = longest_linenum_length + 1)?;
            }
            
            out.set_color(&cyan)?;
            write!(&mut out, " = ")?;

            out.reset()?;
            write!(&mut out, "{}", wrap(&label.message, 100).join("\n   "))?;

            if diagnostic.labels.len() > (i + 1) {
                write!(&mut out, "\n")?;
            }

            i += 1;
        }

        write!(&mut out, "\n")?;
        Ok(())
    }
}