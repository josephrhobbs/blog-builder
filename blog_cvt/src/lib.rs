//! Source-to-output conversion for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use std::path::Path;

use colored::*;

use blog_cfg::Config;

use blog_chk::Handler;

use blog_emt::Emitter;

use blog_err::{
    BlogResult,
    unwrap_or_return,
};

use blog_prs::{
    Parser,
    Expression,
};

use blog_tkn::Tokenizer;

/// Convert a source file into an output file.
/// 
/// # Parameters
/// - `source` (`String`): the source code
/// - `root` (`&Path`): the root directory of the site
/// - `filename` (`&Path`): the filename
/// - `config` (`&Config`): a reference to the configuration
///     information
/// - `verbosity` (`usize`): the verbosity level
///
/// # Returns
/// A `BlogResult<String>` containing the HTML output code or any errors.
pub fn convert(source: String, root: &Path, filename: &Path, config: &Config, verbosity: usize) -> BlogResult<String> {
    // Construct a new tokenizer
    let mut tokenizer = Tokenizer::from(source);

    // Construct a new parser
    let parser = Parser::new();

    // Parse tokens
    let expressions = parser.parse(&mut tokenizer);

    // Print each expression, if very verbose
    if verbosity > 2 {
        for expr in &expressions {
            // All errors occur at the top level, so we can just check
            //  for errors here without the need to recurse
            match expr {
                Expression::Error (e) => println!("{:>10} {}", "Error".bright_red(), e),
                _ => println!("{:>10} '{}'", "Parsed".bright_yellow(), expr),
            }
        }
    }

    // Validate parser output or return errors
    unwrap_or_return!(Handler::validate(&expressions, filename));

    // Construct a new emitter
    let emitter = Emitter::new(config);

    // Emit HTML or return errors
    emitter.emit(expressions, root, filename)
}