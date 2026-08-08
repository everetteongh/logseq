use rustc_hash::FxHashMap;
use std::fmt;
use uuid::Uuid;

#[derive(Default, Debug, Clone)]
pub struct Property(pub String, pub String);

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(key, value) = self;

        write!(f, "{key}:: {value}")
    }
}

#[derive(Debug, Clone)]
pub struct Properties(pub FxHashMap<String, String>);

impl Default for Properties {
    fn default() -> Self {
        let mut properties = FxHashMap::default();
        properties.insert("id".to_string(), Uuid::new_v4().to_string());

        Self(properties)
    }
}

impl fmt::Display for Properties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(map) = self;

        for (key, value) in map {
            write!(f, "{key}:: {value}")?;
        }

        Ok(())
    }
}
