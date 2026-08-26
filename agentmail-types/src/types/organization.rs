pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Organization details with usage limits and counts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Organization {
    #[serde(default)]
    pub organization_id: OrganizationId,
    /// Current number of inboxes.
    #[serde(default)]
    pub inbox_count: i64,
    /// Current number of domains.
    #[serde(default)]
    pub domain_count: i64,
    /// Maximum number of inboxes allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_limit: Option<i64>,
    /// Maximum number of domains allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_limit: Option<i64>,
    /// Provider-agnostic billing customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_id: Option<String>,
    /// Billing provider type (e.g. "stripe").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<String>,
    /// Active billing subscription ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_subscription_id: Option<String>,
    /// Provider-agnostic authentication ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_id: Option<String>,
    /// Authentication provider type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    /// Time at which organization was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Time at which organization was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl Organization {
    pub fn builder() -> OrganizationBuilder {
        <OrganizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationBuilder {
    organization_id: Option<OrganizationId>,
    inbox_count: Option<i64>,
    domain_count: Option<i64>,
    inbox_limit: Option<i64>,
    domain_limit: Option<i64>,
    billing_id: Option<String>,
    billing_type: Option<String>,
    billing_subscription_id: Option<String>,
    authentication_id: Option<String>,
    authentication_type: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl OrganizationBuilder {
    pub fn organization_id(mut self, value: OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn inbox_count(mut self, value: i64) -> Self {
        self.inbox_count = Some(value);
        self
    }

    pub fn domain_count(mut self, value: i64) -> Self {
        self.domain_count = Some(value);
        self
    }

    pub fn inbox_limit(mut self, value: i64) -> Self {
        self.inbox_limit = Some(value);
        self
    }

    pub fn domain_limit(mut self, value: i64) -> Self {
        self.domain_limit = Some(value);
        self
    }

    pub fn billing_id(mut self, value: impl Into<String>) -> Self {
        self.billing_id = Some(value.into());
        self
    }

    pub fn billing_type(mut self, value: impl Into<String>) -> Self {
        self.billing_type = Some(value.into());
        self
    }

    pub fn billing_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.billing_subscription_id = Some(value.into());
        self
    }

    pub fn authentication_id(mut self, value: impl Into<String>) -> Self {
        self.authentication_id = Some(value.into());
        self
    }

    pub fn authentication_type(mut self, value: impl Into<String>) -> Self {
        self.authentication_type = Some(value.into());
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

    /// Consumes the builder and constructs a [`Organization`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](OrganizationBuilder::organization_id)
    /// - [`inbox_count`](OrganizationBuilder::inbox_count)
    /// - [`domain_count`](OrganizationBuilder::domain_count)
    /// - [`updated_at`](OrganizationBuilder::updated_at)
    /// - [`created_at`](OrganizationBuilder::created_at)
    pub fn build(self) -> Result<Organization, BuildError> {
        Ok(Organization {
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            inbox_count: self.inbox_count.ok_or_else(|| BuildError::missing_field("inbox_count"))?,
            domain_count: self.domain_count.ok_or_else(|| BuildError::missing_field("domain_count"))?,
            inbox_limit: self.inbox_limit,
            domain_limit: self.domain_limit,
            billing_id: self.billing_id,
            billing_type: self.billing_type,
            billing_subscription_id: self.billing_subscription_id,
            authentication_id: self.authentication_id,
            authentication_type: self.authentication_type,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
