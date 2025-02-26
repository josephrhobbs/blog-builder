//! HTML emitter for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use std::{
    fs,
    path::Path,
};

use blog_cfg::{
    Config,
    SiteStyle,
};

use blog_err::{
    BlogResult,
    unwrap_result,
};

use blog_env::{
    SOURCE_DIR_NAME,
    STYLESHEET_FILE_NAME,
    INDEX_PAGE_NAME,
};

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
    /// - `root` (`&Path`): the location of the site root
    /// - `filename` (`&Path`): the file stem of the output HTML
    /// 
    /// # Returns
    /// A `String` containing HTML.
    pub fn emit(&self, expressions: Vec<Expression>, root: &Path, filename: &Path) -> BlogResult<String> {
        // Open document and head
        let mut output = String::from("<!DOCTYPE html>\n<html>\n\n<head>\n\n");

        // Initialize result
        let mut result = BlogResult::default();

        // Add analytics tag
        if let Some (a) = &self.config.analytics {
            // Get tag path
            let analytics_tag_path = root.join(SOURCE_DIR_NAME).join(&a.tag);

            // Read analytics file
            let analytics = unwrap_result!(
                fs::read_to_string(&analytics_tag_path),
                result,
                &format!("could not read analytics tag '{}'", analytics_tag_path.display())
            );

            output.push_str(&format!("{}\n\n", analytics));
        }

        // Construct title
        let filename_str: &str = filename.file_name().unwrap().to_str().unwrap();
        let page_title = &filename_str.to_case(Case::Title);

        // Remove "Index" from `index.html`
        if page_title == INDEX_PAGE_NAME {
            output.push_str(&format!("<title>{}</title>\n\n", self.config.site.name));
        } else {
            output.push_str(&format!("<title>{} | {}</title>\n\n", page_title, self.config.site.name));
        }
        
        // Add links to stylesheet and fonts
        if let Some (s) = &self.config.site.style {
            // Link stylesheet
            output.push_str(&format!("<link rel=\"stylesheet\" href=\"/{}\">\n\n", STYLESHEET_FILE_NAME));

            // Link fonts
            use SiteStyle::*;
            let links = match s {
                Tech => links::TECH,
                Book => links::BOOK,
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
                output.push_str(&expression.html(true));
            }
        }

        // Close body and document
        output.push_str("</body>\n\n</html>");

        match result {
            BlogResult::Ok (_) => result.ok(output),
            BlogResult::Err (_) => result,
        }
    }
}