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

    /// Square bracket nesting depth.
    /// 
    /// Text inside bracketed expressions ignore emphasis, etc.
    brackets: usize,

    /// Parenthesis nesting depth.
    /// 
    /// Text inside bracketed expressions ignore emphasis, etc.
    parens: usize,

    /// Are we in an equation?
    equation: bool,
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
            brackets: 0,
            parens: 0,
            equation: false,
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
        self.look_ahead(0)
    }

    /// Look ahead `chars` characters in the character stream.
    /// 
    /// # Parameters
    /// - `chars` (`usize`): the number of characters to look ahead
    ///     (zero is the next character).
    /// 
    /// # Returns
    /// An `Option<char>` containing the character, if it is
    ///     available.
    /// 
    /// **Note**: this method does not advance the character
    /// stream.
    pub fn look_ahead(&self, chars: usize) -> Option<char> {
        if self.index + chars < self.chars.len() {
            Some (self.chars[self.index + chars])
        } else {
            None
        }
    }

    #[allow(clippy::should_implement_trait)]
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
            Emphasis => if !self.equation {
                let mut value = String::new();

                value.push(first);

                // Build the string character-by-character
                while let Some (t) = self.peek() {
                    if TokenClass::class(t) == Emphasis {
                        value.push(t);
                        self.next();
                    } else {
                        break;
                    }
                }

                Token {
                    value,
                    class: Emphasis,
                }
            } else {
                Token {
                    class: Paragraph,
                    value: first.to_string(),
                }
            },
            Paragraph => {
                let mut value = String::new();

                value.push(first);

                // Build the string character-by-character
                // 
                // When inside brackets, ignore everything except
                //  close brackets
                while let Some (t) = self.peek() {
                    let class = TokenClass::class(t);
                    if class == Paragraph
                        || class == Control
                        || (self.brackets != 0 && class != CloseSquare)
                        || (self.parens != 0 && class != CloseParen)
                    {
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
            Newline => Token {
                class: Newline,
                value: "\n".to_string(),
            },
            OpenParen => {
                // Increment nesting depth
                self.parens += 1;

                Token {
                    class: OpenParen,
                    value: "(".to_string(),
                }
            },
            CloseParen => {
                // Decrement nesting depth and
                //  avoid silent underflow
                if self.parens > 0 {
                    self.parens -= 1;
                }

                Token {
                    class: CloseParen,
                    value: ")".to_string(),
                }
            },
            OpenSquare => {
                // Increment nesting depth
                self.brackets += 1;

                Token {
                    class: OpenSquare,
                    value: "[".to_string(),
                }
            },
            CloseSquare => {
                // Decrement nesting depth and
                //  avoid silent underflow
                if self.brackets > 0 {
                    self.brackets -= 1;
                }
                
                Token {
                    class: CloseSquare,
                    value: "]".to_string(),
                }
            },
            Control => if let Some (t) = self.peek() {
                // Check for a second colon
                if TokenClass::class(t) == Control {
                    // Consume the second colon
                    let _ = self.next();
                    Token {
                        class: Control,
                        value: "::".to_string(),
                    }
                } else {
                    // We didn't get our second colon
                    return None;
                }
            } else {
                // Unexpected EOF
                return None;
            },
            Backslash => if let Some (t) = self.peek() {
                // Check for a bracket or parenthesis (this indicates an equation)
                if TokenClass::class(t) == OpenSquare {
                    // Consume the bracket
                    let _ = self.next();

                    // We're now in an equation
                    self.equation = true;

                    Token {
                        class: Paragraph,
                        value: "\\[".to_string(),
                    }
                } else if TokenClass::class(t) == CloseSquare {
                    // Consume the bracket
                    let _ = self.next();

                    // We're no longer in an equation
                    self.equation = false;

                    Token {
                        class: Paragraph,
                        value: "\\]".to_string(),
                    }
                } else if TokenClass::class(t) == OpenParen {
                    // Consume the parenthesis
                    let _ = self.next();

                    // We're now in an equation
                    self.equation = true;

                    Token {
                        class: Paragraph,
                        value: "\\(".to_string(),
                    }
                } else if TokenClass::class(t) == CloseParen {
                    // Consume the parenthesis
                    let _ = self.next();

                    // We're no longer in an equation
                    self.equation = false;

                    Token {
                        class: Paragraph,
                        value: "\\)".to_string(),
                    }
                } else {
                    // We didn't get our bracket
                    Token {
                        class: Paragraph,
                        value: "\\".to_string(),
                    }
                }
            } else {
                // Unexpected EOF
                return None;
            },
            Menu => Token {
                class: Menu,
                value: "~".to_string(),
            },
        };

        Some (token)
    }
}