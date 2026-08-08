#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// A Logseq `SCHEDULED` repeater rule type. See [the official Logseq documentation](https://docs.logseq.com/#/page/tasks?anchor=ls-block-6a0878b3-8530-43f4-8ef6-268a31b39879)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RepeatFrom {
    // ".+1d"
    Completion,
    // "+1d"
    PrevScheduled,
    // "++1d"
    PrevScheduledConstrained,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DueRepeater {
    pub rule: RepeatFrom,
    pub duration: Duration,
}
