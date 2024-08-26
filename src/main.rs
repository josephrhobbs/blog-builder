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

fn main() -> BlogResult<()> {
    // Parse CLI arguments
    let cli = Cli::new();

    // Get the sitetree, if it exists
    let sitetree = SiteTree::get();

    use Subcommand::*;
    match cli.subcommand {
        New (name) => SiteTree::new(name)?,
        Build => sitetree?.build(convert)?,
        Clean => sitetree?.clean()?,
        Help => println!("{}", HELP),
    }

    Ok (())
}