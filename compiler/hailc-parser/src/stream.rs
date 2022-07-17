use std::vec::IntoIter;

use hailc_lexer::Tok;

/// An iterator through a stream of tokens.
#[derive(Clone, Debug)]
pub struct TokenStream<'a> {
    /// The token iterator.
    private: IntoIter<Tok<'a>>,
}

impl<'a> TokenStream<'a> {
    /// Creates a token stream from a list of tokens.
    pub fn from_vec(vec: Vec<Tok<'a>>) -> TokenStream {
        Self { private: vec.into_iter() }
    }

    /// Returns the next token, without iterating to the next token in the stream.
    pub fn peek(&self) -> Option<Tok<'a>> {
        self.clone().next()
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Tok<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.private.next()
    }
}