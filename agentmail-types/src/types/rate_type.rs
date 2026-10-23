pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Type of rate. `bounce` is bounced messages divided by messages sent,
/// counting permanent and transient bounces; `complaint` is spam complaints
/// divided by messages sent.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RateType {
    Bounce,
    Complaint,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RateType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bounce => serializer.serialize_str("bounce"),
            Self::Complaint => serializer.serialize_str("complaint"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bounce" => Ok(Self::Bounce),
            "complaint" => Ok(Self::Complaint),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bounce => write!(f, "bounce"),
            Self::Complaint => write!(f, "complaint"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
