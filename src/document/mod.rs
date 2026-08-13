/// Document-level properties.
mod properties;
pub use properties::*;

use crate::{
    block::{Block, BlockProperties},
    consts::COMRAK_OPTIONS,
    error::Logseq,
    properties::Properties,
};
use comrak::{
    Arena, format_commonmark,
    nodes::{AstNode, NodeValue},
    parse_document,
};
use indexmap::{
    IndexMap,
    map::{Values, ValuesMut},
};
use std::{fmt::Display, str::FromStr};
use uuid::Uuid;

/// Calculate the depth of a given [`AstNode`].
fn determine_depth(mut node: &AstNode) -> usize {
    let mut depth: usize = 0;
    while let Some(parent) = node.parent() {
        if matches!(parent.data().value, NodeValue::List(_)) {
            depth += 1;
        }
        node = parent;
    }

    // All nodes we work with are wrapped by a [`List`], so subtract one for the "true" depth (i.e. 0 = top-level).
    depth.saturating_sub(1)
}

/// A Logseq document, which may or may not have [`DocumentProperties`].
#[derive(Default, Debug, Clone)]
pub struct Document {
    /// The document's [`DocumentProperties`].
    pub properties: Option<DocumentProperties>,
    /// The document's blocks. You should usually avoid accessing directly if [`Document`]'s methods are sufficient, but direct access is useful to e.g. delete all blocks or prefill a document with blocks.
    pub blocks_map: IndexMap<Uuid, Block>,
}

impl Document {
    /// Create a fresh document instance with defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Return an immutable iterator over the document's blocks. Ordered.
    #[must_use]
    pub fn blocks(&self) -> Values<'_, Uuid, Block> {
        self.blocks_map.values()
    }
    /// Return a mutable iterator over the document's blocks. Ordered.
    #[must_use]
    pub fn blocks_mut(&mut self) -> ValuesMut<'_, Uuid, Block> {
        self.blocks_map.values_mut()
    }
    /// Immutable reference to a block by ID.
    #[must_use]
    pub fn get(&self, id: Uuid) -> Option<&Block> {
        self.blocks_map.get(&id)
    }
    /// Mutable reference to a block by ID.
    #[must_use]
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Block> {
        self.blocks_map.get_mut(&id)
    }
    /// Prepend a block to the document.
    pub fn prepend(&mut self, block: Block) -> Option<Block> {
        self.blocks_map.shift_insert(0, block.properties.id, block)
    }
    /// Append a block to the document.
    pub fn append(&mut self, block: Block) -> Option<Block> {
        self.blocks_map.insert(block.properties.id, block)
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref props) = self.properties {
            writeln!(f, "{props}")?;
        }

        for block in self.blocks() {
            writeln!(f, "{block}")?;
        }

        Ok(())
    }
}

impl FromStr for Document {
    type Err = Logseq;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut markdown = input.to_string();
        let mut depth_stack: Vec<(Uuid, usize)> = Vec::new();

        let mut blocks_map = IndexMap::new();
        let properties = DocumentProperties::maybe_take_properties(&mut markdown)
            .transpose()
            .unwrap();

        let arena = Arena::new();
        let root = parse_document(&arena, &markdown, &COMRAK_OPTIONS);

        for node in root.descendants() {
            if let NodeValue::Item(_) = node.data().value {
                let depth = determine_depth(node);
                let mut block_markdown = String::new();

                for child in node
                    .children()
                    .filter(|node| !matches!(node.data().value, NodeValue::List(_)))
                {
                    format_commonmark(child, &COMRAK_OPTIONS, &mut block_markdown)?;
                }

                let block_properties = BlockProperties::take_properties(&mut block_markdown)?;
                let block_id = block_properties.id;

                while let Some((_, parent_depth)) = depth_stack.last() {
                    if parent_depth >= &depth {
                        depth_stack.pop();
                    } else {
                        break;
                    }
                }

                let parent_id = depth_stack.last().map(|(id, _)| *id);

                blocks_map.insert(
                    block_id,
                    Block {
                        markdown: block_markdown,
                        properties: block_properties,
                        parent: parent_id,
                        children: Vec::new(),
                        depth,
                    },
                );

                depth_stack.push((block_id, depth));

                if let Some(parent_block) = parent_id.and_then(|id| blocks_map.get_mut(&id)) {
                    parent_block.children.push(block_id);
                }
            }
        }

        Ok(Self {
            properties,
            blocks_map,
        })
    }
}
