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
    help::help,
    version::VERSION,
};

use colored::*;

fn main() {
    let result = run();

    // Display any error, if it occurred
    if let Err (e) = result {
        // Print the error
        println!("\n{:>10} {}", "Error".bold().bright_red(), e);

        // Exit
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
            if cli.verbosity > 0 {
                println!("{:>10} new site with name '{}'", "Creating".bold().green(), name.bold().bright_blue());
            }

            SiteTree::new(name)?
        },
        Build => {
            if cli.verbosity > 0 {
                println!("{:>10} site output directory", "Building".bold().bright_green());
            }

            sitetree?.build(convert, cli.verbosity)?
        },
        Clean => {
            if cli.verbosity > 0 {
                println!("{:>10} site output directory", "Cleaning".bold().bright_green());
            }

            sitetree?.clean()?
        },
        Version => println!(
            "{}\n{:>10} {}",
            "The Blog Builder".bold().bright_white(),
            "Version".bold().bright_yellow(),
            VERSION,
        ),
        Help => help(),
    }

    Ok (())
}