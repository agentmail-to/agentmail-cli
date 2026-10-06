pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What the event object represents.
/// 
/// - `single`: a one-off event, addressed by its UUID.
/// - `series`: the definition of a recurring event, addressed by its UUID. It carries
/// `recurrence`; `start` and `end` describe its first date.
/// - `instance`: one date of a recurring event, addressed by its dated ID. It carries `series_id`,
/// `original_start` and `is_exception`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalendarEventKind {
    Single,
    Series,
    Instance,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CalendarEventKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Single => serializer.serialize_str("single"),
            Self::Series => serializer.serialize_str("series"),
            Self::Instance => serializer.serialize_str("instance"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CalendarEventKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "single" => Ok(Self::Single),
            "series" => Ok(Self::Series),
            "instance" => Ok(Self::Instance),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CalendarEventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single => write!(f, "single"),
            Self::Series => write!(f, "series"),
            Self::Instance => write!(f, "instance"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
