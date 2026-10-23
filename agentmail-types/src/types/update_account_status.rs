pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Status to set. `disabled` stops the inbox from signing in at the
/// app; `enabled` re-enables it. An enabled account reads back with no
/// `status`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAccountStatus {
    Enabled,
    Disabled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAccountStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Enabled => serializer.serialize_str("enabled"),
            Self::Disabled => serializer.serialize_str("disabled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "enabled" => Ok(Self::Enabled),
            "disabled" => Ok(Self::Disabled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAccountStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Enabled => write!(f, "enabled"),
            Self::Disabled => write!(f, "disabled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
