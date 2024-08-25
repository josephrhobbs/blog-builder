//! Site tree management for the Blog Builder.

use std::{
    fs,
    path::PathBuf,
};

use walkdir::{
    DirEntry,
    WalkDir,
};

use blog_env::{
    SOURCE_DIR_NAME,
    SOURCE_FILE_EXT,
    OUTPUT_DIR_NAME,
    OUTPUT_FILE_EXT,
};

use blog_err::BlogResult;

use blog_grt::getroot;

#[derive(Debug)]
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
    root: PathBuf,

    /// Source directory of the site.
    source_directory: PathBuf,
    
    /// Output directory of the site.
    output_directory: PathBuf,

    /// List of all files relative to the source directory.
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
    pub fn new() -> BlogResult<Self> {
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
        
        Ok (Self {
            root,
            source_directory,
            output_directory,
            files,
        })
    }

    /// Build a site by applying a given closure to each file.
    /// 
    /// # Parameters
    /// - `convert` (`Fn(String) -> String>`): the closure to
    /// apply to each source to construct each output.
    /// 
    /// # Returns
    /// A `BlogResult<()>` indicating whether or not the site
    /// was built correctly.
    pub fn build(&self, convert: impl Fn(String) -> String) -> BlogResult<()> {
        println!("Root: {}", self.root.display());

        for file in &self.files {
            // Construct the source file
            let source_file = self.source_directory.join(file).with_extension(SOURCE_FILE_EXT);

            println!("Source file: {}", source_file.display());

            // Read the source
            let source = fs::read_to_string(&source_file)?;

            // Convert the source into output
            let output = convert(source);

            // Construct the output file
            let output_file = self.output_directory.join(file).with_extension(OUTPUT_FILE_EXT);

            println!("Output file: {}", output_file.display());

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