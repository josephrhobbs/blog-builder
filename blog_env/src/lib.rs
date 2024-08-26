//! Library for Blog Builder environment information.

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

/// Index file name.
pub const INDEX_FILE_NAME: &str = "index.txt";

/// Stylesheet name.
pub const STYLESHEET_FILE_NAME: &str = "style.css";

/// Default index file.
pub const DEFAULT_INDEX: &str = include_str!("../index.txt.example");

/// Default configuation file.
pub const DEFAULT_CONFIG: &str = include_str!("../blog.toml.example");

/// Help menu.
pub const HELP: &str = include_str!("../help.txt");