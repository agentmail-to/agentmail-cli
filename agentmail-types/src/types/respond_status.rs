pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Your response to the invitation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RespondStatus {
    Accepted,
    Declined,
    Tentative,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RespondStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Declined => serializer.serialize_str("declined"),
            Self::Tentative => serializer.serialize_str("tentative"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RespondStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "accepted" => Ok(Self::Accepted),
            "declined" => Ok(Self::Declined),
            "tentative" => Ok(Self::Tentative),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RespondStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "accepted"),
            Self::Declined => write!(f, "declined"),
            Self::Tentative => write!(f, "tentative"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
