pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhooksCreateWebhookRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ids: Option<PodIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<InboxIds>,
    #[serde(default)]
    pub url: WebhooksUrl,
    #[serde(default)]
    pub event_types: WebhooksCreateWebhookEventTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<WebhooksClientId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<WebhooksWebhookHeaders>,
}

impl WebhooksCreateWebhookRequest {
    pub fn builder() -> WebhooksCreateWebhookRequestBuilder {
        <WebhooksCreateWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksCreateWebhookRequestBuilder {
    pod_ids: Option<PodIds>,
    inbox_ids: Option<InboxIds>,
    url: Option<WebhooksUrl>,
    event_types: Option<WebhooksCreateWebhookEventTypes>,
    client_id: Option<WebhooksClientId>,
    headers: Option<WebhooksWebhookHeaders>,
}

impl WebhooksCreateWebhookRequestBuilder {
    pub fn pod_ids(mut self, value: PodIds) -> Self {
        self.pod_ids = Some(value);
        self
    }

    pub fn inbox_ids(mut self, value: InboxIds) -> Self {
        self.inbox_ids = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`WebhooksCreateWebhookRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](WebhooksCreateWebhookRequestBuilder::url)
    /// - [`event_types`](WebhooksCreateWebhookRequestBuilder::event_types)
    pub fn build(self) -> Result<WebhooksCreateWebhookRequest, BuildError> {
        Ok(WebhooksCreateWebhookRequest {
            pod_ids: self.pod_ids,
            inbox_ids: self.inbox_ids,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            event_types: self.event_types.ok_or_else(|| BuildError::missing_field("event_types"))?,
            client_id: self.client_id,
            headers: self.headers,
        })
    }
}

