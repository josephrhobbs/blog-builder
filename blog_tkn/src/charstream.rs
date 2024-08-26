//! Character stream for the Blog Builder tokenizer.

use crate::{
    Token,
    TokenClass,
};

/// A character stream for the Blog Builder tokenizer.
pub struct CharStream {
    /// A list of characters.
    chars: Vec<char>,

    /// The current index in the list.
    index: usize,
}

impl CharStream {
    /// Construct a new character stream from a string.
    /// 
    /// # Parameters
    /// - `string` (`String`): the input string
    /// 
    /// # Returns
    /// A new `CharStream`.
    pub fn from(string: String) -> Self {
        let chars = string.chars().collect::<Vec<char>>();

        Self {
            chars,
            index: 0,
        }
    }

    /// Peek at the next character in the character stream.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `Option<char>` containing the next character in the
    /// character stream, if it is available.
    /// 
    /// **Note**: this method does not advance the character
    /// stream.
    pub fn peek(&self) -> Option<char> {
        if self.index < self.chars.len() {
            Some (self.chars[self.index])
        } else {
            None
        }
    }

    /// Get the next character in the character stream.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `Option<char>` containing the next character in the
    /// character stream, if it is available.
    pub fn next(&mut self) -> Option<char> {
        let next = self.peek();

        self.index += 1;

        next
    }

    /// Get the next token out of this character stream.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `Option<Token>` containing the next token in the
    /// character stream, if it is available.
    pub fn get(&mut self) -> Option<Token> {
        use TokenClass::*;

        // Get the first character in the token
        let first: char = self.next()?;

        // Build the token
        let token = match TokenClass::class(first) {
            Hashes => {
                let mut value = String::new();

                value.push(first);

                // Build the string character-by-character
                while let Some (t) = self.peek() {
                    if TokenClass::class(t) == Hashes {
                        value.push(t);
                        self.next();
                    } else {
                        break;
                    }
                }

                Token {
                    value,
                    class: Hashes,
                }
            },
            Newline => Token {
                class: Newline,
                value: "\n".to_string(),
            },
            Paragraph => {
                let mut value = String::new();

                value.push(first);

                // Build the string character-by-character
                while let Some (t) = self.peek() {
                    if TokenClass::class(t) == Paragraph {
                        value.push(t);
                        self.next();
                    } else {
                        break;
                    }
                }

                Token {
                    value: value.to_string(),
                    class: Paragraph,
                }
            },
        };

        Some (token)
    }
}