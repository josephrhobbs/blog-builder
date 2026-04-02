//! Stylesheets for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

/// Stylesheets.
pub mod style {
    /// Technology stylesheet.
    pub const TECH: &str = include_str!("../stylesheets/tech.css");

    /// Book stylesheet.
    pub const BOOK: &str = include_str!("../stylesheets/book.css");

    /// Linotype stylesheet.
    pub const LINOTYPE: &str = include_str!("../stylesheets/linotype.css");
}

/// Links to fonts, etc.
pub mod links {
    /// Technology links.
    pub const TECH: &str = include_str!("../links/tech.html");

    /// Book links.
    pub const BOOK: &str = include_str!("../links/book.html");

    /// Linotype links.
    pub const LINOTYPE: &str = include_str!("../links/linotype.html");
}
