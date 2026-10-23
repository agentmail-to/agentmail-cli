pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentArmorCompletedEventResultStatus {
    #[serde(rename = "complete")]
    Complete,
}
impl fmt::Display for AgentArmorCompletedEventResultStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Complete => "complete",
        };
        write!(f, "{}", s)
    }
}
