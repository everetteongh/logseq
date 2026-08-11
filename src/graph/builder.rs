use crate::{
    consts::EXCLUDE,
    error::{GraphBuilderError, Logseq},
    graph::Graph,
};
use std::path::PathBuf;

/// A helper object for constructing a [`Graph`].
pub struct GraphBuilder {
    /// Paths for the [`WalkDir`] directory crawler to exclude.
    exclude: Vec<&'static str>,
    /// The root of the Logseq graph.
    dir: Option<PathBuf>,
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self {
            exclude: EXCLUDE.to_vec(),
            dir: None,
        }
    }
}

impl GraphBuilder {
    /// Paths for the [`walkdir::WalkDir`] directory crawler to exclude.
    #[must_use]
    pub fn exclude(mut self, exclude: &[&'static str]) -> Self {
        self.exclude = exclude.to_vec();
        self
    }
    /// The root of the Logseq graph.
    #[must_use]
    pub fn dir(mut self, dir: PathBuf) -> Self {
        self.dir = Some(dir);
        self
    }
    /// Try to construct a [`Graph`] object from the builder fields.
    ///
    /// # Errors
    /// Fails if the root directory (`self.dir`) is unset.
    pub fn build(self) -> Result<Graph, Logseq> {
        let dir = self.dir.ok_or(GraphBuilderError::UndefinedRootDirectory)?;

        Ok(Graph {
            exclude: self.exclude,
            dir,
        })
    }
}
