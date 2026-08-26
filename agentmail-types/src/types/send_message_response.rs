pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendMessageResponse {
    #[serde(default)]
    pub message_id: MessageId,
    #[serde(default)]
    pub thread_id: ThreadId,
}

impl SendMessageResponse {
    pub fn builder() -> SendMessageResponseBuilder {
        <SendMessageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendMessageResponseBuilder {
    message_id: Option<MessageId>,
    thread_id: Option<ThreadId>,
}

impl SendMessageResponseBuilder {
    pub fn message_id(mut self, value: MessageId) -> Self {
        self.message_id = Some(value);
        self
    }

    pub fn thread_id(mut self, value: ThreadId) -> Self {
        self.thread_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendMessageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](SendMessageResponseBuilder::message_id)
    /// - [`thread_id`](SendMessageResponseBuilder::thread_id)
    pub fn build(self) -> Result<SendMessageResponse, BuildError> {
        Ok(SendMessageResponse {
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
        })
    }
}
