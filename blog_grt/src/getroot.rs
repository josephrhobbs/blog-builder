//! Getroot utility for the Blog Builder.

use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
};

use blog_env::CONFIG_FILE_NAME;

use blog_err::{
    BlogError,
    BlogResult,
};

/// Get the root directory of the website.
/// 
/// # Parameters
/// None.
/// 
/// # Returns
/// An `BlogResult<PathBuf>` containing the root
/// directory of the site, if it exists.
pub fn getroot() -> BlogResult<PathBuf> {
    // Construct new result
    let result = BlogResult::default();

    // Get working directory
    let working_directory: PathBuf = match env::current_dir() {
        Ok (dir) => dir,
        Err (e) => return result.err(e),
    };

    // Get root directory by recursing upwards
    let mut root: Option<PathBuf> = None;

    // Recurse upwards from the working directory
    // to find the root directory of the site
    for ancestor in working_directory.ancestors() {
        // Check if there is a configuration file here
        let config_file: &Path = &ancestor.join(CONFIG_FILE_NAME);

        // Did we find the config file?
        if config_file.is_file() {
            root = Some (ancestor.to_owned());
            break;
        }
    }

    if let Some (r) = root {
        result.ok(r)
    } else {
        result.err(BlogError::CouldNotFindRoot)
    }
}