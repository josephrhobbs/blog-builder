//! Emphasized text parselet.

use blog_tkn::{
    Token,
    Tokenizer,
    TokenClass,
};

use crate::{
    Parser,
    Parselet,
    Expression,
    ParseError,
};

/// Parselet for emphasized text.
pub struct EmphasisParselet { }

impl Parselet for EmphasisParselet {
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: &Token) -> Expression {
        match token.value.as_str() {
            // Italics
            "_" | "*" => parse_emphasis(tokenizer, token),
            
            // Bold
            "__" | "**" => parse_emphasis(tokenizer, token),

            // Bold & italics
            "__*" | "***" | "**_" | "___" => parse_emphasis(tokenizer, token),

            // Not sure what this is...
            _ => Expression::Error (ParseError::UnrecognizedEmphasis)
        }
    }
}

/// Parse until we see a matching closing delimiter.
fn parse_emphasis(tokenizer: &mut Tokenizer, token: &Token) -> Expression {
    // Get the next token (should be raw text)
    if let Some (t) = tokenizer.next() {
        if t.class == TokenClass::Paragraph {
            // Consume the closing delimiter
            let closing = if let Some (c) = tokenizer.next() {
                c
            } else {
                return Expression::Error (ParseError::UnexpectedEof);
            };

            // Make sure the delimiters are the same
            if token.value != closing.value {
                return Expression::Error (ParseError::MismatchedDelimiters);
            }

            match closing.value.as_str() {
                "_" | "*" => Expression::Italics (t.value.to_owned()),
                "__" | "**" => Expression::Bold (t.value.to_owned()),
                "__*" | "***" | "**_" | "___" => Expression::BoldItalics (t.value.to_owned()),
                _ => Expression::Error (ParseError::UnrecognizedEmphasis)
            }
        } else {
            Expression::Error (ParseError::ExpectedToken (TokenClass::Paragraph))
        }
    } else {
        Expression::Error (ParseError::UnexpectedEof)
    }
}