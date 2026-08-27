pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BrowserLifecycleActorZeroType {
    #[serde(rename = "api_key")]
    ApiKey,
}
impl fmt::Display for BrowserLifecycleActorZeroType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ApiKey => "api_key",
        };
        write!(f, "{}", s)
    }
}
