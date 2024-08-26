//! Source-to-output conversion for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use blog_cfg::Config;

use blog_emt::Emitter;

use blog_prs::Parser;

use blog_tkn::Tokenizer;

/// Convert a source file into an output file.
/// 
/// # Parameters
/// - `source` (`String`): the source code
/// - `config` (`&Config`): a reference to the configuration
/// information
///
/// # Returns
/// A `String` containing the HTML output code.
pub fn convert(source: String, config: &Config) -> String {
    // Construct a new tokenizer
    let mut tokenizer = dbg!(Tokenizer::from(source));

    // Construct a new parser
    let parser = Parser::new();

    // Parse tokens
    let expressions = dbg!(parser.parse(&mut tokenizer));

    // Construct a new emitter
    let emitter = Emitter::new(config);

    // Emit HTML
    let html = emitter.emit(expressions);

    dbg!(html)
}