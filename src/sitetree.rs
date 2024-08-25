//! Site tree management for the Blog Builder.

use std::path::PathBuf;

use walkdir::{
    DirEntry,
    WalkDir,
};

use crate::{
    BlogError,
    BlogResult,
    getroot,
    SOURCE_DIR_NAME,
    SOURCE_FILE_EXT,
};

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
    /// A list of all files relative to the source directory.
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
            files,
        })
    }
}