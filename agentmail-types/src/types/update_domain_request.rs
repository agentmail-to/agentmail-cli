pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Provide at least one of `feedback_enabled`, `inbound_enabled`,
/// `subdomains_enabled`, or `tracking_enabled`. Omitted fields are left
/// unchanged; an empty body is rejected. Enabling `inbound_enabled` or
/// `subdomains_enabled` on a verified domain returns it to `PENDING` until the
/// newly required MX record (the apex MX, or the wildcard `*.<domain>`) is
/// published and verified.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDomainRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_enabled: Option<FeedbackEnabled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_enabled: Option<InboundEnabled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomains_enabled: Option<SubdomainsEnabled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_enabled: Option<TrackingEnabled>,
}

impl UpdateDomainRequest {
    pub fn builder() -> UpdateDomainRequestBuilder {
        <UpdateDomainRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDomainRequestBuilder {
    feedback_enabled: Option<FeedbackEnabled>,
    inbound_enabled: Option<InboundEnabled>,
    subdomains_enabled: Option<SubdomainsEnabled>,
    tracking_enabled: Option<TrackingEnabled>,
}

impl UpdateDomainRequestBuilder {
    pub fn feedback_enabled(mut self, value: FeedbackEnabled) -> Self {
        self.feedback_enabled = Some(value);
        self
    }

    pub fn inbound_enabled(mut self, value: InboundEnabled) -> Self {
        self.inbound_enabled = Some(value);
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

    /// Consumes the builder and constructs a [`UpdateDomainRequest`].
    pub fn build(self) -> Result<UpdateDomainRequest, BuildError> {
        Ok(UpdateDomainRequest {
            feedback_enabled: self.feedback_enabled,
            inbound_enabled: self.inbound_enabled,
            subdomains_enabled: self.subdomains_enabled,
            tracking_enabled: self.tracking_enabled,
        })
    }
}
