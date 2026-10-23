pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorFeedbackActorOne {
    #[serde(flatten)]
    pub agent_armor_feedback_console_actor_fields: AgentArmorFeedbackConsoleActor,
    pub kind: AgentArmorFeedbackActorOneKind,
}

impl AgentArmorFeedbackActorOne {
    pub fn builder() -> AgentArmorFeedbackActorOneBuilder {
        <AgentArmorFeedbackActorOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorFeedbackActorOneBuilder {
    agent_armor_feedback_console_actor_fields: Option<AgentArmorFeedbackConsoleActor>,
    kind: Option<AgentArmorFeedbackActorOneKind>,
}

impl AgentArmorFeedbackActorOneBuilder {
    pub fn agent_armor_feedback_console_actor_fields(mut self, value: AgentArmorFeedbackConsoleActor) -> Self {
        self.agent_armor_feedback_console_actor_fields = Some(value);
        self
    }

    pub fn kind(mut self, value: AgentArmorFeedbackActorOneKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorFeedbackActorOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_armor_feedback_console_actor_fields`](AgentArmorFeedbackActorOneBuilder::agent_armor_feedback_console_actor_fields)
    /// - [`kind`](AgentArmorFeedbackActorOneBuilder::kind)
    pub fn build(self) -> Result<AgentArmorFeedbackActorOne, BuildError> {
        Ok(AgentArmorFeedbackActorOne {
            agent_armor_feedback_console_actor_fields: self.agent_armor_feedback_console_actor_fields.ok_or_else(|| BuildError::missing_field("agent_armor_feedback_console_actor_fields"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
