pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// `expires_at` must be in the future and is immutable once the key exists.
/// Omitted, the new key inherits the authenticating key's expiry, and never
/// expires only when that key never does. A key cannot create one that
/// outlives it: an `expires_at` later than the authenticating key's own
/// returns `403`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBearerApiKeyRequest {
    #[serde(flatten)]
    pub api_key_mutable_fields_fields: ApiKeyMutableFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<ExpiresAt>,
}

impl CreateBearerApiKeyRequest {
    pub fn builder() -> CreateBearerApiKeyRequestBuilder {
        <CreateBearerApiKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBearerApiKeyRequestBuilder {
    api_key_mutable_fields_fields: Option<ApiKeyMutableFields>,
    expires_at: Option<ExpiresAt>,
}

impl CreateBearerApiKeyRequestBuilder {
    pub fn api_key_mutable_fields_fields(mut self, value: ApiKeyMutableFields) -> Self {
        self.api_key_mutable_fields_fields = Some(value);
        self
    }

    pub fn expires_at(mut self, value: ExpiresAt) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBearerApiKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_mutable_fields_fields`](CreateBearerApiKeyRequestBuilder::api_key_mutable_fields_fields)
    pub fn build(self) -> Result<CreateBearerApiKeyRequest, BuildError> {
        Ok(CreateBearerApiKeyRequest {
            api_key_mutable_fields_fields: self.api_key_mutable_fields_fields.ok_or_else(|| BuildError::missing_field("api_key_mutable_fields_fields"))?,
            expires_at: self.expires_at,
        })
    }
}
