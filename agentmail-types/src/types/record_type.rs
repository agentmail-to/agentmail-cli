pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecordType {
    Txt,
    Cname,
    Mx,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RecordType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Txt => serializer.serialize_str("TXT"),
            Self::Cname => serializer.serialize_str("CNAME"),
            Self::Mx => serializer.serialize_str("MX"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RecordType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TXT" => Ok(Self::Txt),
            "CNAME" => Ok(Self::Cname),
            "MX" => Ok(Self::Mx),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Txt => write!(f, "TXT"),
            Self::Cname => write!(f, "CNAME"),
            Self::Mx => write!(f, "MX"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
