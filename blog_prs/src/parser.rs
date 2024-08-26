//! Parser for the Blog Builder.

use std::collections::HashMap;

use blog_tkn::{
    Tokenizer,
    TokenClass,
};

use crate::{
    Expression,
    Parselet,
    ParseError,
    parselets::{
        HeaderParselet,
        HrefParselet,
        ParagraphParselet,
        MenuParselet,
        NewlineParselet,
    },
};

/// A parser that takes in a stream of tokens and outputs
/// a list of expressions.
pub struct Parser {
    /// Parselets.
    parselets: HashMap<TokenClass, Box<dyn Parselet>>,
}

impl Parser {
    /// Construct a new parser from a token stream.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A new `Parser`.
    pub fn new() -> Self {
        let mut parselets: HashMap<TokenClass, Box<dyn Parselet>> = HashMap::new();

        // Declarative grammar begins here
        parselets.insert(TokenClass::Hashes, Box::new(HeaderParselet { }));
        parselets.insert(TokenClass::Paragraph, Box::new(ParagraphParselet { }));
        parselets.insert(TokenClass::Newline, Box::new(NewlineParselet { }));
        parselets.insert(TokenClass::Menu, Box::new(MenuParselet { }));
        parselets.insert(TokenClass::OpenSquare, Box::new(HrefParselet { }));

        Self {
            parselets,
        }
    }

    /// Parse a list of expressions.
    /// 
    /// # Parameters
    /// - `tokenizer` (`&mut Tokenizer`): a reference to the input token stream
    /// 
    /// # Returns
    /// A `Vec<Expression>` containing the list of expressions.
    pub fn parse(&self, tokenizer: &mut Tokenizer) -> Vec<Expression> {
        // Initialize output list of expressions
        let mut output = Vec::new();

        while let Some (token) = tokenizer.next() {
            // Get the necessary parselet
            let parselet = if let Some (p) = self.parselets.get(&token.class) {
                p
            } else {
                // No parselet available
                output.push(Expression::Error (ParseError::NoParselet (token.class)));
                continue;
            };

            // Parse as far as possible
            let expression = parselet.parse(self, tokenizer, &token);

            output.push(expression);
        }

        output
    }
}