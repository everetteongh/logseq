use humantime::DurationError;
#[cfg(feature = "python")]
use pyo3::{PyErr, exceptions::PyValueError};
use std::{fmt, io, path::PathBuf};
use thiserror::Error;
use time::error::{IndeterminateOffset, InvalidVariant, Parse};

/// A Logseq-related error.
#[derive(Error, Debug)]
pub enum Logseq {
    /// Block error.
    #[error("Block error: {0}")]
    Block(#[from] BlockError),
    /// Graph error.
    #[error("Graph-related failure: {0}")]
    Graph(#[from] GraphError),
    /// Graph builder error.
    #[error("Graph builder failed: {0}")]
    GraphBuilder(#[from] GraphBuilderError),
    /// Entry error.
    #[error("Entry-related error: {0}")]
    Entry(#[from] EntryError),
    /// Repeater parse error.
    #[error("Repeater parsing failed: {0}")]
    ParseRepeater(#[from] ParseRepeaterErr),
    /// Due parse error.
    #[error("Due parsing failed: {0}")]
    ParseScheduled(#[from] ParseDueError),
    /// `std::io` error.
    #[error("Got an I/O error: {0}")]
    IO(#[from] io::Error),
    /// `std::fmt` error.
    #[error("Got a formatting error: {0}")]
    Fmt(#[from] fmt::Error),
    /// Date parse error.
    #[error("Date string parsing failed: {0}")]
    Date(#[from] Parse),
    /// Date offset determination error.
    #[error("Couldn't determine local date offset: {0}")]
    DateOffset(#[from] IndeterminateOffset),
    /// Time string parse error.
    #[error("Time string parsing failed: {0}")]
    Time(#[from] InvalidVariant),
    /// `humantime` parse error.
    #[error("HumanTime duration conversion failed: {0}")]
    HumanTime(#[from] DurationError),
}

#[cfg(feature = "python")]
impl From<Logseq> for PyErr {
    fn from(error: Logseq) -> Self {
        let error_str = format!("{error:?}");
        PyValueError::new_err(error_str)
    }
}

/// Block error.
#[derive(Error, Debug)]
pub enum BlockError {
    /// Invalid block ID (expected [`uuid::Uuid`]-compatible string).
    #[error("Invalid block ID: {0}")]
    InvalidID(String),
}

/// Graph entry error.
#[derive(Error, Debug)]
pub enum EntryError {
    /// Invalid Logseq graph entry path.
    #[error("Invalid graph entry path: {0}")]
    InvalidPath(PathBuf),
}

/// Graph builder error.
#[derive(Error, Debug)]
pub enum GraphBuilderError {
    /// Root directory wasn't defined.
    #[error("Root directory wasn't defined!")]
    UndefinedRootDirectory,
}

/// Graph error.
#[derive(Error, Debug)]
pub enum GraphError {
    /// Invalid Logseq graph entry path.
    #[error("Invalid Logseq graph entry path: {0}")]
    InvalidPath(PathBuf),
}

/// Task status parsing error.
#[derive(Error, Debug)]
pub enum TaskStatusError {
    /// Invalid task marker string.
    #[error("Invalid task marker str!")]
    InvalidMarker,
}

/// Task parsing error.
#[derive(Error, Debug)]
pub enum TaskError {
    /// The given line wasn't a task.
    #[error("The given line was not a task!")]
    NotATask,
    /// The given list item was empty.
    #[error("The given list item was empty!")]
    EmptyItem,
    /// Task status parsing failed.
    #[error("Got an error when processing the task marker: {0}")]
    TaskStatus(#[from] TaskStatusError),
}

/// Repeater parsing error.
#[derive(Error, Debug)]
pub enum ParseRepeaterErr {
    /// Invalid repeater string.
    #[error("Invalid repeater string!")]
    InvalidRepeater,
}

/// Task priority parsing error.
#[derive(Error, Debug)]
pub enum TaskPriorityError {
    /// Invalid task priority.
    #[error("Invalid task priority!")]
    InvalidPriority,
}

/// Due parsing error.
#[derive(Error, Debug)]
pub enum ParseDueError {
    /// Invlaid input.
    #[error("Invalid input!")]
    InvalidInput,
}
