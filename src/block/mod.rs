mod properties;
pub use properties::*;

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

#[derive(Default, Debug, Clone)]
pub struct Block {
    pub markdown: String,
    pub properties: BlockProperties,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
    pub depth: usize,
}

impl Block {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
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
    #[must_use]
    pub fn task(&self) -> Option<Task<'_>> {
        Task::try_from(self).ok()
    }
    #[must_use]
    pub fn task_mut(&mut self) -> Option<TaskMut<'_>> {
        TaskMut::try_from(self).ok()
    }
    #[must_use]
    pub fn due(&self) -> Option<DueBlock<'_>> {
        DueBlock::try_from(self).ok()
    }
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
