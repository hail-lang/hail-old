//! The lexical scanner for the hail compiler.
//! 
//! Implemented using `logos`, which is then wrapped to perform automatic semicolon insertion.

use logos::{Lexer, Logos};

/// This is the raw lexer which will later be wrapped by an automatic semicolon inserter.
#[derive(Clone, Copy, Debug, Eq, Logos, PartialEq)]
pub enum RawTok {
    /// A line breaking token.
    #[regex("[\n\r]")]
    Break,

    /// A punctuator token.
    #[regex("#|::|:|\\.|\\?|<-|->|<=|=>|-=|-|\\*=|\\*|!=|!|&&|&=|&|/=|/|%=|%|\\+=|\\+|<<=|<<|<=|<|>>=|>>|>=|>|\\^=|\\^|\\|=|\\|\\||\\||==|=|\\[|\\]|\\(|\\)|\\{|\\}|,|;")]
    Punct,

    /// An identifier token.
    #[regex("[a-zA-Z_$][a-zA-Z_$0-9]*")]
    Id,

    /// A plain integer token.
    #[regex("-?[0-9]+")]
    Int,

    /// A hexadecimal integer token.
    #[regex("0x[0-9a-fA-F]+")]
    XInt,

    /// A binary integer token.
    #[regex("0b[0-1]+")]
    BInt,

    /// A floating point number token.
    #[regex("[0-9]+\\.([0-9]+([eE][-+]?[0-9]+)?)?")]
    Float,

    /// A string token.
    #[regex("\"([^\"\\\\]|\\\\.)*\"")]
    Str,

    /// An invalid token.
    #[error]
    #[regex("[ \t]+", logos::skip)] // skip white space
    #[regex("//.*", logos::skip)] // skip comments
    Err,
}

