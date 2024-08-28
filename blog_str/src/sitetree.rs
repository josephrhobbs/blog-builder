//! Site tree management for the Blog Builder.

use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
    time::{
        Instant,
        Duration,
    },
};

use colored::*;

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
    MEDIA_DIR_NAME,
    INDEX_FILE_NAME,
    CONFIG_FILE_NAME,
    DEFAULT_INDEX,
    DEFAULT_CONFIG,
    STYLESHEET_FILE_NAME,
};

use blog_cfg::SiteStyle;

use blog_err::BlogResult;

use blog_grt::getroot;

use blog_sty::style;

#[derive(Clone, Default, Debug)]
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
        // Construct a result
        let result = BlogResult::default();

        // Get the root directory of the website
        let root = match getroot() {
            BlogResult::Ok (r) => r,
            BlogResult::Err (e) => return result.errs(e),
        };

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
        let config = match Config::get(&root) {
            BlogResult::Ok (cfg) => cfg,
            BlogResult::Err (e) => return result.errs(e),
        };
        
        BlogResult::Ok (Self {
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

        // Initialize result
        let result = BlogResult::default();

        // Create the root
        if let Err (e) = fs::create_dir_all(&root) {
            return result.err(e);
        }

        // Create the source directory
        let source = root.join(SOURCE_DIR_NAME);
        if let Err (e) = fs::create_dir_all(&source) {
            return result.err(e);
        }

        // Create config file
        let toml = root.join(CONFIG_FILE_NAME);
        if let Err (e) = fs::write(toml, DEFAULT_CONFIG) {
            return result.err(e);
        }

        // Create index file
        let index = source.join(INDEX_FILE_NAME);
        if let Err (e) = fs::write(index, DEFAULT_INDEX) {
            return result.err(e);
        }

        BlogResult::Ok (())
    }

    /// Build a site by applying a given closure to each file.
    /// 
    /// # Parameters
    /// - `convert` (`Fn(String, &Path, &Path, &Config) -> BlogResult<String>`): the closure to
    /// apply to each source to construct each output, given a site root, a filename,
    /// and a configuration structure
    /// - `verbosity` (`usize`): verbosity level of build
    /// 
    /// # Returns
    /// A `BlogResult<Duration>` indicating whether or not the site
    /// was built correctly, and if the build succeeded, how long it took.
    /// 
    /// # Errors
    /// This function returns an error if it was unable to perform any read/write
    /// operations correctly.
    pub fn build(&self, convert: impl Fn(String, &Path, &Path, &Config) -> BlogResult<String>, verbosity: usize) -> BlogResult<Duration> {
        // Start a timer
        let start = Instant::now();

        // Initialize result
        let mut result: BlogResult<Duration> = BlogResult::default();

        // Build each file
        for file in &self.files {
            // Print filename, if verbose
            if verbosity > 1 {
                println!("{:>10} page '/{}'", "Building".bright_green(), file.display());
            }

            // Construct the source file
            let source_file = self.source_directory.join(file).with_extension(SOURCE_FILE_EXT);

            // Read the source
            let source = match fs::read_to_string(&source_file) {
                Ok (src) => src,
                Err (e) => return result.err_context(e, &format!("could not read source file '{}'", source_file.display())),
            };

            // Report analytics tag, if it exists
            if let Some (a) = &self.config.analytics {
                if verbosity > 1 {
                    println!("{:>10} analytics tag '{}'", "Adding".bright_green(), a.tag);
                }
            }

            // Convert the source into output
            let output: String = match convert(source, &self.root, &file, &self.config) {
                BlogResult::Ok (ok) => ok,
                BlogResult::Err (e) => {
                    // Add errors to list
                    result = result.errs(e);

                    // Continue parsing
                    continue;
                },
            };

            // Construct the output file
            let output_file = self.output_directory.join(file).with_extension(OUTPUT_FILE_EXT);

            // Create the output directory
            if let Err (e) = fs::create_dir_all(&output_file.parent().unwrap()) {
                result = result.err(e);
            }

            // Write the output file
            if let Err (e) = fs::write(&output_file, output) {
                result = result.err_context(e, &format!("could not write output file '{}'", output_file.display()));
            }
        }

        // Construct the stylesheet
        if let Some (s) = &self.config.site.style {
            // Print stylesheet name, if verbose
            if verbosity > 1 {
                println!("{:>10} stylesheet '/{}' from style '{}'", "Writing".bright_green(), STYLESHEET_FILE_NAME, s);
            }

            // Build the output filename
            let stylesheet = self.output_directory.join(STYLESHEET_FILE_NAME);

            // Get the stylesheet
            use SiteStyle::*;
            let style = match s {
                Tech => style::TECH,
            };

            // Write the stylesheet
            if let Err (e) = fs::write(&stylesheet, style) {
                result = result.err_context(e, &format!("could not write stylesheet '{}'", stylesheet.display()));
            }
        }

        // Construct the favicon
        if let Some (f) = &self.config.site.icon {
            // Print favicon location, if verbose
            if verbosity > 1 {
                println!("{:>10} favicon '/{}'", "Writing".bright_green(), f);
            }

            // Build source icon
            let source_icon = self.source_directory.join(f);

            // Build output icon
            let output_icon = self.output_directory.join(f);

            // Create the output directory
            if let Err (e) = fs::create_dir_all(&output_icon.parent().unwrap()) {
                result = result.err(e);
            };

            // Copy the icon
            if let Err (e) = fs::copy(&source_icon, output_icon) {
                result = result.err_context(e, &format!("could not find icon '{}'", source_icon.display()));
            }
        }

        // Copy over media
        if let Some (media) = &self.config.media {
            // Look in the media directory
            for m in &media.include {
                // Print media location, if verbose
                if verbosity > 1 {
                    println!("{:>10} media file '/{}/{}'", "Writing".bright_green(), MEDIA_DIR_NAME, m);
                }

                // Build source media
                let source_media = self.source_directory.join(MEDIA_DIR_NAME).join(m);

                // Build output media
                let output_media = self.output_directory.join(MEDIA_DIR_NAME).join(m);

                // Create the output directory
                if let Err (e) = fs::create_dir_all(&self.output_directory.join(MEDIA_DIR_NAME)) {
                    result = result.err(e);
                }

                // Copy the media
                if let Err (e) = fs::copy(&source_media, output_media) {
                    result = result.err_context(e, &format!("could not find media file '{}'", source_media.display()));
                }
            }
        }

        // We're done!
        match result {
            BlogResult::Ok (_) => result.ok(start.elapsed()),
            BlogResult::Err (_) => result,
        }
    }

    /// Clean a site directory by deleting all output files.
    ///
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// None.
    pub fn clean(&self) {
        fs::remove_dir_all(&self.output_directory).unwrap();
    }
}