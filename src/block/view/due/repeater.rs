use crate::error::{Logseq, ParseRepeaterErr};
use humantime::{Duration as HumanDuration, format_duration};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr, time::Duration};

/// The repeater rule for something due. See [Logseq's docs on Deadline and Scheduled](https://github.com/logseq/docs/blob/master/pages/Tasks.md#deadline-and-scheduled).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RepeatFrom {
    /// Repeat from task completion. Equivalent to `.+` in Logseq.
    Completion,
    /// Repeat from the scheduled date. Equivalent to `+` in Logseq.
    PrevScheduled,
    /// Repeat from the scheduled date, with pushing-back constrained to the next date. Equivalent to `++` in Logseq.
    PrevScheduledConstrained,
}

impl fmt::Display for RepeatFrom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Completion => write!(f, ".+"),
            Self::PrevScheduled => write!(f, "+"),
            Self::PrevScheduledConstrained => write!(f, "++"),
        }
    }
}

/// A Logseq repeater.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DueRepeater {
    /// The rule for the repeater.
    pub rule: RepeatFrom,
    /// The duration to apply the rule to.
    pub duration: Duration,
}

impl fmt::Display for DueRepeater {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rule, format_duration(self.duration))
    }
}

impl FromStr for DueRepeater {
    type Err = Logseq;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let maybe_repeater = match chars.next().ok_or(ParseRepeaterErr::InvalidRepeater)? {
            '.' => {
                if chars.next() == Some('+') {
                    Ok((RepeatFrom::Completion, chars))
                } else {
                    Err(ParseRepeaterErr::InvalidRepeater)
                }
            }
            '+' => {
                if chars.as_str().starts_with('+') {
                    _ = chars.next();
                    Ok((RepeatFrom::PrevScheduledConstrained, chars))
                } else {
                    Ok((RepeatFrom::PrevScheduled, chars))
                }
            }
            _ => Err(ParseRepeaterErr::InvalidRepeater),
        };

        let (rule, duration_chars) = maybe_repeater?;
        // HACK: Logseq's `m` means "minute," but that means "month" for `humantime`
        let duration_str: String = duration_chars
            .map(|c| if c == 'm' { 'M' } else { c })
            .collect();
        let duration_human: HumanDuration = duration_str.parse()?;

        Ok(Self {
            duration: duration_human.into(),
            rule,
        })
    }
}
