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

use blog_prs::Expression;

/// A tolerant error handler that validates code before
/// conversion to HTML.
pub struct Handler { }

impl Handler {
    /// Validate a list of expressions.
    /// 
    /// # Parameters
    /// - `expressions` (`Vec<Expression>`): a reference to the list
    /// of expressions to validate
    /// - `filename` (`&str`): the filename of the file being checked
    /// 
    /// # Returns
    /// None.
    ///
    /// **Note**: if unrecoverable errors were found in parsing, then
    /// this function exits.
    pub fn validate(expressions: &Vec<Expression>, filename: &Path) {
        // Construct user-friendly output
        let mut messages = Vec::new();

        // Iterate over all expressions
        for (i, expression) in expressions.iter().enumerate() {
            if let Expression::Error (p) = expression {
                // Find location of error
                let mut location = "at beginning of file".to_string();

                // Iterate backwards
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
                let error_message = format!(
                    "{:>10} in file '{}': {} ({})",
                    "Error".bold().bright_red(),
                    filename.display(),
                    p,
                    location,
                );

                // Add error message to list
                messages.push(error_message);
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
                println!("\n{:>10} due to 1 error message", "Exited".bold().bright_red());
            } else {
                println!("\n{:>10} due to {} error messages", "Exited".bold().bright_red(), messages.len());
            }
            
            exit(1);
        }
    }
}