/// Block-level properties.
mod properties;
pub use properties::*;

/// A view over a block, like a [`Task`].
pub mod view;

use crate::{
    block::view::{DueBlock, DueBlockMut, Task, TaskMut},
    consts::COMRAK_OPTIONS,
};
use comrak::{
    Arena,
    nodes::{AstNode, NodeValue},
    parse_document,
};
use std::fmt;
use uuid::Uuid;

/// Push the content of a given [`AstNode`] into the given buffer. Like [`comrak::arena_tree::Node::collect_text_append`], but uses newlines for line breaks.
fn plain_text<'a>(node: &'a AstNode<'a>, buf: &mut String) {
    match &node.data().value {
        NodeValue::Text(text) => buf.push_str(text),
        NodeValue::SoftBreak | NodeValue::LineBreak => buf.push('\n'),
        NodeValue::Item(_) => {}
        _ => {
            for child in node.children() {
                plain_text(child, buf);
            }
        }
    }
}

/// A [Logseq block](https://github.com/logseq/docs/blob/08f855f24d66e4509b7ea808554c13b4649e6ee1/pages/term___block.md).
#[derive(Default, Debug, Clone)]
pub struct Block {
    /// The markdown of this block. Doesn't include a leading bullet point.
    pub markdown: String,
    /// This block's properties. Contains the ID.
    pub properties: BlockProperties,
    /// The block's parent, if it exists.
    pub parent: Option<Uuid>,
    /// The block's children.
    pub children: Vec<Uuid>,
    /// The block's depth (i.e. number of parents).
    pub depth: usize,
}

impl Block {
    /// Create a new block with defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Extract the non-markdown content of this block.
    #[must_use]
    pub fn content(&self) -> String {
        let mut buf = String::new();
        let arena = Arena::new();
        let block_root = parse_document(&arena, &self.markdown, &COMRAK_OPTIONS);
        for child in block_root.children() {
            match &child.data().value {
                NodeValue::Item(_) => {}
                _ => plain_text(child, &mut buf),
            }
        }

        buf
    }
    /// Extract the plaintext content of this block. Like [`Self::content`], but handles trimming [`view::Due`]/[`Task`] data.
    #[must_use]
    pub fn plain(&self) -> String {
        if let Some(task) = self.task() {
            task.label
        } else if let Some(due) = self.due() {
            due.plain()
        } else {
            self.content()
        }
    }
    /// The task view over this block, if it's a task.
    #[must_use]
    pub fn task(&self) -> Option<Task<'_>> {
        Task::try_from(self).ok()
    }
    /// Same as [`Self::task`], but mutable.
    #[must_use]
    pub fn task_mut(&mut self) -> Option<TaskMut<'_>> {
        TaskMut::try_from(self).ok()
    }
    /// The due view over this block, if it has `SCHEDULED`/`DEADLINE`.
    #[must_use]
    pub fn due(&self) -> Option<DueBlock<'_>> {
        DueBlock::try_from(self).ok()
    }
    /// Same as [`Self::due`], but mutable.
    #[must_use]
    pub fn due_mut(&mut self) -> Option<DueBlockMut<'_>> {
        DueBlockMut::try_from(self).ok()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = "\t".repeat(self.depth);

        for (i, line) in self.markdown.lines().enumerate() {
            if i == 0 {
                writeln!(f, "{indent}- {line}")?;
            } else {
                writeln!(f, "{indent}  {line}")?;
            }
        }

        write!(f, "{indent}  {}", self.properties.to_string().trim_end())
    }
}
