//! Error handling for the Blog Builder.

use std::{
    error::Error,
    fmt::{
        self,
        Debug,
        Display
    },
    process,
};

use anyhow::Result as AnyResult;

use blog_env::CONFIG_FILE_NAME;

/// Result type for Blog Builder.
pub type BlogResult<T> = AnyResult<T>;

/// Error types thrown by the Blog Builder.
pub enum BlogError {
    /// Could not find root.
    CouldNotFindRoot,

    /// Expected token of one class, found token of another.
    ExpectedToken (String, String),

    /// Unexpected EOF.
    UnexpectedEof,
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
        };

        write!(f, "{}", err)
    }
}