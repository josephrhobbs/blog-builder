//! Expressions for the Blog Builder.

use std::fmt::{
    Display,
    self,
};

use crate::ParseError;

#[derive(PartialEq, Clone, Debug)]
/// Expressions available to the Blog Builder.
pub enum Expression {
    /// H1.
    H1 (String),

    /// H2.
    H2 (String),

    /// H3.
    H3 (String),

    /// H4.
    H4 (String),

    /// H5.
    H5 (String),
    
    /// H6.
    H6 (String),

    /// Paragraph (p).
    Paragraph (Vec<Expression>),

    /// Bold text (strong).
    Bold (String),

    /// Italicized text (em).
    Italics (String),

    /// Bold & italicized text (strong + em).
    BoldItalics (String),

    /// Raw text.
    Text (String),

    /// Hyperreference (a).
    Href {
        /// Human-readable text.
        text: String,

        /// URI of reference.
        href: String,
    },

    /// Image (img).
    Image {
        /// Alternate text.
        alt: String,

        /// URI of image.
        href: String,
    },

    /// Tile hyperlink (div.tile).
    Tile {
        /// Title text.
        title: String,

        /// Description.
        description: String,

        /// URI of link.
        href: String,

        /// URI of image.
        image: String,
    },

    /// Notice banner (div.notice).
    Notice (String),

    /// Newline.
    Newline,

    /// Menu.
    Menu,

    /// Parsing error.
    Error (ParseError),
}

// Used for error handling.
impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;
        let output = match self {
            H1 (s) => format!("[h1] {}", s),
            H2 (s) => format!("[h2] {}", s),
            H3 (s) => format!("[h3] {}", s),
            H4 (s) => format!("[h4] {}", s),
            H5 (s) => format!("[h5] {}", s),
            H6 (s) => format!("[h6] {}", s),
            Text (s) => s.to_string(),
            Paragraph (l) => {
                let mut output = String::new();

                for expr in l {
                    output.push_str(&expr.to_string());
                }

                output
            },
            Href {
                text,
                href,
            } => format!("[{}]({})", text, href),
            Image {
                alt,
                href,
            } => format!("[image] [{}][{}]", alt, href),
            Tile {
                title,
                description,
                href,
                image,
            } => format!("[tile] [{}][{}][{}][{}]", title, description, href, image),
            Notice (message) => format!("[notice] [{}]", message),
            Newline => "[newline]".to_string(),
            Menu => "[menu]".to_string(),
            Bold (s) => format!("**{}**", s),
            Italics (s) => format!("_{}_", s),
            BoldItalics (s) => format!("**_{}_**", s),
            Error (e) => e.to_string(),
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
    /// 
    /// # Panics
    /// This function panics when it tries to emit a `Menu` or `Error` variant
    ///     because these should be handled by the emitter directly _before_
    ///     direct conversion to HTML.
    pub fn html(&self, top: bool) -> String {
        use Expression::*;
        match self {
            H1 (s) => format!("<h1>{}</h1>", s),
            H2 (s) => format!("<h2>{}</h2>", s),
            H3 (s) => format!("<h3>{}</h3>", s),
            H4 (s) => format!("<h4>{}</h4>", s),
            H5 (s) => format!("<h5>{}</h5>", s),
            H6 (s) => format!("<h6>{}</h6>", s),
            Text (s) => s.to_string(),
            Paragraph (l) => {
                let mut output = String::new();

                // Format each interior expression
                for expr in l {
                    output.push_str(&expr.html(false));
                }

                if top {
                    format!("<p>{}</p>", output)
                } else {
                    output.to_string()
                }
            },
            Bold (s) => if top {
                format!("<p><strong>{}</strong></p>", s)
            } else {
                format!("<strong>{}</strong>", s)
            },
            Italics (s) => if top {
                format!("<p><em>{}</em></p>", s)
            } else {
                format!("<em>{}</em>", s)
            },
            BoldItalics (s) => if top {
                format!("<p><strong><em>{}</em></strong></p>", s)
            } else {
                format!("<strong><em>{}</em></strong>", s)
            },
            Href {
                text,
                href,
            } => if top {
                format!("<p><a href=\"{}\">{}</a></p>", href, text)
            } else {
                format!("<a href=\"{}\">{}</a>", href, text)
            },
            Image {
                alt,
                href,
            } => format!("<img src=\"{}\" alt=\"{}\">", href, alt),
            Tile {
                title,
                description,
                href,
                image,
            } => format!("<div class=\"tile\" onclick=\"window.location='{}';\" style=\"background-image: url('{}'); cursor: pointer; background-position: center;\"><div>{}</div><br><div class=\"desc\">{}</div></div>", href, image, title, description),
            Notice (message) => format!("<div class=\"notice\">{}</div>", message),
            Newline => "\n\n".to_string(),
            Menu => unreachable!(),
            Error (_) => unreachable!(),
        }
    }
}