pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What the scan concluded. `clear` means Agent Armor found no threat. It is advisory, not a guarantee that the
/// message is safe. `review` means the message may try to manipulate an agent: inspect it before an agent acts on it.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentArmorVerdict {
    Clear,
    Review,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentArmorVerdict {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Clear => serializer.serialize_str("clear"),
            Self::Review => serializer.serialize_str("review"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentArmorVerdict {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "clear" => Ok(Self::Clear),
            "review" => Ok(Self::Review),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentArmorVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clear => write!(f, "clear"),
            Self::Review => write!(f, "review"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
