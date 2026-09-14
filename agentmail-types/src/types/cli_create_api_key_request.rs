pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Omit `public_key` to mint a bearer key; include it to register a
/// public key. `client_id` and `expires_at` apply only with `public_key`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CliCreateApiKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
    /// Public P-256 JWK with exactly `kty` ("EC"), `crv` ("P-256"), `x`,
    /// and `y`, as JSON. Registers a public key instead of minting a
    /// bearer key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<PublicKeyClientId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<ExpiresAt>,
}

impl CliCreateApiKeyRequest {
    pub fn builder() -> CliCreateApiKeyRequestBuilder {
        <CliCreateApiKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CliCreateApiKeyRequestBuilder {
    name: Option<Name>,
    permissions: Option<ApiKeyPermissions>,
    public_key: Option<HashMap<String, serde_json::Value>>,
    client_id: Option<PublicKeyClientId>,
    expires_at: Option<ExpiresAt>,
}

impl CliCreateApiKeyRequestBuilder {
    pub fn name(mut self, value: Name) -> Self {
        self.name = Some(value);
        self
    }

    pub fn permissions(mut self, value: ApiKeyPermissions) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn public_key(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.public_key = Some(value);
        self
    }

    pub fn client_id(mut self, value: PublicKeyClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn expires_at(mut self, value: ExpiresAt) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CliCreateApiKeyRequest`].
    pub fn build(self) -> Result<CliCreateApiKeyRequest, BuildError> {
        Ok(CliCreateApiKeyRequest {
            name: self.name,
            permissions: self.permissions,
            public_key: self.public_key,
            client_id: self.client_id,
            expires_at: self.expires_at,
        })
    }
}
