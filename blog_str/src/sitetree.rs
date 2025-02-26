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

use blog_err::{
    BlogResult,
    unwrap_result_or_return,
    unwrap_result,
    unwrap_or_return,
    unwrap_or_continue,
};

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
        let config = unwrap_or_return!(Config::get(&root));
        
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
    ///     was constructed correctly.
    pub fn create(name: String) -> BlogResult<()> {
        let root: PathBuf = PathBuf::from(&name);

        // Create the root
        unwrap_result_or_return!(fs::create_dir_all(&root));

        // Create the source directory
        let source = root.join(SOURCE_DIR_NAME);
        unwrap_result_or_return!(fs::create_dir_all(&source));

        // Create config file
        let toml = root.join(CONFIG_FILE_NAME);
        unwrap_result_or_return!(fs::write(toml, DEFAULT_CONFIG));

        // Create index file
        let index = source.join(INDEX_FILE_NAME);
        unwrap_result_or_return!(fs::write(index, DEFAULT_INDEX));

        BlogResult::default()
    }

    /// Build a site by applying a given closure to each file.
    /// 
    /// # Parameters
    /// - `convert` (`Fn(String, &Path, &Path, &Config, usize) -> BlogResult<String>`): the closure to
    ///     apply to each source to construct each output, given a site root, a filename,
    ///     a configuration structure, and a verbosity level
    /// - `verbosity` (`usize`): verbosity level of build
    /// 
    /// # Returns
    /// A `BlogResult<Duration>` indicating whether or not the site
    ///     was built correctly, and if the build succeeded, how long it took.
    /// 
    /// # Errors
    /// This function returns an error if it was unable to perform any read/write
    ///     operations correctly.
    pub fn build(&self, convert: impl Fn(String, &Path, &Path, &Config, usize) -> BlogResult<String>, verbosity: usize) -> BlogResult<Duration> {
        // Start a timer
        let start = Instant::now();

        // Initialize result
        let mut result: BlogResult<Duration> = BlogResult::default();

        // Build each file
        for file in &self.files {
            // Print filename, if verbose
            if verbosity > 1 {
                println!("{:>12} page '/{}'", "Building".bright_green(), file.display());
            }

            // Construct the source file
            let source_file = self.source_directory.join(file).with_extension(SOURCE_FILE_EXT);

            // Read the source
            let source = unwrap_result_or_return!(
                fs::read_to_string(&source_file),
                &format!("could not read source file '{}'", source_file.display())
            );

            // Report analytics tag, if it exists
            if let Some (a) = &self.config.analytics {
                if verbosity > 1 {
                    println!("{:>12} analytics tag '{}'", "Adding".bright_green(), a.tag);
                }
            }

            // Convert the source into output
            let output: String = unwrap_or_continue!(
                convert(source, &self.root, file, &self.config, verbosity),
                result
            );

            // Construct the output file
            let output_file = self.output_directory.join(file).with_extension(OUTPUT_FILE_EXT);

            // Create the output directory
            unwrap_result!(
                fs::create_dir_all(output_file.parent().unwrap()),
                result
            );

            // Write the output file
            unwrap_result!(
                fs::write(&output_file, output),
                result,
                &format!("could not write output file '{}'", output_file.display())
            );
        }

        // Construct the stylesheet
        if let Some (s) = &self.config.site.style {
            // Print stylesheet name, if verbose
            if verbosity > 1 {
                println!("{:>12} stylesheet '/{}' from style '{}'", "Writing".bright_green(), STYLESHEET_FILE_NAME, s);
            }

            // Build the output filename
            let stylesheet = self.output_directory.join(STYLESHEET_FILE_NAME);

            // Get the stylesheet
            use SiteStyle::*;
            let style = match s {
                Tech => style::TECH,
                Book => style::BOOK,
            };

            // Write the stylesheet
            unwrap_result!(
                fs::write(&stylesheet, style),
                result,
                &format!("could not write stylesheet '{}'", stylesheet.display())
            );
        }

        // Construct the favicon
        if let Some (f) = &self.config.site.icon {
            // Print favicon location, if verbose
            if verbosity > 1 {
                println!("{:>12} favicon '/{}'", "Writing".bright_green(), f);
            }

            // Build source icon
            let source_icon = self.source_directory.join(f);

            // Build output icon
            let output_icon = self.output_directory.join(f);

            // Create the output directory
            unwrap_result!(
                fs::create_dir_all(output_icon.parent().unwrap()),
                result
            );

            // Copy the icon
            unwrap_result!(
                fs::copy(&source_icon, output_icon),
                result,
                &format!("could not find icon '{}'", source_icon.display())
            );
        }

        // Copy over media
        if let Some (media) = &self.config.media {
            // Look in the media directory
            for m in &media.include {
                // Print media location, if verbose
                if verbosity > 1 {
                    println!("{:>12} media file '/{}/{}'", "Writing".bright_green(), MEDIA_DIR_NAME, m);
                }

                // Build source media
                let source_media = self.source_directory.join(MEDIA_DIR_NAME).join(m);

                // Build output media
                let output_media = self.output_directory.join(MEDIA_DIR_NAME).join(m);

                // Create the output directory
                unwrap_result!(
                    fs::create_dir_all(self.output_directory.join(MEDIA_DIR_NAME)),
                    result
                );

                // Copy the media
                unwrap_result!(
                    fs::copy(&source_media, output_media),
                    result,
                    &format!("could not find media file '{}'", source_media.display())
                );
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
        // If we can't clean the directory, it doesn't exist
        // and our job is already done for us... so we want
        // to ignore the result of this function
        let _ = fs::remove_dir_all(&self.output_directory);
    }
}