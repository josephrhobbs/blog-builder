//! HTML emitter for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use blog_cfg::Config;

use blog_prs::Expression;

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
        let mut output = String::new();

        for expression in expressions {
            output.push_str(&expression.to_string());
        }

        output
    }
}