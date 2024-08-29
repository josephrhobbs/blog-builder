//! Error handling for the Blog Builder.

use std::{
    error::Error,
    fmt::{
        self,
        Debug,
        Display
    },
    path::PathBuf,
    process,
};

use blog_env::CONFIG_FILE_NAME;

use crate::BlogErrorLocation;

/// Error types thrown by the Blog Builder.
pub enum BlogError {
    /// Could not find root.
    CouldNotFindRoot,

    /// Expected token of one class, found token of another.
    ExpectedToken (String, String),

    /// Unexpected EOF.
    UnexpectedEof,

    /// Error occurred in parsing.
    ParseError {
        /// Error message from parser.
        message: String,

        /// Filename.
        filename: PathBuf,

        /// Location of error.
        location: BlogErrorLocation,
    }
}

impl BlogError {
    /// Throw this error and exit.
    pub fn throw(&self) -> ! {
        println!("Error: {}", self);

        // Exit
        process::exit(1);
    }
}

impl Error for BlogError { }

impl Debug for BlogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for BlogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BlogError::*;
        let err = match self {
            CouldNotFindRoot => &format!("could not find file '{}' in any parent directory", CONFIG_FILE_NAME),
            ExpectedToken (a, b) => &format!("expected token of class '{}', found token of class '{}'", a, b),
            UnexpectedEof => "unexpected EOF",
            ParseError {
                message,
                filename,
                location,
            } => &format!(
                "could not parse file '{}': {} ({})",
                filename.display(),
                message,
                location,
            ),
        };

        write!(f, "{}", err)
    }
}