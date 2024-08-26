//! Stylesheets for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

/// Stylesheets.
pub mod style {
    /// Modern stylesheet.
    pub const MODERN: &str = include_str!("../stylesheets/modern.css");

    /// Technology stylesheet.
    pub const TECH: &str = include_str!("../stylesheets/tech.css");
}

/// Links to fonts, etc.
pub mod links {
    /// Modern links.
    pub const MODERN: &str = include_str!("../links/modern.html");

    /// Technology links.
    pub const TECH: &str = include_str!("../links/tech.html");
}