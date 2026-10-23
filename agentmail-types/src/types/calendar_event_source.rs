pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Where the event came from. `api` events were created through this API and the inbox is their
/// organizer. `email` events were created from a calendar invitation (iMIP) received by the inbox;
/// their organizer is the sender of the invitation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalendarEventSource {
    Api,
    Email,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CalendarEventSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Api => serializer.serialize_str("api"),
            Self::Email => serializer.serialize_str("email"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CalendarEventSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "api" => Ok(Self::Api),
            "email" => Ok(Self::Email),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CalendarEventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api => write!(f, "api"),
            Self::Email => write!(f, "email"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
