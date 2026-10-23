pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CalendarEventEndingEventEventType {
    #[serde(rename = "calendar.event.ending")]
    CalendarEventEnding,
}
impl fmt::Display for CalendarEventEndingEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CalendarEventEnding => "calendar.event.ending",
        };
        write!(f, "{}", s)
    }
}
