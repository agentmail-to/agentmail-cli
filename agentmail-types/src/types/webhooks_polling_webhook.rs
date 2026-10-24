pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A subscription the agent polls for events instead of receiving them at a URL. Same event types,
/// channels (pods and inboxes) and permissions as a webhook; no public endpoint is needed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksPollingWebhook {
    #[serde(default)]
    pub webhook_id: WebhooksWebhookId,
    #[serde(default)]
    pub client_id: WebhooksPollingClientId,
    #[serde(default)]
    pub event_types: EventTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ids: Option<PodIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<InboxIds>,
    /// ID of the pod the subscription belongs to: the pod it was created in, or the pod of the inbox it
    /// belongs to. Absent for an organization subscription. This is not the list of pods it receives
    /// events for; see `pod_ids`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
    /// ID of the inbox the subscription belongs to, if it was created for an inbox. This is not the
    /// list of inboxes it receives events for; see `inbox_ids`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    /// Time at which the subscription was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Time at which the subscription was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl WebhooksPollingWebhook {
    pub fn builder() -> WebhooksPollingWebhookBuilder {
        <WebhooksPollingWebhookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksPollingWebhookBuilder {
    webhook_id: Option<WebhooksWebhookId>,
    client_id: Option<WebhooksPollingClientId>,
    event_types: Option<EventTypes>,
    pod_ids: Option<PodIds>,
    inbox_ids: Option<InboxIds>,
    pod_id: Option<String>,
    inbox_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl WebhooksPollingWebhookBuilder {
    pub fn webhook_id(mut self, value: WebhooksWebhookId) -> Self {
        self.webhook_id = Some(value);
        self
    }

    pub fn client_id(mut self, value: WebhooksPollingClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn event_types(mut self, value: EventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn pod_ids(mut self, value: PodIds) -> Self {
        self.pod_ids = Some(value);
        self
    }

    pub fn inbox_ids(mut self, value: InboxIds) -> Self {
        self.inbox_ids = Some(value);
        self
    }

    pub fn pod_id(mut self, value: impl Into<String>) -> Self {
        self.pod_id = Some(value.into());
        self
    }

    pub fn inbox_id(mut self, value: impl Into<String>) -> Self {
        self.inbox_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`WebhooksPollingWebhook`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhook_id`](WebhooksPollingWebhookBuilder::webhook_id)
    /// - [`client_id`](WebhooksPollingWebhookBuilder::client_id)
    /// - [`event_types`](WebhooksPollingWebhookBuilder::event_types)
    /// - [`updated_at`](WebhooksPollingWebhookBuilder::updated_at)
    /// - [`created_at`](WebhooksPollingWebhookBuilder::created_at)
    pub fn build(self) -> Result<WebhooksPollingWebhook, BuildError> {
        Ok(WebhooksPollingWebhook {
            webhook_id: self.webhook_id.ok_or_else(|| BuildError::missing_field("webhook_id"))?,
            client_id: self.client_id.ok_or_else(|| BuildError::missing_field("client_id"))?,
            event_types: self.event_types.ok_or_else(|| BuildError::missing_field("event_types"))?,
            pod_ids: self.pod_ids,
            inbox_ids: self.inbox_ids,
            pod_id: self.pod_id,
            inbox_id: self.inbox_id,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
