mod builder;
mod entry;

pub use builder::*;
pub use entry::*;
use time::{Date, OffsetDateTime};

use std::{ffi::OsStr, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

use crate::error::Alleged;

pub struct Graph {
    exclude: &'static [&'static str],
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
    #[must_use]
    pub fn builder() -> GraphBuilder {
        GraphBuilder::default()
    }
    pub fn entries(&self) -> impl Iterator<Item = GraphEntry> {
        self.markdown_files()
            .filter_map(|entry| GraphEntry::try_new(entry.into_path()).ok())
    }
    pub fn journals(&self) -> impl Iterator<Item = GraphEntry> {
        self.entries()
            .filter(|entry| matches!(entry.kind, EntryKind::Journal(_)))
    }
    pub fn pages(&self) -> impl Iterator<Item = GraphEntry> {
        self.entries()
            .filter(|entry| matches!(entry.kind, EntryKind::Page(_)))
    }
    fn entry(&self, entry: &EntryKind) -> Result<GraphEntry, Alleged> {
        let relative_path: PathBuf = entry.as_relative_path().into();
        GraphEntry::try_new(self.dir.join(relative_path))
    }
    pub fn journal<D>(&self, date: D) -> Result<GraphEntry, Alleged>
    where
        D: Into<Date>,
    {
        self.entry(&EntryKind::Journal(date.into()))
    }
    pub fn today(&self) -> Result<GraphEntry, Alleged> {
        self.journal(OffsetDateTime::now_local()?.date())
    }
    pub fn page(&self, key: &str) -> Result<GraphEntry, Alleged> {
        for entry in self.entries() {
            if let Some(ref properties) = entry.document.properties
                && properties.alias.iter().any(|a| a == key)
            {
                return Ok(entry);
            }
        }

        self.entry(&EntryKind::Page(Namespace::from(key.to_string())))
    }
}
