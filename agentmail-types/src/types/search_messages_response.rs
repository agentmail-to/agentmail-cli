pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchMessagesResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by relevance, best match first.
    #[serde(default)]
    pub messages: Vec<SearchMessageItem>,
}

impl SearchMessagesResponse {
    pub fn builder() -> SearchMessagesResponseBuilder {
        <SearchMessagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchMessagesResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    messages: Option<Vec<SearchMessageItem>>,
}

impl SearchMessagesResponseBuilder {
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

    pub fn messages(mut self, value: Vec<SearchMessageItem>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchMessagesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](SearchMessagesResponseBuilder::count)
    /// - [`messages`](SearchMessagesResponseBuilder::messages)
    pub fn build(self) -> Result<SearchMessagesResponse, BuildError> {
        Ok(SearchMessagesResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
        })
    }
}
