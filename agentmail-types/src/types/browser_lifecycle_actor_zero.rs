pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BrowserLifecycleActorZero {
    #[serde(flatten)]
    pub browser_lifecycle_api_key_actor_fields: BrowserLifecycleApiKeyActor,
    pub r#type: BrowserLifecycleActorZeroType,
}

impl BrowserLifecycleActorZero {
    pub fn builder() -> BrowserLifecycleActorZeroBuilder {
        <BrowserLifecycleActorZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserLifecycleActorZeroBuilder {
    browser_lifecycle_api_key_actor_fields: Option<BrowserLifecycleApiKeyActor>,
    r#type: Option<BrowserLifecycleActorZeroType>,
}

impl BrowserLifecycleActorZeroBuilder {
    pub fn browser_lifecycle_api_key_actor_fields(mut self, value: BrowserLifecycleApiKeyActor) -> Self {
        self.browser_lifecycle_api_key_actor_fields = Some(value);
        self
    }

    pub fn r#type(mut self, value: BrowserLifecycleActorZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrowserLifecycleActorZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`browser_lifecycle_api_key_actor_fields`](BrowserLifecycleActorZeroBuilder::browser_lifecycle_api_key_actor_fields)
    /// - [`r#type`](BrowserLifecycleActorZeroBuilder::r#type)
    pub fn build(self) -> Result<BrowserLifecycleActorZero, BuildError> {
        Ok(BrowserLifecycleActorZero {
            browser_lifecycle_api_key_actor_fields: self.browser_lifecycle_api_key_actor_fields.ok_or_else(|| BuildError::missing_field("browser_lifecycle_api_key_actor_fields"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
