pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An AgentID sign-in credential, scoped like a bearer key; `api_key_id` is
/// the JWS `kid`. A sign-in key carries `status`, gains `public_key` once
/// the client has proved it, expires 30 days after
/// activation, and carries exactly `provider_connect` and
/// `provider_share_owner`, snapshotted from the bearer key that created it
/// and enforced from the key itself.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicKeyCredential {
    pub r#type: PublicKeyCredentialType,
    #[serde(default)]
    pub api_key_id: ApiKeyId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<PublicKeyClientId>,
    #[serde(default)]
    pub name: Name,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<PublicKeyMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<PodScopeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<InboxScopeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PublicKeyStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_at: Option<UsedAt>,
    #[serde(default)]
    pub permissions: ApiKeyPermissions,
    #[serde(default)]
    pub created_by: ApiKeyCreator,
    #[serde(default)]
    pub created_at: CreatedAt,
    #[serde(default)]
    pub updated_at: UpdatedAt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<ExpiresAt>,
}

impl PublicKeyCredential {
    pub fn builder() -> PublicKeyCredentialBuilder {
        <PublicKeyCredentialBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicKeyCredentialBuilder {
    r#type: Option<PublicKeyCredentialType>,
    api_key_id: Option<ApiKeyId>,
    client_id: Option<PublicKeyClientId>,
    name: Option<Name>,
    public_key: Option<PublicKeyMaterial>,
    pod_id: Option<PodScopeId>,
    inbox_id: Option<InboxScopeId>,
    status: Option<PublicKeyStatus>,
    used_at: Option<UsedAt>,
    permissions: Option<ApiKeyPermissions>,
    created_by: Option<ApiKeyCreator>,
    created_at: Option<CreatedAt>,
    updated_at: Option<UpdatedAt>,
    expires_at: Option<ExpiresAt>,
}

impl PublicKeyCredentialBuilder {
    pub fn r#type(mut self, value: PublicKeyCredentialType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn api_key_id(mut self, value: ApiKeyId) -> Self {
        self.api_key_id = Some(value);
        self
    }

    pub fn client_id(mut self, value: PublicKeyClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn name(mut self, value: Name) -> Self {
        self.name = Some(value);
        self
    }

    pub fn public_key(mut self, value: PublicKeyMaterial) -> Self {
        self.public_key = Some(value);
        self
    }

    pub fn pod_id(mut self, value: PodScopeId) -> Self {
        self.pod_id = Some(value);
        self
    }

    pub fn inbox_id(mut self, value: InboxScopeId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn status(mut self, value: PublicKeyStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn used_at(mut self, value: UsedAt) -> Self {
        self.used_at = Some(value);
        self
    }

    pub fn permissions(mut self, value: ApiKeyPermissions) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn created_by(mut self, value: ApiKeyCreator) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: UpdatedAt) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: ExpiresAt) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicKeyCredential`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PublicKeyCredentialBuilder::r#type)
    /// - [`api_key_id`](PublicKeyCredentialBuilder::api_key_id)
    /// - [`name`](PublicKeyCredentialBuilder::name)
    /// - [`permissions`](PublicKeyCredentialBuilder::permissions)
    /// - [`created_by`](PublicKeyCredentialBuilder::created_by)
    /// - [`created_at`](PublicKeyCredentialBuilder::created_at)
    /// - [`updated_at`](PublicKeyCredentialBuilder::updated_at)
    pub fn build(self) -> Result<PublicKeyCredential, BuildError> {
        Ok(PublicKeyCredential {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
            client_id: self.client_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            public_key: self.public_key,
            pod_id: self.pod_id,
            inbox_id: self.inbox_id,
            status: self.status,
            used_at: self.used_at,
            permissions: self.permissions.ok_or_else(|| BuildError::missing_field("permissions"))?,
            created_by: self.created_by.ok_or_else(|| BuildError::missing_field("created_by"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            expires_at: self.expires_at,
        })
    }
}
