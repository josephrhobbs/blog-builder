//! Result handling for the Blog Builder.

use anyhow::Error as AnyError;

#[derive(Debug)]
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

#[macro_export]
/// Unwrap a `BlogError` or return its errors.
macro_rules! unwrap_or_return {
    ($expr:expr) => {
        match $expr {
            BlogResult::Ok (ok) => ok,
            BlogResult::Err (e) => return BlogResult::Err (e),
        }
    };
    ($expr:expr, $before_return:expr) => {
        match $expr {
            BlogResult::Ok (ok) => ok,
            BlogResult::Err (e) => {
                $before_return;
                return BlogResult::Err (e)
            },
        }
    };
}

#[macro_export]
/// Unwrap a `BlogError` or continue a loop.
macro_rules! unwrap_or_continue {
    ($expr:expr, $result:ident) => {
        match $expr {
            BlogResult::Ok (ok) => ok,
            BlogResult::Err (e) => {
                $result = $result.errs(e);

                continue;
            },
        }
    };
}

#[macro_export]
/// Unwrap a `Result` or return its error.
macro_rules! unwrap_result_or_return {
    ($expr:expr) => {
        match $expr {
            Ok (ok) => ok,
            Err (e) => return BlogResult::default().err(e),
        }
    };
    ($expr:expr, $cxt:expr) => {
        match $expr {
            Ok (ok) => ok,
            Err (e) => return BlogResult::default().err_context(e, $cxt),
        }
    };
}

#[macro_export]
/// Unwrap a `Result` or handle its error.
macro_rules! unwrap_result {
    ($expr:expr, $result:ident) => {
        match $expr {
            Ok (ok) => ok,
            Err (e) => {
                $result = $result.err(e);

                Default::default()
            },
        }
    };
    ($expr:expr, $result:ident, $cxt:expr) => {
        match $expr {
            Ok (ok) => ok,
            Err (e) => {
                $result = $result.err_context(e, $cxt);

                Default::default()
            },
        }
    };
}