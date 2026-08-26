pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApiKeyResponse {
    #[serde(default)]
    pub api_key_id: ApiKeyId,
    /// API key.
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub prefix: Prefix,
    #[serde(default)]
    pub name: Name,
    /// Pod ID the api key is scoped to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
    /// Inbox ID the api key is scoped to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
    #[serde(default)]
    pub created_at: CreatedAt,
}

impl CreateApiKeyResponse {
    pub fn builder() -> CreateApiKeyResponseBuilder {
        <CreateApiKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeyResponseBuilder {
    api_key_id: Option<ApiKeyId>,
    api_key: Option<String>,
    prefix: Option<Prefix>,
    name: Option<Name>,
    pod_id: Option<String>,
    inbox_id: Option<String>,
    permissions: Option<ApiKeyPermissions>,
    created_at: Option<CreatedAt>,
}

impl CreateApiKeyResponseBuilder {
    pub fn api_key_id(mut self, value: ApiKeyId) -> Self {
        self.api_key_id = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn prefix(mut self, value: Prefix) -> Self {
        self.prefix = Some(value);
        self
    }

    pub fn name(mut self, value: Name) -> Self {
        self.name = Some(value);
        self
    }

    pub fn pod_id(mut self, value: impl Into<String>) -> Self {
        self.pod_id = Some(value.into());
        self
    }

    pub fn inbox_id(mut self, value: impl Into<String>) -> Self {
        self.inbox_id = Some(value.into());
        self
    }

    pub fn permissions(mut self, value: ApiKeyPermissions) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](CreateApiKeyResponseBuilder::api_key_id)
    /// - [`api_key`](CreateApiKeyResponseBuilder::api_key)
    /// - [`prefix`](CreateApiKeyResponseBuilder::prefix)
    /// - [`name`](CreateApiKeyResponseBuilder::name)
    /// - [`created_at`](CreateApiKeyResponseBuilder::created_at)
    pub fn build(self) -> Result<CreateApiKeyResponse, BuildError> {
        Ok(CreateApiKeyResponse {
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
            api_key: self.api_key.ok_or_else(|| BuildError::missing_field("api_key"))?,
            prefix: self.prefix.ok_or_else(|| BuildError::missing_field("prefix"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            pod_id: self.pod_id,
            inbox_id: self.inbox_id,
            permissions: self.permissions,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
