//! Control sequence parselet.

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

/// Parselet for control sequences.
pub struct ControlParselet { }

impl Parselet for ControlParselet {
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, _token: &Token) -> Expression {
        // Get control sequence type
        let ctrl = if let Some (opt_t) = tokenizer.expect(TokenClass::Paragraph) {
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

        // Consume opening square bracket
        if let Some (opt_t) = tokenizer.eat(TokenClass::OpenSquare) {
            if let Some (()) = opt_t {
                // Good!
            } else {
                // Expected closing square, found something else
                return Expression::Error (ParseError::ExpectedToken (TokenClass::OpenSquare));
            }
        } else {
            // We ran out of tokens :(
            return Expression::Error (ParseError::UnexpectedEof);
        };

        // Get value
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

        // Consume closing square bracket
        if let Some (opt_t) = tokenizer.eat(TokenClass::CloseSquare) {
            if let Some (()) = opt_t {
                // Good!
            } else {
                // Expected closing square, found something else
                return Expression::Error (ParseError::ExpectedToken (TokenClass::CloseSquare));
            }
        } else {
            // We ran out of tokens :(
            return Expression::Error (ParseError::UnexpectedEof);
        };

        // Stop here?
        match ctrl.as_str() {
            "wip" => return Expression::WorkInProgress (value),
            _ => (),
        }

        // Consume opening parenthesis
        if let Some (opt_t) = tokenizer.eat(TokenClass::OpenParen) {
            if let Some (()) = opt_t {
                // Good!
            } else {
                // Expected closing square, found something else
                return Expression::Error (ParseError::ExpectedToken (TokenClass::OpenParen));
            }
        } else {
            // We ran out of tokens :(
            return Expression::Error (ParseError::UnexpectedEof);
        };

        // Get argument
        let argument = if let Some (opt_t) = tokenizer.expect(TokenClass::Paragraph) {
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

        // Consume closing parenthesis
        if let Some (opt_t) = tokenizer.eat(TokenClass::CloseParen) {
            if let Some (()) = opt_t {
                // Good!
            } else {
                // Expected closing square, found something else
                return Expression::Error (ParseError::ExpectedToken (TokenClass::CloseParen));
            }
        } else {
            // We ran out of tokens :(
            return Expression::Error (ParseError::UnexpectedEof);
        };

        match ctrl.as_str() {
            "image" => Expression::Image {
                alt: value,
                href: argument,
            },
            _ => Expression::Error (ParseError::UnrecognizedControl (ctrl.to_owned())),
        }
    }
}