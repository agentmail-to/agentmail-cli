pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One inbox signed in at one provider.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Account {
    #[serde(default)]
    pub account_id: AccountId,
    #[serde(default)]
    pub provider_id: ProviderId,
    /// Display name of provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub pod_id: PodsPodId,
    #[serde(default)]
    pub organization_id: OrganizationId,
    /// Time of first sign-in at provider.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub first_signed_in_at: DateTime<FixedOffset>,
    /// Time of most recent sign-in at provider.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub last_signed_in_at: DateTime<FixedOffset>,
    /// Number of sign-ins at provider.
    #[serde(default)]
    pub sign_in_count: i64,
}

impl Account {
    pub fn builder() -> AccountBuilder {
        <AccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountBuilder {
    account_id: Option<AccountId>,
    provider_id: Option<ProviderId>,
    provider_name: Option<String>,
    inbox_id: Option<InboxesInboxId>,
    pod_id: Option<PodsPodId>,
    organization_id: Option<OrganizationId>,
    first_signed_in_at: Option<DateTime<FixedOffset>>,
    last_signed_in_at: Option<DateTime<FixedOffset>>,
    sign_in_count: Option<i64>,
}

impl AccountBuilder {
    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn provider_id(mut self, value: ProviderId) -> Self {
        self.provider_id = Some(value);
        self
    }

    pub fn provider_name(mut self, value: impl Into<String>) -> Self {
        self.provider_name = Some(value.into());
        self
    }

    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn pod_id(mut self, value: PodsPodId) -> Self {
        self.pod_id = Some(value);
        self
    }

    pub fn organization_id(mut self, value: OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn first_signed_in_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_signed_in_at = Some(value);
        self
    }

    pub fn last_signed_in_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_signed_in_at = Some(value);
        self
    }

    pub fn sign_in_count(mut self, value: i64) -> Self {
        self.sign_in_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Account`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](AccountBuilder::account_id)
    /// - [`provider_id`](AccountBuilder::provider_id)
    /// - [`inbox_id`](AccountBuilder::inbox_id)
    /// - [`pod_id`](AccountBuilder::pod_id)
    /// - [`organization_id`](AccountBuilder::organization_id)
    /// - [`first_signed_in_at`](AccountBuilder::first_signed_in_at)
    /// - [`last_signed_in_at`](AccountBuilder::last_signed_in_at)
    /// - [`sign_in_count`](AccountBuilder::sign_in_count)
    pub fn build(self) -> Result<Account, BuildError> {
        Ok(Account {
            account_id: self.account_id.ok_or_else(|| BuildError::missing_field("account_id"))?,
            provider_id: self.provider_id.ok_or_else(|| BuildError::missing_field("provider_id"))?,
            provider_name: self.provider_name,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            pod_id: self.pod_id.ok_or_else(|| BuildError::missing_field("pod_id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            first_signed_in_at: self.first_signed_in_at.ok_or_else(|| BuildError::missing_field("first_signed_in_at"))?,
            last_signed_in_at: self.last_signed_in_at.ok_or_else(|| BuildError::missing_field("last_signed_in_at"))?,
            sign_in_count: self.sign_in_count.ok_or_else(|| BuildError::missing_field("sign_in_count"))?,
        })
    }
}
