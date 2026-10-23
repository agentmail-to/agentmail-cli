pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageSecurityCompletedEventEventType {
    #[serde(rename = "message.security.completed")]
    MessageSecurityCompleted,
}
impl fmt::Display for MessageSecurityCompletedEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MessageSecurityCompleted => "message.security.completed",
        };
        write!(f, "{}", s)
    }
}
