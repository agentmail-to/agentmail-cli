pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentArmorResultOneStatus {
    #[serde(rename = "complete")]
    Complete,
}
impl fmt::Display for AgentArmorResultOneStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Complete => "complete",
        };
        write!(f, "{}", s)
    }
}
