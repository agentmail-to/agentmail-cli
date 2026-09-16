pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainRequest {
    #[serde(default)]
    pub domain: DomainName,
    /// Allow registration when the domain already has Google Workspace MX records.
    /// Defaults to false; registration otherwise returns 422 when a conflicting
    /// provider is detected.
    /// This flag does not configure DNS or inbound routing. For shared Google
    /// Workspace domains, follow the [Google Workspace guide](/google-workspace).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_conflicting_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_enabled: Option<FeedbackEnabled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomains_enabled: Option<SubdomainsEnabled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_enabled: Option<TrackingEnabled>,
}

impl CreateDomainRequest {
    pub fn builder() -> CreateDomainRequestBuilder {
        <CreateDomainRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainRequestBuilder {
    domain: Option<DomainName>,
    allow_conflicting_provider: Option<bool>,
    feedback_enabled: Option<FeedbackEnabled>,
    subdomains_enabled: Option<SubdomainsEnabled>,
    tracking_enabled: Option<TrackingEnabled>,
}

impl CreateDomainRequestBuilder {
    pub fn domain(mut self, value: DomainName) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn allow_conflicting_provider(mut self, value: bool) -> Self {
        self.allow_conflicting_provider = Some(value);
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

    /// Consumes the builder and constructs a [`CreateDomainRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](CreateDomainRequestBuilder::domain)
    pub fn build(self) -> Result<CreateDomainRequest, BuildError> {
        Ok(CreateDomainRequest {
            domain: self.domain.ok_or_else(|| BuildError::missing_field("domain"))?,
            allow_conflicting_provider: self.allow_conflicting_provider,
            feedback_enabled: self.feedback_enabled,
            subdomains_enabled: self.subdomains_enabled,
            tracking_enabled: self.tracking_enabled,
        })
    }
}
