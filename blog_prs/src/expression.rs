//! Enumerates the types of expressions available in the Blog Builder.

use std::{
    fmt::{
        Display,
        Formatter,
        Result,
    },
    fs,
};

use chrono::prelude::*;

use blog_err::BlogError;

#[derive(PartialEq, Clone, Debug)]
/// Types of expressions available to the Blog Builder.
pub enum Expression {
    /// Webpage title equivalent to HTML `h2`.
    Title (Vec<Expression>),

    /// Webpage heading equivalent to HTML `h3`.
    Heading (Vec<Expression>),

    /// Webpage subheading equivalent to HTML `h4`.
    Subheading (Vec<Expression>),

    /// Webpage subtitle equivalent to HTML `h5`.
    Subtitle (Vec<Expression>),

    /// Webpage subsubtitle equivalent to HTML `h6`.
    Subsubtitle (Vec<Expression>),

    /// Webpage paragraph equivalent to HTML `p`.
    Paragraph (Vec<Expression>),

    /// Alphanumeric string.
    Alphanumeric (String),
    
    /// Newline character equivalent to HTML `<br>`.
    Newline,

    /// Bold text.
    Bold (Vec<Expression>),

    /// Italic text.
    Italic (Vec<Expression>),

    /// Block quote.
    BlockQuote {
        /// Quote content.
        quote: Vec<Expression>,

        /// Quote citation.
        citation: Vec<Expression>,
    },

    /// Hyperlink to another URL.
    Hyperlink {
        /// Text displayed.
        name: Vec<Expression>,
        
        /// Reference or URL.
        href: String,
    },

    /// Software source code.
    Code {
        /// Language of the source code.
        language: String,

        /// Source code.
        code: String,
    },

    /// Tile with image and hyperlink to another page.
    Tile {
        /// Name displayed on the tile.
        name: Vec<Expression>,
        
        /// URL to the image of the tile.
        img: String,

        /// Hyperlink followed after clicking on the tile.
        href: String,
    },

    /// Tile with description, image, and hyperlink to another page.
    TileDesc {
        /// Name displayed on the tile.
        name: Vec<Expression>,

        /// Description displayed on the tile.
        desc: Vec<Expression>,

        /// URL to the image of the tile.
        img: String,

        /// Hyperlink followed after clicking on the tile.
        href: String,
    },

    /// Image.
    Image {
        /// URL to the image source.
        img: String,

        /// Alternate text.
        alt: String,

        /// Scale of the image.
        scale: String,
    },

    /// Floating image within a block of text.
    FloatingImage {
        /// URL to the source image.
        img: String,

        /// Alternate text.
        alt: String,
    },

    /// Header equivalent to HTML `h1`.
    Header (Vec<Expression>),

    /// Footer equivalent to HTML `h6` with class `footer`.
    Footer (Vec<Expression>),

    /// Footnote.
    Footnote (Vec<Expression>),

    /// Collection of footnotes.
    Footnotes,

    /// Block at the top of a webpage.
    Topblock (Vec<Expression>),

    /// Web menu.
    Menu,

    /// "Last Updated" date.
    Date,

    /// Collection of tiles.
    Tiles (Vec<Expression>),

    /// Webpage name.
    Pagename (String),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Expression::*;
        
