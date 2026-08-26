pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Set, replace, or remove custom delivery headers. Provide at least one of `headers` or
/// `remove_headers`. A header cannot be set and removed in the same request, regardless of casing.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhooksUpdateWebhookHeadersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<WebhooksWebhookHeaders>,
    /// Names of custom delivery headers to remove.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_headers: Option<Vec<String>>,
}

impl WebhooksUpdateWebhookHeadersRequest {
    pub fn builder() -> WebhooksUpdateWebhookHeadersRequestBuilder {
        <WebhooksUpdateWebhookHeadersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksUpdateWebhookHeadersRequestBuilder {
    headers: Option<WebhooksWebhookHeaders>,
    remove_headers: Option<Vec<String>>,
}

impl WebhooksUpdateWebhookHeadersRequestBuilder {
    pub fn headers(mut self, value: WebhooksWebhookHeaders) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn remove_headers(mut self, value: Vec<String>) -> Self {
        self.remove_headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksUpdateWebhookHeadersRequest`].
    pub fn build(self) -> Result<WebhooksUpdateWebhookHeadersRequest, BuildError> {
        Ok(WebhooksUpdateWebhookHeadersRequest {
            headers: self.headers,
            remove_headers: self.remove_headers,
        })
    }
}
