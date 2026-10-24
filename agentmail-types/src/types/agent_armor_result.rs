pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AgentArmorResult {
        AgentArmorResultZero(AgentArmorResultZero),

        AgentArmorResultOne(AgentArmorResultOne),

        AgentArmorResultTwo(AgentArmorResultTwo),
}

impl AgentArmorResult {
    pub fn is_agent_armor_result_zero(&self) -> bool {
        matches!(self, Self::AgentArmorResultZero(_))
    }

    pub fn is_agent_armor_result_one(&self) -> bool {
        matches!(self, Self::AgentArmorResultOne(_))
    }

    pub fn is_agent_armor_result_two(&self) -> bool {
        matches!(self, Self::AgentArmorResultTwo(_))
    }


    pub fn as_agent_armor_result_zero(&self) -> Option<&AgentArmorResultZero> {
        match self {
                    Self::AgentArmorResultZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_agent_armor_result_zero(self) -> Option<AgentArmorResultZero> {
        match self {
                    Self::AgentArmorResultZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_agent_armor_result_one(&self) -> Option<&AgentArmorResultOne> {
        match self {
                    Self::AgentArmorResultOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_agent_armor_result_one(self) -> Option<AgentArmorResultOne> {
        match self {
                    Self::AgentArmorResultOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_agent_armor_result_two(&self) -> Option<&AgentArmorResultTwo> {
        match self {
                    Self::AgentArmorResultTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_agent_armor_result_two(self) -> Option<AgentArmorResultTwo> {
        match self {
                    Self::AgentArmorResultTwo(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for AgentArmorResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgentArmorResultZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::AgentArmorResultOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::AgentArmorResultTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
