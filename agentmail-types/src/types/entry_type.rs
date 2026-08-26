pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the entry is an email address or domain.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntryType {
    Email,
    Domain,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EntryType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Email => serializer.serialize_str("email"),
            Self::Domain => serializer.serialize_str("domain"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EntryType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email" => Ok(Self::Email),
            "domain" => Ok(Self::Domain),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Domain => write!(f, "domain"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
