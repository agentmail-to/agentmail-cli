pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CalendarEventDeletedEventEventType {
    #[serde(rename = "calendar.event.deleted")]
    CalendarEventDeleted,
}
impl fmt::Display for CalendarEventDeletedEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CalendarEventDeleted => "calendar.event.deleted",
        };
        write!(f, "{}", s)
    }
}
