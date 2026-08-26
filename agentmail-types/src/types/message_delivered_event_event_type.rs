pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageDeliveredEventEventType {
    #[serde(rename = "message.delivered")]
    MessageDelivered,
}
impl fmt::Display for MessageDeliveredEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MessageDelivered => "message.delivered",
        };
        write!(f, "{}", s)
    }
}
