pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksListPollingWebhooksResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by `created_at` descending. Lists the subscriptions created through this route
    /// (organization, pod, or inbox); subscriptions created through a pod or inbox route are listed
    /// through that route.
    #[serde(default)]
    pub polling_webhooks: Vec<WebhooksPollingWebhook>,
}

impl WebhooksListPollingWebhooksResponse {
    pub fn builder() -> WebhooksListPollingWebhooksResponseBuilder {
        <WebhooksListPollingWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksListPollingWebhooksResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    polling_webhooks: Option<Vec<WebhooksPollingWebhook>>,
}

impl WebhooksListPollingWebhooksResponseBuilder {
    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: PageToken) -> Self {
        self.next_page_token = Some(value);
        self
    }

    pub fn polling_webhooks(mut self, value: Vec<WebhooksPollingWebhook>) -> Self {
        self.polling_webhooks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksListPollingWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](WebhooksListPollingWebhooksResponseBuilder::count)
    /// - [`polling_webhooks`](WebhooksListPollingWebhooksResponseBuilder::polling_webhooks)
    pub fn build(self) -> Result<WebhooksListPollingWebhooksResponse, BuildError> {
        Ok(WebhooksListPollingWebhooksResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            polling_webhooks: self.polling_webhooks.ok_or_else(|| BuildError::missing_field("polling_webhooks"))?,
        })
    }
}
