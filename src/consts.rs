use comrak::options::{Extension, Options, Render};
#[cfg(not(feature = "regex-lite"))]
use regex::Regex;
#[cfg(feature = "regex-lite")]
use regex_lite::Regex;
use std::sync::LazyLock;
use time::{format_description::StaticFormatDescription, macros::format_description};

/// The default paths for [`crate::graph::Graph`]'s underlying [`walkdir::WalkDir`] directory crawler to exclude.
pub static LOGSEQ_EXCLUDE: [&str; 2] = ["logseq", "contents.md"];

/// Globally used sane default [`comrak`] [`Options`] for Logseq parsing.
pub(crate) static COMRAK_OPTIONS: LazyLock<Options<'static>> = LazyLock::new(|| Options {
    extension: Extension {
        strikethrough: true,
        tasklist: true,
        footnotes: true,
        autolink: true,
        underline: true,
        ..Default::default()
    },
    render: Render {
        experimental_minimize_commonmark: true,
        ..Default::default()
    },
    ..Default::default()
});

/// The default format for Logseq file stems.
pub(crate) static JOURNAL_FORMAT: StaticFormatDescription =
    format_description!("[year]_[month]_[day]");
/// The default format for dates in Logseq.
pub(crate) static DATE_FORMAT: StaticFormatDescription =
    format_description!("[year]-[month]-[day]");
/// A 24-hour time format.
pub(crate) static TIME_FORMAT: StaticFormatDescription = format_description!("[hour]:[minute]");

/// Regular expression to parse Logseq properties.
#[allow(clippy::unwrap_used)]
pub(crate) static PROPERTY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<key>[a-zA-Z0-9_-]+)::\s+(?P<value>.*)$").unwrap());
/// Regular expression to parse [`Due`] Logseq properties. This follows standard [Org Mode format](https://orgmode.org/manual/Deadlines-and-Scheduling.html), so might be delegated to an Org dependency in the future.
#[allow(clippy::unwrap_used)]
pub(crate) static DUE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(SCHEDULED|DEADLINE):\s*<(\d{4}-\d{2}-\d{2})\s+([A-Za-z]{3})(?:\s+(\d{1,2}:\d{2}))?(?:\s+([\.\+]*\+\d+[ymwdh]))?>$"
    ).unwrap()
});

/// The delimiter for "SCHEDULED" items.
pub(crate) const SCHEDULED_DELIM: &str = "SCHEDULED:";
/// The delimiter for "DEADLINE" items.
pub(crate) const DEADLINE_DELIM: &str = "DEADLINE:";
/// A list of delimiters for any [`Due`] property.
pub(crate) const DUE_DELIMS: [&str; 2] = [SCHEDULED_DELIM, DEADLINE_DELIM];

// TODO: Allow changing this
/// The delimiter for file namespaces -- uses Logseq's default (triple lowbar).
pub(crate) const NAMESPACE_DELIM: &str = "___";
