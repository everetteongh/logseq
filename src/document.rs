use crate::{block::Block, properties::Properties};
use indexmap::{
    IndexMap,
    map::{Values, ValuesMut},
};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Default, Debug, Clone)]
pub struct Document {
    pub properties: Option<Properties>,
    blocks: IndexMap<Uuid, Block>,
}

impl Document {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn blocks(&self) -> Values<'_, Uuid, Block> {
        self.blocks.values()
    }
    #[must_use]
    pub fn blocks_mut(&mut self) -> ValuesMut<'_, Uuid, Block> {
        self.blocks.values_mut()
    }
    #[must_use]
    pub fn get_block(&self, id: Uuid) -> Option<&Block> {
        self.blocks.get(&id)
    }
    #[must_use]
    pub fn get_block_mut(&mut self, id: Uuid) -> Option<&mut Block> {
        self.blocks.get_mut(&id)
    }
    pub fn push(&mut self, block: Block) -> Option<Block> {
        self.blocks.insert(block.id, block)
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref props) = self.properties {
            writeln!(f, "{props}\n")?;
        }

        for block in self.blocks() {
            writeln!(f, "{block}")?;
        }

        Ok(())
    }
}
