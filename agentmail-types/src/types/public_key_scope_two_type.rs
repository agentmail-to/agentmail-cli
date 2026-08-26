pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PublicKeyScopeTwoType {
    #[serde(rename = "inbox")]
    Inbox,
}
impl fmt::Display for PublicKeyScopeTwoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Inbox => "inbox",
        };
        write!(f, "{}", s)
    }
}
