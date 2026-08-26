pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubscribedType {
    #[serde(rename = "subscribed")]
    Subscribed,
}
impl fmt::Display for SubscribedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Subscribed => "subscribed",
        };
        write!(f, "{}", s)
    }
}