/// A token after automatic semicolon insertion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tok<'a> {
    /// `#`
    Hash,

    /// `::`
    ColCol,

    /// `:`
    Col,

    /// `.`
    Dot,

    /// `?`
    Quest,

    /// `<-`
    LArrow,

    /// `->`
    RArrow,

    /// `=>`
    FatArrow,

    /// `-=`
    MinEq,

    /// `-`
    Min,

    /// `*=`
    StarEq,

    /// `*`
    Star,

    /// `!=`
    BangEq,

    /// `!`
    Bang,

    /// `&&`
    AmpAmp,

    /// `&=`
    AmpEq,

    /// `&`
    Amp,

    /// `/=`
    SlashEq,

    /// `/`
    Slash,

    /// `%=`
    PercEq,

    /// `%`
    Perc,

    /// `+=`
    PlusEq,

    /// `+`
    Plus,

    /// `<<=`
    LtLtEq,

    /// `<<`
    LtLt,

    /// `<=`
    LtEq,

    /// `<`
    Lt,

    /// `>>=`
    GtGtEq,

    /// `>>`
    GtGt,

    /// `>=`
    GtEq,

    /// `>`
    Gt,

    /// `^=`
    CaretEq,

    /// `^`
    Caret,

    /// `|=`
    PipeEq,

    /// `||`
    PipePipe,

    /// `|`
    Pipe,

    /// `==`
    EqEq,

    /// `=`
    Eq,

    /// `[`
    LBrack,

    /// `]`
    RBrack,

    /// `(`
    LParen,

    /// `)`
    RParen,

    /// `{`
    LCurly,

    /// `}`
    RCurly,

    /// `,`
    Comma,

    /// `;`
    Semi,

    /// `true`
    KTrue,

    /// `false`
    KFalse,

    /// `fluid`
    KFluid,

    /// `as`
    KAs,

    /// `routine`
    KRoutine,

    /// `val`
    KVal,

    /// `shared`
    KShared,

    /// `import`
    KImport,

    /// `from`
    KFrom,

    /// `if`
    KIf,

    /// `else`
    KElse,

    /// `while`
    KWhile,

    /// `match`
    KMatch,

    /// `struct`
    KStruct,

    /// `type`
    KType,

    /// `enum`
    KEnum,

    /// `break`
    KBreak,

    /// `continue`
    KContinue,

    /// `return`
    KReturn,

    /// `apply`
    KApply,

    /// `contract`
    KContract,

    /// `to`
    KTo,

    /// An identifier token.
    Id(&'a str),

    /// A plain integer token.
    Int(&'a str),

    /// A hexadecimal integer token.
    XInt(&'a str),

    /// A binary integer token.
    BInt(&'a str),

    /// A floating point number token.
    Float(&'a str),

    /// A string token.
    Str(&'a str),
}

/// An automatic semicolon inserter for the lexer generated by `logos`.
pub struct Asi<'a> {
    /// The lexer that this semicolon inserter wraps.
    private: Lexer<'a, RawTok>,

    /// Whether or not the we could end on the last token.
    /// 
    /// ```hail
    /// val my_number = 42 + // cannot end here, don't insert semicolon
    ///     42
    /// 
    /// val my_number = 42 + 42 // can end here, insert semicolon
    /// ```
    can_insert: bool,

    /// Whether or not the next token should be skipped and replaced with a semicolon.
    do_insert: bool,
}

impl<'a> Asi<'a> {
    /// Creates a lexer for the provided source string.
    pub fn lex(src: &'a str) -> Self {
        Self { private: RawTok::lexer(src), can_insert: false, do_insert: false }
    }

    /// Skips line breaks and inserts semicolons where necessary.
    fn skip(&mut self) {
        let mut iter = self.private.clone();
        while let Some(next) = iter.next() { // clone it to prevent iterating the underlying lexer
            if next == RawTok::Break {
                self.private.next();

                if self.can_insert {
                    self.do_insert = true;
                    break;
                }
            } else {
                break;
            }
        }
    }
}

impl<'a> Iterator for Asi<'a> {
    type Item = Result<(usize, Tok<'a>, usize), ()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip(); // insert semicolons?

        if self.do_insert {
            self.do_insert = false;
            self.can_insert = false;
            let span = self.private.span();
            return Some(Ok((span.start, Tok::Semi, span.end)));
        }

        if let Some(tok) = self.private.next() {
            match tok {
                RawTok::Punct => {
                    self.can_insert = false;

                    let slice = self.private.slice();
                    let tok = match slice {
                        "#" => Tok::Hash,
                        "::" => Tok::ColCol,
                        ":" => Tok::Col,
                        "." => Tok::Dot,
                        "?" => {
                            self.can_insert = true;
                            Tok::Quest
                        },
                        "<-" => Tok::LArrow,
                        "->" => Tok::RArrow,
                        "=>" => Tok::FatArrow,
                        "-=" => Tok::MinEq,
                        "-" => Tok::Min,
                        "*=" => Tok::StarEq,
                        "*" => Tok::Star,
                        "!=" => Tok::BangEq,
                        "!" => Tok::Bang,
                        "&&" => Tok::AmpAmp,
                        "&=" => Tok::AmpEq,
                        "&" => Tok::Amp,
                        "/=" => Tok::SlashEq,
                        "/" => Tok::Slash,
                        "%=" => Tok::PercEq,
                        "%" => Tok::Perc,
                        "+=" => Tok::PlusEq,
                        "+" => Tok::Plus,
                        "<<=" => Tok::LtLtEq,
                        "<<" => Tok::LtLt,
                        "<=" => Tok::LtEq,
                        "<" => Tok::Lt,
                        ">>=" => Tok::GtGtEq,
                        ">>" => Tok::GtGt,
                        ">=" => Tok::GtEq,
                        ">" => Tok::Gt,
                        "^=" => Tok::CaretEq,
                        "^" => Tok::Caret,
                        "|=" => Tok::PipeEq,
                        "||" => Tok::PipePipe,
                        "|" => Tok::Pipe,
                        "==" => Tok::EqEq,
                        "=" => Tok::Eq,
                        "[" => Tok::LBrack,
                        "]" => {
                            self.can_insert = true;
                            Tok::RBrack
                        },
                        "(" => Tok::LParen,
                        ")" => {
                            self.can_insert = true;
                            Tok::RParen
                        },
                        "{" => Tok::LCurly,
                        "}" => {
                            self.can_insert = true;
                            Tok::RCurly
                        },
                        "," => Tok::Comma,
                        ";" => Tok::Semi,
                        _ => unreachable!(),
                    };

                    let span = self.private.span();
                    return Some(Ok((span.start, tok, span.end)));
                },
                RawTok::Id => {
                    self.can_insert = true;

                    // TODO: check if token is an identifier or a keyword.
                    let slice = self.private.slice();
                    let tok = match slice {
                        "true" => Tok::KTrue,
                        "false" => Tok::KFalse,
                        "fluid" => {
                            self.can_insert = false;
                            Tok::KFluid
                        },
                        "as" => {
                            self.can_insert = false;
                            Tok::KAs
                        },
                        "routine" => {
                            self.can_insert = false;
                            Tok::KRoutine
                        },
                        "val" => {
                            self.can_insert = false;
                            Tok::KVal
                        },
                        "shared" => {
                            self.can_insert = false;
                            Tok::KShared
                        },
                        "import" => {
                            self.can_insert = false;
                            Tok::KImport
                        },
                        "from" => {
                            self.can_insert = false;
                            Tok::KFrom
                        },
                        "if" => {
                            self.can_insert = false;
                            Tok::KIf
                        },
                        "else" => {
                            self.can_insert = false;
                            Tok::KElse
                        },
                        "while" => {
                            self.can_insert = false;
                            Tok::KWhile
                        },
                        "match" => {
                            self.can_insert = false;
                            Tok::KMatch
                        },
                        "struct" => {
                            self.can_insert = false;
                            Tok::KStruct
                        },
                        "type" => {
                            self.can_insert = false;
                            Tok::KType
                        },
                        "enum" => {
                            self.can_insert = false;
                            Tok::KEnum
                        },
                        "break" => {
                            self.can_insert = false;
                            Tok::KBreak
                        },
                        "continue" => {
                            self.can_insert = false;
                            Tok::KContinue
                        },
                        "return" => {
                            self.can_insert = false;
                            Tok::KReturn
                        },
                        "apply" => {
                            self.can_insert = false;
                            Tok::KApply
                        },
                        "contract" => {
                            self.can_insert = false;
                            Tok::KContract
                        },
                        "to" => {
                            self.can_insert = false;
                            Tok::KTo
                        },
                        _ => Tok::Id(slice),
                    };

                    let span = self.private.span();
                    return Some(Ok((span.start, tok, span.end)));
                },
                RawTok::Int => {
                    self.can_insert = true;
                    let span = self.private.span();
                    return Some(Ok((span.start, Tok::Int(self.private.slice()), span.end)));
                },
                RawTok::XInt => {
                    self.can_insert = true;
                    let span = self.private.span();
                    return Some(Ok((span.start, Tok::XInt(self.private.slice()), span.end)));
                },
                RawTok::BInt => {
                    self.can_insert = true;
                    let span = self.private.span();
                    return Some(Ok((span.start, Tok::BInt(self.private.slice()), span.end)));
                },
                RawTok::Float => {
                    self.can_insert = true;
                    let span = self.private.span();
                    return Some(Ok((span.start, Tok::Float(self.private.slice()), span.end)));
                },
                RawTok::Str => {
                    self.can_insert = true;
                    let span = self.private.span();
                    return Some(Ok((span.start, Tok::Str(self.private.slice()), span.end)));
                },
                RawTok::Err => return Some(Err(())),
                // line breaks are skipped, so there is no chance of finding them.
                _ => unreachable!(),
            }
        }

        None
    }
}