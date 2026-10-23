pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CalendarEventUpdatedEventEventType {
    #[serde(rename = "calendar.event.updated")]
    CalendarEventUpdated,
}
impl fmt::Display for CalendarEventUpdatedEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CalendarEventUpdated => "calendar.event.updated",
        };
        write!(f, "{}", s)
    }
}
