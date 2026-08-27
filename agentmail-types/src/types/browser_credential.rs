pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Owner-facing metadata for an active browser credential. Private key material never leaves the browser.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BrowserCredential {
    #[serde(default)]
    pub credential_id: String,
    #[serde(default)]
    pub public_key_fingerprint_prefix: String,
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub pod_id: String,
    #[serde(default)]
    pub inbox_id: String,
    pub created_by: BrowserCredentialCreator,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl BrowserCredential {
    pub fn builder() -> BrowserCredentialBuilder {
        <BrowserCredentialBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserCredentialBuilder {
    credential_id: Option<String>,
    public_key_fingerprint_prefix: Option<String>,
    organization_id: Option<String>,
    pod_id: Option<String>,
    inbox_id: Option<String>,
    created_by: Option<BrowserCredentialCreator>,
    created_at: Option<DateTime<FixedOffset>>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl BrowserCredentialBuilder {
    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    pub fn public_key_fingerprint_prefix(mut self, value: impl Into<String>) -> Self {
        self.public_key_fingerprint_prefix = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
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

    pub fn created_by(mut self, value: BrowserCredentialCreator) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrowserCredential`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credential_id`](BrowserCredentialBuilder::credential_id)
    /// - [`public_key_fingerprint_prefix`](BrowserCredentialBuilder::public_key_fingerprint_prefix)
    /// - [`organization_id`](BrowserCredentialBuilder::organization_id)
    /// - [`pod_id`](BrowserCredentialBuilder::pod_id)
    /// - [`inbox_id`](BrowserCredentialBuilder::inbox_id)
    /// - [`created_by`](BrowserCredentialBuilder::created_by)
    /// - [`created_at`](BrowserCredentialBuilder::created_at)
    /// - [`expires_at`](BrowserCredentialBuilder::expires_at)
    pub fn build(self) -> Result<BrowserCredential, BuildError> {
        Ok(BrowserCredential {
            credential_id: self.credential_id.ok_or_else(|| BuildError::missing_field("credential_id"))?,
            public_key_fingerprint_prefix: self.public_key_fingerprint_prefix.ok_or_else(|| BuildError::missing_field("public_key_fingerprint_prefix"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            pod_id: self.pod_id.ok_or_else(|| BuildError::missing_field("pod_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            created_by: self.created_by.ok_or_else(|| BuildError::missing_field("created_by"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
