//! Parsing errors for the Blog Builder.

use blog_tkn::TokenClass;

#[derive(Clone, Debug)]
/// Parse errors available to the Blog Builder.
pub enum ParseError {
    /// Unexpected EOF.
    UnexpectedEof,

    /// Expected token of given class.
    ExpectedToken (TokenClass),

    /// Too many hashes
    TooManyHashes,

    /// No parselet available for token class.
    NoParselet (TokenClass),
}