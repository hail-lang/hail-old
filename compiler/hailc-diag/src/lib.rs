//! Diagnostics for the Hailc compiler.
//! 
//! This crate contains tools for creating useful and informative diagnostics that are user-oriented.

pub mod builder;
pub mod driver;

#[cfg(feature = "codespan")]
pub mod codespan;

pub use builder::DiagBuilder;

use hailc_loc::Loc;

/// The severity of a diagnostic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrLevel {
    /// An [`ErrLevel::Bug`] error level signifies that the compiler is aware of the bug, but a fix for it has not yet been implemented.
    Bug,

    /// An [`ErrLevel::Err`] error level signifies that the source code being compiled is not valid in some way.  Errors can be thrown because of
    /// invalid syntax, type checking problems, a module not existing, etc.
    Err,

    /// An [`ErrLevel::Warn`] error level means that the source code being compiled is valid at compile time, but it may produce unwanted output,
    /// or maybe the source code contains unused code.  Anything that shouldn't halt compilation, but that should be known to the user.
    Warn,

    /// An [`ErrLevel::Help`] error level means that the diagnostic provides help for another diagnostic which was thrown, which is usually of
    /// [`ErrLevel::Error`] severity.
    Help,

    /// An [`ErrLevel::Note`] error level means that the diagnostic states states the cause of another diagnostic.
    Note,
}

/// A diagnostic created by the compiler.
/// 
/// Diagnostics store debug information, such as an error code, the location that the error was found, and a good description of what causes the
/// problem.
#[derive(Clone, Debug, PartialEq)]
pub struct Diag<'a> {
    /// The severity, or importance, of this diagnostic.
    level: ErrLevel,

    /// The error code of this diagnostic.
    code: Option<&'a str>,

    /// The highlighted points of this diagnostic.
    highlight: Option<Loc<'a>>,

    /// The message of the diagnostic.
    msg: Option<&'a str>,
}

impl<'a> Diag<'a> {
    /// Creates a new, empty diagnostic with the provided [`ErrLevel`].
    pub fn new(level: ErrLevel) -> Self {
        Self { level, code: None, highlight: None, msg: None }
    }

    /// Returns the level of this diagnostic.
    pub fn level(&self) -> ErrLevel {
        self.level
    }

    /// Returns the error code of this diagnostic, if any.
    pub fn code(&self) -> Option<&'a str> {
        self.code
    }

    /// Sets the code of this diagnostic to the one provided.
    pub fn with_code(mut self, code: &'a str) -> Self {
        self.code = Some(code);
        self
    }

    /// Returns the message of the diagnostic.
    pub fn msg(&self) -> Option<&'a str> {
        self.msg
    }

    /// Returns this diagnostic, with the provided message.
    pub fn with_msg(mut self, msg: &'a str) -> Self {
        self.msg = Some(msg);
        self
    }

    /// Returns the highlight of this diagnostic, if any.
    pub fn highlight(&self) -> Option<Loc> {
        self.highlight
    }

    /// Makes this diagnostic highlight the provided location.
    pub fn with_highlight(mut self, loc: Loc<'a>) -> Self {
        self.highlight = Some(loc);
        self
    }
}