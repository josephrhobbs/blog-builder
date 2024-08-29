//! Source code validation for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use std::path::Path;

use blog_err::{
    BlogError,
    BlogErrorLocation,
    BlogResult,
};

use blog_prs::{
    Expression,
    ParseError,
};

/// A tolerant error handler that validates code before
/// conversion to HTML.
pub struct Handler { }

impl Handler {
    /// Validate a list of expressions.
    /// 
    /// # Parameters
    /// - `expressions` (`Vec<Expression>`): a reference to the list
    ///     of expressions to validate
    /// - `filename` (`&str`): the filename of the file being checked
    /// 
    /// # Returns
    /// A `BlogResult<()>` indicating if parsing was successful.
    ///
    /// **Note**: if unrecoverable errors were found in parsing, then
    /// this function exits.
    pub fn validate(expressions: &[Expression], filename: &Path) -> BlogResult<()> {
        // Construct user-friendly output
        let mut result = BlogResult::default();

        // Iterate over all expressions
        for (i, expression) in expressions.iter().enumerate() {
            // If this expression is an error, construct an error message
            // 
            // All errors must occur at the top level, so we don't need to
            // recurse through nested expressions
            if let Expression::Error (p) = expression {
                // Check individual expressions
                result = result.err(construct_error(p, filename, i, expressions));
            }
        }

        result
    }
}

/// Construct an error message.
/// 
/// # Parameters
/// - `p` (`&ParseError`): the parse error
/// - `filename` (`&Path`): the file path of the error
/// - `i` (`usize`): the index of the erroneous expression in the list of expressions
/// - `expressions` (`&Vec<Expression>`): a reference to the list of expressions
/// 
/// # Returns
/// A `BlogError` representing the error message.
fn construct_error(p: &ParseError, filename: &Path, i: usize, expressions: &[Expression]) -> BlogError {
    // Find location of error
    let mut location = BlogErrorLocation::Beginning;

    // Iterate backwards to find nearest valid expression
    let mut j = i;
    while j > 0 {
        let prev = &expressions[j - 1];
        if let Expression::Error (_) = prev {
            // Skip, not a valid expression
        } else if let Expression::Newline = prev {
            // Skip, not useful
        } else {
            // Found the nearest valid expression!
            location = BlogErrorLocation::After (prev.to_string());
            break;
        }

        // Go back one more expresion
        j -= 1;
    }

    // Construct error message
    BlogError::ParseError {
        filename: filename.to_owned(),
        message: p.to_string(),
        location,
    }
}