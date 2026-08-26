pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageOpenedEventEventType {
    #[serde(rename = "message.opened")]
    MessageOpened,
}
impl fmt::Display for MessageOpenedEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MessageOpened => "message.opened",
        };
        write!(f, "{}", s)
    }
}
