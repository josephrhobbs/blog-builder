//! Header parselet.

use blog_tkn::{
    Token,
    TokenClass,
    Tokenizer,
};

use crate::{
    Parser,
    Parselet,
    Expression,
    ParseError,
};

/// Parselet for headers.
pub struct HeaderParselet { }

impl Parselet for HeaderParselet {
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: &Token) -> Expression {
        // Copy next token's value
        let value = if let Some (opt_t) = tokenizer.expect(TokenClass::Paragraph) {
            if let Some (t) = opt_t {
                // Make sure to trim the value
                t.value.trim().to_owned()
            } else {
                // Expected text, found something else
                return Expression::Error (ParseError::ExpectedToken (TokenClass::Paragraph));
            }
        } else {
            // We ran out of tokens :(
            return Expression::Error (ParseError::UnexpectedEof);
        };

        // Check the number of hashes
        match token.value.len() {
            1 => Expression::H1 (value),
            2 => Expression::H2 (value),
            3 => Expression::H3 (value),
            4 => Expression::H4 (value),
            5 => Expression::H5 (value),
            6 => Expression::H6 (value),
            _ => Expression::Error (ParseError::TooManyHashes),
        }
    }
}