pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The verdict that held the message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentArmorManualOverrideOriginalVerdict {
    #[serde(rename = "review")]
    Review,
}
impl fmt::Display for AgentArmorManualOverrideOriginalVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Review => "review",
        };
        write!(f, "{}", s)
    }
}
