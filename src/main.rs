//! Main executable for the Blog Builder.

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

fn main() -> BlogResult<()> {
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
            println!("{:>10} site", "Building".bold().green());

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