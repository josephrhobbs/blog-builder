//! Newline parselet.

use blog_tkn::{
    Token,
    TokenClass,
    Tokenizer,
};

use crate::{
    Parser,
    Parselet,
    Expression,
};

/// Parselet for newlines.
pub struct NewlineParselet { }

impl Parselet for NewlineParselet {
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, _token: &Token) -> Expression {
        // Consume all newlines
        while let Some (t) = tokenizer.peek() {
            if t.class == TokenClass::Newline {
                tokenizer.next();
            } else {
                break;
            }
        }

        Expression::Newline
    }
}