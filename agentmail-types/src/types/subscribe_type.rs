pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubscribeType {
    #[serde(rename = "subscribe")]
    Subscribe,
}
impl fmt::Display for SubscribeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Subscribe => "subscribe",
        };
        write!(f, "{}", s)
    }
}
