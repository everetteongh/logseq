use crate::consts::NAMESPACE_DELIM;
use std::fmt;

/// A Logseq page namespace. Used to build page filenames and represent a virtual directory structure for pages.
#[derive(Debug)]
pub struct Namespace(pub Vec<String>);

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(NAMESPACE_DELIM))
    }
}

impl From<String> for Namespace {
    fn from(s: String) -> Self {
        Self(s.split(NAMESPACE_DELIM).map(String::from).collect())
    }
}
