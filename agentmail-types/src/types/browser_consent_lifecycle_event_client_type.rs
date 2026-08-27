pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BrowserConsentLifecycleEventClientType {
    #[serde(rename = "closed")]
    Closed,
}
impl fmt::Display for BrowserConsentLifecycleEventClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Closed => "closed",
        };
        write!(f, "{}", s)
    }
}
