pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the attendee's presence is required.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttendeeRole {
    Required,
    Optional,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AttendeeRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Required => serializer.serialize_str("required"),
            Self::Optional => serializer.serialize_str("optional"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AttendeeRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "required" => Ok(Self::Required),
            "optional" => Ok(Self::Optional),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AttendeeRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Required => write!(f, "required"),
            Self::Optional => write!(f, "optional"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
