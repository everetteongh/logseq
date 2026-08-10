use crate::{block::BlockProperties, consts::PROPERTY_REGEX, document::DocumentProperties};
use rustc_hash::FxHashMap;

/// Extract the properties from an input [`String`], removing them and returning an [`FxHashMap<String, String>`]. Returns when it reaches a non-property line if `stop_at_content` is `true`.
fn take_properties(input: &mut String, stop_at_content: bool) -> FxHashMap<String, String> {
    let mut properties = FxHashMap::default();
    let mut lines: Vec<&str> = input.lines().collect();

    let lines = if stop_at_content {
        let mut lines = lines.into_iter().peekable();
        while let Some((_, [key, value])) = lines
            .peek()
            .and_then(|line| PROPERTY_REGEX.captures(line).map(|caps| caps.extract()))
        {
            properties.insert(key.to_string(), value.to_string());
            lines.next();
        }
        lines.collect()
    } else {
        lines.retain(|line| {
            if let Some((_, [key, value])) =
                PROPERTY_REGEX.captures(line).map(|caps| caps.extract())
            {
                properties.insert(key.to_string(), value.to_string());
                false
            } else {
                true
            }
        });
        lines
    };
    *input = lines.join("\n");
    properties
}

/// A common trait for property access. You probably want an implementor, like [`BlockProperties`] or [`DocumentProperties`].
pub trait Properties: TryFrom<FxHashMap<String, String>> {
    /// Extract the properties from an input, removing them from said input. This returns an empty object if there were no properties.
    ///
    /// # Errors
    /// This function only fails if an `id` property is present that can't be parsed to [`uuid::Uuid`].
    fn take_properties(input: &mut String) -> Result<Self, Self::Error>;
    /// Extract the properties from an input, removing them from said input. If there were no properties, this returns [`None`].
    fn maybe_take_properties(input: &mut String) -> Option<Result<Self, Self::Error>>;
}

impl Properties for BlockProperties {
    fn take_properties(input: &mut String) -> Result<Self, Self::Error> {
        let properties = take_properties(input, false);
        properties.try_into()
    }
    fn maybe_take_properties(input: &mut String) -> Option<Result<Self, Self::Error>> {
        let properties = take_properties(input, false);
        if properties.is_empty() {
            None
        } else {
            Some(properties.try_into())
        }
    }
}

impl Properties for DocumentProperties {
    fn take_properties(input: &mut String) -> Result<Self, Self::Error> {
        let properties = take_properties(input, true);
        properties.try_into()
    }
    fn maybe_take_properties(input: &mut String) -> Option<Result<Self, Self::Error>> {
        let properties = take_properties(input, true);
        if properties.is_empty() {
            None
        } else {
            Some(properties.try_into())
        }
    }
}
