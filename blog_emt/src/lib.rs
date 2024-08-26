//! HTML emitter for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use std::path::Path;

use blog_cfg::{
    Config,
    SiteStyle,
};

use blog_env::STYLESHEET_FILE_NAME;

use blog_prs::Expression;

use blog_sty::links;

use convert_case::{
    Case,
    Casing,
};

/// An HTML emitter that takes in a list of expressions and returns
/// HTML source code.
pub struct Emitter {
    #[allow(dead_code)]
    /// Configuration information.
    config: Config,
}

impl Emitter {
    /// Construct a new emitter.
    /// 
    /// # Parameters
    /// - `config` (`Config`): configuration options
    ///
    /// # Returns
    /// A new `Emitter` structure.
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Emit HTML code from a list of expressions.
    ///
    /// # Parameters
    /// - `expressions` (`Vec<Expression>`): the list of expressions
    /// - `filename` (`&Path`): the file stem of the output HTML
    /// 
    /// # Returns
    /// A `String` containing HTML.
    pub fn emit(&self, expressions: Vec<Expression>, filename: &Path) -> String {
        // Open document and head
        let mut output = String::from("<!DOCTYPE html>\n<html>\n\n<head>\n\n");

        // Add title
        let page_title: &str = &filename.file_name().unwrap().to_str().unwrap().to_case(Case::Title);
        output.push_str(&format!("<title>{} | {}</title>", page_title, self.config.site.name));
        
        // Add links to stylesheet and fonts
        if let Some (s) = &self.config.site.style {
            // Link stylesheet
            output.push_str(&format!("<link rel=\"stylesheet\" href=\"/{}\">\n\n", STYLESHEET_FILE_NAME));

            // Link fonts
            use SiteStyle::*;
            let links = match s {
                Tech => links::TECH,
            };
            output.push_str(&format!("{}\n\n", links));
        }

        // Add favicon
        if let Some (f) = &self.config.site.icon {
            // Link favicon
            output.push_str(&format!("<link rel=\"icon\" type=\"image/x-icon\" href=\"/{}\">\n\n", f));
        }

        // Close head and open body
        output.push_str("</head>\n\n<body>\n\n");

        // Emit each expression
        for expression in expressions {
            if expression == Expression::Menu {
                // Generate a menu
                if let Some (m) = &self.config.menu {
                    // Open a new DIV
                    let mut menu = String::from("<div class=\"menu\">\n\n");

                    // Emit menu based on config info
                    for (text, href) in m.names.iter().zip(&m.links) {
                        menu.push_str(&format!("<a href=\"{}\">{}</a>\n\n", href, text));
                    }

                    // Close DIV
                    menu.push_str("</div>\n\n");

                    // Concatenante menu to output
                    output.push_str(&menu);
                } else {
                    // No menu available in TOML, just skip for now
                    continue;
                }
            } else {
                // Output expression as normal
                output.push_str(&expression.display(true));
            }
        }

        // Close body and document
        output.push_str("</body>\n\n</html>");

        output
    }
}