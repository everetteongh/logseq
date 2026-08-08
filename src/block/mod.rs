pub mod view;

use crate::{
    block::view::{Task, TaskMut},
    properties::Properties,
};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Block {
    pub id: Uuid,
    pub markdown: String,
    pub properties: Properties,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
    pub depth: usize,
}

impl Block {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn plain() -> String {
        todo!()
    }
    pub fn task(&self) -> Option<Task<'_>> {
        None
    }
    pub fn task_mut(&mut self) -> Option<TaskMut<'_>> {
        None
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            markdown: String::new(),
            properties: Properties::default(),
            parent: None,
            children: Vec::new(),
            depth: 0,
        }
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

        write!(f, "{indent}  {}", self.properties)
    }
}
