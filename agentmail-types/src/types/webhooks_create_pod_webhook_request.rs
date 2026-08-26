pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Create a webhook scoped to a pod. The pod comes from the path, so `pod_ids` is not accepted.
/// Optionally pass `inbox_ids` to narrow the webhook to specific inboxes within the pod; omit to
/// receive events for the whole pod.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhooksCreatePodWebhookRequest {
    #[serde(flatten)]
    pub webhooks_create_inbox_webhook_request_fields: WebhooksCreateInboxWebhookRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<InboxIds>,
}

impl WebhooksCreatePodWebhookRequest {
    pub fn builder() -> WebhooksCreatePodWebhookRequestBuilder {
        <WebhooksCreatePodWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksCreatePodWebhookRequestBuilder {
    webhooks_create_inbox_webhook_request_fields: Option<WebhooksCreateInboxWebhookRequest>,
    inbox_ids: Option<InboxIds>,
}

impl WebhooksCreatePodWebhookRequestBuilder {
    pub fn webhooks_create_inbox_webhook_request_fields(mut self, value: WebhooksCreateInboxWebhookRequest) -> Self {
        self.webhooks_create_inbox_webhook_request_fields = Some(value);
        self
    }

    pub fn inbox_ids(mut self, value: InboxIds) -> Self {
        self.inbox_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksCreatePodWebhookRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhooks_create_inbox_webhook_request_fields`](WebhooksCreatePodWebhookRequestBuilder::webhooks_create_inbox_webhook_request_fields)
    pub fn build(self) -> Result<WebhooksCreatePodWebhookRequest, BuildError> {
        Ok(WebhooksCreatePodWebhookRequest {
            webhooks_create_inbox_webhook_request_fields: self.webhooks_create_inbox_webhook_request_fields.ok_or_else(|| BuildError::missing_field("webhooks_create_inbox_webhook_request_fields"))?,
            inbox_ids: self.inbox_ids,
        })
    }
}
