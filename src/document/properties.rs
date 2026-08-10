use rustc_hash::FxHashMap;
use std::{convert::Infallible, fmt};

/// Document properties. See [the official Logseq documentation](https://github.com/logseq/docs/blob/08f855f24d66e4509b7ea808554c13b4649e6ee1/pages/Built-in%20Properties.md).
#[derive(Default, Debug, Clone)]
pub struct DocumentProperties {
    /// The icon identifier for a page.
    pub icon: Option<String>,
    /// Custom page title; overrides filename.
    pub title: Option<String>,
    /// From the official docs:
    /// > get listed in their own section "Pages tagged with X" below a page.
    ///
    /// We just return the property value -- you'll have to parse it yourself.
    pub tags: Option<String>,
    /// Designates a page/block as a template.
    pub template: Option<String>,
    /// Specifies whether the parent level content of a block should be included when using a template.
    pub template_including_parent: Option<bool>,
    /// Page synonyms. [`crate::graph::Graph::page`] checks these before returning the [`crate::graph::GraphEntry`] object.
    pub alias: Vec<String>,
    /// From the official docs:
    /// > store selected filters for linked references on page-level. object with booleans.
    ///
    /// We just return the property value -- you'll have to parse it yourself.
    pub filters: Option<String>,
    /// Whether this page should be included in an export.
    pub public: bool,
    /// Whether this page is excluded from the global graph view.
    pub exclude_from_graph_view: bool,
    /// Any other properties.
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
                "tags" => properties.tags = Some(value),
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

        if let Some(tags) = &self.tags {
            writeln!(f, "tags:: {tags}")?;
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
