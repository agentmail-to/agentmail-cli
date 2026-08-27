pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Remembered approval for one closed AgentID client and inbox.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BrowserConsent {
    #[serde(default)]
    pub consent_id: String,
    #[serde(default)]
    pub inbox_id: String,
    pub client_type: BrowserConsentClientType,
    #[serde(default)]
    pub client_id: String,
    /// Registered client URL, when one is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_url: Option<String>,
    /// At least one non-empty scope approved for this client.
    #[serde(default)]
    pub approved_scopes: Vec<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl BrowserConsent {
    pub fn builder() -> BrowserConsentBuilder {
        <BrowserConsentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserConsentBuilder {
    consent_id: Option<String>,
    inbox_id: Option<String>,
    client_type: Option<BrowserConsentClientType>,
    client_id: Option<String>,
    client_url: Option<String>,
    approved_scopes: Option<Vec<String>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl BrowserConsentBuilder {
    pub fn consent_id(mut self, value: impl Into<String>) -> Self {
        self.consent_id = Some(value.into());
        self
    }

    pub fn inbox_id(mut self, value: impl Into<String>) -> Self {
        self.inbox_id = Some(value.into());
        self
    }

    pub fn client_type(mut self, value: BrowserConsentClientType) -> Self {
        self.client_type = Some(value);
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_url(mut self, value: impl Into<String>) -> Self {
        self.client_url = Some(value.into());
        self
    }

    pub fn approved_scopes(mut self, value: Vec<String>) -> Self {
        self.approved_scopes = Some(value);
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

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrowserConsent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`consent_id`](BrowserConsentBuilder::consent_id)
    /// - [`inbox_id`](BrowserConsentBuilder::inbox_id)
    /// - [`client_type`](BrowserConsentBuilder::client_type)
    /// - [`client_id`](BrowserConsentBuilder::client_id)
    /// - [`approved_scopes`](BrowserConsentBuilder::approved_scopes)
    /// - [`created_at`](BrowserConsentBuilder::created_at)
    /// - [`updated_at`](BrowserConsentBuilder::updated_at)
    /// - [`expires_at`](BrowserConsentBuilder::expires_at)
    pub fn build(self) -> Result<BrowserConsent, BuildError> {
        Ok(BrowserConsent {
            consent_id: self.consent_id.ok_or_else(|| BuildError::missing_field("consent_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            client_type: self.client_type.ok_or_else(|| BuildError::missing_field("client_type"))?,
            client_id: self.client_id.ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_url: self.client_url,
            approved_scopes: self.approved_scopes.ok_or_else(|| BuildError::missing_field("approved_scopes"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
