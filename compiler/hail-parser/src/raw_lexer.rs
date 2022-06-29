//! A handwritten lexer for Hail.

use std::str::Chars;

use crate::raw_token::*;

use hail_diagnostic::{Diag, Label};
use hail_shared::iter::{BetterIter, IntoBetterIterator};
use unicode_xid::UnicodeXID;

/// A lexer which inputs a source string and converts it to tokens.
#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    /// The source string.
    source: &'a str,

    /// The characters of the source string.
    chars: BetterIter<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from the provided source string.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().better_iter(),
        }
    }

    /// Returns whether or not `char` is a Unicode (non-line-breaking) whitespace character.
    pub fn is_whitespace(char: char) -> bool {
        match char {
            '\u{0009}' | '\u{0020}' | '\u{00A0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}'..='\u{2029}' | '\u{202f}' | '\u{205f}'
            | '\u{3000}' => true,
            _ => false,
        }
    }

    /// Returns whether or not `char` is a Unicode line break character.
    pub fn is_line_break(char: char) -> bool {
        match char {
            '\u{000a}'..='\u{000d}' | '\u{0085}' | '\u{2028}'..='\u{2029}' => true,
            _ => false,
        }
    }

    /// Returns whether or not `char` is an XID starting character or an underscore (`_`).
    pub fn is_iden_start(char: char) -> bool {
        UnicodeXID::is_xid_start(char) || char == '_'
    }

    /// Returns whether or not `char` is an XID continuing character.
    pub fn is_iden_continue(char: char) -> bool {
        UnicodeXID::is_xid_continue(char)
    }

    /// Returns whether or not `char` is a punctuator in Hail.
    pub fn is_punct(char: char) -> bool {
        match char {
            '~' | '!' | '@' | '#' | '%' | '^' | '&' | '*' | '-' | '=' | '+' | '|' | ';' | ':' | '/' | '.' | ',' | '<' | '>' | '?' => true,
            _ => false,
        }
    }

    /// Returns whether or not `char` is a digit.
    pub fn is_digit(char: char) -> bool {
        match char {
            '0'..='9' => true,
            _ => false
        }
    }

    /// Tokenizes a single identifier token.
    fn lex_iden(&mut self) -> Iden<'a> {
        let start_idx = self.chars.idx();

        while let Some(char) = self.chars.peek() {
            if !Self::is_iden_continue(char) {
                break;
            }

            self.chars.next();
        }

        Iden {
            value: &self.source[start_idx..self.chars.idx()],
        }
    }

    /// Tokenizes all whitespace until the next character is not a whitespace.
    fn lex_whitespace(&mut self) -> TokNode<'a> {
        let start_idx = self.chars.idx();

        while let Some(char) = self.chars.peek() {
            if !Self::is_whitespace(char) {
                break;
            }

            self.chars.next();
        }

        TokNode::new(start_idx..self.chars.idx(), Tok::Whitespace())
    }

    /// Tokenizes all line breaks until the next character is not a line break.
    fn lex_line_break(&mut self) -> TokNode<'a> {
        let start_idx = self.chars.idx();

        while let Some(char) = self.chars.peek() {
            if !Self::is_line_break(char) {
                break;
            }

            self.chars.next();
        }

        TokNode::new(start_idx..self.chars.idx(), Tok::LineBreak())
    }

    /// Tokenizes a single number token.
    fn lex_num(&mut self) -> Result<TokNode<'a>, Diag> {
        let start_idx = self.chars.idx();
        let mut kind = NumKind::Int;

        if let Some(first_digit) = self.chars.next() {
            if first_digit == '0' {
                if let Some(second_digit) = self.chars.peek() {
                    if second_digit == 'x' {
                        unimplemented!(); // TODO: implement hex numbers
                    } else if second_digit == 'b' {
                        unimplemented!(); // TODO: implement binary numbers
                    } else if !Self::is_digit(second_digit) {
                        return Ok(TokNode::new(start_idx..self.chars.idx(), Num {
                            value: &self.source[start_idx..self.chars.idx()],
                            kind,
                        }));
                    }
                }
            }
        }

        while let Some(char) = self.chars.peek() {
            if !Self::is_digit(char) {
                break;
            }

            self.chars.next();
        }

        if let Some(char) = self.chars.peek() {
            if char == '.' {
                self.next();
                kind = NumKind::Float;

                let mut has_decimal_value = false;
                while let Some(char) = self.chars.peek() {
                    if !Self::is_digit(char) {
                        break;
                    }
                    
                    has_decimal_value = true;
                    self.chars.next();
                }
                
                if let Some(char) = self.chars.peek() {
                    if char == 'e' || char == 'E' {
                        if has_decimal_value {
                            self.chars.next();

                            if let Some(char) = self.chars.peek() {
                                if char == '+' || char == '-' {
                                    self.chars.next();
                                }
                            } else {
                                let label = Label::new("No value defined for this exponent. \
                                Exponents should have a value immediately after the 'e', like so: \
                                '4.2e1', '4.2e+1' or '4.2e-1'.")
                                    .highlight(self.chars.idx()..self.chars.idx() + 1);
                                
                                let diag = Diag::error()
                                    .with_code("E0002(lexer)")
                                    .with_labels(vec![label])
                                    .with_message("no value defined for exponent.");
                                
                                return Err(diag);
                            }

                            while let Some(char) = self.chars.peek() {
                                if !Self::is_digit(char) {
                                    break;
                                }
                    
                                self.chars.next();
                            }
                        }
                    }
                }
            }
        }

        return Ok(TokNode::new(start_idx..self.chars.idx(), Num {
            value: &self.source[start_idx..self.chars.idx()],
            kind,
        }));
    }

    /// Tokenizes a string token.
    fn lex_str(&mut self) -> Result<TokNode<'a>, Diag> {
        let start_idx = self.chars.idx();
        self.chars.next(); // skip over starting quote
        let mut ended = false;

        while let Some(char) = self.chars.peek() {
            if char == '"' {
                self.chars.next();
                ended = true;
                break;
            } else if char == '\\' {
                self.chars.next();
                self.chars.next();
            } else {
                self.chars.next();
            }
        }

        if !ended {
            let label = Label::new("This string was started but never ended.")
                .highlight(start_idx..start_idx + 1);
            
            let label2 = Label::new("As you can see, the string starts, but no closing quote was found before the end of the file.")
                .highlight(start_idx..self.chars.idx());
            
            let diag = Diag::error()
                .with_code("E0004(lexer")
                .with_labels(vec![label, label2])
                .with_message("unclosed string.");
            
            return Err(diag);
        }

        Ok(TokNode::new(start_idx..self.chars.idx(), Str { value: &self.source[start_idx..self.chars.idx()] }))
    }

    /// A token group.
    fn lex_group(&mut self) -> Result<TokNode<'a>, Diag> {
        let open = self.chars.next().unwrap();
        let close = match open {
            '(' => ')',
            '{' => '}',
            '[' => ']',
            _ => unreachable!(),
        };
        let mut ended = false;
        let mut tokens = vec![];
        let start_idx = self.chars.idx();
        
        while let Some(char) = self.chars.peek() {
            if char == close {
                self.chars.next();
                ended = true;
                break;
            }

            let tok = self.next().unwrap();
            tokens.push(tok?);
        }

        if !ended {
            let label = Label::new("This group never closes \
            Groups start with '(', '{' and '[' must be closed with ')', '}', ']' respectively.")
                .highlight(start_idx..start_idx + 1);

            let label2 = Label::new("The group starts there, and as you can see, never closes.")
                .highlight(start_idx..self.chars.idx());
            
            let diag = Diag::error()
                .with_code("E0003(lexer)")
                .with_labels(vec![label, label2])
                .with_message("unclosed group");
            
            return Err(diag);
        }

        Ok(TokNode::new(start_idx..self.chars.idx(), Group { tokens }))
    }

    /// Tokenizes a single comment token.
    fn lex_comment(&mut self) -> TokNode<'a> {
        let start_idx = self.chars.idx() - 1;

        self.chars.next();

        let commentary = if let Some(third) = self.chars.peek() {
            third == '/'
        } else {
            false
        };

        while let Some(char) = self.chars.peek() {
            if !Self::is_line_break(char) {
                self.chars.next();
                continue;
            }

            self.chars.next();
            break;
        }

        TokNode::new(start_idx..self.chars.idx(), Comment {
            value: &self.source[start_idx..self.chars.idx() - 1],
            commentary,
        })
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<TokNode<'a>, Diag>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(char) = self.chars.peek() {
            // TODO: skip tokens before this token.
            let start_idx = self.chars.idx();

            if Self::is_whitespace(char) {
                return Some(Ok(self.lex_whitespace()));
            } else if Self::is_line_break(char) {
                return Some(Ok(self.lex_line_break()));
            } else if Self::is_punct(char) {
                self.chars.next();
                if char == '/' && self.chars.peek() == Some('/') {
                    return Some(Ok(self.lex_comment()));
                }


                return Some(Ok(TokNode::new(start_idx..self.chars.idx(), Punct {
                    char: char,
                })));
            } else if Self::is_iden_start(char) {
                let iden = self.lex_iden();
                return Some(Ok(TokNode::new(start_idx..self.chars.idx(), iden)));
            } else if Self::is_digit(char) {
                return Some(self.lex_num());
            } else if char == '(' || char == '{' || char == '['  {
                return Some(self.lex_group());
            } else if char == '"' {
                return Some(self.lex_str());
            } else {
                let label = Label::new(format!("I don't what '{}' means. \
                It has no meaning in Hail, and I am simply a little Hail compiler.", char))
                    .highlight(start_idx..start_idx + 1);
                
                let diagnostic = Diag::error()
                    .with_code("E0001(lexer)")
                    .with_labels(vec![label])
                    .with_message("invalid character found.");
                
                return Some(Err(diagnostic));
            }
        }

        None
    }
}