pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrowserLifecycleCredentialActor {
    #[serde(default)]
    pub credential_id: String,
    #[serde(default)]
    pub authorizing_api_key_id: String,
}

impl BrowserLifecycleCredentialActor {
    pub fn builder() -> BrowserLifecycleCredentialActorBuilder {
        <BrowserLifecycleCredentialActorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserLifecycleCredentialActorBuilder {
    credential_id: Option<String>,
    authorizing_api_key_id: Option<String>,
}

impl BrowserLifecycleCredentialActorBuilder {
    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    pub fn authorizing_api_key_id(mut self, value: impl Into<String>) -> Self {
        self.authorizing_api_key_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrowserLifecycleCredentialActor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credential_id`](BrowserLifecycleCredentialActorBuilder::credential_id)
    /// - [`authorizing_api_key_id`](BrowserLifecycleCredentialActorBuilder::authorizing_api_key_id)
    pub fn build(self) -> Result<BrowserLifecycleCredentialActor, BuildError> {
        Ok(BrowserLifecycleCredentialActor {
            credential_id: self.credential_id.ok_or_else(|| BuildError::missing_field("credential_id"))?,
            authorizing_api_key_id: self.authorizing_api_key_id.ok_or_else(|| BuildError::missing_field("authorizing_api_key_id"))?,
        })
    }
}
