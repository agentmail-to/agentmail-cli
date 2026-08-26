pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchUpdateMessagesRequest {
    #[serde(default)]
    pub message_ids: BatchUpdateMessagesMessageIds,
    /// Label or labels to add to every message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_labels: Option<UpdateMessageLabels>,
    /// Label or labels to remove from every message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_labels: Option<UpdateMessageLabels>,
}

impl BatchUpdateMessagesRequest {
    pub fn builder() -> BatchUpdateMessagesRequestBuilder {
        <BatchUpdateMessagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchUpdateMessagesRequestBuilder {
    message_ids: Option<BatchUpdateMessagesMessageIds>,
    add_labels: Option<UpdateMessageLabels>,
    remove_labels: Option<UpdateMessageLabels>,
}

impl BatchUpdateMessagesRequestBuilder {
    pub fn message_ids(mut self, value: BatchUpdateMessagesMessageIds) -> Self {
        self.message_ids = Some(value);
        self
    }

    pub fn add_labels(mut self, value: UpdateMessageLabels) -> Self {
        self.add_labels = Some(value);
        self
    }

    pub fn remove_labels(mut self, value: UpdateMessageLabels) -> Self {
        self.remove_labels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchUpdateMessagesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_ids`](BatchUpdateMessagesRequestBuilder::message_ids)
    pub fn build(self) -> Result<BatchUpdateMessagesRequest, BuildError> {
        Ok(BatchUpdateMessagesRequest {
            message_ids: self.message_ids.ok_or_else(|| BuildError::missing_field("message_ids"))?,
            add_labels: self.add_labels,
            remove_labels: self.remove_labels,
        })
    }
}

