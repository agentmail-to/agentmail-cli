pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentArmorFeedbackActorOneKind {
    #[serde(rename = "console")]
    Console,
}
impl fmt::Display for AgentArmorFeedbackActorOneKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Console => "console",
        };
        write!(f, "{}", s)
    }
}
