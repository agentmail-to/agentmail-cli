pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Why no verdict is available.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentArmorUnavailableReason {
    UnsupportedDsn,
    PayloadTooLarge,
    NoScannableContent,
    EnqueueFailed,
    ScorerFailed,
    ContractError,
    DeadlineExceeded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentArmorUnavailableReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UnsupportedDsn => serializer.serialize_str("unsupported_dsn"),
            Self::PayloadTooLarge => serializer.serialize_str("payload_too_large"),
            Self::NoScannableContent => serializer.serialize_str("no_scannable_content"),
            Self::EnqueueFailed => serializer.serialize_str("enqueue_failed"),
            Self::ScorerFailed => serializer.serialize_str("scorer_failed"),
            Self::ContractError => serializer.serialize_str("contract_error"),
            Self::DeadlineExceeded => serializer.serialize_str("deadline_exceeded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentArmorUnavailableReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unsupported_dsn" => Ok(Self::UnsupportedDsn),
            "payload_too_large" => Ok(Self::PayloadTooLarge),
            "no_scannable_content" => Ok(Self::NoScannableContent),
            "enqueue_failed" => Ok(Self::EnqueueFailed),
            "scorer_failed" => Ok(Self::ScorerFailed),
            "contract_error" => Ok(Self::ContractError),
            "deadline_exceeded" => Ok(Self::DeadlineExceeded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentArmorUnavailableReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedDsn => write!(f, "unsupported_dsn"),
            Self::PayloadTooLarge => write!(f, "payload_too_large"),
            Self::NoScannableContent => write!(f, "no_scannable_content"),
            Self::EnqueueFailed => write!(f, "enqueue_failed"),
            Self::ScorerFailed => write!(f, "scorer_failed"),
            Self::ContractError => write!(f, "contract_error"),
            Self::DeadlineExceeded => write!(f, "deadline_exceeded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
