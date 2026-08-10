use rustc_hash::FxHashMap;
use std::fmt;
use uuid::Uuid;

use crate::error::BlockError;

/// Block properties. See [the official Logseq documentation](https://github.com/logseq/docs/blob/08f855f24d66e4509b7ea808554c13b4649e6ee1/pages/Built-in%20Properties.md).
#[derive(Debug, Clone)]
pub struct BlockProperties {
    /// Designates a page/block as a template.
    pub template: Option<String>,
    /// Specifies whether the parent level content of a block should be included when using a template.
    pub template_including_parent: Option<bool>,
    /// Whether or not this block is collapsed.
    pub collapsed: bool,
    /// The unique [`Uuid`] for this block.
    pub id: Uuid,
    /// Any other properties.
    pub custom: FxHashMap<String, String>,
}

impl Default for BlockProperties {
    fn default() -> Self {
        Self {
            template: None,
            template_including_parent: None,
            collapsed: false,
            id: Uuid::new_v4(),
            custom: FxHashMap::default(),
        }
    }
}

impl TryFrom<FxHashMap<String, String>> for BlockProperties {
    type Error = BlockError;

    fn try_from(map: FxHashMap<String, String>) -> Result<Self, Self::Error> {
        let mut properties = Self::default();

        for (key, value) in map {
            match key.as_str() {
                "template" => properties.template = Some(value),
                "template-including-parent" => {
                    properties.template_including_parent = Some(value.trim() == "true");
                }
                "collapsed" => properties.collapsed = value.trim() == "true",
                "id" => {
                    properties.id =
                        Uuid::try_parse(&value).map_err(|_| BlockError::InvalidID(value))?;
                }
                _ => _ = properties.custom.insert(key, value),
            }
        }

        Ok(properties)
    }
}

impl fmt::Display for BlockProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(template) = &self.template {
            writeln!(f, "template:: {template}")?;
            if self.template_including_parent.unwrap_or(false) {
                writeln!(f, "template-including-parent:: true")?;
            }
        }

        if self.collapsed {
            writeln!(f, "collapsed:: true")?;
        }

        for (key, value) in &self.custom {
            writeln!(f, "{key}:: {value}")?;
        }

        write!(f, "id:: {}", self.id)?;

        Ok(())
    }
}
