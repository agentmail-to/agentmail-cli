pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Create a webhook scoped to an inbox. The inbox comes from the path, so `inbox_ids` and `pod_ids`
/// are not accepted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhooksCreateInboxWebhookRequest {
    #[serde(default)]
    pub url: WebhooksUrl,
    #[serde(default)]
    pub event_types: WebhooksCreateWebhookEventTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<WebhooksClientId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<WebhooksWebhookHeaders>,
}

impl WebhooksCreateInboxWebhookRequest {
    pub fn builder() -> WebhooksCreateInboxWebhookRequestBuilder {
        <WebhooksCreateInboxWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksCreateInboxWebhookRequestBuilder {
    url: Option<WebhooksUrl>,
    event_types: Option<WebhooksCreateWebhookEventTypes>,
    client_id: Option<WebhooksClientId>,
    headers: Option<WebhooksWebhookHeaders>,
}

impl WebhooksCreateInboxWebhookRequestBuilder {
    pub fn url(mut self, value: WebhooksUrl) -> Self {
        self.url = Some(value);
        self
    }

    pub fn event_types(mut self, value: WebhooksCreateWebhookEventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn client_id(mut self, value: WebhooksClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn headers(mut self, value: WebhooksWebhookHeaders) -> Self {
        self.headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksCreateInboxWebhookRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](WebhooksCreateInboxWebhookRequestBuilder::url)
    /// - [`event_types`](WebhooksCreateInboxWebhookRequestBuilder::event_types)
    pub fn build(self) -> Result<WebhooksCreateInboxWebhookRequest, BuildError> {
        Ok(WebhooksCreateInboxWebhookRequest {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            event_types: self.event_types.ok_or_else(|| BuildError::missing_field("event_types"))?,
            client_id: self.client_id,
            headers: self.headers,
        })
    }
}
