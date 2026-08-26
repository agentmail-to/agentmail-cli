pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PublicKeyScopeOneType {
    #[serde(rename = "pod")]
    Pod,
}
impl fmt::Display for PublicKeyScopeOneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pod => "pod",
        };
        write!(f, "{}", s)
    }
}
