pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AgentArmorReleaseActor {
        AgentArmorReleaseActorZero(AgentArmorReleaseActorZero),

        AgentArmorReleaseActorOne(AgentArmorReleaseActorOne),
}

impl AgentArmorReleaseActor {
    pub fn is_agent_armor_release_actor_zero(&self) -> bool {
        matches!(self, Self::AgentArmorReleaseActorZero(_))
    }

    pub fn is_agent_armor_release_actor_one(&self) -> bool {
        matches!(self, Self::AgentArmorReleaseActorOne(_))
    }


    pub fn as_agent_armor_release_actor_zero(&self) -> Option<&AgentArmorReleaseActorZero> {
        match self {
                    Self::AgentArmorReleaseActorZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_agent_armor_release_actor_zero(self) -> Option<AgentArmorReleaseActorZero> {
        match self {
                    Self::AgentArmorReleaseActorZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_agent_armor_release_actor_one(&self) -> Option<&AgentArmorReleaseActorOne> {
        match self {
                    Self::AgentArmorReleaseActorOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_agent_armor_release_actor_one(self) -> Option<AgentArmorReleaseActorOne> {
        match self {
                    Self::AgentArmorReleaseActorOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for AgentArmorReleaseActor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgentArmorReleaseActorZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::AgentArmorReleaseActorOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
