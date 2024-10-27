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

        // Initialize list of values
        let mut values = Vec::new();

        // Parse each value
        while let Some (t) = tokenizer.peek() {
            if t.class == TokenClass::OpenSquare {
                // Parse the expression
                let expr = parse_square_expr(tokenizer);

                // Check if any errors occurred
                let string = if let Expression::Text (t) = expr {
                    t
                } else {
                    // This is an error, return it
                    return expr;
                };

                // Store the string
                values.push(string);
            } else {
                break;
            }
        }

        build_expr(&ctrl, values)
    }
}

/// Build a control sequence expression from a list of values.
fn build_expr(ctrl: &str, values: Vec<String>) -> Expression {
    // How long should the list be?
    let len: usize = match ctrl {
        "image" => 2,
        "float" => 2,
        "notice" => 1,
        "tile" => 4,
        _ => return Expression::Error (ParseError::UnrecognizedControl (ctrl.to_owned())),
    };

    // Enforce list length
    if values.len() != len {
        return Expression::Error (ParseError::IncorrectArgumentCount {
            expected: len,
            actual: values.len(),
            control: ctrl.to_string(),
        });
    }

    match ctrl {
        "image" => Expression::FullImage {
            alt: values[0].to_owned(),
            href: values[1].to_owned(),
        },
        "float" => Expression::FloatImage {
            alt: values[0].to_owned(),
            href: values[1].to_owned(),
        },
        "notice" => Expression::Notice (values[0].to_owned()),
        "tile" => Expression::Tile {
            title: values[0].to_owned(),
            description: values[1].to_owned(),
            href: values[2].to_owned(),
            image: values[3].to_owned(),
        },

        // We already checked above that this is a valid control sequence
        _ => unreachable!(),
    }
}

/// Parse a bracketed string into an expression.
fn parse_square_expr(tokenizer: &mut Tokenizer) -> Expression {
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

    Expression::Text (value)
}