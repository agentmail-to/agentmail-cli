pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Type of metric event.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetricEventType {
    MessageReceived,
    MessageReceivedSpam,
    MessageReceivedBlocked,
    MessageReceivedUnauthenticated,
    MessageSent,
    MessageDelivered,
    MessageBounced,
    MessageComplained,
    MessageRejected,
    DomainVerified,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MetricEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MessageReceived => serializer.serialize_str("message.received"),
            Self::MessageReceivedSpam => serializer.serialize_str("message.received.spam"),
            Self::MessageReceivedBlocked => serializer.serialize_str("message.received.blocked"),
            Self::MessageReceivedUnauthenticated => serializer.serialize_str("message.received.unauthenticated"),
            Self::MessageSent => serializer.serialize_str("message.sent"),
            Self::MessageDelivered => serializer.serialize_str("message.delivered"),
            Self::MessageBounced => serializer.serialize_str("message.bounced"),
            Self::MessageComplained => serializer.serialize_str("message.complained"),
            Self::MessageRejected => serializer.serialize_str("message.rejected"),
            Self::DomainVerified => serializer.serialize_str("domain.verified"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MetricEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "message.received" => Ok(Self::MessageReceived),
            "message.received.spam" => Ok(Self::MessageReceivedSpam),
            "message.received.blocked" => Ok(Self::MessageReceivedBlocked),
            "message.received.unauthenticated" => Ok(Self::MessageReceivedUnauthenticated),
            "message.sent" => Ok(Self::MessageSent),
            "message.delivered" => Ok(Self::MessageDelivered),
            "message.bounced" => Ok(Self::MessageBounced),
            "message.complained" => Ok(Self::MessageComplained),
            "message.rejected" => Ok(Self::MessageRejected),
            "domain.verified" => Ok(Self::DomainVerified),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MetricEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MessageReceived => write!(f, "message.received"),
            Self::MessageReceivedSpam => write!(f, "message.received.spam"),
            Self::MessageReceivedBlocked => write!(f, "message.received.blocked"),
            Self::MessageReceivedUnauthenticated => write!(f, "message.received.unauthenticated"),
            Self::MessageSent => write!(f, "message.sent"),
            Self::MessageDelivered => write!(f, "message.delivered"),
            Self::MessageBounced => write!(f, "message.bounced"),
            Self::MessageComplained => write!(f, "message.complained"),
            Self::MessageRejected => write!(f, "message.rejected"),
            Self::DomainVerified => write!(f, "domain.verified"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
