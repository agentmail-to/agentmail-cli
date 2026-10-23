pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageSecurityOverrideEventEventType {
    #[serde(rename = "message.security.override")]
    MessageSecurityOverride,
}
impl fmt::Display for MessageSecurityOverrideEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MessageSecurityOverride => "message.security.override",
        };
        write!(f, "{}", s)
    }
}
