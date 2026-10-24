pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentArmorResultTwoStatus {
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl fmt::Display for AgentArmorResultTwoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unavailable => "unavailable",
        };
        write!(f, "{}", s)
    }
}
