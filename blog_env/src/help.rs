//! Help menu for the Blog Builder.

use colored::*;

/// Prints help information to the terminal.
/// 
/// # Parameters
/// None.
/// 
/// # Returns
/// None.
pub fn help() {
    let output = format!("{}
    
A simple static web framework.

{}

    blog {} {}

{}

    {} {}\t\tcreate a new site called {}

    {}\t\tbuild the current site directory

    {}\t\tclean the current directory

    {}\t\tdisplay the software version

    {}\t\tdisplay this help menu

{}
    
    {}, {}\tdisplay more output

    {}, {}\tdisplay even more output

    {}, {}\t\tdisplay less output

Executing `blog` with no subcommands will display this help menu.",
    "The Blog Builder".bold().bright_white(),
    "Usage".bold().bright_green(),
    "[SUBCOMMAND]".bright_blue(),
    "[FLAGS]".bright_blue(),
    "Subcommands".bold().bright_green(),
    "new".bold().bright_cyan(),
    "<NAME>".bright_blue(),
    "<NAME>".bright_blue(),
    "build".bold().bright_cyan(),
    "clean".bold().bright_cyan(),
    "version".bold().bright_cyan(),
    "help".bold().bright_cyan(),
    "Flags".bold().bright_green(),
    "--verbose".bold().bright_cyan(),
    "-v".bold().bright_cyan(),
    "--very-verbose".bold().bright_cyan(),
    "-vv".bold().bright_cyan(),
    "--quiet".bold().bright_cyan(),
    "-q".bold().bright_cyan(),
    );

    println!("{}", output);
}