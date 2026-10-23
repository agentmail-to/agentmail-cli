pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentArmorReleaseApiKeyActor {
    /// The API key that released it.
    #[serde(default)]
    pub api_key_id: String,
}

impl AgentArmorReleaseApiKeyActor {
    pub fn builder() -> AgentArmorReleaseApiKeyActorBuilder {
        <AgentArmorReleaseApiKeyActorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorReleaseApiKeyActorBuilder {
    api_key_id: Option<String>,
}

impl AgentArmorReleaseApiKeyActorBuilder {
    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorReleaseApiKeyActor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](AgentArmorReleaseApiKeyActorBuilder::api_key_id)
    pub fn build(self) -> Result<AgentArmorReleaseApiKeyActor, BuildError> {
        Ok(AgentArmorReleaseApiKeyActor {
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
        })
    }
}
