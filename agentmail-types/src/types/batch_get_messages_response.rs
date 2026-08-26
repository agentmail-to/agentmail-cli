pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BatchGetMessagesResponse {
    #[serde(default)]
    pub limit: Limit,
    #[serde(default)]
    pub count: Count,
    /// Found messages. Order matches `message_ids` in the request. Body
    /// fields (`text`, `html`, `extracted_text`, `extracted_html`) are
    /// never populated; use the single-message endpoint to retrieve bodies.
    #[serde(default)]
    pub messages: Vec<Message>,
}

impl BatchGetMessagesResponse {
    pub fn builder() -> BatchGetMessagesResponseBuilder {
        <BatchGetMessagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchGetMessagesResponseBuilder {
    limit: Option<Limit>,
    count: Option<Count>,
    messages: Option<Vec<Message>>,
}

impl BatchGetMessagesResponseBuilder {
    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<Message>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchGetMessagesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`limit`](BatchGetMessagesResponseBuilder::limit)
    /// - [`count`](BatchGetMessagesResponseBuilder::count)
    /// - [`messages`](BatchGetMessagesResponseBuilder::messages)
    pub fn build(self) -> Result<BatchGetMessagesResponse, BuildError> {
        Ok(BatchGetMessagesResponse {
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
        })
    }
}
