//! Error location management for the Blog Builder.

use std::fmt::{
    self,
    Display,
};

use colored::*;

/// Location that an error occurred.
pub enum BlogErrorLocation {
    /// Error occurred at the beginning of a file.
    Beginning,

    /// Error occurred after the contained sequence.
    After (String),
}

impl Display for BlogErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BlogErrorLocation::*;
        let output = match self {
            Beginning => "at beginning of file",
            After (s) => &format!("after expression '{}'", s.bold().bright_blue()),
        };

        write!(f, "{}", output)
    }
}