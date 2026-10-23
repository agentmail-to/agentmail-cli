pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CalendarEventCreatedEventEventType {
    #[serde(rename = "calendar.event.created")]
    CalendarEventCreated,
}
impl fmt::Display for CalendarEventCreatedEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CalendarEventCreated => "calendar.event.created",
        };
        write!(f, "{}", s)
    }
}
