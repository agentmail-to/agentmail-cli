pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Update an inbox-scoped webhook. It is fixed to its inbox, so only `event_types` can change.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksUpdateInboxWebhookRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<WebhooksUpdateWebhookEventTypes>,
}

impl WebhooksUpdateInboxWebhookRequest {
    pub fn builder() -> WebhooksUpdateInboxWebhookRequestBuilder {
        <WebhooksUpdateInboxWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksUpdateInboxWebhookRequestBuilder {
    event_types: Option<WebhooksUpdateWebhookEventTypes>,
}

impl WebhooksUpdateInboxWebhookRequestBuilder {
    pub fn event_types(mut self, value: WebhooksUpdateWebhookEventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksUpdateInboxWebhookRequest`].
    pub fn build(self) -> Result<WebhooksUpdateInboxWebhookRequest, BuildError> {
        Ok(WebhooksUpdateInboxWebhookRequest {
            event_types: self.event_types,
        })
    }
}
