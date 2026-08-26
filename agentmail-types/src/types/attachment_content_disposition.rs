pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Content disposition of attachment.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttachmentContentDisposition {
    Inline,
    Attachment,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AttachmentContentDisposition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Inline => serializer.serialize_str("inline"),
            Self::Attachment => serializer.serialize_str("attachment"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AttachmentContentDisposition {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "inline" => Ok(Self::Inline),
            "attachment" => Ok(Self::Attachment),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AttachmentContentDisposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inline => write!(f, "inline"),
            Self::Attachment => write!(f, "attachment"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
