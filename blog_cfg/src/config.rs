//! Configuration file management for the Blog Builder.

use std::{
    fs,
    path::PathBuf,
};

use serde::Deserialize;

use blog_err::{
    BlogError,
    BlogResult,
};

use blog_env::CONFIG_FILE_NAME;

use blog_str::SiteTree;

#[derive(Deserialize, Debug)]
/// A configuration file that dictates Blog Builder settings.
/// 
/// This data is stored in the root directory of the site in
/// a TOML file.  Its name is set by the constant `CONFIG_FILE_NAME`
/// in `src/lib.rs`.
pub struct Config {
    /// Site configuration information.
    pub site: SiteConfig,

    /// Analytics information.
    pub analytics: AnalyticsConfig,

    /// Menu information.
    pub menu: MenuConfig,
}

#[derive(Deserialize, Debug)]
/// Configuration information for the site.
pub struct SiteConfig {
    /// Site name (to appear in page title).
    pub name: String,

    /// Site icon path (for favicon).
    pub icon: Option<String>,

    /// Site style (for CSS source).
    pub style: Option<SiteStyle>,
}

#[derive(Deserialize, Debug)]
/// Configuration information for analytics.
pub struct AnalyticsConfig {
    /// Analytics source file.
    pub path: Option<String>,
}

#[derive(Deserialize, Debug)]
/// Configuration information for site menu.
pub struct MenuConfig {
    /// Menu source file.
    pub path: Option<String>,
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
    /// - `sitetree` (`&BlogResult<SiteTree>`): a reference
    /// to the sitetree, if it exists.
    /// 
    /// # Returns
    /// A `BlogResult<Config>` structure with all configuration information.
    pub fn get(sitetree: &BlogResult<SiteTree>) -> BlogResult<Config> {
        // Get site root
        let root = if let Ok (st) = sitetree.as_ref() {
            st.root.to_owned()
        } else {
            return Err (BlogError::CouldNotFindRoot.into());
        };

        // Construct configuration file name
        let config_file: PathBuf = root.join(CONFIG_FILE_NAME);

        // Read config file to string
        let config: String = fs::read_to_string(&config_file)?;

        Ok (toml::from_str(&config)?)
    }
}