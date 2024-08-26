//! Parsing errors for the Blog Builder.

use std::fmt::{
    Display,
    self,
};

use blog_tkn::TokenClass;

#[derive(PartialEq, Clone, Debug)]
/// Parse errors available to the Blog Builder.
pub enum ParseError {
    /// Unexpected EOF.
    UnexpectedEof,

    /// Expected token of given class.
    ExpectedToken (TokenClass),

    /// Unrecognized emphasis sequence.
    UnrecognizedEmphasis,

    /// Too many hashes.
    TooManyHashes,

    /// Mismatched delimiters.
    MismatchedDelimiters,

    /// Unrecognized control sequence.
    UnrecognizedControl (String),

    /// No parselet available for token class.
    NoParselet (TokenClass),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError::*;
        let output = match self {
            UnexpectedEof => "unexpected EOF",
            ExpectedToken (c) => &format!("expected token of class '{}'", c.display()),
            TooManyHashes => "too many hashes provided",
            UnrecognizedEmphasis => "unrecognized emphasis sequence",
            UnrecognizedControl (c) => &format!("unrecognized control sequence '{}'", c),
            MismatchedDelimiters => "mismatched delimiters",
            NoParselet (c) => &format!("could not handle token of class '{}'", c.display()),
        };

        write!(f, "{}", output)
    }
}