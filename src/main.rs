//! Main executable for the Blog Builder.

use blog::{
    err::BlogResult,
    cli::{
        Cli,
        Subcommand,
    },
    cfg::Config,
    site::SiteTree,
    help::HELP,
};

fn main() -> BlogResult<()> {
    // Parse CLI arguments
    let cli = Cli::new();

    // Get the sitetree, if it exists
    let sitetree = SiteTree::get();

    // Get configuration information
    // from TOML file
    let _config = Config::get(&sitetree);

    use Subcommand::*;
    match cli.subcommand {
        New (name) => SiteTree::new(name)?,
        Build => sitetree?.build(|i| i)?,
        Clean => sitetree?.clean()?,
        Help => println!("{}", HELP),
    }

    Ok (())
}