pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorReleaseActorOne {
    #[serde(flatten)]
    pub agent_armor_release_console_actor_fields: AgentArmorReleaseConsoleActor,
    pub kind: AgentArmorReleaseActorOneKind,
}

impl AgentArmorReleaseActorOne {
    pub fn builder() -> AgentArmorReleaseActorOneBuilder {
        <AgentArmorReleaseActorOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorReleaseActorOneBuilder {
    agent_armor_release_console_actor_fields: Option<AgentArmorReleaseConsoleActor>,
    kind: Option<AgentArmorReleaseActorOneKind>,
}

impl AgentArmorReleaseActorOneBuilder {
    pub fn agent_armor_release_console_actor_fields(mut self, value: AgentArmorReleaseConsoleActor) -> Self {
        self.agent_armor_release_console_actor_fields = Some(value);
        self
    }

    pub fn kind(mut self, value: AgentArmorReleaseActorOneKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorReleaseActorOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_armor_release_console_actor_fields`](AgentArmorReleaseActorOneBuilder::agent_armor_release_console_actor_fields)
    /// - [`kind`](AgentArmorReleaseActorOneBuilder::kind)
    pub fn build(self) -> Result<AgentArmorReleaseActorOne, BuildError> {
        Ok(AgentArmorReleaseActorOne {
            agent_armor_release_console_actor_fields: self.agent_armor_release_console_actor_fields.ok_or_else(|| BuildError::missing_field("agent_armor_release_console_actor_fields"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
