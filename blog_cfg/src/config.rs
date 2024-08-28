//! Configuration file management for the Blog Builder.

use std::{
    fmt,
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

    /// Menu information.
    pub menu: Option<MenuConfig>,

    /// Analytics information.
    pub analytics: Option<AnalyticsConfig>,

    /// Media information.
    pub media: Option<MediaConfig>,
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
#[serde(rename_all = "kebab-case")]
/// Site style options.
pub enum SiteStyle {
    /// Technology style.
    Tech,
}

// Used for debugging.
impl fmt::Display for SiteStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SiteStyle::*;
        let output = match self {
            Tech => "tech",
        };

        write!(f, "{}", output)
    }
}

#[derive(Clone, Deserialize, Debug)]
/// Configuration information for site menu.
pub struct MenuConfig {
    /// Menu button names.
    pub names: Vec<String>,

    /// Menu button links.
    pub links: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
/// Configuration information for analytics data.
pub struct AnalyticsConfig {
    /// Location of analytics tag relative to source directory.
    pub tag: String,
}

#[derive(Clone, Deserialize, Debug)]
/// Configuration information for site media (images, etc.).
pub struct MediaConfig {
    /// File paths, relative to media subdir, of
    /// media to include.
    pub include: Vec<String>,
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