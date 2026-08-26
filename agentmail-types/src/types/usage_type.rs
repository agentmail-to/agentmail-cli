pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Type of usage metric. Inbox-scoped queries carry `storage_bytes`,
/// `message_count`, and `thread_count`; pod-scoped queries add `inbox_count`
/// and `domain_count`; organization-scoped queries add `pod_count`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsageType {
    StorageBytes,
    MessageCount,
    ThreadCount,
    InboxCount,
    PodCount,
    DomainCount,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UsageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StorageBytes => serializer.serialize_str("storage_bytes"),
            Self::MessageCount => serializer.serialize_str("message_count"),
            Self::ThreadCount => serializer.serialize_str("thread_count"),
            Self::InboxCount => serializer.serialize_str("inbox_count"),
            Self::PodCount => serializer.serialize_str("pod_count"),
            Self::DomainCount => serializer.serialize_str("domain_count"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "storage_bytes" => Ok(Self::StorageBytes),
            "message_count" => Ok(Self::MessageCount),
            "thread_count" => Ok(Self::ThreadCount),
            "inbox_count" => Ok(Self::InboxCount),
            "pod_count" => Ok(Self::PodCount),
            "domain_count" => Ok(Self::DomainCount),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UsageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StorageBytes => write!(f, "storage_bytes"),
            Self::MessageCount => write!(f, "message_count"),
            Self::ThreadCount => write!(f, "thread_count"),
            Self::InboxCount => write!(f, "inbox_count"),
            Self::PodCount => write!(f, "pod_count"),
            Self::DomainCount => write!(f, "domain_count"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
