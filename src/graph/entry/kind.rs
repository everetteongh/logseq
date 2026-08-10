use crate::{
    consts::JOURNAL_FORMAT,
    error::{Alleged, EntryError},
    graph::Namespace,
};
use std::path::Path;
use time::Date;

#[derive(Debug)]
pub enum EntryKind {
    Journal(Date),
    Page(Namespace),
}

impl EntryKind {
    #[must_use]
    pub fn as_relative_path(&self) -> String {
        match self {
            // NOTE: `JOURNAL_FORMAT` is guaranteed valid @ compile time, so **this will never panic**.
            #[allow(clippy::unwrap_used)]
            Self::Journal(date) => format!("journals/{}.md", date.format(JOURNAL_FORMAT).unwrap()),
            Self::Page(namespace) => format!("pages/{namespace}.md"),
        }
    }
}

impl TryFrom<&Path> for EntryKind {
    type Error = Alleged;

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
