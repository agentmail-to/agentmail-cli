pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Type of inbox event. Wire format is dot.case to match the
/// convention used by webhook events (`message.received`,
/// `domain.verified`, etc. in events.yml). Pre-2026-04 these were
/// `label_added`/`label_removed` (snake_case). The Fern enum's `name`
/// field stays uppercase-snake (Fern convention); only the wire
/// `value` changed.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InboxEventType {
    LabelAdded,
    LabelRemoved,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InboxEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::LabelAdded => serializer.serialize_str("label.added"),
            Self::LabelRemoved => serializer.serialize_str("label.removed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InboxEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "label.added" => Ok(Self::LabelAdded),
            "label.removed" => Ok(Self::LabelRemoved),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InboxEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LabelAdded => write!(f, "label.added"),
            Self::LabelRemoved => write!(f, "label.removed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
