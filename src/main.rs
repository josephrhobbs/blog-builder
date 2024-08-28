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

    // Initialize result
    let result = BlogResult::default();

    use Subcommand::*;
    match cli.subcommand {
        New (name) => {
            if cli.verbosity > 0 {
                println!("{:>10} new site with name '{}'", "Creating".bold().green(), name.bold().bright_blue());
            }

            // Handle errors
            match SiteTree::new(name) {
                BlogResult::Ok (ok) => result.ok(ok),
                BlogResult::Err (e) => result.errs(e),
            }
        },
        Build => {
            if cli.verbosity > 0 {
                println!("{:>10} site output directory", "Building".bold().bright_green());
            }

            // Unwrap site tree
            let sitetree = match sitetree {
                BlogResult::Ok (st) => st,
                BlogResult::Err (e) => return result.errs(e),
            };
            
            // Handle errors
            let duration = match sitetree.build(convert, cli.verbosity) {
                BlogResult::Ok (duration) => duration,
                BlogResult::Err (e) => {
                    // Clean up erroneous output
                    sitetree.clean();

                    // Return errors
                    return result.errs(e);
                },
            };

            // We're done here!  Print time elapsed and return
            if cli.verbosity > 0 {
                println!(
                    "{:>10} building output directory in {:.2} ms",
                    "Finished".bold().bright_green(),
                    duration.as_micros() as f64 / 1000.0,
                );
            }

            result
        },
        Clean => {
            if cli.verbosity > 0 {
                println!("{:>10} site output directory", "Cleaning".bold().bright_green());
            }

            // Unwrap site tree
            let sitetree = match sitetree {
                BlogResult::Ok (st) => st,
                BlogResult::Err (e) => return result.errs(e),
            };

            // Clean up
            sitetree.clean();

            result
        },
        Version => {
            // Print version info
            println!(
                "{}\n{:>10} {}",
                "The Blog Builder".bold().bright_white(),
                "Version".bold().bright_yellow(),
                VERSION,
            );

            result
        },
        Help => {
            // Print help info
            help();

            result
        },
    }
}