pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Create a polling webhook scoped to an inbox. The inbox comes from the path, so `inbox_ids` and
/// `pod_ids` are not accepted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksCreateInboxPollingWebhookRequest {
    #[serde(default)]
    pub client_id: WebhooksPollingClientId,
    #[serde(default)]
    pub event_types: WebhooksCreateWebhookEventTypes,
}

impl WebhooksCreateInboxPollingWebhookRequest {
    pub fn builder() -> WebhooksCreateInboxPollingWebhookRequestBuilder {
        <WebhooksCreateInboxPollingWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksCreateInboxPollingWebhookRequestBuilder {
    client_id: Option<WebhooksPollingClientId>,
    event_types: Option<WebhooksCreateWebhookEventTypes>,
}

impl WebhooksCreateInboxPollingWebhookRequestBuilder {
    pub fn client_id(mut self, value: WebhooksPollingClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn event_types(mut self, value: WebhooksCreateWebhookEventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksCreateInboxPollingWebhookRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](WebhooksCreateInboxPollingWebhookRequestBuilder::client_id)
    /// - [`event_types`](WebhooksCreateInboxPollingWebhookRequestBuilder::event_types)
    pub fn build(self) -> Result<WebhooksCreateInboxPollingWebhookRequest, BuildError> {
        Ok(WebhooksCreateInboxPollingWebhookRequest {
            client_id: self.client_id.ok_or_else(|| BuildError::missing_field("client_id"))?,
            event_types: self.event_types.ok_or_else(|| BuildError::missing_field("event_types"))?,
        })
    }
}
