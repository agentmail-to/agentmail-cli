pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrowserLifecycleApiKeyActor {
    #[serde(default)]
    pub api_key_id: String,
}

impl BrowserLifecycleApiKeyActor {
    pub fn builder() -> BrowserLifecycleApiKeyActorBuilder {
        <BrowserLifecycleApiKeyActorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserLifecycleApiKeyActorBuilder {
    api_key_id: Option<String>,
}

impl BrowserLifecycleApiKeyActorBuilder {
    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrowserLifecycleApiKeyActor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](BrowserLifecycleApiKeyActorBuilder::api_key_id)
    pub fn build(self) -> Result<BrowserLifecycleApiKeyActor, BuildError> {
        Ok(BrowserLifecycleApiKeyActor {
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
        })
    }
}
