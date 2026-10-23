pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The result in a `message.security.completed` or `message.security.review` event, which is always `complete`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorCompletedEventResult {
    #[serde(flatten)]
    pub agent_armor_complete_result_fields: AgentArmorCompleteResult,
    pub status: AgentArmorCompletedEventResultStatus,
}

impl AgentArmorCompletedEventResult {
    pub fn builder() -> AgentArmorCompletedEventResultBuilder {
        <AgentArmorCompletedEventResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorCompletedEventResultBuilder {
    agent_armor_complete_result_fields: Option<AgentArmorCompleteResult>,
    status: Option<AgentArmorCompletedEventResultStatus>,
}

impl AgentArmorCompletedEventResultBuilder {
    pub fn agent_armor_complete_result_fields(mut self, value: AgentArmorCompleteResult) -> Self {
        self.agent_armor_complete_result_fields = Some(value);
        self
    }

    pub fn status(mut self, value: AgentArmorCompletedEventResultStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorCompletedEventResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_armor_complete_result_fields`](AgentArmorCompletedEventResultBuilder::agent_armor_complete_result_fields)
    /// - [`status`](AgentArmorCompletedEventResultBuilder::status)
    pub fn build(self) -> Result<AgentArmorCompletedEventResult, BuildError> {
        Ok(AgentArmorCompletedEventResult {
            agent_armor_complete_result_fields: self.agent_armor_complete_result_fields.ok_or_else(|| BuildError::missing_field("agent_armor_complete_result_fields"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
