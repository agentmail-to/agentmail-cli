pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CalendarEventStartingEventEventType {
    #[serde(rename = "calendar.event.starting")]
    CalendarEventStarting,
}
impl fmt::Display for CalendarEventStartingEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CalendarEventStarting => "calendar.event.starting",
        };
        write!(f, "{}", s)
    }
}
