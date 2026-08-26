pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchGetMessagesRequest {
    #[serde(default)]
    pub message_ids: BatchGetMessagesMessageIds,
}

impl BatchGetMessagesRequest {
    pub fn builder() -> BatchGetMessagesRequestBuilder {
        <BatchGetMessagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchGetMessagesRequestBuilder {
    message_ids: Option<BatchGetMessagesMessageIds>,
}

impl BatchGetMessagesRequestBuilder {
    pub fn message_ids(mut self, value: BatchGetMessagesMessageIds) -> Self {
        self.message_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchGetMessagesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_ids`](BatchGetMessagesRequestBuilder::message_ids)
    pub fn build(self) -> Result<BatchGetMessagesRequest, BuildError> {
        Ok(BatchGetMessagesRequest {
            message_ids: self.message_ids.ok_or_else(|| BuildError::missing_field("message_ids"))?,
        })
    }
}

