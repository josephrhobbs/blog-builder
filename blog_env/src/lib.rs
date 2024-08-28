//! Library for Blog Builder environment information.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

mod help;

pub use help::help;

/// Software version number.
pub const VERSION: &str = "0.1.0";

/// Configuration file name.
pub const CONFIG_FILE_NAME: &str = "blog.toml";

/// Source directory name.
pub const SOURCE_DIR_NAME: &str = "source";

/// Media subdirectory name.
pub const MEDIA_DIR_NAME: &str = "media";

/// Output directory name.
pub const OUTPUT_DIR_NAME: &str = "html";

/// Source file extension.
pub const SOURCE_FILE_EXT: &str = "txt";

/// Output file extension.
pub const OUTPUT_FILE_EXT: &str = "html";

/// Index file name.
pub const INDEX_FILE_NAME: &str = "index.txt";

/// Index page name.
/// 
/// The Blog Builder will omit this from the title
/// of any page with this name.  This forces `index.html`
/// to have a title that is only the website name (for
/// example, "My Website" rather than "Index | My Website").
pub const INDEX_PAGE_NAME: &str = "Index";

/// Stylesheet name.
pub const STYLESHEET_FILE_NAME: &str = "style.css";

/// JavaScript file name.
pub const JAVASCRIPT_FILE_NAME: &str = "script.js";

/// Default index file.
pub const DEFAULT_INDEX: &str = include_str!("../index.txt.example");

/// Default configuation file.
pub const DEFAULT_CONFIG: &str = include_str!("../blog.toml.example");