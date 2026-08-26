pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePublicKeyRequest {
    pub public_key: PublicJwk,
    /// Defaults to `AgentID key {first eight fingerprint characters}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Omit to inherit the registering bearer key's exact scope. An explicit
    /// scope must be the caller's scope or a live descendant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<PublicKeyScope>,
    /// Future absolute expiry. Omit to inherit the registering bearer key's
    /// expiry. A child credential cannot outlive its creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
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
    name: Option<String>,
    scope: Option<PublicKeyScope>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl CreatePublicKeyRequestBuilder {
    pub fn public_key(mut self, value: PublicJwk) -> Self {
        self.public_key = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scope(mut self, value: PublicKeyScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePublicKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`public_key`](CreatePublicKeyRequestBuilder::public_key)
    pub fn build(self) -> Result<CreatePublicKeyRequest, BuildError> {
        Ok(CreatePublicKeyRequest {
            public_key: self.public_key.ok_or_else(|| BuildError::missing_field("public_key"))?,
            name: self.name,
            scope: self.scope,
            expires_at: self.expires_at,
        })
    }
}

