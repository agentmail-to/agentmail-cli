pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageBouncedEventEventType {
    #[serde(rename = "message.bounced")]
    MessageBounced,
}
impl fmt::Display for MessageBouncedEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MessageBounced => "message.bounced",
        };
        write!(f, "{}", s)
    }
}
