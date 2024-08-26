//! Stylesheets for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

/// Stylesheets.
pub mod style {
    /// Modern stylesheet.
    pub const MODERN: &str = include_str!("../stylesheets/modern.css");
}