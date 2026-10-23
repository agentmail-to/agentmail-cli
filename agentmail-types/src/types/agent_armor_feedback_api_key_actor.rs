pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentArmorFeedbackApiKeyActor {
    /// The API key that sent it.
    #[serde(default)]
    pub api_key_id: String,
}

impl AgentArmorFeedbackApiKeyActor {
    pub fn builder() -> AgentArmorFeedbackApiKeyActorBuilder {
        <AgentArmorFeedbackApiKeyActorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorFeedbackApiKeyActorBuilder {
    api_key_id: Option<String>,
}

impl AgentArmorFeedbackApiKeyActorBuilder {
    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorFeedbackApiKeyActor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](AgentArmorFeedbackApiKeyActorBuilder::api_key_id)
    pub fn build(self) -> Result<AgentArmorFeedbackApiKeyActor, BuildError> {
        Ok(AgentArmorFeedbackApiKeyActor {
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
        })
    }
}
