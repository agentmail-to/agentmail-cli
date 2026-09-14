pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Registers a public P-256 JWK at the route's scope. `type` and
/// `api_key_id` are server-owned. `name` defaults to
/// `AgentID key {first eight fingerprint characters}`; `permissions`
/// defaults to the registering key's, and only grants it holds may be
/// true; `expires_at` defaults to the registering key's expiry and is
/// independent of that key afterward.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePublicKeyRequest {
    pub public_key: PublicJwk,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<PublicKeyClientId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<ExpiresAt>,
}

impl CreatePublicKeyRequest {
    pub fn builder() -> CreatePublicKeyRequestBuilder {
        <CreatePublicKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePublicKeyRequestBuilder {
    public_key: Option<PublicJwk>,
    client_id: Option<PublicKeyClientId>,
    name: Option<Name>,
    permissions: Option<ApiKeyPermissions>,
    expires_at: Option<ExpiresAt>,
}

impl CreatePublicKeyRequestBuilder {
    pub fn public_key(mut self, value: PublicJwk) -> Self {
        self.public_key = Some(value);
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

    pub fn permissions(mut self, value: ApiKeyPermissions) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn expires_at(mut self, value: ExpiresAt) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePublicKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`public_key`](CreatePublicKeyRequestBuilder::public_key)
    pub fn build(self) -> Result<CreatePublicKeyRequest, BuildError> {
        Ok(CreatePublicKeyRequest {
            public_key: self.public_key.ok_or_else(|| BuildError::missing_field("public_key"))?,
            client_id: self.client_id,
            name: self.name,
            permissions: self.permissions,
            expires_at: self.expires_at,
        })
    }
}
