//! Command line interface for the Blog Builder.

use std::env;

#[derive(Debug)]
/// A command line interface.
pub struct Cli {
    /// The user-provided subcommand.
    pub subcommand: Subcommand,

    /// The verbosity of the output.
    /// 
    /// Silent (no output) is 0.
    /// Default (some output) is 1.
    /// Verbose (all output) is 2 or greater.
    pub verbosity: usize,
}

impl Cli {
    #[allow(clippy::new_without_default)]
    /// Construct the command line interface.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A new command line interface.
    pub fn new() -> Self {
        let subcommand = Subcommand::from(env::args().nth(1));

        // Default verbosity is 1
        let mut verbosity = 1;

        // Check for verbosity
        for arg in env::args() {
            match arg.as_str() {
                "--verbose" => verbosity = 2,
                "--quiet" => verbosity = 0,
                a if a.contains("-v") => verbosity = 1 + a.chars().filter(|c| *c == 'v').count(),
                "-q" => verbosity = 0,
                _ => (),
            }
        }

        Self {
            subcommand,
            verbosity,
        }
    }
}

#[derive(PartialEq, Debug)]
/// Subcommands available to the user.
pub enum Subcommand {
    /// Print help information to the user.
    Help,
    
    /// Generate a new site with the provided name.
    New (String),

    /// Build the site.
    Build,

    /// Print version information.
    Version,

    /// Clean the site (delete the output directory).
    Clean,
}

impl Subcommand {
    /// Construct a subcommand from a given string.
    /// 
    /// # Parameters
    /// - `string` (`Option<String>`): the provided subcommand,
    ///     if one was provided
    /// 
    /// # Returns
    /// A `Subcommand` containing the subcommand.
    pub fn from(string: Option<String>) -> Self {
        // If no subcommand provided, return a help menu
        let string = if let Some (s) = string {
            s
        } else {
            return Subcommand::Help;
        };

        match string.as_str() {
            "new" => {
                // Get the name of the new site
                let argument = if let Some (a) = env::args().nth(2) {
                    a
                } else {
                    return Subcommand::Help;
                };

                Subcommand::New (argument)
            },
            "build" => Subcommand::Build,
            "clean" => Subcommand::Clean,
            "version" => Subcommand::Version,
            _ => Subcommand::Help,
        }
    }
}