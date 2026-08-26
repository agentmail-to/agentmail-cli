pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageRejectedEventType {
    #[serde(rename = "event")]
    Event,
}
impl fmt::Display for MessageRejectedEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Event => "event",
        };
        write!(f, "{}", s)
    }
}
