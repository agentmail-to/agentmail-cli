pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksCreatePollingWebhookRequest {
    /// Pods for which to receive events. Maximum 10 channels (pods plus inboxes) per subscription. The
    /// subscription receives an event that matches any listed pod or inbox, so a listed pod already
    /// covers every inbox in it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ids: Option<PodIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<InboxIds>,
    #[serde(default)]
    pub client_id: WebhooksPollingClientId,
    #[serde(default)]
    pub event_types: WebhooksCreateWebhookEventTypes,
}

impl WebhooksCreatePollingWebhookRequest {
    pub fn builder() -> WebhooksCreatePollingWebhookRequestBuilder {
        <WebhooksCreatePollingWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksCreatePollingWebhookRequestBuilder {
    pod_ids: Option<PodIds>,
    inbox_ids: Option<InboxIds>,
    client_id: Option<WebhooksPollingClientId>,
    event_types: Option<WebhooksCreateWebhookEventTypes>,
}

impl WebhooksCreatePollingWebhookRequestBuilder {
    pub fn pod_ids(mut self, value: PodIds) -> Self {
        self.pod_ids = Some(value);
        self
    }

    pub fn inbox_ids(mut self, value: InboxIds) -> Self {
        self.inbox_ids = Some(value);
        self
    }

    pub fn client_id(mut self, value: WebhooksPollingClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn event_types(mut self, value: WebhooksCreateWebhookEventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksCreatePollingWebhookRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](WebhooksCreatePollingWebhookRequestBuilder::client_id)
    /// - [`event_types`](WebhooksCreatePollingWebhookRequestBuilder::event_types)
    pub fn build(self) -> Result<WebhooksCreatePollingWebhookRequest, BuildError> {
        Ok(WebhooksCreatePollingWebhookRequest {
            pod_ids: self.pod_ids,
            inbox_ids: self.inbox_ids,
            client_id: self.client_id.ok_or_else(|| BuildError::missing_field("client_id"))?,
            event_types: self.event_types.ok_or_else(|| BuildError::missing_field("event_types"))?,
        })
    }
}

