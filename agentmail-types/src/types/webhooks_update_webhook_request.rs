pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksUpdateWebhookRequest {
    /// Pod IDs to subscribe to the webhook. The webhook can list at most 10 pods and inboxes combined
    /// after the update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_pod_ids: Option<PodIds>,
    /// Pod IDs to unsubscribe from the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_pod_ids: Option<PodIds>,
    /// Inbox IDs to subscribe to the webhook. This only adds to what the webhook receives: on a webhook
    /// that covers its whole pod, it changes nothing. To receive only specific inboxes, create a webhook
    /// with `inbox_ids` instead. The webhook can list at most 10 inboxes and pods combined after the
    /// update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_inbox_ids: Option<InboxIds>,
    /// Inbox IDs to unsubscribe from the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_inbox_ids: Option<InboxIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<WebhooksUpdateWebhookEventTypes>,
    /// Set to true to re-enable a webhook that was disabled after repeated failed deliveries, or false
    /// to disable it. AgentMail does not notify you when a webhook is disabled automatically; check this
    /// field when you get or list webhooks. Events that occurred while the webhook was disabled are not
    /// redelivered. Re-enabling requires the read permission for every restricted event type the webhook
    /// subscribes to: `label_spam_read`, `label_blocked_read`, or `label_unauthenticated_read` for the
    /// received variants, `message_read` for `message.security.*`, and `calendar_event_read` for
    /// `calendar.event.*`. A webhook with no event type filter is checked against all of them.
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

