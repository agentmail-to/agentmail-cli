pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Read consistency. `eventual` (default) reads from the region that serves your request and may
/// lag a write made moments earlier by up to a few seconds. `primary` reads from the primary region
/// and always reflects every completed write.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalendarConsistency {
    Eventual,
    Primary,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CalendarConsistency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eventual => serializer.serialize_str("eventual"),
            Self::Primary => serializer.serialize_str("primary"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CalendarConsistency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eventual" => Ok(Self::Eventual),
            "primary" => Ok(Self::Primary),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CalendarConsistency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eventual => write!(f, "eventual"),
            Self::Primary => write!(f, "primary"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
