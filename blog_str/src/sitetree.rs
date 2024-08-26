//! Site tree management for the Blog Builder.

use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use walkdir::{
    DirEntry,
    WalkDir,
};

use blog_cfg::Config;

use blog_env::{
    SOURCE_DIR_NAME,
    SOURCE_FILE_EXT,
    OUTPUT_DIR_NAME,
    OUTPUT_FILE_EXT,
    INDEX_FILE_NAME,
    CONFIG_FILE_NAME,
    DEFAULT_INDEX,
    DEFAULT_CONFIG,
};

use blog_err::BlogResult;

use blog_grt::getroot;

#[derive(Clone, Debug)]
/// A website tree.
/// 
/// This represents a list of all files in this website
/// and their locations, relative to the source or HTML
/// directories.
/// 
/// This list will be constructed by recursing through the
/// source directory of the site, and it is used to construct
/// the HTML directory of the site.
pub struct SiteTree {
    /// Root directory of the site.
    pub root: PathBuf,

    /// Source directory of the site.
    source_directory: PathBuf,
    
    /// Output directory of the site.
    output_directory: PathBuf,

    /// Configuration information.
    config: Config,

    /// List of all file stems relative to the source directory.
    files: Vec<PathBuf>,
}

impl SiteTree {
    /// Construct a new website tree.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A new website tree object.
    pub fn get() -> BlogResult<Self> {
        // Get the root directory of the website
        let root = getroot()?;

        // Construct the source directory
        let source_directory: PathBuf = root.join(SOURCE_DIR_NAME);

        // Construct the output directory
        let output_directory: PathBuf = root.join(OUTPUT_DIR_NAME);

        // Construct a source file check closure
        let is_source_file = |f: &DirEntry| f.path()
            .extension()
            .map(|osstr| osstr.to_str()) == Some (Some (SOURCE_FILE_EXT));

        // Walk through the source directory
        let files = WalkDir::new(&source_directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(is_source_file)
            .map(|f| f.path().to_owned())
            .filter_map(|p| pathdiff::diff_paths(p, &source_directory))
            .map(|f| f.with_extension(""))
            .collect::<Vec<PathBuf>>();

        // Get configuration information
        let config = Config::get(&root)?;
        
        Ok (Self {
            root,
            source_directory,
            output_directory,
            files,
            config,
        })
    }

    /// Construct a new site.
    /// 
    /// # Parameters
    /// - `name` (`String`): the name of the site directory
    /// 
    /// # Returns
    /// A `BlogResult<()>` indicating whether or not the site
    /// was constructed correctly.
    pub fn new(name: String) -> BlogResult<()> {
        let root: PathBuf = PathBuf::from(&name);

        // Create the root
        fs::create_dir_all(&name)?;

        // Create the source directory
        let source = root.join(SOURCE_DIR_NAME);
        fs::create_dir_all(&source)?;

        // Create config file
        let toml = root.join(CONFIG_FILE_NAME);
        fs::write(toml, DEFAULT_CONFIG)?;

        // Create index file
        let index = source.join(INDEX_FILE_NAME);
        fs::write(index, DEFAULT_INDEX)?;

        Ok (())
    }

    /// Build a site by applying a given closure to each file.
    /// 
    /// # Parameters
    /// - `convert` (`Fn(String, &Path, &Config) -> String>`): the closure to
    /// apply to each source to construct each output, given a filename and a 
    /// configuration structure.
    /// 
    /// # Returns
    /// A `BlogResult<()>` indicating whether or not the site
    /// was built correctly.
    pub fn build(&self, convert: impl Fn(String, &Path, &Config) -> String) -> BlogResult<()> {
        for file in &self.files {
            // Construct the source file
            let source_file = self.source_directory.join(file).with_extension(SOURCE_FILE_EXT);

            // Read the source
            let source = fs::read_to_string(&source_file)?;

            // Convert the source into output
            let output = convert(source, &file, &self.config);

            // Construct the output file
            let output_file = self.output_directory.join(file).with_extension(OUTPUT_FILE_EXT);

            // Create the output directory
            fs::create_dir_all(&output_file.parent().unwrap())?;

            fs::write(output_file, output)?;
        }

        Ok (())
    }

    /// Clean a site directory by deleting all output files.
    ///
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `BlogResult<()>` indicating whether or not the output
    /// files were cleaned correctly.
    pub fn clean(&self) -> BlogResult<()> {
        fs::remove_dir_all(&self.output_directory).unwrap();

        Ok (())
    }
}