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
    let cli = Cli::new();

    let sitetree = SiteTree::get();

    let config = Config::get();

    dbg!(&cli.subcommand);
    dbg!(&sitetree);
    dbg!(&config);

    use Subcommand::*;
    match cli.subcommand {
        New (name) => SiteTree::new(name)?,
        Build => sitetree?.build(|i| i)?,
        Clean => sitetree?.clean()?,
        Help => println!("{}", HELP),
    }

    Ok (())
}