//! Tokenizer for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

mod charstream;
mod token;

pub use charstream::CharStream;

pub use token::{
    TokenClass,
    Token,
};

#[derive(Debug)]
/// A tokenizer that processes input strings and
/// returns a list of tokens.
pub struct Tokenizer {
    /// List of tokens.
    tokens: Vec<Token>,
    
    /// Current index in token list.
    index: usize,
}

impl Tokenizer {
    /// Construct a new tokenizer from an input string.
    /// 
    /// # Parameters
    /// - `string` (`String`): a string input
    /// 
    /// # Returns
    /// A `Tokenizer` containing the full token stream.
    pub fn from(string: String) -> Self {
        // Construct a character stream
        let mut charstream = CharStream::from(string);

        // Initialize a list of tokens
        let mut tokens = Vec::new();

        // Read out each token
        while let Some (t) = charstream.get() {
            tokens.push(t);
        }

        Self {
            tokens,
            index: 0,
        }
    }

    /// Get the next token in the stream.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `Option<Token>` containing the next token
    /// in the stream, if it is available.
    pub fn next(&mut self) -> Option<Token> {
        let next = self.peek();

        self.index += 1;

        next
    }

    /// Peek at the next token in the stream.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `Option<Token>` containing the next token
    /// in the stream, if it is available.
    /// 
    /// **Note**: this function does not advance the token
    /// stream.
    pub fn peek(&self) -> Option<Token> {
        if self.index < self.tokens.len() {
            Some (self.tokens[self.index].to_owned())
        } else {
            None
        }
    }

    /// Eats the next token, if it is of the given class.
    /// 
    /// # Parameters
    /// - `class` (`TokenClass`): the expected class of the token
    /// 
    /// # Returns
    /// An `Option<Option<()>>` indicating whether or not the next token
    /// was of the expected class.
    pub fn eat(&mut self, class: TokenClass) -> Option<Option<()>> {
        self.expect(class).map(|o| o.map(|_| ()))
    }

    /// Expects a token of a given class.
    /// 
    /// # Parameter
    /// - `class` (`TokenClass`): the expected class of the token
    /// 
    /// # Returns
    /// An `Option<Option<Token>>` wrapping the token, if it was of the expected
    /// class.
    pub fn expect(&mut self, class: TokenClass) -> Option<Option<Token>> {
        if let Some (token) = self.next() {
            if token.class == class {
                Some (Some (token))
            } else {
                Some (None)
            }
        } else {
            None
        }
    }
}