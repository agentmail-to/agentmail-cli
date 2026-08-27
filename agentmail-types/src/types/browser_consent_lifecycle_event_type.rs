pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BrowserConsentLifecycleEventType {
    BrowserConsentCreated,
    BrowserConsentUpdated,
    BrowserConsentReused,
    BrowserConsentRevoked,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BrowserConsentLifecycleEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BrowserConsentCreated => serializer.serialize_str("browser_consent_created"),
            Self::BrowserConsentUpdated => serializer.serialize_str("browser_consent_updated"),
            Self::BrowserConsentReused => serializer.serialize_str("browser_consent_reused"),
            Self::BrowserConsentRevoked => serializer.serialize_str("browser_consent_revoked"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BrowserConsentLifecycleEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "browser_consent_created" => Ok(Self::BrowserConsentCreated),
            "browser_consent_updated" => Ok(Self::BrowserConsentUpdated),
            "browser_consent_reused" => Ok(Self::BrowserConsentReused),
            "browser_consent_revoked" => Ok(Self::BrowserConsentRevoked),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BrowserConsentLifecycleEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BrowserConsentCreated => write!(f, "browser_consent_created"),
            Self::BrowserConsentUpdated => write!(f, "browser_consent_updated"),
            Self::BrowserConsentReused => write!(f, "browser_consent_reused"),
            Self::BrowserConsentRevoked => write!(f, "browser_consent_revoked"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
