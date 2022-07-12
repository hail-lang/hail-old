use logos::{Logos};

/// The raw Logos lexer.
#[derive(Clone, Copy, Debug, Logos, PartialEq)]
pub enum RawTok {
    /// A line breaking token.
    #[regex(r"[\n\r\f]+")]
    LineBreak,

    /// A floating point number constant.
    #[regex("-?[0-9]+\\.([eE][-+]?[0-9]+)?")]
    Float,

    /// An integer constant.
    /// 
    /// ```hail
    /// 42
    /// 10000000
    /// -1000
    /// ```
    #[regex("-?[0-9]+")]
    Int,

    /// A hexadecimal integer constant.
    /// 
    /// ```hail
    /// 0x42
    /// ```
    #[regex("0x[0-9a-fA-F]+")]
    HexInt,

    /// A binary integer constant.
    /// 
    /// ```hail
    /// 0x10101010
    /// ```
    #[regex("0b[01]+")]
    BinInt,

    /// An identifier literal.
    /// 
    /// ```hail
    /// my_identifier
    /// _ // reserved identifier
    /// ```
    #[regex("[a-zA-Z_$][a-zA-Z_$0-9]*")]
    Id,
    
    /// A string literal.
    /// 
    /// ```hail
    /// "Hello, world!"
    /// ```
    #[regex(r#""([^"\\]|\\")*""#)]
    Str,

    /// A punctuator token.
    #[token("->")]
    #[token("<-")]
    #[token("::")]
    #[token(".")]
    #[token("?")]
    #[token("-=")]
    #[token("-")]
    #[token("*=")]
    #[token("*")]
    #[token("!")]
    #[token("&&")]
    #[token("&=")]
    #[token("&")]
    #[token("/=")]
    #[token("/")]
    #[token("%=")]
    #[token("%")]
    #[token("+=")]
    #[token("+")]
    #[token("<<=")]
    #[token("<<")]
    #[token(">>=")]
    #[token(">>")]
    #[token("^=")]
    #[token("^")]
    #[token("||")]
    #[token("|")]
    #[token("=")]
    #[token("(")]
    #[token(")")]
    #[token("[")]
    #[token("]")]
    #[token("{")]
    #[token("}")]
    #[token(";")]
    #[token(",")]
    Punct,

    /// An error token.
    #[regex(r"[ \t]+", logos::skip)]
    #[regex(r"//.*", logos::skip)]
    #[error]
    Err,
}