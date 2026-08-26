pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMessagesResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by `timestamp` descending.
    #[serde(default)]
    pub messages: Vec<MessageItem>,
}

impl ListMessagesResponse {
    pub fn builder() -> ListMessagesResponseBuilder {
        <ListMessagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMessagesResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    messages: Option<Vec<MessageItem>>,
}

impl ListMessagesResponseBuilder {
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

    pub fn messages(mut self, value: Vec<MessageItem>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMessagesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListMessagesResponseBuilder::count)
    /// - [`messages`](ListMessagesResponseBuilder::messages)
    pub fn build(self) -> Result<ListMessagesResponse, BuildError> {
        Ok(ListMessagesResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
        })
    }
}
