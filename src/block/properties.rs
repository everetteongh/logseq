use rustc_hash::FxHashMap;
use std::fmt;
use uuid::Uuid;

use crate::error::BlockError;

#[derive(Debug, Clone)]
pub struct BlockProperties {
    pub template: Option<String>,
    pub template_including_parent: Option<bool>,
    pub collapsed: bool,
    pub id: Uuid,
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
