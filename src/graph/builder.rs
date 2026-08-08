use crate::{
    consts::EXCLUDE,
    error::{Alleged, GraphBuilderError},
    graph::Graph,
};
use std::path::PathBuf;

/// Helper struct to construct a [`Graph`] object.
pub struct GraphBuilder {
    exclude: &'static [&'static str],
    dir: Option<PathBuf>,
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self {
            exclude: &EXCLUDE,
            dir: None,
        }
    }
}

impl GraphBuilder {
    /// List of file/directory names to exclude from the directory walker
    #[must_use]
    pub const fn exclude(mut self, exclude: &'static [&'static str]) -> Self {
        self.exclude = exclude;
        self
    }
    /// The root of your Logseq graph
    #[must_use]
    pub fn dir(mut self, dir: PathBuf) -> Self {
        self.dir = Some(dir);
        self
    }
    /// Try to build a [`crate::graph::Graph`]
    ///
    /// # Errors
    /// Fails if the root directory isn't set.
    pub fn build(self) -> Result<Graph, Alleged> {
        let dir = self.dir.ok_or(GraphBuilderError::UndefinedRootDirectory)?;

        Ok(Graph {
            exclude: self.exclude,
            dir,
        })
    }
}
