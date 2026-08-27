pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BrowserCredentialCreatorKind {
    #[serde(rename = "bearer_api_key")]
    BearerApiKey,
}
impl fmt::Display for BrowserCredentialCreatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::BearerApiKey => "bearer_api_key",
        };
        write!(f, "{}", s)
    }
}
