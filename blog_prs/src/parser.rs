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
        ControlParselet,
        EmphasisParselet,
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
    /// None.mod image;

    /// 
    /// # Returns
    /// A new `Parser`.
    pub fn new() -> Self {
        let mut parselets: HashMap<TokenClass, Box<dyn Parselet>> = HashMap::new();

        // Declarative grammar begins here
        use TokenClass::*;
        parselets.insert(Hashes, Box::new(HeaderParselet { }));
        parselets.insert(Paragraph, Box::new(ParagraphParselet { }));
        parselets.insert(Newline, Box::new(NewlineParselet { }));
        parselets.insert(Menu, Box::new(MenuParselet { }));
        parselets.insert(OpenSquare, Box::new(HrefParselet { }));
        parselets.insert(Emphasis, Box::new(EmphasisParselet { }));
        parselets.insert(Control, Box::new(ControlParselet { }));

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

        while let Some (_) = tokenizer.peek() {
            // Get the next expression from the token stream
            let expression = self.parse_next(tokenizer);

            output.push(expression);
        }

        output
    }

    /// Parse the next expression out of the tokenizer.
    /// 
    /// # Parameters
    /// - `tokenizer` (`&mut Tokenizer`): a reference to the input token stream
    /// 
    /// # Returns
    /// An `Expression` containing the next expression in the stream.
    pub fn parse_next(&self, tokenizer: &mut Tokenizer) -> Expression {
        // Get the next token in the stream
        let token = if let Some (t) = tokenizer.next() {
            t
        } else {
            // We checked to make sure there was another token
            unreachable!()
        };
    
        // Get the necessary parselet
        let parselet = if let Some (p) = self.parselets.get(&token.class) {
            p
        } else {
            // No parselet available
            return Expression::Error (ParseError::NoParselet (token.class));
        };

        // Parse as far as possible and return
        parselet.parse(self, tokenizer, &token)
    }
}