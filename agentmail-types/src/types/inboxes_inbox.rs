pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InboxesInbox {
    #[serde(default)]
    pub pod_id: PodsPodId,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub email: InboxesEmail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<InboxesDisplayName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<InboxesClientId>,
    /// Custom metadata attached to the inbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<InboxesMetadata>,
    /// Time at which inbox was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Time at which inbox was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl InboxesInbox {
    pub fn builder() -> InboxesInboxBuilder {
        <InboxesInboxBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesInboxBuilder {
    pod_id: Option<PodsPodId>,
    inbox_id: Option<InboxesInboxId>,
    email: Option<InboxesEmail>,
    display_name: Option<InboxesDisplayName>,
    client_id: Option<InboxesClientId>,
    metadata: Option<InboxesMetadata>,
    updated_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl InboxesInboxBuilder {
    pub fn pod_id(mut self, value: PodsPodId) -> Self {
        self.pod_id = Some(value);
        self
    }

    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn email(mut self, value: InboxesEmail) -> Self {
        self.email = Some(value);
        self
    }

    pub fn display_name(mut self, value: InboxesDisplayName) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn client_id(mut self, value: InboxesClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn metadata(mut self, value: InboxesMetadata) -> Self {
        self.metadata = Some(value);
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

    /// Consumes the builder and constructs a [`InboxesInbox`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pod_id`](InboxesInboxBuilder::pod_id)
    /// - [`inbox_id`](InboxesInboxBuilder::inbox_id)
    /// - [`email`](InboxesInboxBuilder::email)
    /// - [`updated_at`](InboxesInboxBuilder::updated_at)
    /// - [`created_at`](InboxesInboxBuilder::created_at)
    pub fn build(self) -> Result<InboxesInbox, BuildError> {
        Ok(InboxesInbox {
            pod_id: self.pod_id.ok_or_else(|| BuildError::missing_field("pod_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            display_name: self.display_name,
            client_id: self.client_id,
            metadata: self.metadata,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
