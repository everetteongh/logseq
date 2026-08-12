/// Task priority -- one of [#A], [#B], or [#C].
mod priority;
/// Task status.
mod status;

pub use priority::*;
pub use status::*;

#[cfg(feature = "icalendar")]
use crate::block::view::DueKind;
use crate::{
    block::{Block, view::Due},
    consts::DUE_DELIMS,
    error::TaskError,
};
#[cfg(feature = "icalendar")]
use icalendar::{Component, DatePerhapsTime, EventLike, Todo};
use std::str::FromStr;

/// View over a [`Block`] which starts with a [`TaskStatus`] marker..
#[derive(Debug, Clone)]
pub struct Task<'a> {
    /// Underlying block.
    pub block: &'a Block,
    /// The task's status.
    pub status: TaskStatus,
    /// The task's label.
    pub label: String,
    /// The task's priority.
    pub priority: Option<TaskPriority>,
    /// The task's due date.
    pub due: Option<Due>,
}

/// Same as [`Task`], but mutable.
#[derive(Debug)]
pub struct TaskMut<'a> {
    /// Underlying mutable block.
    pub block: &'a mut Block,
    /// The task's status.
    pub status: TaskStatus,
    /// The task's label.
    pub label: String,
    /// The task's priority.
    pub priority: Option<TaskPriority>,
    /// The task's due date.
    pub due: Option<Due>,
}

impl TaskMut<'_> {
    /// Update the status of a task.
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

#[cfg(feature = "icalendar")]
impl From<Task<'_>> for Todo {
    fn from(task: Task<'_>) -> Self {
        let mut to_do = Self::with_uid(&task.block.properties.id.to_string());
        let maybe_due = task.due.clone().map(DatePerhapsTime::from);
        let maybe_due_kind = task.due.map(|d| d.kind);

        if let Some(due) = maybe_due {
            if matches!(maybe_due_kind, Some(DueKind::Deadline)) {
                to_do.due(due);
            } else {
                to_do.starts(due);
            }
        }

        to_do.status(task.status.into());
        to_do.summary(&task.label);

        to_do
    }
}
