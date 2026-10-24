pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorResultTwo {
    #[serde(flatten)]
    pub agent_armor_unavailable_result_fields: AgentArmorUnavailableResult,
    pub status: AgentArmorResultTwoStatus,
}

impl AgentArmorResultTwo {
    pub fn builder() -> AgentArmorResultTwoBuilder {
        <AgentArmorResultTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorResultTwoBuilder {
    agent_armor_unavailable_result_fields: Option<AgentArmorUnavailableResult>,
    status: Option<AgentArmorResultTwoStatus>,
}

impl AgentArmorResultTwoBuilder {
    pub fn agent_armor_unavailable_result_fields(mut self, value: AgentArmorUnavailableResult) -> Self {
        self.agent_armor_unavailable_result_fields = Some(value);
        self
    }

    pub fn status(mut self, value: AgentArmorResultTwoStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorResultTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_armor_unavailable_result_fields`](AgentArmorResultTwoBuilder::agent_armor_unavailable_result_fields)
    /// - [`status`](AgentArmorResultTwoBuilder::status)
    pub fn build(self) -> Result<AgentArmorResultTwo, BuildError> {
        Ok(AgentArmorResultTwo {
            agent_armor_unavailable_result_fields: self.agent_armor_unavailable_result_fields.ok_or_else(|| BuildError::missing_field("agent_armor_unavailable_result_fields"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
