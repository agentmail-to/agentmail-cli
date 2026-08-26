pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PublicKeyScopeZeroType {
    #[serde(rename = "organization")]
    Organization,
}
impl fmt::Display for PublicKeyScopeZeroType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Organization => "organization",
        };
        write!(f, "{}", s)
    }
}
