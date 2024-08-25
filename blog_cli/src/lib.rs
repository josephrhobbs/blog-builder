//! Library for Blog Builder CLI operations.

/// Enforce all warnings.
#[deny(warnings)]

/// Enforce all documentation.
#[deny(missing_docs)]

mod cli;

pub use cli::{
    Cli,
    Subcommand,
};