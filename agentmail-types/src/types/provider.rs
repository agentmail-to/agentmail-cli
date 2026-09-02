pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A provider an inbox can sign in to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Provider {
    #[serde(default)]
    pub provider_id: ProviderId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Time at which provider was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_url: Option<String>,
}

impl Provider {
    pub fn builder() -> ProviderBuilder {
        <ProviderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProviderBuilder {
    provider_id: Option<ProviderId>,
    name: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    logo_url: Option<String>,
    terms_url: Option<String>,
    privacy_url: Option<String>,
}

impl ProviderBuilder {
    pub fn provider_id(mut self, value: ProviderId) -> Self {
        self.provider_id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn terms_url(mut self, value: impl Into<String>) -> Self {
        self.terms_url = Some(value.into());
        self
    }

    pub fn privacy_url(mut self, value: impl Into<String>) -> Self {
        self.privacy_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Provider`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider_id`](ProviderBuilder::provider_id)
    pub fn build(self) -> Result<Provider, BuildError> {
        Ok(Provider {
            provider_id: self.provider_id.ok_or_else(|| BuildError::missing_field("provider_id"))?,
            name: self.name,
            updated_at: self.updated_at,
            description: self.description,
            logo_url: self.logo_url,
            terms_url: self.terms_url,
            privacy_url: self.privacy_url,
        })
    }
}
