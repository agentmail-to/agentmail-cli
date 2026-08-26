pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageRejectedEventEventType {
    #[serde(rename = "message.rejected")]
    MessageRejected,
}
impl fmt::Display for MessageRejectedEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MessageRejected => "message.rejected",
        };
        write!(f, "{}", s)
    }
}
