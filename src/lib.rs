//! Main library for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

/// Command-line interface.
pub mod cli {
    pub use blog_cli::{
        Cli,
        Subcommand,
    };
}

/// Configuration information.
pub mod cfg {
    pub use blog_cfg::{
        Config,
    };
}

/// Source-to-output conversion.
pub mod cvt {
    pub use blog_cvt::convert;
}

/// User help utility.
pub mod help {
    pub use blog_env::HELP;
}

/// Error handling.
pub mod err {
    pub use blog_err::{
        BlogError,
        BlogResult,
    };
}

/// Website tree management.
pub mod site {
    pub use blog_str::SiteTree;
}