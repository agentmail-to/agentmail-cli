pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Direction of list entry.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Send,
    Receive,
    Reply,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Direction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Send => serializer.serialize_str("send"),
            Self::Receive => serializer.serialize_str("receive"),
            Self::Reply => serializer.serialize_str("reply"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "send" => Ok(Self::Send),
            "receive" => Ok(Self::Receive),
            "reply" => Ok(Self::Reply),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Send => write!(f, "send"),
            Self::Receive => write!(f, "receive"),
            Self::Reply => write!(f, "reply"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
