//! Emitter for the Blog Builder.

use blog_cfg::Config;

use blog_prs::Expression;

/// Emitter for the Blog Builder.
pub struct Emitter {
    /// Configuration information from TOML file.
    config: Config,
}

impl Emitter {
    /// Construct a new emitter from configuration data.
    ///
    /// # Parameters
    /// - `config` (`Config`): configuration information from parsing
    /// TOML input file
    /// 
    /// # Returns
    /// A new `Emitter` structure.
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }

    /// Emit a list of expressions into an HTML output file.
    /// 
    /// # Parameters
    /// - `expressions` (`Vec<Expression>`): a list of expressions to emit
    /// 
    /// # Returns
    /// A `String` to be written to an HTML file.
    pub fn emit(&self, expressions: Vec<Expression>) -> String {
        let mut output = String::new();

        // Begin HTML & head
        output.push_str("<html>\n<head>");

        // Check for analytics
        if let Some (_t) = &self.config.analytics.path {
            todo!();
        }

        // Check for style
        if let Some (_s) = &self.config.site.style {
            todo!();
        }

        // Check for icon
        if let Some (_i) = &self.config.site.icon {
            todo!();
        }

        // Construct full page name
        let mut pagename: Option<String> = None;
        for expr in &expressions {
            if let Expression::Pagename (name) = expr {
                pagename = Some (name.to_owned());
            }
        }
        let full_pagename = if let Some (p) = pagename {
            format!("<title>{} | {}</title>", &p, &self.config.site.name)
        } else {
            format!("<title>{}</title>", &self.config.site.name)
        };
        output.push_str(&full_pagename);

        // End head
        output.push_str("</head>\n");

        // Begin body
        output.push_str("\n<body>\n");

        // Emit each expression
        for expr in expressions {
            let output_str = format!("{}\n", expr);
            output.push_str(&output_str);
        }

        // End body
        output.push_str("\n</body>\n");

        // End HTML
        output.push_str("</html>");

        // Check for menu
        if let Some (_m) = &self.config.menu.path {
            todo!();
            // output = str::replace(&output, "<menu>", &m);
        }

        // // Assemble footnotes
        // let mut footnote_number = 1;
        // let mut new_output = String::new();
        // let mut footnotes = Vec::<String>::new();
        // for footnote in output.split("<footnote>") {
        //     let end_index = footnote.find("</footnote>").unwrap_or(footnote.len());
        //     if end_index < footnote.len() {
        //         let content = &footnote[0..end_index];
        //         let rest = &footnote[end_index+11..];
        //         new_output.push_str(
        //             &format!(
        //                 "<a id=\"citation-{}\" href=\"#footnote-{}\"><sup>{}</sup></a>",
        //                 footnote_number,
        //                 footnote_number,
        //                 footnote_number,
        //             ),
        //         );
        //         footnote_number += 1;
        //         new_output.push_str(&rest);
        //         footnotes.push(content.to_string());
        //     } else {
        //         new_output.push_str(&footnote);
        //     }
        // }
        // output = new_output;

        // let mut footnote_text = "<h3>Footnotes</h3>\n".to_string();
        // for (index, footnote) in footnotes.iter().enumerate() {
        //     footnote_text.push_str(
        //         &format!(
        //             "<p>[<a id=\"footnote-{}\" href=\"#citation-{}\">{}</a>] {}</p>",
        //             index + 1,
        //             index + 1,
        //             index + 1,
        //             footnote,
        //         ),
        //     );
        // }

        // output = str::replace(&output, "<footnotes>", &footnote_text);

        output
    }
}