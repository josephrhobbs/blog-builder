//! Expressions for the Blog Builder.

use std::fmt::{
    self,
    Display,
};

use crate::ParseError;

#[derive(Clone, Debug)]
/// Expressions available to the Blog Builder.
pub enum Expression {
    /// Title (h1).
    Title (String),

    /// Header (h2).
    Header (String),

    /// Subheader (h3).
    Subheader (String),

    /// Paragraph (p).
    Paragraph (String),

    /// Newline.
    Newline,

    /// Parsing error.
    Error (ParseError),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;
        let output = match self {
            Title (s) => format!("<h1>{}</h1>", s),
            Header (s) => format!("<h2>{}</h2>", s),
            Subheader (s) => format!("<h3>{}</h3>", s),
            Paragraph (s) => format!("<p>{}</p>", s),
            Newline => "\n\n".to_string(),
            Error (_) => unreachable!(),
        };

        write!(f, "{}", output)
    }
}