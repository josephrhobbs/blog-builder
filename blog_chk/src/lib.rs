//! Source code validation for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

use std::{
    process::exit,
    path::Path,
};

use colored::*;

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
    /// None.
    ///
    /// **Note**: if unrecoverable errors were found in parsing, then
    /// this function exits.
    pub fn validate(expressions: &[Expression], filename: &Path) {
        // Construct user-friendly output
        let mut messages = Vec::new();

        // Iterate over all expressions
        for (i, expression) in expressions.iter().enumerate() {
            if let Expression::Error (p) = expression {
                // Check individual expressions
                messages.push(construct_error(p, filename, i, expressions));
            } else if let Expression::Paragraph (l) = expression {
                // Check nested expressions
                for (j, expr) in l.iter().enumerate() {
                    if let Expression::Error (p) = expr {
                        messages.push(construct_error(p, filename, j, l));
                    }
                }
            }
        }

        // Exit on error message
        if !messages.is_empty() {
            // Print each message
            for message in &messages {
                println!("\n{}", message);
            }

            // Provide summary
            if messages.len() == 1 {
                println!("\n{:>10} due to error message", "Exiting".bold().bright_red());
            } else {
                println!("\n{:>10} due to {} error messages", "Exiting".bold().bright_red(), messages.len());
            }
            
            exit(1);
        }
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
/// A `String` representing the error message
fn construct_error(p: &ParseError, filename: &Path, i: usize, expressions: &[Expression]) -> String {
    // Find location of error
    let mut location = "at beginning of file".to_string();

    // Iterate backwards to find nearest valid expression
    let mut j = i;
    while j > 0 {
        let prev = &expressions[j - 1];
        if let Expression::Error (_) = prev {
            // Skip
        } else if let Expression::Newline = prev {
            // Skip
        } else {
            // Found the location!
            location = format!("after expression '{}'", prev.to_string().bold().bright_blue());
            break;
        }

        // Go back one more expresion
        j -= 1;
    }

    // Construct error message
    format!(
        "{:>10} in file '{}': {} ({})",
        "Error".bold().bright_red(),
        filename.display(),
        p,
        location,
    )
}