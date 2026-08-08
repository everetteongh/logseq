mod kind;
mod namespace;
pub use kind::*;
pub use namespace::*;

use crate::document::Document;

pub struct GraphEntry {
    pub kind: EntryKind,
    document: Option<Document>,
}
