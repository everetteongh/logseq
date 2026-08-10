mod priority;
mod status;

pub use priority::*;
pub use status::*;

use crate::{
    block::{Block, view::Due},
    consts::DUE_DELIMS,
    error::TaskError,
};
use std::str::FromStr;

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
    pub fn status(&mut self, status: &TaskStatus) {
        self.block.markdown =
            self.block
                .markdown
                .replacen(&self.status.to_string(), &status.to_string(), 1);
        self.status = status.clone();
    }
}

impl<'a> TryFrom<&'a mut Block> for TaskMut<'a> {
    type Error = TaskError;

    fn try_from(block: &'a mut Block) -> Result<Self, Self::Error> {
        let plain = block.content();
        let mut words = plain.split_whitespace().peekable();

        let status = TaskStatus::from_str(words.next().ok_or(TaskError::EmptyItem)?)?;
        let priority = words.peek().and_then(|w| w.parse().ok()).inspect(|_| {
            words.next();
        });

        let label = words.collect::<Vec<_>>().join(" ");
        let (label, maybe_due_str) = DUE_DELIMS
            .iter()
            .find_map(|d| label.find(d))
            .map_or((label.as_str(), ""), |idx| label.split_at(idx));

        Ok(Self {
            label: label.to_string(),
            due: Due::from_str(maybe_due_str).ok(),
            block,
            status,
            priority,
        })
    }
}

impl<'a> TryFrom<&'a Block> for Task<'a> {
    type Error = TaskError;

    fn try_from(block: &'a Block) -> Result<Self, Self::Error> {
        let plain = block.content();
        let mut words = plain.split_whitespace().peekable();

        let status = TaskStatus::from_str(words.next().ok_or(TaskError::EmptyItem)?)?;
        let priority = words.peek().and_then(|w| w.parse().ok()).inspect(|_| {
            words.next();
        });

        let label = words.collect::<Vec<_>>().join(" ");
        let (label, maybe_due_str) = DUE_DELIMS
            .iter()
            .find_map(|d| label.find(d))
            .map_or((label.as_str(), ""), |idx| label.split_at(idx));

        Ok(Self {
            label: label.to_string(),
            due: Due::from_str(maybe_due_str).ok(),
            block,
            status,
            priority,
        })
    }
}
