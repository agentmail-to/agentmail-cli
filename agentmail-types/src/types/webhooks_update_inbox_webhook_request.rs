pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Update an inbox-scoped webhook. It is fixed to its inbox, so only `event_types` and `enabled` can
/// change.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksUpdateInboxWebhookRequest {
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

impl WebhooksUpdateInboxWebhookRequest {
    pub fn builder() -> WebhooksUpdateInboxWebhookRequestBuilder {
        <WebhooksUpdateInboxWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksUpdateInboxWebhookRequestBuilder {
    event_types: Option<WebhooksUpdateWebhookEventTypes>,
    enabled: Option<bool>,
}

impl WebhooksUpdateInboxWebhookRequestBuilder {
    pub fn event_types(mut self, value: WebhooksUpdateWebhookEventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksUpdateInboxWebhookRequest`].
    pub fn build(self) -> Result<WebhooksUpdateInboxWebhookRequest, BuildError> {
        Ok(WebhooksUpdateInboxWebhookRequest {
            event_types: self.event_types,
            enabled: self.enabled,
        })
    }
}
