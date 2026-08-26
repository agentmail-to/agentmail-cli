pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKey {
    #[serde(default)]
    pub api_key_id: ApiKeyId,
    #[serde(default)]
    pub prefix: Prefix,
    #[serde(default)]
    pub name: Name,
    /// Pod ID the api key is scoped to. If set, the key can only access resources within this pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
    /// Inbox ID the api key is scoped to. If set, the key can only access resources within this inbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    /// Time at which api key was last used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
    #[serde(default)]
    pub created_at: CreatedAt,
}

impl ApiKey {
    pub fn builder() -> ApiKeyBuilder {
        <ApiKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyBuilder {
    api_key_id: Option<ApiKeyId>,
    prefix: Option<Prefix>,
    name: Option<Name>,
    pod_id: Option<String>,
    inbox_id: Option<String>,
    used_at: Option<DateTime<FixedOffset>>,
    permissions: Option<ApiKeyPermissions>,
    created_at: Option<CreatedAt>,
}

impl ApiKeyBuilder {
    pub fn api_key_id(mut self, value: ApiKeyId) -> Self {
        self.api_key_id = Some(value);
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

    pub fn used_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.used_at = Some(value);
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

    /// Consumes the builder and constructs a [`ApiKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](ApiKeyBuilder::api_key_id)
    /// - [`prefix`](ApiKeyBuilder::prefix)
    /// - [`name`](ApiKeyBuilder::name)
    /// - [`created_at`](ApiKeyBuilder::created_at)
    pub fn build(self) -> Result<ApiKey, BuildError> {
        Ok(ApiKey {
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
            prefix: self.prefix.ok_or_else(|| BuildError::missing_field("prefix"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            pod_id: self.pod_id,
            inbox_id: self.inbox_id,
            used_at: self.used_at,
            permissions: self.permissions,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
