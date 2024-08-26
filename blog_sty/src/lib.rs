//! Stylesheets for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

/// Stylesheets.
pub mod style {
    /// Technology stylesheet.
    pub const TECH: &str = include_str!("../stylesheets/tech.css");
}

/// Links to fonts, etc.
pub mod links {
    /// Technology links.
    pub const TECH: &str = include_str!("../links/tech.html");
}