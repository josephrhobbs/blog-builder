//! Source-to-output conversion for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use blog_cfg::Config;

/// Convert a source file into an output file.
/// 
/// # Parameters
/// - `source` (`String`): the source code
/// - `config` (`&Config`): a reference to the configuration
/// information
///
/// # Returns
/// A `String` containing the HTML output code.
pub fn convert(_source: String, _config: &Config) -> String {
    todo!()
}