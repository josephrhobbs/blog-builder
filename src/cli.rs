//! Command line interface for the Blog Builder.

use std::env;

#[derive(Debug)]
/// A command line interface.
pub struct Cli {
    /// The user-provided subcommand.
    pub subcommand: Subcommand,
}

impl Cli {
    /// Construct the command line interface.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A new command line interface.
    pub fn new() -> Self {
        let subcommand = Subcommand::from(env::args().nth(1));

        Self {
            subcommand,
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

    /// Clean the site (delete the output directory).
    Clean,
}

impl Subcommand {
    /// Construct a subcommand from a given string.
    /// 
    /// # Parameters
    /// - `string` (`Option<String>`): the provided subcommand,
    /// if one was provided
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
                let argument = if let Some (a) = env::args().nth(2) {
                    a
                } else {
                    return Subcommand::Help;
                };

                Subcommand::New (argument)
            },
            "build" => Subcommand::Build,
            "clean" => Subcommand::Clean,
            _ => Subcommand::Help,
        }
    }
}