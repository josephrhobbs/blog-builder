//! Main library for the Blog Builder.

/// Enforce all warnings.
#[deny(warnings)]

/// Enforce all documentation.
#[deny(missing_docs)]

mod cli;
mod config;
mod error;
mod siteroot;
mod sitetree;

/// Configuration file name.
pub const CONFIG_FILE_NAME: &str = "blog.toml";

/// Source directory name.
pub const SOURCE_DIR_NAME: &str = "source";

/// Source file extension.
pub const SOURCE_FILE_EXT: &str = "txt";

pub use cli::Cli;

pub use config::Config;

pub use error::{
    BlogError,
    BlogResult,
};

pub use siteroot::getroot;

pub use sitetree::SiteTree;