pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The subscription plus the one-time `token` that reads it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksCreatePollingWebhookResponse {
    #[serde(flatten)]
    pub webhooks_polling_webhook_fields: WebhooksPollingWebhook,
    #[serde(default)]
    pub token: WebhooksPollingToken,
}

impl WebhooksCreatePollingWebhookResponse {
    pub fn builder() -> WebhooksCreatePollingWebhookResponseBuilder {
        <WebhooksCreatePollingWebhookResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksCreatePollingWebhookResponseBuilder {
    webhooks_polling_webhook_fields: Option<WebhooksPollingWebhook>,
    token: Option<WebhooksPollingToken>,
}

impl WebhooksCreatePollingWebhookResponseBuilder {
    pub fn webhooks_polling_webhook_fields(mut self, value: WebhooksPollingWebhook) -> Self {
        self.webhooks_polling_webhook_fields = Some(value);
        self
    }

    pub fn token(mut self, value: WebhooksPollingToken) -> Self {
        self.token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksCreatePollingWebhookResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhooks_polling_webhook_fields`](WebhooksCreatePollingWebhookResponseBuilder::webhooks_polling_webhook_fields)
    /// - [`token`](WebhooksCreatePollingWebhookResponseBuilder::token)
    pub fn build(self) -> Result<WebhooksCreatePollingWebhookResponse, BuildError> {
        Ok(WebhooksCreatePollingWebhookResponse {
            webhooks_polling_webhook_fields: self.webhooks_polling_webhook_fields.ok_or_else(|| BuildError::missing_field("webhooks_polling_webhook_fields"))?,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
