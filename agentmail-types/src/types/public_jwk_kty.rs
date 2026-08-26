pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PublicJwkKty {
    #[serde(rename = "EC")]
    Ec,
}
impl fmt::Display for PublicJwkKty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ec => "EC",
        };
        write!(f, "{}", s)
    }
}
