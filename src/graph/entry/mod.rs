mod kind;
mod namespace;

pub use kind::*;
pub use namespace::*;

use crate::{document::Document, error::Alleged};
use std::{fs, path::PathBuf, str::FromStr};

#[derive(Debug)]
pub struct GraphEntry {
    pub kind: EntryKind,
    pub path: PathBuf,
    pub document: Document,
}

impl GraphEntry {
    pub fn try_new(path: PathBuf) -> Result<Self, Alleged> {
        let kind = EntryKind::try_from(path.as_path())?;
        let document = Document::from_str(
            fs::read_to_string(&path)
                .unwrap_or_else(|_| String::new())
                .as_str(),
        )?;

        Ok(Self {
            kind,
            path,
            document,
        })
    }
    pub fn save(&self) -> Result<(), Alleged> {
        fs::write(&self.path, self.document.to_string().as_bytes())?;

        Ok(())
    }
}
