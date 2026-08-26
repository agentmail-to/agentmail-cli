pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksWebhookHeaderNamesResponse {
    /// Names of the custom delivery headers configured for this webhook. Header values are never returned.
    #[serde(default)]
    pub header_names: Vec<String>,
}

impl WebhooksWebhookHeaderNamesResponse {
    pub fn builder() -> WebhooksWebhookHeaderNamesResponseBuilder {
        <WebhooksWebhookHeaderNamesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksWebhookHeaderNamesResponseBuilder {
    header_names: Option<Vec<String>>,
}

impl WebhooksWebhookHeaderNamesResponseBuilder {
    pub fn header_names(mut self, value: Vec<String>) -> Self {
        self.header_names = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksWebhookHeaderNamesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`header_names`](WebhooksWebhookHeaderNamesResponseBuilder::header_names)
    pub fn build(self) -> Result<WebhooksWebhookHeaderNamesResponse, BuildError> {
        Ok(WebhooksWebhookHeaderNamesResponse {
            header_names: self.header_names.ok_or_else(|| BuildError::missing_field("header_names"))?,
        })
    }
}
