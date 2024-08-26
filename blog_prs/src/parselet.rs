//! Parselet trait for the Blog Builder parser.

use blog_tkn::{
    Token,
    Tokenizer,
};

use crate::{
    Parser,
    Expression,
};

/// An object that implements a `parse` function to parse a
/// stream of tokens into an output expression.
pub trait Parselet {
    /// Parse a stream of tokens.
    /// 
    /// # Parameters
    /// - `parser` (`&Parser`): the original parser
    /// - `tokenizer` (`&mut Tokenizer`): the token stream
    /// - `token` (`&Token`): the current token
    /// 
    /// # Returns
    /// An `Expression` contaning the parsed expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: &Token) -> Expression;
}