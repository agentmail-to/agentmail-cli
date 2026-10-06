pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CalendarEventRespondedEventEventType {
    #[serde(rename = "calendar.event.responded")]
    CalendarEventResponded,
}
impl fmt::Display for CalendarEventRespondedEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CalendarEventResponded => "calendar.event.responded",
        };
        write!(f, "{}", s)
    }
}
