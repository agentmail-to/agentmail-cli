pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BrowserLifecycleActorOne {
    #[serde(flatten)]
    pub browser_lifecycle_credential_actor_fields: BrowserLifecycleCredentialActor,
    pub r#type: BrowserLifecycleActorOneType,
}

impl BrowserLifecycleActorOne {
    pub fn builder() -> BrowserLifecycleActorOneBuilder {
        <BrowserLifecycleActorOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserLifecycleActorOneBuilder {
    browser_lifecycle_credential_actor_fields: Option<BrowserLifecycleCredentialActor>,
    r#type: Option<BrowserLifecycleActorOneType>,
}

impl BrowserLifecycleActorOneBuilder {
    pub fn browser_lifecycle_credential_actor_fields(mut self, value: BrowserLifecycleCredentialActor) -> Self {
        self.browser_lifecycle_credential_actor_fields = Some(value);
        self
    }

    pub fn r#type(mut self, value: BrowserLifecycleActorOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrowserLifecycleActorOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`browser_lifecycle_credential_actor_fields`](BrowserLifecycleActorOneBuilder::browser_lifecycle_credential_actor_fields)
    /// - [`r#type`](BrowserLifecycleActorOneBuilder::r#type)
    pub fn build(self) -> Result<BrowserLifecycleActorOne, BuildError> {
        Ok(BrowserLifecycleActorOne {
            browser_lifecycle_credential_actor_fields: self.browser_lifecycle_credential_actor_fields.ok_or_else(|| BuildError::missing_field("browser_lifecycle_credential_actor_fields"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
