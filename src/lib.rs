//! Main library for the Blog Builder.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

/// Configuration file name.
pub const CONFIG_FILE_NAME: &str = "blog.toml";

/// Source directory name.
pub const SOURCE_DIR_NAME: &str = "source";

/// Output directory name.
pub const OUTPUT_DIR_NAME: &str = "html";

/// Source file extension.
pub const SOURCE_FILE_EXT: &str = "txt";

/// Output file extension.
pub const OUTPUT_FILE_EXT: &str = "html";

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
    // pub use blog_emt::Emitter;
    // pub use blog_prs::Parser;
}