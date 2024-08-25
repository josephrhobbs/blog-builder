//! Main executable for the Blog Builder.

use blog::{
    Cli,
    Config,
    SiteTree,
};

fn main() {
    println!("Blog Builder");

    let cli = Cli::new();

    let sitetree = SiteTree::new();

    let config = Config::get();

    dbg!(cli.subcommand);
    dbg!(sitetree);
    dbg!(config);
}