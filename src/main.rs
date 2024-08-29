//! Main executable for the Blog Builder.

use std::process::exit;

use blog::{
    err::{
        BlogResult,
        unwrap_or_return,
    },
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
    // Display any error, if it occurred
    match run() {
        BlogResult::Ok (_) => exit(0),
        BlogResult::Err (e) => {
            // Print the errors
            for err in &e {
                println!("\n{:>10} {}", "Error".bold().bright_red(), err);
            }

            // Exit
            match e.len() {
                0 => unreachable!(),
                1 => println!("\n{:>10} due to error message", "Exiting".bold().bright_red()),
                _ => println!("\n{:>10} due to {} error messages", "Exiting".bold().bright_red(), e.len()),
            }

            // Exit
            exit(1);
        }
    }
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

            unwrap_or_return!(SiteTree::new(name))
        },
        Build => {
            if cli.verbosity > 0 {
                println!("{:>10} site output directory", "Building".bold().bright_green());
            }

            // Unwrap site tree
            let sitetree = unwrap_or_return!(sitetree);
            
            // Handle errors or clean
            let duration = unwrap_or_return!(
                sitetree.build(convert, cli.verbosity),
                sitetree.clean()
            );

            // We're done here!  Print time elapsed and return
            if cli.verbosity > 0 {
                println!(
                    "{:>10} building output directory in {:.2} ms",
                    "Finished".bold().bright_green(),
                    duration.as_micros() as f64 / 1000.0,
                );
            }
        },
        Clean => {
            if cli.verbosity > 0 {
                println!("{:>10} site output directory", "Cleaning".bold().bright_green());
            }

            // Unwrap site tree
            let sitetree = unwrap_or_return!(sitetree);

            // Clean up
            sitetree.clean();
        },
        Version => println!(
            "{}\n{:>10} {}",
            "The Blog Builder".bold().bright_white(),
            "Version".bold().bright_yellow(),
            VERSION,
        ),
        Help => help(),
    }

    BlogResult::default()
}