        let value = match self {
            Title (v) => {
                let mut inside = String::new();
                inside.push_str("<h2>");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</h2>");
                inside
            },
            Subtitle (v) => {
                let mut inside = String::new();
                inside.push_str("<h5>");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</h5>");
                inside
            },
            Subsubtitle (v) => {
                let mut inside = String::new();
                inside.push_str("<h6>");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</h6>");
                inside
            },
            Heading (v) => {
                let mut inside = String::new();
                inside.push_str("<h3>");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</h3>");
                inside
            },
            Subheading (v) => {
                let mut inside = String::new();
                inside.push_str("<h4>");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</h4>");
                inside
            },
            Alphanumeric (s) => {
                s.to_owned()
            },
            Paragraph (v) => {
                let mut inside = String::new();
                inside.push_str("<p>");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside
            },
            Newline => {
                String::new()
            },
            Bold (v) => {
                let mut inside = String::new();
                inside.push_str(" <strong>");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</strong> ");
                inside
            },
            Italic (v) => {
                let mut inside = String::new();
                inside.push_str(" <em>");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</em> ");
                inside
            },
            BlockQuote {
                quote: q,
                citation: c,
            } => {
                let mut inside = String::new();
                inside.push_str(" <p class=\"block\">");
                for expr in q {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</p><p class=\"citation\">~ ");
                for expr in c {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</p>");
                inside
            },
            Hyperlink {
                name: n,
                href: h,
            } => {
                let mut inside = String::new();
                inside.push_str(" <a href=\"");
                inside.push_str(&h);
                inside.push_str("\">");
                for expr in n {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</a> ");
                inside
            },
            Code {
                language: l,
                code: f,
            } => {
                let mut code = match fs::read_to_string(f) {
                    Ok (c) => c,
                    Err (_) => BlogError::CannotOpenFile (f.to_string()).throw(),
                };

                // Raw < and > in HTML will cause the browser
                // to interpret this as a tag
                code = code.replace("<", "&lt;");
                code = code.replace(">", "&gt;");

                format!(
                    "<pre><code class=\"language-{}\">{}</code></pre>",
                    l,
                    code,
                )
            },
            Tile {
                name: n,
                img: i,
                href: h,
            } => {
                let mut inside = format!(
                    " <div class=\"tile\" onclick=\"window.location=\'{}\';\" style=\"background-image: url('{}'); cursor: pointer; background-position: center;\"><div>",
                    &h,
                    &i,
                );
                for expr in n {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</div></div> ");
                inside
            },
            TileDesc {
                name: n,
                desc: d,
                img: i,
                href: h,
            } => {
                let mut inside = format!(
                    " <div class=\"tile\" onclick=\"window.location=\'{}\';\" style=\"background-image: url('{}'); cursor: pointer; background-position: center;\"><div>",
                    &h,
                    &i,
                );
                for expr in n {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</div><br><div class=\"desc\">");
                for expr in d {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</div></div> ");
                inside
            },
            Image {
                img: i,
                alt: a,
                scale: s,
            } => format!(
                " <img src=\"{}\" style=\"height: {}px;\" alt=\"{}\"> ",
                i,
                s,
                a,
            ),
            FloatingImage {
                img: i,
                alt: a,
            } => format!(
                " <img src=\"{}\" class=\"floating\" alt=\"{}\"> ",
                i,
                a,
            ),
            Header (v) => {
                let mut inside = String::new();
                inside.push_str("<h1>");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</h1>");
                inside
            },
            Footer (v) => {
                let mut inside = String::new();
                inside.push_str("<h6 class=\"footer\">");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</h6>");
                inside
            },
            Footnote (v) => {
                let mut inside = String::new();
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                format!(
                    "<footnote>{}</footnote>",
                    &inside,
                )
            },
            Footnotes => {
                "<footnotes>".to_string()
            }
            Topblock (v) => {
                let mut inside = String::new();
                inside.push_str("<div class=\"topblock\">");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("</div>");
                inside
            }
            Menu => {
                "<menu>".to_string()
            },
            Date => {
                let local: DateTime<Local> = Local::now();
                let date = local.format("%A, %B %d, %Y").to_string();

                format!(
                    "<h6 class=\"last-updated-date\">Last Updated {}</h6>",
                    date
                )
            },
            Tiles (v) => {
                let mut inside = String::new();
                inside.push_str("<div class=\"tiles\">\n");
                for expr in v {
                    let string = format!("{}", expr);
                    inside.push_str(&string);
                }
                inside.push_str("\n</div>");
                inside
            },
            Pagename (_) => {
                String::new()
            },
        };

        write!(f, "{}", value)
    }
}