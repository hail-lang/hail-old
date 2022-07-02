//! Tokens produced by Hail's lexer.

use std::ops::Range;

/// A punctuator token.
#[derive(Clone, Debug, PartialEq)]
pub struct Punct {
    /// The punctuation character.
    pub char: char,
}

impl<'a> Into<Tok<'a>> for Punct {
    fn into(self) -> Tok<'a> {
        Tok::Punct(self)
    }
}

/// An identifier token.
/// 
/// Identifiers in Hail use Unicode XID, except identifiers may start with underscores (`_`).
#[derive(Clone, Debug, PartialEq)]
pub struct Iden<'a> {
    /// The value of the identifier.
    pub value: &'a str,
}

impl<'a> Into<Tok<'a>> for Iden<'a> {
    fn into(self) -> Tok<'a> {
        Tok::Iden(self)
    }
}

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
    /// The raw unparsed integer.
    pub value: &'a str,

    /// The kind of this number.
    pub kind: NumKind,
}

impl<'a> Into<Tok<'a>> for Num<'a> {
    fn into(self) -> Tok<'a> {
        Tok::Num(self)
    }
}

/// A group of tokens.
#[derive(Clone, Debug, PartialEq)]
pub struct Group<'a> {
    /// The tokens in this group.
    pub tokens: Vec<TokNode<'a>>,
}

impl<'a> Into<Tok<'a>> for Group<'a> {
    fn into(self) -> Tok<'a> {
        Tok::Group(self)
    }
}

/// A string literal.
#[derive(Clone, Debug, PartialEq)]
pub struct Str<'a> {
    /// The value of the string literal.
    pub value: &'a str,
}

impl<'a> Into<Tok<'a>> for Str<'a> {
    fn into(self) -> Tok<'a> {
        Tok::Str(self)
    }
}

/// A comment token.
#[derive(Clone, Debug, PartialEq)]
pub struct Comment<'a> {
    /// The raw value of the comment, including the starting slashes.
    pub value: &'a str,
    
    /// Whether or not the comment was a doc comment/commentary.
    pub commentary: bool,
}

impl<'a> Into<Tok<'a>> for Comment<'a> {
    fn into(self) -> Tok<'a> {
        Tok::Comment(self)
    }
}

/// A raw token outputted by the lexer.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok<'a> {
    /// A whitespace token.
    Whitespace(),

    /// A line breaking token.
    LineBreak(),

    Punct(Punct),
    Iden(Iden<'a>),
    Num(Num<'a>),
    Str(Str<'a>),
    Group(Group<'a>),
    Comment(Comment<'a>)
}

/// Stores debug information about a token.
#[derive(Clone, Debug, PartialEq)]
pub struct TokNode<'a> {
    /// The location of this token node, in the originating source string.
    pub loc: Range<usize>,

    /// The token that this node wraps.
    pub node: Tok<'a>
}

impl<'a> TokNode<'a> {
    /// Creates a new token node.
    pub fn new<Token: Into<Tok<'a>>>(loc: Range<usize>, node: Token) -> Self {
        Self {
            loc,
            node: node.into()
        }
    }
}