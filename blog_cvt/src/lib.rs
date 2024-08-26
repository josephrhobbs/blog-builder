//! Source-to-output conversion for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use blog_cfg::Config;

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
pub fn convert(source: String, _config: &Config) -> String {
    // Construct a new tokenizer
    let tokenizer = Tokenizer::from(source);

    dbg!(tokenizer);

    String::new()
}