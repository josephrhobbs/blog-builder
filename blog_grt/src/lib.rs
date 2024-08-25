//! Getroot utility for the Blog Builder.

/// Enforce all warnings.
#[deny(warnings)]

/// Enforce all documentation.
#[deny(missing_docs)]

mod getroot;

pub use getroot::getroot;