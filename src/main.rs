//! Main executable for the Blog Builder.

use blog::{
    err::BlogResult,
    cli::{
        Cli,
        Subcommand,
    },
    cfg::Config,
    site::{
        SiteTree,
        Parser,
        Emitter,
    },
    help::HELP,
};

fn main() -> BlogResult<()> {
    // Parse CLI arguments
    let cli = Cli::new();

    // Get the sitetree, if it exists
    let sitetree = SiteTree::get();

    // Get configuration information
    // from TOML file
    let config = Config::get(&sitetree);

    // Construct a parser
    let parser = Parser::new();

    // Construct an emitter
    let emitter = Emitter::new(config?);

    // Construct a map from source to output
    let convert = |source: String| emitter.emit(parser.parse(&source));

    use Subcommand::*;
    match cli.subcommand {
        New (name) => SiteTree::new(name)?,
        Build => sitetree?.build(convert)?,
        Clean => sitetree?.clean()?,
        Help => println!("{}", HELP),
    }

    Ok (())
}