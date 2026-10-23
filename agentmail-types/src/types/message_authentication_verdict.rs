pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result of one email authentication mechanism, in the vocabulary of the
/// `Authentication-Results` header (RFC 8601). `none` means there was nothing to evaluate
/// (Amazon SES calls this GRAY): no SPF record for the envelope MAIL FROM domain, no DKIM
/// signature to check, or no DMARC record for the From domain.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageAuthenticationVerdict {
    Pass,
    Fail,
    None,
    Neutral,
    Softfail,
    Temperror,
    Permerror,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MessageAuthenticationVerdict {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pass => serializer.serialize_str("pass"),
            Self::Fail => serializer.serialize_str("fail"),
            Self::None => serializer.serialize_str("none"),
            Self::Neutral => serializer.serialize_str("neutral"),
            Self::Softfail => serializer.serialize_str("softfail"),
            Self::Temperror => serializer.serialize_str("temperror"),
            Self::Permerror => serializer.serialize_str("permerror"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MessageAuthenticationVerdict {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pass" => Ok(Self::Pass),
            "fail" => Ok(Self::Fail),
            "none" => Ok(Self::None),
            "neutral" => Ok(Self::Neutral),
            "softfail" => Ok(Self::Softfail),
            "temperror" => Ok(Self::Temperror),
            "permerror" => Ok(Self::Permerror),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MessageAuthenticationVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass => write!(f, "pass"),
            Self::Fail => write!(f, "fail"),
            Self::None => write!(f, "none"),
            Self::Neutral => write!(f, "neutral"),
            Self::Softfail => write!(f, "softfail"),
            Self::Temperror => write!(f, "temperror"),
            Self::Permerror => write!(f, "permerror"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
