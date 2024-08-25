//! Configuration file management for the Blog Builder.

use std::{
    fs,
    path::PathBuf,
};

use serde::Deserialize;

use blog_err::BlogResult;

use blog_env::CONFIG_FILE_NAME;

use blog_grt::getroot;

#[derive(Deserialize, Debug)]
/// A configuration file that dictates Blog Builder settings.
/// 
/// This data is stored in the root directory of the site in
/// a TOML file.  Its name is set by the constant `CONFIG_FILE_NAME`
/// in `src/lib.rs`.
pub struct Config {
    /// Site configuration information.
    pub site: SiteConfig,
}

#[derive(Deserialize, Debug)]
/// Configuration information for the site.
pub struct SiteConfig {
    /// Site name (to appear in page title).
    pub sitename: String,

    /// Site style (for CSS source).
    pub style: SiteStyle,
}

#[derive(Deserialize, Debug)]
/// Site style options.
pub enum SiteStyle {
    /// Modern style.
    Modern,
}

impl Config {
    /// Get information from the site configuration file.
    ///
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `BlogResult<Config>` structure with all configuration information.
    pub fn get() -> BlogResult<Config> {
        // Get site root
        let root = getroot()?;

        // Construct configuration file name
        let config_file: PathBuf = root.join(CONFIG_FILE_NAME);

        // Read config file to string
        let config: String = fs::read_to_string(&config_file)?;

        Ok (toml::from_str(&config)?)
    }
}