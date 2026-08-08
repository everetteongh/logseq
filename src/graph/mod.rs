mod builder;
mod entry;

pub use builder::*;
pub use entry::*;

use std::{ffi::OsStr, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Representation of a Logseq graph
pub struct Graph {
    exclude: &'static [&'static str],
    /// The path to your Logseq graph root -- i.e., a folder with the following subdirectories:
    /// - `journals/`
    /// - `logseq/`
    /// - `pages/`
    pub dir: PathBuf,
}

impl Graph {
    fn is_excluded(&self, entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .is_some_and(|name| self.exclude.contains(&name))
    }
    fn markdown_files(&self) -> impl Iterator<Item = DirEntry> {
        WalkDir::new(&self.dir)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !self.is_excluded(e))
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.file_type().is_file() && entry.path().extension() == Some(OsStr::new("md"))
            })
    }
    /// Create an instance of [`Graph`] with the builder pattern
    #[must_use]
    pub fn builder() -> GraphBuilder {
        GraphBuilder::default()
    }
}
