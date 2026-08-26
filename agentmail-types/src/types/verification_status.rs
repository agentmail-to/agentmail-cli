pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerificationStatus {
    NotStarted,
    Pending,
    Invalid,
    Failed,
    Verifying,
    Verified,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VerificationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotStarted => serializer.serialize_str("NOT_STARTED"),
            Self::Pending => serializer.serialize_str("PENDING"),
            Self::Invalid => serializer.serialize_str("INVALID"),
            Self::Failed => serializer.serialize_str("FAILED"),
            Self::Verifying => serializer.serialize_str("VERIFYING"),
            Self::Verified => serializer.serialize_str("VERIFIED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VerificationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NOT_STARTED" => Ok(Self::NotStarted),
            "PENDING" => Ok(Self::Pending),
            "INVALID" => Ok(Self::Invalid),
            "FAILED" => Ok(Self::Failed),
            "VERIFYING" => Ok(Self::Verifying),
            "VERIFIED" => Ok(Self::Verified),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotStarted => write!(f, "NOT_STARTED"),
            Self::Pending => write!(f, "PENDING"),
            Self::Invalid => write!(f, "INVALID"),
            Self::Failed => write!(f, "FAILED"),
            Self::Verifying => write!(f, "VERIFYING"),
            Self::Verified => write!(f, "VERIFIED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
