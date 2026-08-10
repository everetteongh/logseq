use crate::{
    consts::JOURNAL_FORMAT,
    error::{EntryError, Logseq},
    graph::Namespace,
};
use std::path::Path;
use time::Date;

/// A type of entry in a Logseq graph, either journal or page.
#[derive(Debug)]
pub enum EntryKind {
    /// A journal entry for a specific date.
    Journal(Date),
    /// A page with a specific [`Namespace`].
    Page(Namespace),
}

impl EntryKind {
    /// Converts this entry kind to a graph-root-relative path.
    ///
    /// # Panics
    /// This function calls [`Date::format`] with a format validated at compile-time, so **it will never panic..**
    #[must_use]
    pub fn as_relative_path(&self) -> String {
        match self {
            #[allow(clippy::unwrap_used)]
            Self::Journal(date) => format!("journals/{}.md", date.format(JOURNAL_FORMAT).unwrap()),
            Self::Page(namespace) => format!("pages/{namespace}.md"),
        }
    }
}

impl TryFrom<&Path> for EntryKind {
    type Error = Logseq;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let stem = path
            .file_stem()
            .ok_or_else(|| EntryError::InvalidPath(path.to_path_buf()))?
            .to_string_lossy();

        for ancestor in path.ancestors() {
            if ancestor.ends_with("journals") {
                return Ok(Self::Journal(Date::parse(&stem, JOURNAL_FORMAT)?));
            } else if ancestor.ends_with("pages") {
                return Ok(Self::Page(Namespace::from(stem.to_string())));
            }
        }

        Err(EntryError::InvalidPath(path.to_path_buf()).into())
    }
}
