//! Source-to-output conversion for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use std::path::Path;

use blog_cfg::Config;

use blog_chk::Handler;

use blog_emt::Emitter;

use blog_prs::Parser;

use blog_tkn::Tokenizer;

/// Convert a source file into an output file.
/// 
/// # Parameters
/// - `source` (`String`): the source code
/// - `filename` (`&Path`): the filename
/// - `config` (`&Config`): a reference to the configuration
/// information
///
/// # Returns
/// A `String` containing the HTML output code.
pub fn convert(source: String, filename: &Path, config: &Config) -> String {
    // Construct a new tokenizer
    let mut tokenizer = Tokenizer::from(source);

    // Construct a new parser
    let parser = Parser::new();

    // Parse tokens
    let expressions = parser.parse(&mut tokenizer);

    // Validate parser output or exit
    Handler::validate(&expressions, filename);

    // Construct a new emitter
    let emitter = Emitter::new(config);

    // Emit HTML
    emitter.emit(expressions)
}