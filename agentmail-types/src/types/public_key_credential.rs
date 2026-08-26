pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An AgentID sign-in credential. `type` and `api_key_id` are server-owned;
/// use `api_key_id` as the JWS `kid`. This response never contains a bearer
/// secret or private key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicKeyCredential {
    /// Server-generated credential ID. Store this value as the signing key's `kid`.
    #[serde(default)]
    pub api_key_id: String,
    /// Server-owned credential discriminator. Callers cannot select or update it.
    pub r#type: PublicKeyCredentialType,
    /// Human-readable credential name.
    #[serde(default)]
    pub name: Name,
    pub public_key: PublicKeyMaterial,
    pub scope: PublicKeyScope,
    /// Immutable absolute expiry. Omitted when the credential does not expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Present when organization-wide revoke-all invalidated this credential generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl PublicKeyCredential {
    pub fn builder() -> PublicKeyCredentialBuilder {
        <PublicKeyCredentialBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicKeyCredentialBuilder {
    api_key_id: Option<String>,
    r#type: Option<PublicKeyCredentialType>,
    name: Option<Name>,
    public_key: Option<PublicKeyMaterial>,
    scope: Option<PublicKeyScope>,
    expires_at: Option<DateTime<FixedOffset>>,
    revoked_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl PublicKeyCredentialBuilder {
    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PublicKeyCredentialType) -> Self {
        self.r#type = Some(value);
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

    pub fn scope(mut self, value: PublicKeyScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn revoked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicKeyCredential`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](PublicKeyCredentialBuilder::api_key_id)
    /// - [`r#type`](PublicKeyCredentialBuilder::r#type)
    /// - [`name`](PublicKeyCredentialBuilder::name)
    /// - [`public_key`](PublicKeyCredentialBuilder::public_key)
    /// - [`scope`](PublicKeyCredentialBuilder::scope)
    /// - [`created_at`](PublicKeyCredentialBuilder::created_at)
    /// - [`updated_at`](PublicKeyCredentialBuilder::updated_at)
    pub fn build(self) -> Result<PublicKeyCredential, BuildError> {
        Ok(PublicKeyCredential {
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            public_key: self.public_key.ok_or_else(|| BuildError::missing_field("public_key"))?,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            expires_at: self.expires_at,
            revoked_at: self.revoked_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
