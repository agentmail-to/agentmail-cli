pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InboxEvent {
    #[serde(default)]
    pub organization_id: OrganizationId,
    /// ID of pod.
    #[serde(default)]
    pub pod_id: String,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub event_id: InboxEventId,
    pub event_type: InboxEventType,
    /// ID of message.
    #[serde(default)]
    pub message_id: String,
    /// Label added or removed.
    #[serde(default)]
    pub label: String,
    /// Time at which the event occurred.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub event_at: DateTime<FixedOffset>,
    /// Time at which the event was recorded.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl InboxEvent {
    pub fn builder() -> InboxEventBuilder {
        <InboxEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxEventBuilder {
    organization_id: Option<OrganizationId>,
    pod_id: Option<String>,
    inbox_id: Option<InboxesInboxId>,
    event_id: Option<InboxEventId>,
    event_type: Option<InboxEventType>,
    message_id: Option<String>,
    label: Option<String>,
    event_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl InboxEventBuilder {
    pub fn organization_id(mut self, value: OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn pod_id(mut self, value: impl Into<String>) -> Self {
        self.pod_id = Some(value.into());
        self
    }

    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn event_id(mut self, value: InboxEventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn event_type(mut self, value: InboxEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn event_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.event_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](InboxEventBuilder::organization_id)
    /// - [`pod_id`](InboxEventBuilder::pod_id)
    /// - [`inbox_id`](InboxEventBuilder::inbox_id)
    /// - [`event_id`](InboxEventBuilder::event_id)
    /// - [`event_type`](InboxEventBuilder::event_type)
    /// - [`message_id`](InboxEventBuilder::message_id)
    /// - [`label`](InboxEventBuilder::label)
    /// - [`event_at`](InboxEventBuilder::event_at)
    /// - [`created_at`](InboxEventBuilder::created_at)
    pub fn build(self) -> Result<InboxEvent, BuildError> {
        Ok(InboxEvent {
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            pod_id: self.pod_id.ok_or_else(|| BuildError::missing_field("pod_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            event_at: self.event_at.ok_or_else(|| BuildError::missing_field("event_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
