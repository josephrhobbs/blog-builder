//! Paragraph parselet.

use blog_tkn::{
    Token,
    Tokenizer,
    TokenClass,
};

use crate::{
    Parser,
    Parselet,
    Expression,
};

/// Parselet for paragraphs.
pub struct ParagraphParselet { }

impl Parselet for ParagraphParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: &Token) -> Expression {
        // Initialize list of expressions
        let mut output = Vec::new();

        // Push the first token, since it's already been consumed
        output.push(Expression::Text (token.value.to_owned()));

        // Parse until we see a newline
        while let Some (t) = tokenizer.peek() {
            // Is this a newline?
            if t.class == TokenClass::Newline {
                // Consume the newline
                let _ = tokenizer.next();

                // Return
                return Expression::Paragraph (output);
            }

            // Parse raw text or another expression
            // Include parentheses
            if t.class == TokenClass::Paragraph
                || t.class == TokenClass::OpenParen
                || t.class == TokenClass::CloseParen
            {
                // Consume the text
                let _ = tokenizer.next();

                // Push the text to the paragraph
                output.push(Expression::Text (t.value.to_owned()));
            } else {
                // Get the next expression out of the token stream
                let expr = parser.parse_next(tokenizer);

                // Return any errors, if found
                // 
                // All errors must occur at the top level
                if let Expression::Error (_) = expr {
                    return expr;
                }

                output.push(expr);
            }
        }

        // We never found the newline, but that's ok, we're at EOF
        Expression::Paragraph (output)
    }
}