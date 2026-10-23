pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorResultZero {
    #[serde(flatten)]
    pub agent_armor_pending_result_fields: AgentArmorPendingResult,
    pub status: AgentArmorResultZeroStatus,
}

impl AgentArmorResultZero {
    pub fn builder() -> AgentArmorResultZeroBuilder {
        <AgentArmorResultZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorResultZeroBuilder {
    agent_armor_pending_result_fields: Option<AgentArmorPendingResult>,
    status: Option<AgentArmorResultZeroStatus>,
}

impl AgentArmorResultZeroBuilder {
    pub fn agent_armor_pending_result_fields(mut self, value: AgentArmorPendingResult) -> Self {
        self.agent_armor_pending_result_fields = Some(value);
        self
    }

    pub fn status(mut self, value: AgentArmorResultZeroStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorResultZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_armor_pending_result_fields`](AgentArmorResultZeroBuilder::agent_armor_pending_result_fields)
    /// - [`status`](AgentArmorResultZeroBuilder::status)
    pub fn build(self) -> Result<AgentArmorResultZero, BuildError> {
        Ok(AgentArmorResultZero {
            agent_armor_pending_result_fields: self.agent_armor_pending_result_fields.ok_or_else(|| BuildError::missing_field("agent_armor_pending_result_fields"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
