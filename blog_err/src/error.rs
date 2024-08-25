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
    /// Could not find root
    CouldNotFindRoot,

    /// Unrecognized control sequence
    UnrecognizedControlSequence (String),

    /// Could not open file
    CannotOpenFile (String),

    /// Unrecognized token
    UnrecognizedToken (String),

    /// Expected token of given class
    ExpectedTokenOfClass (String),

    /// Unexpected EOF
    UnexpectedEof,

    /// Too many hashes
    TooManyHashes (String),

    /// Cannot find file
    CannotFindFile (String),

    /// Cannot read file
    CannotReadFile (String),

    /// Cannot write to file
    CannotWriteFile (String),
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
            UnrecognizedControlSequence (ctrl) => &format!("unrecognized control sequence: {}", ctrl),
            CannotOpenFile (file) => &format!("could not open file '{}'", file),
            UnrecognizedToken (token) => &format!("unrecognized token: {}", token),
            ExpectedTokenOfClass (class) => &format!("expected token of class: {}", class),
            UnexpectedEof => "unexpected end of file",
            TooManyHashes (hashes) => &format!("too many hashes: {}", hashes),
            CannotFindFile (file) => &format!("cannot find file: {}", file),
            CannotReadFile (file) => &format!("cannot read file: {}", file),
            CannotWriteFile (file) => &format!("cannot write to file: {}", file),
        };

        write!(f, "{}", err)
    }
}