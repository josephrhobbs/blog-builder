//! Token enumeration for the Blog Builder.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// Types of tokens available to the Blog Builder.
pub enum TokenClass {
    /// String token.
    Paragraph,

    /// Hash token.
    Hashes,

    /// Newline token.
    Newline,
}

impl TokenClass {
    /// Display this token class.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `String` for display.
    pub fn display(&self) -> String {
        use TokenClass::*;
        let string = match self {
            Paragraph => "Paragraph",
            Hashes => "Hashes",
            Newline => "Newline",
        };

        string.to_string()
    }

    /// Get the class associated to a given
    /// start character.
    /// 
    /// # Parameters
    /// - `c` (`char`): the character
    /// 
    /// # Returns
    /// A `TokenClass`.
    pub fn class(c: char) -> Self {
        use TokenClass::*;
        match c {
            '#' => Hashes,
            '\n' => Newline,
            _ => Paragraph,
        }
    }
}

#[derive(Clone, Debug)]
/// Token structure.
pub struct Token {
    /// Token value.
    pub value: String,

    /// Token class.
    pub class: TokenClass,
}