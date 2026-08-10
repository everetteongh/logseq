use rustc_hash::FxHashMap;
use std::{convert::Infallible, fmt};

#[derive(Default, Debug, Clone)]
pub struct DocumentProperties {
    pub icon: Option<String>,
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub template: Option<String>,
    pub template_including_parent: Option<bool>,
    pub alias: Vec<String>,
    pub filters: Option<String>,
    pub public: bool,
    pub exclude_from_graph_view: bool,
    pub custom: FxHashMap<String, String>,
}

#[allow(clippy::infallible_try_from)]
impl TryFrom<FxHashMap<String, String>> for DocumentProperties {
    type Error = Infallible;

    fn try_from(map: FxHashMap<String, String>) -> Result<Self, Infallible> {
        let mut properties = Self::default();

        for (key, value) in map {
            match key.as_str() {
                "icon" => properties.icon = Some(value),
                "title" => properties.title = Some(value),
                "tags" => properties.tags = value.split(',').map(String::from).collect(),
                "template" => properties.template = Some(value),
                "template-including-parent" => {
                    properties.template_including_parent = Some(value.trim() == "true");
                }
                "alias" => properties.alias = value.split(',').map(String::from).collect(),
                "filters" => properties.filters = Some(value),
                "public" => properties.public = value == "true",
                "exclude-from-graph-view" => properties.exclude_from_graph_view = value == "true",
                _ => _ = properties.custom.insert(key, value),
            }
        }

        Ok(properties)
    }
}

impl fmt::Display for DocumentProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(icon) = &self.icon {
            writeln!(f, "icon:: {icon}")?;
        }

        if let Some(title) = &self.title {
            writeln!(f, "title:: {title}")?;
        }

        if !self.tags.is_empty() {
            writeln!(f, "tags:: {}", self.tags.join(","))?;
        }

        if let Some(template) = &self.template {
            writeln!(f, "template:: {template}")?;
            if self.template_including_parent.unwrap_or(false) {
                writeln!(f, "template-including-parent:: true")?;
            }
        }

        if !self.alias.is_empty() {
            writeln!(f, "alias:: {}", self.alias.join(","))?;
        }

        if let Some(filters) = &self.filters {
            writeln!(f, "filters:: {filters}")?;
        }

        if self.public {
            writeln!(f, "public:: true")?;
        }

        if self.exclude_from_graph_view {
            writeln!(f, "exclude-from-graph-view:: true")?;
        }

        for (key, value) in &self.custom {
            writeln!(f, "{key}:: {value}")?;
        }

        Ok(())
    }
}
