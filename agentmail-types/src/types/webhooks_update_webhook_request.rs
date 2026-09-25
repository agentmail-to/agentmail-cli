pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksUpdateWebhookRequest {
    /// Pod IDs to subscribe to the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_pod_ids: Option<PodIds>,
    /// Pod IDs to unsubscribe from the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_pod_ids: Option<PodIds>,
    /// Inbox IDs to subscribe to the webhook. This only adds to what the webhook receives: on a webhook
    /// that covers its whole pod, it changes nothing. To receive only specific inboxes, create a webhook
    /// with `inbox_ids` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_inbox_ids: Option<InboxIds>,
    /// Inbox IDs to unsubscribe from the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_inbox_ids: Option<InboxIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<WebhooksUpdateWebhookEventTypes>,
    /// Set to true to re-enable a webhook that was disabled after repeated failed deliveries, or false
    /// to disable it. Events that occurred while the webhook was disabled are not redelivered.
    /// Re-enabling a webhook subscribed to `message.received.spam`, `message.received.blocked`, or
    /// `message.received.unauthenticated` (or to every event type, with no filter) requires the
    /// matching label permissions on the API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl WebhooksUpdateWebhookRequest {
    pub fn builder() -> WebhooksUpdateWebhookRequestBuilder {
        <WebhooksUpdateWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksUpdateWebhookRequestBuilder {
    add_pod_ids: Option<PodIds>,
    remove_pod_ids: Option<PodIds>,
    add_inbox_ids: Option<InboxIds>,
    remove_inbox_ids: Option<InboxIds>,
    event_types: Option<WebhooksUpdateWebhookEventTypes>,
    enabled: Option<bool>,
}

impl WebhooksUpdateWebhookRequestBuilder {
    pub fn add_pod_ids(mut self, value: PodIds) -> Self {
        self.add_pod_ids = Some(value);
        self
    }

    pub fn remove_pod_ids(mut self, value: PodIds) -> Self {
        self.remove_pod_ids = Some(value);
        self
    }

    pub fn add_inbox_ids(mut self, value: InboxIds) -> Self {
        self.add_inbox_ids = Some(value);
        self
    }

    pub fn remove_inbox_ids(mut self, value: InboxIds) -> Self {
        self.remove_inbox_ids = Some(value);
        self
    }

    pub fn event_types(mut self, value: WebhooksUpdateWebhookEventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksUpdateWebhookRequest`].
    pub fn build(self) -> Result<WebhooksUpdateWebhookRequest, BuildError> {
        Ok(WebhooksUpdateWebhookRequest {
            add_pod_ids: self.add_pod_ids,
            remove_pod_ids: self.remove_pod_ids,
            add_inbox_ids: self.add_inbox_ids,
            remove_inbox_ids: self.remove_inbox_ids,
            event_types: self.event_types,
            enabled: self.enabled,
        })
    }
}

