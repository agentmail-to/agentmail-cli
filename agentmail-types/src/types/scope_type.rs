pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The scope tier the authenticated credential is bound to.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeType {
    Organization,
    Pod,
    Inbox,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScopeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Organization => serializer.serialize_str("organization"),
            Self::Pod => serializer.serialize_str("pod"),
            Self::Inbox => serializer.serialize_str("inbox"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "organization" => Ok(Self::Organization),
            "pod" => Ok(Self::Pod),
            "inbox" => Ok(Self::Inbox),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScopeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Organization => write!(f, "organization"),
            Self::Pod => write!(f, "pod"),
            Self::Inbox => write!(f, "inbox"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
