pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentArmorReleaseActorOneKind {
    #[serde(rename = "console")]
    Console,
}
impl fmt::Display for AgentArmorReleaseActorOneKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Console => "console",
        };
        write!(f, "{}", s)
    }
}
