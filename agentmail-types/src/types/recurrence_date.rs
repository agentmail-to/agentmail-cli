pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RecurrenceDate {
        WallTime(WallTime),

        RecurrencePeriod(RecurrencePeriod),
}

impl RecurrenceDate {
    pub fn is_wall_time(&self) -> bool {
        matches!(self, Self::WallTime(_))
    }

    pub fn is_recurrence_period(&self) -> bool {
        matches!(self, Self::RecurrencePeriod(_))
    }


    pub fn as_wall_time(&self) -> Option<&WallTime> {
        match self {
                    Self::WallTime(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_wall_time(self) -> Option<WallTime> {
        match self {
                    Self::WallTime(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_recurrence_period(&self) -> Option<&RecurrencePeriod> {
        match self {
                    Self::RecurrencePeriod(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_recurrence_period(self) -> Option<RecurrencePeriod> {
        match self {
                    Self::RecurrencePeriod(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for RecurrenceDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WallTime(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RecurrencePeriod(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
