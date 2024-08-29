//! Library for Blog Builder error handling.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

mod error;
mod location;
mod result;

pub use error::BlogError;

pub use location::BlogErrorLocation;

pub use result::BlogResult;