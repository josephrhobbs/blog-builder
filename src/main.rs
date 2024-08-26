//! Main executable for the Blog Builder.

use std::process::exit;

use blog::{
    err::BlogResult,
    cli::{
        Cli,
        Subcommand,
    },
    site::SiteTree,
    cvt::convert,
    help::HELP,
};

use colored::*;

fn main() {
    let result = run();

    // Display any error, if it occurred
    if let Err (e) = result {
        // Print the error
        println!("\n{:>10} {}", "Error".bold().bright_red(), e);

        // Print a help message
        println!("\n{:>10} you may be trying to access a file that does not exist", "Hint".bold().bright_yellow());

        // Print the error
        println!("\n{:>10} due to previous error message", "Exiting".bold().bright_red());

        // Exit
        exit(1);
    }

    exit(0);
}

fn run() -> BlogResult<()> {
    // Parse CLI arguments
    let cli = Cli::new();

    // Get the sitetree, if it exists
    let sitetree = SiteTree::get();

    use Subcommand::*;
    match cli.subcommand {
        New (name) => {
            println!("{:>10} new site with name '{}'", "Creating".bold().green(), name.bold().bright_blue());

            SiteTree::new(name)?
        },
        Build => {
            println!("{:>10} site output directory", "Building".bold().green());

            sitetree?.build(convert)?
        },
        Clean => {
            println!("{:>10} site output directory", "Cleaning".bold().green());

            sitetree?.clean()?
        },
        Help => println!("{}", HELP),
    }

    Ok (())
}