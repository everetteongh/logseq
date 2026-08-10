/// Helper to construct a Logseq graph.
mod builder;
/// Logseq graph entries.
mod entry;

pub use builder::*;
pub use entry::*;
use time::{Date, OffsetDateTime};

use crate::error::Logseq;
use std::{ffi::OsStr, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Representation of a Logseq graph.
pub struct Graph {
    /// Paths for the [`WalkDir`] directory crawler to exclude.
    exclude: &'static [&'static str],
    /// The graph's root.
    pub dir: PathBuf,
}

impl Graph {
    /// Check `self.exclude` to see whether or not the entry is excluded.
    fn is_excluded(&self, entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .is_some_and(|name| self.exclude.contains(&name))
    }
    /// Shorthand for [`walkdir::WalkDir`] crawler access. Filtered by `Self::is_excluded` and requiring the `.md` extension.
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
    /// Convenient access to the [`GraphBuilder`].
    #[must_use]
    pub fn builder() -> GraphBuilder {
        GraphBuilder::default()
    }
    /// All entries in the graph.
    pub fn entries(&self) -> impl Iterator<Item = GraphEntry> {
        self.markdown_files()
            .filter_map(|entry| GraphEntry::try_new(entry.into_path()).ok())
    }
    /// All journal entries in the graph.
    pub fn journals(&self) -> impl Iterator<Item = GraphEntry> {
        self.entries()
            .filter(|entry| matches!(entry.kind, EntryKind::Journal(_)))
    }
    /// All non-journal pages in the graph.
    pub fn pages(&self) -> impl Iterator<Item = GraphEntry> {
        self.entries()
            .filter(|entry| matches!(entry.kind, EntryKind::Page(_)))
    }
    /// Construct a [`GraphEntry`] from the given [`EntryKind`].
    fn entry(&self, entry: &EntryKind) -> Result<GraphEntry, Logseq> {
        let relative_path: PathBuf = entry.as_relative_path().into();
        GraphEntry::try_new(self.dir.join(relative_path))
    }
    /// Try to get a [`GraphEntry`] for the given date.
    ///
    /// # Errors
    /// Fails if the given path doesn't reside in a `journals/` or `pages/` directory.
    pub fn journal<D>(&self, date: D) -> Result<GraphEntry, Logseq>
    where
        D: Into<Date>,
    {
        self.entry(&EntryKind::Journal(date.into()))
    }
    /// Convenience function to access today's journal entry. Scoped to the local timezone via [`OffsetDateTime::now_local`].
    ///
    /// # Errors
    /// Fails if the underlying `Self::journal` call does.
    pub fn today(&self) -> Result<GraphEntry, Logseq> {
        self.journal(OffsetDateTime::now_local()?.date())
    }
    /// Convenience page access function. First checks aliases, and if no aliases match the given `key`, returns a new entry from that key after turning it into a [`Namespace`].
    ///
    /// # Errors
    /// Fails if the given path doesn't reside in a `journals/` or `pages/` directory.
    pub fn page(&self, key: &str) -> Result<GraphEntry, Logseq> {
        for entry in self.entries() {
            if let Some(ref properties) = entry.document.properties
                && properties.alias.iter().any(|a| a == key)
            {
                return Ok(entry);
            }
        }

        self.entry(&EntryKind::Page(Namespace::from(key.to_string())))
    }
    /// Save a graph entry to disk. Prefer using [`GraphEntry::save_to_disk`].
    ///
    /// # Errors
    /// Fails if the [`std::fs::write`] call does.
    pub fn save_to_disk(&self, entry: &GraphEntry) -> Result<(), Logseq> {
        entry.save_to_disk()
    }
}
