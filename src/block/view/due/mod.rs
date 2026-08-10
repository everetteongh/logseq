/// A due block.
mod block;
/// The kind of due.
mod kind;
/// A due repeater.
mod repeater;

use crate::{
    consts::{DATE_FORMAT, DUE_REGEX, TIME_FORMAT},
    error::{Logseq, ParseDueError},
};
pub use block::*;
pub use kind::*;
pub use repeater::*;
use std::{fmt, str::FromStr};
use time::{Date, Time, Weekday, error::InvalidVariant};

/// Parser for SCHEDULED/DEADLINE. This follows standard [Org Mode format](https://orgmode.org/manual/Deadlines-and-Scheduling.html), so might be delegated to an Org dependency in the future.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Due {
    /// The due date.
    pub date: Date,
    /// The due day.
    pub day: Weekday,
    /// The due time.
    pub time: Option<Time>,
    /// The due repeater.
    pub repeater: Option<DueRepeater>,
    /// The due kind; one of [`DueKind::Scheduled`], [`DueKind::Deadline`].
    pub kind: DueKind,
}

impl fmt::Display for Due {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: `DATE_FORMAT` is guaranteed valid @ compile time, so **this will never panic**.
        #[allow(clippy::unwrap_used)]
        write!(
            f,
            "{} <{} {}",
            self.kind,
            self.date.format(DATE_FORMAT).unwrap(),
            shorten_weekday(&self.day.to_string())
        )?;

        if let Some(time) = &self.time {
            // NOTE: `TIME_FORMAT` is guaranteed valid @ compile time, so **this will never panic**.
            #[allow(clippy::unwrap_used)]
            write!(f, " {}", time.format(TIME_FORMAT).unwrap())?;
        }

        if let Some(repeater) = &self.repeater {
            write!(f, " {repeater}")?;
        }

        write!(f, ">")
    }
}

impl FromStr for Due {
    type Err = Logseq;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = DUE_REGEX.captures(s) {
            let kind = captures
                .get(1)
                .ok_or(ParseDueError::InvalidInput)
                .and_then(|m| DueKind::from_str(m.as_str()))?;
            let date = captures
                .get(2)
                .ok_or(Logseq::ParseScheduled(ParseDueError::InvalidInput))
                .and_then(|m| Date::parse(m.as_str(), DATE_FORMAT).map_err(Logseq::from))?;
            let day = captures
                .get(3)
                .ok_or(Logseq::ParseScheduled(ParseDueError::InvalidInput))
                .and_then(|m| custom_parse_weekday(m.as_str()).map_err(Logseq::from))?;
            let time = captures
                .get(4)
                .map(|m| m.as_str())
                .and_then(|t| Time::parse(t, TIME_FORMAT).ok());
            let repeater = captures
                .get(5)
                .map(|m| m.as_str())
                .and_then(|r| DueRepeater::from_str(r).ok());

            return Ok(Self {
                date,
                day,
                time,
                repeater,
                kind,
            });
        }

        Err(ParseDueError::InvalidInput.into())
    }
}

/// Parse strings as weekdays with support for shortened weekdays.
fn custom_parse_weekday(s: &str) -> Result<Weekday, InvalidVariant> {
    match s.trim().to_lowercase().as_str() {
        "mon" | "monday" => Ok(Weekday::Monday),
        "tue" | "tuesday" => Ok(Weekday::Tuesday),
        "wed" | "wednesday" => Ok(Weekday::Wednesday),
        "thu" | "thursday" => Ok(Weekday::Thursday),
        "fri" | "friday" => Ok(Weekday::Friday),
        "sat" | "saturday" => Ok(Weekday::Saturday),
        "sun" | "sunday" => Ok(Weekday::Sunday),
        _ => Err(InvalidVariant),
    }
}

/// Convert a weekday string into its shortened counterpart.
fn shorten_weekday(s: &str) -> &str {
    match s.trim() {
        "Monday" => "Mon",
        "Tuesday" => "Tue",
        "Wednesday" => "Wed",
        "Thursday" => "Thu",
        "Friday" => "Fri",
        "Saturday" => "Sat",
        "Sunday" => "Sun",
        other => other,
    }
}
