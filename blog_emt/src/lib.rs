//! HTML Emitter module for the Blog Builder.

/// Enforce all warnings.
#[deny(warnings)]

/// Enforce all documentation.
#[deny(missing_docs)]

mod emitter;

pub use emitter::Emitter;