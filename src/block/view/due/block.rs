use crate::{
    block::{Block, view::Due},
    error::ParseDueError,
};

/// A view over a Logseq block for [`Due`]. Should be revised in the future; currently exists just to provide a `plain` method.
#[derive(Debug, Clone)]
pub struct DueBlock<'a> {
    /// The underlying block.
    pub block: &'a Block,
    /// The block's associated [`Due`].
    pub due: Due,
}

impl DueBlock<'_> {
    /// Get a plaintext representation of this block's content.
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

/// Same as [`DueBlock`], but mutable access. More-or-less unimplemented.
#[derive(Debug)]
pub struct DueBlockMut<'a> {
    /// The underlying block, mutable.
    pub block: &'a mut Block,
    /// The block's associated [`Due`].
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
