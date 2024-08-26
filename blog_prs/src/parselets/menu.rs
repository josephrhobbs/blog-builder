//! Menu parselet.

use blog_tkn::{
    Token,
    Tokenizer,
};

use crate::{
    Parser,
    Parselet,
    Expression,
};

/// Parselet for menus.
pub struct MenuParselet { }

impl Parselet for MenuParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, _token: &Token) -> Expression {
        Expression::Menu
    }
}