pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ThreadItem {
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub thread_id: ThreadId,
    #[serde(default)]
    pub labels: ThreadLabels,
    #[serde(default)]
    pub timestamp: ThreadTimestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_timestamp: Option<ThreadReceivedTimestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_timestamp: Option<ThreadSentTimestamp>,
    #[serde(default)]
    pub senders: ThreadSenders,
    #[serde(default)]
    pub recipients: ThreadRecipients,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<ThreadSubject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<ThreadPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<ThreadAttachments>,
    #[serde(default)]
    pub last_message_id: ThreadLastMessageId,
    #[serde(default)]
    pub message_count: ThreadMessageCount,
    #[serde(default)]
    pub size: ThreadSize,
    #[serde(default)]
    pub updated_at: ThreadUpdatedAt,
    #[serde(default)]
    pub created_at: ThreadCreatedAt,
}

impl ThreadItem {
    pub fn builder() -> ThreadItemBuilder {
        <ThreadItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadItemBuilder {
    inbox_id: Option<InboxesInboxId>,
    thread_id: Option<ThreadId>,
    labels: Option<ThreadLabels>,
    timestamp: Option<ThreadTimestamp>,
    received_timestamp: Option<ThreadReceivedTimestamp>,
    sent_timestamp: Option<ThreadSentTimestamp>,
    senders: Option<ThreadSenders>,
    recipients: Option<ThreadRecipients>,
    subject: Option<ThreadSubject>,
    preview: Option<ThreadPreview>,
    attachments: Option<ThreadAttachments>,
    last_message_id: Option<ThreadLastMessageId>,
    message_count: Option<ThreadMessageCount>,
    size: Option<ThreadSize>,
    updated_at: Option<ThreadUpdatedAt>,
    created_at: Option<ThreadCreatedAt>,
}

impl ThreadItemBuilder {
    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn thread_id(mut self, value: ThreadId) -> Self {
        self.thread_id = Some(value);
        self
    }

    pub fn labels(mut self, value: ThreadLabels) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn timestamp(mut self, value: ThreadTimestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn received_timestamp(mut self, value: ThreadReceivedTimestamp) -> Self {
        self.received_timestamp = Some(value);
        self
    }

    pub fn sent_timestamp(mut self, value: ThreadSentTimestamp) -> Self {
        self.sent_timestamp = Some(value);
        self
    }

    pub fn senders(mut self, value: ThreadSenders) -> Self {
        self.senders = Some(value);
        self
    }

    pub fn recipients(mut self, value: ThreadRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn subject(mut self, value: ThreadSubject) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn preview(mut self, value: ThreadPreview) -> Self {
        self.preview = Some(value);
        self
    }

    pub fn attachments(mut self, value: ThreadAttachments) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn last_message_id(mut self, value: ThreadLastMessageId) -> Self {
        self.last_message_id = Some(value);
        self
    }

    pub fn message_count(mut self, value: ThreadMessageCount) -> Self {
        self.message_count = Some(value);
        self
    }

    pub fn size(mut self, value: ThreadSize) -> Self {
        self.size = Some(value);
        self
    }

    pub fn updated_at(mut self, value: ThreadUpdatedAt) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: ThreadCreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThreadItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](ThreadItemBuilder::inbox_id)
    /// - [`thread_id`](ThreadItemBuilder::thread_id)
    /// - [`labels`](ThreadItemBuilder::labels)
    /// - [`timestamp`](ThreadItemBuilder::timestamp)
    /// - [`senders`](ThreadItemBuilder::senders)
    /// - [`recipients`](ThreadItemBuilder::recipients)
    /// - [`last_message_id`](ThreadItemBuilder::last_message_id)
    /// - [`message_count`](ThreadItemBuilder::message_count)
    /// - [`size`](ThreadItemBuilder::size)
    /// - [`updated_at`](ThreadItemBuilder::updated_at)
    /// - [`created_at`](ThreadItemBuilder::created_at)
    pub fn build(self) -> Result<ThreadItem, BuildError> {
        Ok(ThreadItem {
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            received_timestamp: self.received_timestamp,
            sent_timestamp: self.sent_timestamp,
            senders: self.senders.ok_or_else(|| BuildError::missing_field("senders"))?,
            recipients: self.recipients.ok_or_else(|| BuildError::missing_field("recipients"))?,
            subject: self.subject,
            preview: self.preview,
            attachments: self.attachments,
            last_message_id: self.last_message_id.ok_or_else(|| BuildError::missing_field("last_message_id"))?,
            message_count: self.message_count.ok_or_else(|| BuildError::missing_field("message_count"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
