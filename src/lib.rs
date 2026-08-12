#![warn(clippy::unwrap_used)]
#![doc = include_str!("../README.md")]

/// [Block](https://github.com/logseq/docs/blob/08f855f24d66e4509b7ea808554c13b4649e6ee1/pages/term___block.md)-level code, including views over blocks, like [`block::view::Task`] and [`block::view::DueBlock`].
pub mod block;
/// Internal constants.
pub mod consts;
/// Document-specific code.
pub mod document;
/// Error types.
pub mod error;
/// Graph code, providing the entrypoint for working with a graph directory.
pub mod graph;
/// Module containing a common [`properties::Properties`] trait for block and document properties.
pub mod properties;

/// Convenience module for glob imports.
pub mod prelude {
    pub use crate::block::{
        Block, BlockProperties,
        view::{Due, DueBlock, DueBlockMut, Task, TaskMut, TaskPriority, TaskStatus},
    };
    pub use crate::consts::LOGSEQ_EXCLUDE;
    pub use crate::document::{Document, DocumentProperties};
    pub use crate::error::*;
    pub use crate::graph::{EntryKind, Graph, GraphBuilder, GraphEntry, Namespace};
    pub use crate::properties::Properties;
}
