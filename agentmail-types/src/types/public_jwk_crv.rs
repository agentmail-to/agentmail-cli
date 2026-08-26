pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PublicJwkCrv {
    #[serde(rename = "P-256")]
    P256,
}
impl fmt::Display for PublicJwkCrv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::P256 => "P-256",
        };
        write!(f, "{}", s)
    }
}
