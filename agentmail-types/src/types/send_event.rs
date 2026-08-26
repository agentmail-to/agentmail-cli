pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendEvent {
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub thread_id: ThreadId,
    #[serde(default)]
    pub message_id: MessageId,
    #[serde(default)]
    pub timestamp: Timestamp,
    /// Sent recipients.
    #[serde(default)]
    pub recipients: Vec<String>,
}

impl SendEvent {
    pub fn builder() -> SendEventBuilder {
        <SendEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendEventBuilder {
    inbox_id: Option<InboxesInboxId>,
    thread_id: Option<ThreadId>,
    message_id: Option<MessageId>,
    timestamp: Option<Timestamp>,
    recipients: Option<Vec<String>>,
}

impl SendEventBuilder {
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

    pub fn recipients(mut self, value: Vec<String>) -> Self {
        self.recipients = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](SendEventBuilder::inbox_id)
    /// - [`thread_id`](SendEventBuilder::thread_id)
    /// - [`message_id`](SendEventBuilder::message_id)
    /// - [`timestamp`](SendEventBuilder::timestamp)
    /// - [`recipients`](SendEventBuilder::recipients)
    pub fn build(self) -> Result<SendEvent, BuildError> {
        Ok(SendEvent {
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            recipients: self.recipients.ok_or_else(|| BuildError::missing_field("recipients"))?,
        })
    }
}
