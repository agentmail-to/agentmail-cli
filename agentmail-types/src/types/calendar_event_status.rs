pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Status of the event. A `cancelled` event or date stays readable but sends no
/// `calendar.event.starting` or `calendar.event.ending` webhooks (cancelling a date that is
/// already running still sends its `calendar.event.ending`). A date cancelled by deleting its
/// dated ID is removed from the series instead, so reading it can return `404` or `410`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalendarEventStatus {
    Confirmed,
    Tentative,
    Cancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CalendarEventStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Confirmed => serializer.serialize_str("confirmed"),
            Self::Tentative => serializer.serialize_str("tentative"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CalendarEventStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "confirmed" => Ok(Self::Confirmed),
            "tentative" => Ok(Self::Tentative),
            "cancelled" => Ok(Self::Cancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CalendarEventStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Confirmed => write!(f, "confirmed"),
            Self::Tentative => write!(f, "tentative"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
