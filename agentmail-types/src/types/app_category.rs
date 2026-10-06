pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Kind of app.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppCategory {
    Ai,
    Search,
    Scraping,
    Browser,
    Data,
    DeveloperTools,
    Communication,
    Productivity,
    Payments,
    Finance,
    Commerce,
    Marketing,
    Analytics,
    Security,
    Infrastructure,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AppCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ai => serializer.serialize_str("ai"),
            Self::Search => serializer.serialize_str("search"),
            Self::Scraping => serializer.serialize_str("scraping"),
            Self::Browser => serializer.serialize_str("browser"),
            Self::Data => serializer.serialize_str("data"),
            Self::DeveloperTools => serializer.serialize_str("developer-tools"),
            Self::Communication => serializer.serialize_str("communication"),
            Self::Productivity => serializer.serialize_str("productivity"),
            Self::Payments => serializer.serialize_str("payments"),
            Self::Finance => serializer.serialize_str("finance"),
            Self::Commerce => serializer.serialize_str("commerce"),
            Self::Marketing => serializer.serialize_str("marketing"),
            Self::Analytics => serializer.serialize_str("analytics"),
            Self::Security => serializer.serialize_str("security"),
            Self::Infrastructure => serializer.serialize_str("infrastructure"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AppCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ai" => Ok(Self::Ai),
            "search" => Ok(Self::Search),
            "scraping" => Ok(Self::Scraping),
            "browser" => Ok(Self::Browser),
            "data" => Ok(Self::Data),
            "developer-tools" => Ok(Self::DeveloperTools),
            "communication" => Ok(Self::Communication),
            "productivity" => Ok(Self::Productivity),
            "payments" => Ok(Self::Payments),
            "finance" => Ok(Self::Finance),
            "commerce" => Ok(Self::Commerce),
            "marketing" => Ok(Self::Marketing),
            "analytics" => Ok(Self::Analytics),
            "security" => Ok(Self::Security),
            "infrastructure" => Ok(Self::Infrastructure),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AppCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ai => write!(f, "ai"),
            Self::Search => write!(f, "search"),
            Self::Scraping => write!(f, "scraping"),
            Self::Browser => write!(f, "browser"),
            Self::Data => write!(f, "data"),
            Self::DeveloperTools => write!(f, "developer-tools"),
            Self::Communication => write!(f, "communication"),
            Self::Productivity => write!(f, "productivity"),
            Self::Payments => write!(f, "payments"),
            Self::Finance => write!(f, "finance"),
            Self::Commerce => write!(f, "commerce"),
            Self::Marketing => write!(f, "marketing"),
            Self::Analytics => write!(f, "analytics"),
            Self::Security => write!(f, "security"),
            Self::Infrastructure => write!(f, "infrastructure"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
