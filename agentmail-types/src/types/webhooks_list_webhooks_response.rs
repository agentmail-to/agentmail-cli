pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksListWebhooksResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by `created_at` descending.
    #[serde(default)]
    pub webhooks: Vec<WebhooksWebhook>,
}

impl WebhooksListWebhooksResponse {
    pub fn builder() -> WebhooksListWebhooksResponseBuilder {
        <WebhooksListWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksListWebhooksResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    webhooks: Option<Vec<WebhooksWebhook>>,
}

impl WebhooksListWebhooksResponseBuilder {
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

    pub fn webhooks(mut self, value: Vec<WebhooksWebhook>) -> Self {
        self.webhooks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksListWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](WebhooksListWebhooksResponseBuilder::count)
    /// - [`webhooks`](WebhooksListWebhooksResponseBuilder::webhooks)
    pub fn build(self) -> Result<WebhooksListWebhooksResponse, BuildError> {
        Ok(WebhooksListWebhooksResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            webhooks: self.webhooks.ok_or_else(|| BuildError::missing_field("webhooks"))?,
        })
    }
}
