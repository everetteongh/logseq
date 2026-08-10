/// Entry types.
mod kind;
/// Page namespaces; used to build filenames.
mod namespace;

pub use kind::*;
pub use namespace::*;

use crate::{document::Document, error::Logseq};
use std::{
    fs,
    ops::{Deref, DerefMut},
    path::PathBuf,
    str::FromStr,
};

/// An entry in a Logseq graph.
#[derive(Debug)]
pub struct GraphEntry {
    /// The kind of entry.
    pub kind: EntryKind,
    /// The underlying document.
    pub document: Document,
    /// The path to this entry.
    pub path: PathBuf,
}

impl GraphEntry {
    /// Try to create a new entry from a given path.
    ///
    /// # Errors
    /// Fails if the given path doesn't reside in a `journals/` or `pages/` directory.
    pub fn try_new(path: PathBuf) -> Result<Self, Logseq> {
        let kind = EntryKind::try_from(path.as_path())?;
        let document = Document::from_str(
            fs::read_to_string(&path)
                .unwrap_or_else(|_| String::new())
                .as_str(),
        )?;

        Ok(Self {
            kind,
            document,
            path,
        })
    }
    /// Save this graph entry to disk.
    ///
    /// # Errors
    /// Fails if the [`fs::write`] call does.
    pub fn save_to_disk(&self) -> Result<(), Logseq> {
        fs::write(&self.path, self.document.to_string().as_bytes())?;

        Ok(())
    }
}

impl Deref for GraphEntry {
    type Target = Document;

    fn deref(&self) -> &Self::Target {
        &self.document
    }
}

impl DerefMut for GraphEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.document
    }
}
