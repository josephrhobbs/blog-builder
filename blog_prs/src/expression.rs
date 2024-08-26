//! Expressions for the Blog Builder.

use std::fmt::{
    Display,
    self,
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

    /// Hyperreference (a).
    Href {
        /// Human-readable text.
        text: String,

        /// URI of reference.
        href: String,
    },

    /// Newline.
    Newline,

    /// Parsing error.
    Error (ParseError),
}

// For error handling
impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;
        let output = match self {
            Title (s) => format!("# {}", s),
            Header (s) => format!("## {}", s),
            Subheader (s) => format!("### {}", s),
            Paragraph (s) => format!("{}", s),
            Href {
                text,
                href,
            } => format!("[{}]({})", href, text),
            Newline => "[newline]".to_string(),
            Error (_) => unreachable!(),
        };

        write!(f, "{}", output)
    }
}

impl Expression {
    /// Convert the expression to an HTML string.
    /// 
    /// # Parameters
    /// - `top` (`bool`): indicates whether this call is at the top level
    /// 
    /// # Returns
    /// A `String` with the formatted expression.
    pub fn display(&self, top: bool) -> String {
        use Expression::*;
        let output = match self {
            Title (s) => format!("<h1>{}</h1>", s),
            Header (s) => format!("<h2>{}</h2>", s),
            Subheader (s) => format!("<h3>{}</h3>", s),
            Paragraph (s) => if top {
                format!("<p>{}</p>", s)
            } else {
                format!("{}", s)
            },
            Href {
                text,
                href,
            } => if top {
                format!("<p><a href=\"{}\">{}</a></p>", href, text)
            } else {
                format!("<a href=\"{}\">{}</a>", href, text)
            },
            Newline => "\n\n".to_string(),
            Error (_) => unreachable!(),
        };

        output
    }
}