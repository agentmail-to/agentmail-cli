pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorReleaseActorZero {
    #[serde(flatten)]
    pub agent_armor_release_api_key_actor_fields: AgentArmorReleaseApiKeyActor,
    pub kind: AgentArmorReleaseActorZeroKind,
}

impl AgentArmorReleaseActorZero {
    pub fn builder() -> AgentArmorReleaseActorZeroBuilder {
        <AgentArmorReleaseActorZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorReleaseActorZeroBuilder {
    agent_armor_release_api_key_actor_fields: Option<AgentArmorReleaseApiKeyActor>,
    kind: Option<AgentArmorReleaseActorZeroKind>,
}

impl AgentArmorReleaseActorZeroBuilder {
    pub fn agent_armor_release_api_key_actor_fields(mut self, value: AgentArmorReleaseApiKeyActor) -> Self {
        self.agent_armor_release_api_key_actor_fields = Some(value);
        self
    }

    pub fn kind(mut self, value: AgentArmorReleaseActorZeroKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorReleaseActorZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_armor_release_api_key_actor_fields`](AgentArmorReleaseActorZeroBuilder::agent_armor_release_api_key_actor_fields)
    /// - [`kind`](AgentArmorReleaseActorZeroBuilder::kind)
    pub fn build(self) -> Result<AgentArmorReleaseActorZero, BuildError> {
        Ok(AgentArmorReleaseActorZero {
            agent_armor_release_api_key_actor_fields: self.agent_armor_release_api_key_actor_fields.ok_or_else(|| BuildError::missing_field("agent_armor_release_api_key_actor_fields"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
