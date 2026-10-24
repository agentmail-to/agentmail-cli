pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorResultOne {
    #[serde(flatten)]
    pub agent_armor_complete_result_fields: AgentArmorCompleteResult,
    pub status: AgentArmorResultOneStatus,
}

impl AgentArmorResultOne {
    pub fn builder() -> AgentArmorResultOneBuilder {
        <AgentArmorResultOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorResultOneBuilder {
    agent_armor_complete_result_fields: Option<AgentArmorCompleteResult>,
    status: Option<AgentArmorResultOneStatus>,
}

impl AgentArmorResultOneBuilder {
    pub fn agent_armor_complete_result_fields(mut self, value: AgentArmorCompleteResult) -> Self {
        self.agent_armor_complete_result_fields = Some(value);
        self
    }

    pub fn status(mut self, value: AgentArmorResultOneStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorResultOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_armor_complete_result_fields`](AgentArmorResultOneBuilder::agent_armor_complete_result_fields)
    /// - [`status`](AgentArmorResultOneBuilder::status)
    pub fn build(self) -> Result<AgentArmorResultOne, BuildError> {
        Ok(AgentArmorResultOne {
            agent_armor_complete_result_fields: self.agent_armor_complete_result_fields.ok_or_else(|| BuildError::missing_field("agent_armor_complete_result_fields"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
