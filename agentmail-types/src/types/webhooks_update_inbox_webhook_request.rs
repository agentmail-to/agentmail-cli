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
    /// to disable it. AgentMail does not notify you when a webhook is disabled automatically; check this
    /// field when you get or list webhooks. Events that occurred while the webhook was disabled are not
    /// redelivered. Re-enabling requires the read permission for every restricted event type the webhook
    /// subscribes to: `label_spam_read`, `label_blocked_read`, or `label_unauthenticated_read` for the
    /// received variants, `message_read` for `message.security.*`, and `calendar_event_read` for
    /// `calendar.event.*`. A webhook with no event type filter is checked against all of them.
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
