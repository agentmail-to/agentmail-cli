pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Update a pod-scoped webhook. You can adjust which inboxes within the pod it listens to and replace
/// its `event_types`, but not the pod scope itself.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksUpdatePodWebhookRequest {
    #[serde(flatten)]
    pub webhooks_update_inbox_webhook_request_fields: WebhooksUpdateInboxWebhookRequest,
    /// Inbox IDs to subscribe to the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_inbox_ids: Option<InboxIds>,
    /// Inbox IDs to unsubscribe from the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_inbox_ids: Option<InboxIds>,
}

impl WebhooksUpdatePodWebhookRequest {
    pub fn builder() -> WebhooksUpdatePodWebhookRequestBuilder {
        <WebhooksUpdatePodWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksUpdatePodWebhookRequestBuilder {
    webhooks_update_inbox_webhook_request_fields: Option<WebhooksUpdateInboxWebhookRequest>,
    add_inbox_ids: Option<InboxIds>,
    remove_inbox_ids: Option<InboxIds>,
}

impl WebhooksUpdatePodWebhookRequestBuilder {
    pub fn webhooks_update_inbox_webhook_request_fields(mut self, value: WebhooksUpdateInboxWebhookRequest) -> Self {
        self.webhooks_update_inbox_webhook_request_fields = Some(value);
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

    /// Consumes the builder and constructs a [`WebhooksUpdatePodWebhookRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhooks_update_inbox_webhook_request_fields`](WebhooksUpdatePodWebhookRequestBuilder::webhooks_update_inbox_webhook_request_fields)
    pub fn build(self) -> Result<WebhooksUpdatePodWebhookRequest, BuildError> {
        Ok(WebhooksUpdatePodWebhookRequest {
            webhooks_update_inbox_webhook_request_fields: self.webhooks_update_inbox_webhook_request_fields.ok_or_else(|| BuildError::missing_field("webhooks_update_inbox_webhook_request_fields"))?,
            add_inbox_ids: self.add_inbox_ids,
            remove_inbox_ids: self.remove_inbox_ids,
        })
    }
}
