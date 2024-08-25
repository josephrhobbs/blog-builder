//! Main executable for the Blog Builder.

use blog::{
    BlogResult,
    Cli,
    Config,
    SiteTree,
    Subcommand,
};

fn main() -> BlogResult<()> {
    println!("Blog Builder");

    let cli = Cli::new();

    let sitetree = SiteTree::new().unwrap();

    let config = Config::get();

    dbg!(&cli.subcommand);
    dbg!(&sitetree);
    dbg!(&config);

    if cli.subcommand == Subcommand::Build {
        sitetree.build(|i| i)?;
    } else if cli.subcommand == Subcommand::Clean {
        sitetree.clean()?;
    }

    Ok (())
}