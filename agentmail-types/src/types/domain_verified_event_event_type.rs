pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomainVerifiedEventEventType {
    #[serde(rename = "domain.verified")]
    DomainVerified,
}
impl fmt::Display for DomainVerifiedEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DomainVerified => "domain.verified",
        };
        write!(f, "{}", s)
    }
}
