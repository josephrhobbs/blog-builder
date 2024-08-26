//! Library for Blog Builder configuration.

// Enforce all warnings.
#![deny(warnings)]

// Enforce all documentation.
#![deny(missing_docs)]

mod config;

pub use config::{
    Config,
    SiteConfig,
    SiteStyle,
};