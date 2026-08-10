use crate::error::TaskStatusError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// The status of a task. We only support the TODO/DOING Logseq workflow at the moment.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TaskStatus {
    /// Represents `TODO`
    ToDo,
    /// Represents `DOING`
    Doing,
    /// Represents `DONE`
    Done,
    /// Represents `CANCELLED`
    Cancelled,
    /// Represents `WAITING`
    Waiting,
}

impl FromStr for TaskStatus {
    type Err = TaskStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TODO" => Ok(Self::ToDo),
            "DOING" => Ok(Self::Doing),
            "DONE" => Ok(Self::Done),
            "CANCELED" | "CANCELLED" => Ok(Self::Cancelled),
            "WAITING" => Ok(Self::Waiting),
            _ => Err(TaskStatusError::InvalidMarker),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToDo => write!(f, "TODO"),
            Self::Doing => write!(f, "DOING"),
            Self::Done => write!(f, "DONE"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Waiting => write!(f, "WAITING"),
        }
    }
}
