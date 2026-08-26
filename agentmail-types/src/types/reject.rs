pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Reject {
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub thread_id: ThreadId,
    #[serde(default)]
    pub message_id: MessageId,
    #[serde(default)]
    pub timestamp: Timestamp,
    /// Reject reason.
    #[serde(default)]
    pub reason: String,
}

impl Reject {
    pub fn builder() -> RejectBuilder {
        <RejectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RejectBuilder {
    inbox_id: Option<InboxesInboxId>,
    thread_id: Option<ThreadId>,
    message_id: Option<MessageId>,
    timestamp: Option<Timestamp>,
    reason: Option<String>,
}

impl RejectBuilder {
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

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Reject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](RejectBuilder::inbox_id)
    /// - [`thread_id`](RejectBuilder::thread_id)
    /// - [`message_id`](RejectBuilder::message_id)
    /// - [`timestamp`](RejectBuilder::timestamp)
    /// - [`reason`](RejectBuilder::reason)
    pub fn build(self) -> Result<Reject, BuildError> {
        Ok(Reject {
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
