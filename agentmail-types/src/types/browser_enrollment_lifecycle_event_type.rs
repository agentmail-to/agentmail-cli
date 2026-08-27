pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BrowserEnrollmentLifecycleEventType {
    BrowserEnrollmentIntentCreated,
    BrowserCredentialActivated,
    BrowserEnrollmentCancelled,
    BrowserCredentialDeleted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BrowserEnrollmentLifecycleEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BrowserEnrollmentIntentCreated => serializer.serialize_str("browser_enrollment_intent_created"),
            Self::BrowserCredentialActivated => serializer.serialize_str("browser_credential_activated"),
            Self::BrowserEnrollmentCancelled => serializer.serialize_str("browser_enrollment_cancelled"),
            Self::BrowserCredentialDeleted => serializer.serialize_str("browser_credential_deleted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BrowserEnrollmentLifecycleEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "browser_enrollment_intent_created" => Ok(Self::BrowserEnrollmentIntentCreated),
            "browser_credential_activated" => Ok(Self::BrowserCredentialActivated),
            "browser_enrollment_cancelled" => Ok(Self::BrowserEnrollmentCancelled),
            "browser_credential_deleted" => Ok(Self::BrowserCredentialDeleted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BrowserEnrollmentLifecycleEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BrowserEnrollmentIntentCreated => write!(f, "browser_enrollment_intent_created"),
            Self::BrowserCredentialActivated => write!(f, "browser_credential_activated"),
            Self::BrowserEnrollmentCancelled => write!(f, "browser_enrollment_cancelled"),
            Self::BrowserCredentialDeleted => write!(f, "browser_credential_deleted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
