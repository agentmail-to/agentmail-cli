pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Domain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<PodsPodId>,
    #[serde(default)]
    pub domain_id: DomainId,
    #[serde(default)]
    pub domain: DomainName,
    pub status: Status,
    /// Why the domain is not (yet) VERIFIED, when known. `dns_records_missing` / `dns_records_invalid` point at the DNS records. The `ses_*` values mean the records look right and sending-infrastructure validation has not converged: `ses_dkim_pending` / `ses_mail_from_pending` (still checking), `ses_dkim_temporary_failure` / `ses_mail_from_temporary_failure` (a transient error the infrastructure keeps retrying on its own — usually resolves without changes), `ses_dkim_failed` / `ses_mail_from_failed` (a terminal verdict; re-verify after fixing), `ses_dkim_not_started` / `ses_mail_from_not_started` (the attribute was never configured on the identity — re-verify to push it), and `ses_not_verified_for_sending`. Absent when VERIFIED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    pub feedback_enabled: FeedbackEnabled,
    #[serde(default)]
    pub subdomains_enabled: SubdomainsEnabled,
    #[serde(default)]
    pub tracking_enabled: TrackingEnabled,
    /// A list of DNS records required to verify the domain. Includes a
    /// wildcard MX record (`*.<domain>`) when `subdomains_enabled` is true.
    #[serde(default)]
    pub records: Vec<VerificationRecord>,
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

impl Domain {
    pub fn builder() -> DomainBuilder {
        <DomainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainBuilder {
    pod_id: Option<PodsPodId>,
    domain_id: Option<DomainId>,
    domain: Option<DomainName>,
    status: Option<Status>,
    reason: Option<String>,
    feedback_enabled: Option<FeedbackEnabled>,
    subdomains_enabled: Option<SubdomainsEnabled>,
    tracking_enabled: Option<TrackingEnabled>,
    records: Option<Vec<VerificationRecord>>,
    client_id: Option<ClientId>,
    updated_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl DomainBuilder {
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

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
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

    pub fn records(mut self, value: Vec<VerificationRecord>) -> Self {
        self.records = Some(value);
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

    /// Consumes the builder and constructs a [`Domain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_id`](DomainBuilder::domain_id)
    /// - [`domain`](DomainBuilder::domain)
    /// - [`status`](DomainBuilder::status)
    /// - [`feedback_enabled`](DomainBuilder::feedback_enabled)
    /// - [`subdomains_enabled`](DomainBuilder::subdomains_enabled)
    /// - [`tracking_enabled`](DomainBuilder::tracking_enabled)
    /// - [`records`](DomainBuilder::records)
    /// - [`updated_at`](DomainBuilder::updated_at)
    /// - [`created_at`](DomainBuilder::created_at)
    pub fn build(self) -> Result<Domain, BuildError> {
        Ok(Domain {
            pod_id: self.pod_id,
            domain_id: self.domain_id.ok_or_else(|| BuildError::missing_field("domain_id"))?,
            domain: self.domain.ok_or_else(|| BuildError::missing_field("domain"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            reason: self.reason,
            feedback_enabled: self.feedback_enabled.ok_or_else(|| BuildError::missing_field("feedback_enabled"))?,
            subdomains_enabled: self.subdomains_enabled.ok_or_else(|| BuildError::missing_field("subdomains_enabled"))?,
            tracking_enabled: self.tracking_enabled.ok_or_else(|| BuildError::missing_field("tracking_enabled"))?,
            records: self.records.ok_or_else(|| BuildError::missing_field("records"))?,
            client_id: self.client_id,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
