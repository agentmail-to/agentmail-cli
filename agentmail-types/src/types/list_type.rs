pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Type of list entry.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListType {
    Allow,
    Block,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Allow => serializer.serialize_str("allow"),
            Self::Block => serializer.serialize_str("block"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "allow" => Ok(Self::Allow),
            "block" => Ok(Self::Block),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allow => write!(f, "allow"),
            Self::Block => write!(f, "block"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
