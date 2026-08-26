pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMessageResponse {
    #[serde(default)]
    pub message_id: MessageId,
    #[serde(default)]
    pub labels: MessageLabels,
}

impl UpdateMessageResponse {
    pub fn builder() -> UpdateMessageResponseBuilder {
        <UpdateMessageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMessageResponseBuilder {
    message_id: Option<MessageId>,
    labels: Option<MessageLabels>,
}

impl UpdateMessageResponseBuilder {
    pub fn message_id(mut self, value: MessageId) -> Self {
        self.message_id = Some(value);
        self
    }

    pub fn labels(mut self, value: MessageLabels) -> Self {
        self.labels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMessageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](UpdateMessageResponseBuilder::message_id)
    /// - [`labels`](UpdateMessageResponseBuilder::labels)
    pub fn build(self) -> Result<UpdateMessageResponse, BuildError> {
        Ok(UpdateMessageResponse {
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
        })
    }
}
