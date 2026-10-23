pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How the event's length is kept when a date crosses a daylight-saving change.
/// 
/// - `exact`: the event lasts the same elapsed time. Default for one-off timed events.
/// - `nominal`: the event keeps its wall-clock end time (a 09:00 to 10:00 meeting stays 09:00 to
/// 10:00 local time). Default for recurring timed events.
/// 
/// All-day events are always `nominal`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DurationMode {
    Nominal,
    Exact,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DurationMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Nominal => serializer.serialize_str("nominal"),
            Self::Exact => serializer.serialize_str("exact"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DurationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "nominal" => Ok(Self::Nominal),
            "exact" => Ok(Self::Exact),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DurationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nominal => write!(f, "nominal"),
            Self::Exact => write!(f, "exact"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
