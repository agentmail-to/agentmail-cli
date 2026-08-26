pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecordStatus {
    Missing,
    Invalid,
    Valid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RecordStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Missing => serializer.serialize_str("MISSING"),
            Self::Invalid => serializer.serialize_str("INVALID"),
            Self::Valid => serializer.serialize_str("VALID"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RecordStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "MISSING" => Ok(Self::Missing),
            "INVALID" => Ok(Self::Invalid),
            "VALID" => Ok(Self::Valid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RecordStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => write!(f, "MISSING"),
            Self::Invalid => write!(f, "INVALID"),
            Self::Valid => write!(f, "VALID"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
