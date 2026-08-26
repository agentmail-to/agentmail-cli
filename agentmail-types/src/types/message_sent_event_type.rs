pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageSentEventType {
    #[serde(rename = "event")]
    Event,
}
impl fmt::Display for MessageSentEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Event => "event",
        };
        write!(f, "{}", s)
    }
}
