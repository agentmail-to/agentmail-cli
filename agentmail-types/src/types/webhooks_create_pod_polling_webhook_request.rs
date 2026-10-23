pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Create a polling webhook scoped to a pod. The pod comes from the path, so `pod_ids` is not accepted.
/// Optionally pass `inbox_ids` to narrow the subscription to specific inboxes within the pod; omit to
/// receive events for the whole pod.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksCreatePodPollingWebhookRequest {
    #[serde(flatten)]
    pub webhooks_create_inbox_polling_webhook_request_fields: WebhooksCreateInboxPollingWebhookRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<InboxIds>,
}

impl WebhooksCreatePodPollingWebhookRequest {
    pub fn builder() -> WebhooksCreatePodPollingWebhookRequestBuilder {
        <WebhooksCreatePodPollingWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksCreatePodPollingWebhookRequestBuilder {
    webhooks_create_inbox_polling_webhook_request_fields: Option<WebhooksCreateInboxPollingWebhookRequest>,
    inbox_ids: Option<InboxIds>,
}

impl WebhooksCreatePodPollingWebhookRequestBuilder {
    pub fn webhooks_create_inbox_polling_webhook_request_fields(mut self, value: WebhooksCreateInboxPollingWebhookRequest) -> Self {
        self.webhooks_create_inbox_polling_webhook_request_fields = Some(value);
        self
    }

    pub fn inbox_ids(mut self, value: InboxIds) -> Self {
        self.inbox_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksCreatePodPollingWebhookRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhooks_create_inbox_polling_webhook_request_fields`](WebhooksCreatePodPollingWebhookRequestBuilder::webhooks_create_inbox_polling_webhook_request_fields)
    pub fn build(self) -> Result<WebhooksCreatePodPollingWebhookRequest, BuildError> {
        Ok(WebhooksCreatePodPollingWebhookRequest {
            webhooks_create_inbox_polling_webhook_request_fields: self.webhooks_create_inbox_polling_webhook_request_fields.ok_or_else(|| BuildError::missing_field("webhooks_create_inbox_polling_webhook_request_fields"))?,
            inbox_ids: self.inbox_ids,
        })
    }
}
