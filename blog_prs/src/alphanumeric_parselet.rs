//! Alphanumeric parselet.

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    Tokenizer,
};

/// Parselet for alphanumeric expressions.
pub struct AlphanumericParselet;

impl PrefixParselet for AlphanumericParselet {
    fn parse(&self, _: &Parser, _: &mut Tokenizer, token: &Token) -> Expression {
        Expression::Alphanumeric (token.get_value())
    }
}