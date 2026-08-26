pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksWebhook {
    #[serde(default)]
    pub webhook_id: WebhooksWebhookId,
    #[serde(default)]
    pub url: WebhooksUrl,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<EventTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ids: Option<PodIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<InboxIds>,
    /// Secret for webhook signature verification.
    #[serde(default)]
    pub secret: String,
    /// Webhook is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Time at which webhook was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Time at which webhook was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<WebhooksClientId>,
}

impl WebhooksWebhook {
    pub fn builder() -> WebhooksWebhookBuilder {
        <WebhooksWebhookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksWebhookBuilder {
    webhook_id: Option<WebhooksWebhookId>,
    url: Option<WebhooksUrl>,
    event_types: Option<EventTypes>,
    pod_ids: Option<PodIds>,
    inbox_ids: Option<InboxIds>,
    secret: Option<String>,
    enabled: Option<bool>,
    updated_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    client_id: Option<WebhooksClientId>,
}

impl WebhooksWebhookBuilder {
    pub fn webhook_id(mut self, value: WebhooksWebhookId) -> Self {
        self.webhook_id = Some(value);
        self
    }

    pub fn url(mut self, value: WebhooksUrl) -> Self {
        self.url = Some(value);
        self
    }

    pub fn event_types(mut self, value: EventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn pod_ids(mut self, value: PodIds) -> Self {
        self.pod_ids = Some(value);
        self
    }

    pub fn inbox_ids(mut self, value: InboxIds) -> Self {
        self.inbox_ids = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn client_id(mut self, value: WebhooksClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksWebhook`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhook_id`](WebhooksWebhookBuilder::webhook_id)
    /// - [`url`](WebhooksWebhookBuilder::url)
    /// - [`secret`](WebhooksWebhookBuilder::secret)
    /// - [`enabled`](WebhooksWebhookBuilder::enabled)
    /// - [`updated_at`](WebhooksWebhookBuilder::updated_at)
    /// - [`created_at`](WebhooksWebhookBuilder::created_at)
    pub fn build(self) -> Result<WebhooksWebhook, BuildError> {
        Ok(WebhooksWebhook {
            webhook_id: self.webhook_id.ok_or_else(|| BuildError::missing_field("webhook_id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            event_types: self.event_types,
            pod_ids: self.pod_ids,
            inbox_ids: self.inbox_ids,
            secret: self.secret.ok_or_else(|| BuildError::missing_field("secret"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            client_id: self.client_id,
        })
    }
}
