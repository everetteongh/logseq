mod kind;
mod repeater;
pub use kind::*;
pub use repeater::*;
use time::{Date, Time, Weekday};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Due {
    pub date: Date,
    pub day: Weekday,
    pub time: Option<Time>,
    pub repeater: Option<DueRepeater>,
    pub kind: DueKind,
}
