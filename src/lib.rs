#![warn(clippy::unwrap_used)]
#![doc = include_str!("../README.md")]

pub mod block;
pub(crate) mod consts;
pub mod document;
pub mod error;
pub mod graph;
pub mod properties;

pub mod prelude {
    pub use crate::block::{
        Block, BlockProperties,
        view::{Task, TaskMut, TaskPriority, TaskStatus},
    };
    pub use crate::document::*;
    pub use crate::error::*;
    pub use crate::graph::*;
    pub use crate::properties::*;
}
