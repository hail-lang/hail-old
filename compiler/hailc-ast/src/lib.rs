use hailc_loc::Loc;

/// A name, such as an identifier token.
#[derive(Clone, Debug, PartialEq)]
pub struct Name<'a> {
    /// The location of the name.
    pub loc: Loc<'a>,

    /// The raw value of the name.
    pub value: &'a str,
}

pub mod value {
    use super::*;

    /// A boolean value.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Bool<'a> {
        /// The location of the boolean.
        pub loc: Loc<'a>,

        /// The parsed boolean value.
        pub value: bool,
    }

    /// A value expression.
    #[derive(Clone, Debug, PartialEq)]
    pub enum Value<'a> {
        /// A boolean value.
        Bool(Bool<'a>),

        /// A name value.
        Name(Name<'a>),
    }
}

pub use value::Value;

/// An AST tree that represents a single Hail unit.
#[derive(Clone, Debug, PartialEq)]
pub struct AstUnit {
    
}

impl AstUnit {
    /// Creates an empty [`AstUnit`], for the parser.
    pub fn new() -> Self {
        Self {}
    }
}