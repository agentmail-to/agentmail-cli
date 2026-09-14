pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The bearer API key that created the credential. Provenance only; the credential outlives it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeyCreator {
    #[serde(default)]
    pub api_key_id: ApiKeyId,
}

impl ApiKeyCreator {
    pub fn builder() -> ApiKeyCreatorBuilder {
        <ApiKeyCreatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyCreatorBuilder {
    api_key_id: Option<ApiKeyId>,
}

impl ApiKeyCreatorBuilder {
    pub fn api_key_id(mut self, value: ApiKeyId) -> Self {
        self.api_key_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyCreator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](ApiKeyCreatorBuilder::api_key_id)
    pub fn build(self) -> Result<ApiKeyCreator, BuildError> {
        Ok(ApiKeyCreator {
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
        })
    }
}
