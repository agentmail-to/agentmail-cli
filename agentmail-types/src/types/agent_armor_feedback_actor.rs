pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AgentArmorFeedbackActor {
        AgentArmorFeedbackActorZero(AgentArmorFeedbackActorZero),

        AgentArmorFeedbackActorOne(AgentArmorFeedbackActorOne),
}

impl AgentArmorFeedbackActor {
    pub fn is_agent_armor_feedback_actor_zero(&self) -> bool {
        matches!(self, Self::AgentArmorFeedbackActorZero(_))
    }

    pub fn is_agent_armor_feedback_actor_one(&self) -> bool {
        matches!(self, Self::AgentArmorFeedbackActorOne(_))
    }


    pub fn as_agent_armor_feedback_actor_zero(&self) -> Option<&AgentArmorFeedbackActorZero> {
        match self {
                    Self::AgentArmorFeedbackActorZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_agent_armor_feedback_actor_zero(self) -> Option<AgentArmorFeedbackActorZero> {
        match self {
                    Self::AgentArmorFeedbackActorZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_agent_armor_feedback_actor_one(&self) -> Option<&AgentArmorFeedbackActorOne> {
        match self {
                    Self::AgentArmorFeedbackActorOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_agent_armor_feedback_actor_one(self) -> Option<AgentArmorFeedbackActorOne> {
        match self {
                    Self::AgentArmorFeedbackActorOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for AgentArmorFeedbackActor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgentArmorFeedbackActorZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::AgentArmorFeedbackActorOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
