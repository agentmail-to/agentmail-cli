pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorFeedbackActorZero {
    #[serde(flatten)]
    pub agent_armor_feedback_api_key_actor_fields: AgentArmorFeedbackApiKeyActor,
    pub kind: AgentArmorFeedbackActorZeroKind,
}

impl AgentArmorFeedbackActorZero {
    pub fn builder() -> AgentArmorFeedbackActorZeroBuilder {
        <AgentArmorFeedbackActorZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorFeedbackActorZeroBuilder {
    agent_armor_feedback_api_key_actor_fields: Option<AgentArmorFeedbackApiKeyActor>,
    kind: Option<AgentArmorFeedbackActorZeroKind>,
}

impl AgentArmorFeedbackActorZeroBuilder {
    pub fn agent_armor_feedback_api_key_actor_fields(mut self, value: AgentArmorFeedbackApiKeyActor) -> Self {
        self.agent_armor_feedback_api_key_actor_fields = Some(value);
        self
    }

    pub fn kind(mut self, value: AgentArmorFeedbackActorZeroKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorFeedbackActorZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_armor_feedback_api_key_actor_fields`](AgentArmorFeedbackActorZeroBuilder::agent_armor_feedback_api_key_actor_fields)
    /// - [`kind`](AgentArmorFeedbackActorZeroBuilder::kind)
    pub fn build(self) -> Result<AgentArmorFeedbackActorZero, BuildError> {
        Ok(AgentArmorFeedbackActorZero {
            agent_armor_feedback_api_key_actor_fields: self.agent_armor_feedback_api_key_actor_fields.ok_or_else(|| BuildError::missing_field("agent_armor_feedback_api_key_actor_fields"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
