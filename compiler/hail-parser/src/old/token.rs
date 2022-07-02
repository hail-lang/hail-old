use std::{ops::Range};

use hail_diagnostic::Diag;

pub type Loc = Range<usize>;

/// The spacing between this token and the next.
#[derive(Clone, Debug, PartialEq)]
pub enum Space {
    /// No spacing between this token and the next.
    None,

    /// Whitespace between this token and the next.
    Whitespace,

    /// A line break token.
    LineBreak,
}

/// A comment token, which is usually skipped.
/// 
/// It can be assumed that this comment is a doc comment.
pub struct Comment<'a> {
    /// The raw value of the comment.
    pub value: &'a str,
}

/// A punctuation token.
#[derive(Clone, Debug, PartialEq)]
pub enum PunctKind {
    Tilde,
    Bang,
    At,
    Hash,
    Perc,
    Caret,
    Amp,
    Star,
    Dash,
    Eq,
    Plus,
    Pipe,
    /// Whether or not the semicolon was inserted automatically by the lexer.
    Semi(bool),
    Colon,
    Slash,
    Dot,
    Comma,
    Gt,
    Lt,
    Quest,
}

/// A punctuatiun token.
#[derive(Clone, Debug, PartialEq)]
pub struct Punct {
    /// The location of this token.
    pub loc: Loc,

    /// The spacing of this token.
    pub spacing: Space,

    /// The kind of this punctuator.
    pub kind: PunctKind,
}

/// A keyword token.
#[derive(Clone, Debug, PartialEq)]
pub enum KeywordKind {
    Return,
    Break,
    Continue,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Keyword {
    /// The location of this token.
    pub loc: Loc,

    /// The spacing of this token.
    pub spacing: Space,

    /// The kind of this keyword.
    pub kind: KeywordKind,
}

/// The kind of a number literal.
#[derive(Clone, Debug, PartialEq)]
pub enum NumKind {
    Int,
    Hex,
    Bin,
    Float,
}

/// A number literal.
#[derive(Clone, Debug, PartialEq)]
pub struct Num<'a> {
    /// The location of this token.
    pub loc: Loc,

    /// The spacing of this token.
    pub spacing: Space,

    /// The kind of number literal.
    pub kind: NumKind,

    /// The raw value of the number literal.
    pub value: &'a str,
}

/// An identifier literal.
#[derive(Clone, Debug, PartialEq)]
pub struct Iden<'a> {
    /// The location of this token.
    pub loc: Loc,

    /// The spacing of this token.
    pub spacing: Space,

    /// An identifier literal.
    pub value: &'a str,
}

/// A string literal.
#[derive(Clone, Debug, PartialEq)]
pub struct Str<'a> {
    /// The location of this token.
    pub loc: Loc,

    /// The spacing of this token.
    pub spacing: Space,

    /// The value of the string.
    pub value: &'a str,
}

/// A group of tokens.
#[derive(Clone, Debug, PartialEq)]
pub struct Group<'a> {
    /// The location of this token.
    pub loc: Loc,

    /// The spacing of this token.
    pub spacing: Space,

    /// The tokens in this group.
    pub tokens: Vec<Tok<'a>>,
}

/// A token type for Hail.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok<'a> {
    Punct(Punct),
    Num(Num<'a>),
    Keyword(Keyword),
    Iden(Iden<'a>),
    Str(Str<'a>),
    Group(Group<'a>),
}