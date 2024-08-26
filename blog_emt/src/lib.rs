//! HTML emitter for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use blog_cfg::{
    Config,
    SiteStyle,
};

use blog_env::STYLESHEET_FILE_NAME;

use blog_prs::Expression;

use blog_sty::links;

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
    /// 
    /// # Returns
    /// A `String` containing HTML.
    pub fn emit(&self, expressions: Vec<Expression>) -> String {
        // Open document and head
        let mut output = String::from("<!DOCTYPE html>\n<html>\n\n<head>\n\n");
        
        // Add links to stylesheet and fonts
        if let Some (s) = &self.config.site.style {
            // Link stylesheet
            output.push_str(&format!("<link rel=\"stylesheet\" href=\"/{}\">\n\n", STYLESHEET_FILE_NAME));

            // Link fonts
            use SiteStyle::*;
            let links = match s {
                Modern => links::MODERN,
                Tech => links::TECH,
            };
            output.push_str(&format!("{}\n\n", links));
        }

        // Close head and open body
        output.push_str("</head>\n\n<body>\n\n");

        // Emit each expression
        for expression in expressions {
            output.push_str(&expression.display(true));
        }

        // Close body and document
        output.push_str("</body>\n\n</html>");

        output
    }
}