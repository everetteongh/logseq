mod priority;
mod status;
pub use priority::*;
pub use status::*;

use crate::block::{Block, view::Due};

#[derive(Debug, Clone)]
pub struct Task<'a> {
    pub block: &'a Block,
    pub status: TaskStatus,
    pub label: String,
    pub priority: Option<TaskPriority>,
    pub due: Option<Due>,
}

#[derive(Debug)]
pub struct TaskMut<'a> {
    pub block: &'a mut Block,
    pub status: TaskStatus,
    pub label: String,
    pub priority: Option<TaskPriority>,
    pub due: Option<Due>,
}

impl TaskMut<'_> {
    pub fn status(&mut self, marker: &TaskStatus) {
        self.block.markdown =
            self.block
                .markdown
                .replacen(&self.status.to_string(), &marker.to_string(), 1);
    }
}
