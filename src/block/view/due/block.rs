use crate::{
    block::{Block, view::Due},
    error::ParseDueError,
};

#[derive(Debug, Clone)]
pub struct DueBlock<'a> {
    pub block: &'a Block,
    pub due: Due,
}

impl DueBlock<'_> {
    #[must_use]
    pub fn plain(&self) -> String {
        self.block
            .content()
            .replacen(&self.due.to_string(), "", 1)
            .trim()
            .to_string()
    }
}

impl<'a> TryFrom<&'a Block> for DueBlock<'a> {
    type Error = ParseDueError;

    fn try_from(block: &'a Block) -> Result<Self, Self::Error> {
        let due = block
            .content()
            .parse()
            .map_err(|_| ParseDueError::InvalidInput)?;
        Ok(Self { block, due })
    }
}

#[derive(Debug)]
pub struct DueBlockMut<'a> {
    pub block: &'a mut Block,
    pub due: Due,
}

impl<'a> TryFrom<&'a mut Block> for DueBlockMut<'a> {
    type Error = ParseDueError;

    fn try_from(block: &'a mut Block) -> Result<Self, Self::Error> {
        let due = block
            .content()
            .parse()
            .map_err(|_| ParseDueError::InvalidInput)?;
        Ok(Self { block, due })
    }
}
