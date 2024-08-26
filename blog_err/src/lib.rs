//! Library for Blog Builder error handling.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

mod error;

pub use error::{
    BlogError,
    BlogResult,
};