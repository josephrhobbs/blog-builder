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

use anyhow::Error as AnyError;

use blog_env::CONFIG_FILE_NAME;

/// Result type for Blog Builder.
pub enum BlogResult<T: Default> {
    /// Function returned OK.
    Ok (T),

    /// Function returned one or more errors.
    Err (Vec<AnyError>),
}

impl<T: Default> BlogResult<T> {
    /// Construct a new OK result.
    /// 
    /// **Note**: this function consumes its input.
    /// 
    /// # Parameters
    /// - `result` (`T`): the result
    /// 
    /// # Returns
    /// A new `Ok` result.
    pub fn ok(self, result: T) -> Self {
        Self::Ok (result)
    }

    /// Construct a new error result, or add a new error to an
    /// existing list of errors.
    /// 
    /// **Note**: this function consumes its input.
    /// 
    /// # Parameters
    /// - `error` (`impl Into<anyhow::Error>`): the error
    /// 
    /// # Returns
    /// A new `Err` result.
    pub fn err(self, error: impl Into<AnyError>) -> Self {
        match self {
            Self::Ok (_) => Self::Err (vec![error.into()]),
            Self::Err (mut e) => {
                // Append the new error to the existing list
                e.push(error.into());

                Self::Err (e)
            },
        }
    }

    /// Construct a new error result, or add a new error to an
    /// existing list of errors, adding context to the error.
    /// 
    /// **Note**: this function consumes its input.
    /// 
    /// # Parameters
    /// - `error` (`impl Into<anyhow::Error>`): the error
    /// - `context` (`&str`): the context
    /// 
    /// # Returns
    /// A new `Err` result.
    pub fn err_context(self, error: impl Into<AnyError>, context: &str) -> Self {
        match self {
            Self::Ok (_) => Self::Err (vec![error.into().context(context.to_string())]),
            Self::Err (mut e) => {
                // Contextualize the error
                let anyerror: AnyError = error.into().context(context.to_string());

                // Append the new error to the existing list
                e.push(anyerror);

                Self::Err (e)
            },
        }
    }

    /// Construct a new error result, or add new errors to an
    /// existing list of errors.
    /// 
    /// **Note**: this function consumes its input.
    /// 
    /// # Parameters
    /// - `errors` (`Vec<anyhow::Error>`): the errors
    /// 
    /// # Returns
    /// A new `Err` result.
    pub fn errs(self, errors: Vec<AnyError>) -> Self {
        match self {
            Self::Ok (_) => Self::Err (errors),
            Self::Err (mut e) => {
                // Append the new errors to the existing list
                e.extend(errors);

                Self::Err (e)
            },
        }
    }
}

impl<T: Default> Default for BlogResult<T> {
    fn default() -> Self {
        Self::Ok (Default::default())
    }
}

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