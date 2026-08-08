use crate::graph::Namespace;
use time::Date;

pub enum EntryKind {
    Journal(Date),
    Page(Namespace),
}
