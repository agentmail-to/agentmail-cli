pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageSentEventEventType {
    #[serde(rename = "message.sent")]
    MessageSent,
}
impl fmt::Display for MessageSentEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MessageSent => "message.sent",
        };
        write!(f, "{}", s)
    }
}
