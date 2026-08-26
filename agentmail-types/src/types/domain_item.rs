pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<PodsPodId>,
    #[serde(default)]
    pub domain_id: DomainId,
    #[serde(default)]
    pub domain: DomainName,
    #[serde(default)]
    pub feedback_enabled: FeedbackEnabled,
    #[serde(default)]
    pub subdomains_enabled: SubdomainsEnabled,
    #[serde(default)]
    pub tracking_enabled: TrackingEnabled,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    /// Time at which the domain was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Time at which the domain was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl DomainItem {
    pub fn builder() -> DomainItemBuilder {
        <DomainItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainItemBuilder {
    pod_id: Option<PodsPodId>,
    domain_id: Option<DomainId>,
    domain: Option<DomainName>,
    feedback_enabled: Option<FeedbackEnabled>,
    subdomains_enabled: Option<SubdomainsEnabled>,
    tracking_enabled: Option<TrackingEnabled>,
    client_id: Option<ClientId>,
    updated_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl DomainItemBuilder {
    pub fn pod_id(mut self, value: PodsPodId) -> Self {
        self.pod_id = Some(value);
        self
    }

    pub fn domain_id(mut self, value: DomainId) -> Self {
        self.domain_id = Some(value);
        self
    }

    pub fn domain(mut self, value: DomainName) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn feedback_enabled(mut self, value: FeedbackEnabled) -> Self {
        self.feedback_enabled = Some(value);
        self
    }

    pub fn subdomains_enabled(mut self, value: SubdomainsEnabled) -> Self {
        self.subdomains_enabled = Some(value);
        self
    }

    pub fn tracking_enabled(mut self, value: TrackingEnabled) -> Self {
        self.tracking_enabled = Some(value);
        self
    }

    pub fn client_id(mut self, value: ClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_id`](DomainItemBuilder::domain_id)
    /// - [`domain`](DomainItemBuilder::domain)
    /// - [`feedback_enabled`](DomainItemBuilder::feedback_enabled)
    /// - [`subdomains_enabled`](DomainItemBuilder::subdomains_enabled)
    /// - [`tracking_enabled`](DomainItemBuilder::tracking_enabled)
    /// - [`updated_at`](DomainItemBuilder::updated_at)
    /// - [`created_at`](DomainItemBuilder::created_at)
    pub fn build(self) -> Result<DomainItem, BuildError> {
        Ok(DomainItem {
            pod_id: self.pod_id,
            domain_id: self.domain_id.ok_or_else(|| BuildError::missing_field("domain_id"))?,
            domain: self.domain.ok_or_else(|| BuildError::missing_field("domain"))?,
            feedback_enabled: self.feedback_enabled.ok_or_else(|| BuildError::missing_field("feedback_enabled"))?,
            subdomains_enabled: self.subdomains_enabled.ok_or_else(|| BuildError::missing_field("subdomains_enabled"))?,
            tracking_enabled: self.tracking_enabled.ok_or_else(|| BuildError::missing_field("tracking_enabled"))?,
            client_id: self.client_id,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
