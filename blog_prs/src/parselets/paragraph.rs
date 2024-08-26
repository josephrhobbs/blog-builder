//! Paragraph parselet.

use blog_tkn::{
    Token,
    Tokenizer,
};

use crate::{
    Parser,
    Parselet,
    Expression,
};

/// Parselet for paragraphs.
pub struct ParagraphParselet { }

impl Parselet for ParagraphParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: &Token) -> Expression {
        Expression::Paragraph (token.value.to_owned())
    }
}