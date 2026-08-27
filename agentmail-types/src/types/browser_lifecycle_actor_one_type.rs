pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BrowserLifecycleActorOneType {
    #[serde(rename = "browser_credential")]
    BrowserCredential,
}
impl fmt::Display for BrowserLifecycleActorOneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::BrowserCredential => "browser_credential",
        };
        write!(f, "{}", s)
    }
}
