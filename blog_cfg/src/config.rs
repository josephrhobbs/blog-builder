//! Configuration file management for the Blog Builder.

use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use serde::Deserialize;

use blog_err::BlogResult;

use blog_env::CONFIG_FILE_NAME;

#[derive(Clone, Deserialize, Debug)]
/// A configuration file that dictates Blog Builder settings.
/// 
/// This data is stored in the root directory of the site in
/// a TOML file.  Its name is set by the constant `CONFIG_FILE_NAME`
/// in `src/lib.rs`.
pub struct Config {
    /// Site configuration information.
    pub site: SiteConfig,

    /// Analytics information.
    pub analytics: Option<AnalyticsConfig>,

    /// Menu information.
    pub menu: Option<MenuConfig>,
}

#[derive(Clone, Deserialize, Debug)]
/// Configuration information for the site.
pub struct SiteConfig {
    /// Site name (to appear in page title).
    pub name: String,

    /// Site icon path (for favicon).
    pub icon: Option<String>,

    /// Site style (for CSS source).
    pub style: Option<SiteStyle>,
}

#[derive(Clone, Deserialize, Debug)]
/// Configuration information for analytics.
pub struct AnalyticsConfig {
    /// Analytics source file.
    pub path: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
/// Configuration information for site menu.
pub struct MenuConfig {
    /// Menu source file.
    pub path: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
/// Site style options.
pub enum SiteStyle {
    /// Modern style.
    Modern,

    /// Technology style.
    Tech,
}

impl Config {
    /// Get information from the site configuration file.
    ///
    /// # Parameters
    /// - `root` (`&Path`): a reference to the site root
    /// 
    /// # Returns
    /// A `BlogResult<Config>` structure with all configuration information.
    pub fn get(root: &Path) -> BlogResult<Config> {
        // Construct configuration file name
        let config_file: PathBuf = root.join(CONFIG_FILE_NAME);

        // Read config file to string
        let config: String = fs::read_to_string(&config_file)?;

        Ok (toml::from_str(&config)?)
    }
}