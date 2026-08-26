pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Open {
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub thread_id: ThreadId,
    #[serde(default)]
    pub message_id: MessageId,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl Open {
    pub fn builder() -> OpenBuilder {
        <OpenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenBuilder {
    inbox_id: Option<InboxesInboxId>,
    thread_id: Option<ThreadId>,
    message_id: Option<MessageId>,
    timestamp: Option<Timestamp>,
}

impl OpenBuilder {
    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn thread_id(mut self, value: ThreadId) -> Self {
        self.thread_id = Some(value);
        self
    }

    pub fn message_id(mut self, value: MessageId) -> Self {
        self.message_id = Some(value);
        self
    }

    pub fn timestamp(mut self, value: Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Open`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](OpenBuilder::inbox_id)
    /// - [`thread_id`](OpenBuilder::thread_id)
    /// - [`message_id`](OpenBuilder::message_id)
    /// - [`timestamp`](OpenBuilder::timestamp)
    pub fn build(self) -> Result<Open, BuildError> {
        Ok(Open {
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
        })
    }
}
