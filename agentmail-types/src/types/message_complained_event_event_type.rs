pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageComplainedEventEventType {
    #[serde(rename = "message.complained")]
    MessageComplained,
}
impl fmt::Display for MessageComplainedEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MessageComplained => "message.complained",
        };
        write!(f, "{}", s)
    }
}
