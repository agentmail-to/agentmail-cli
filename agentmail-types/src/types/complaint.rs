pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Complaint {
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub thread_id: ThreadId,
    #[serde(default)]
    pub message_id: MessageId,
    #[serde(default)]
    pub timestamp: Timestamp,
    /// Complaint type.
    #[serde(default)]
    pub r#type: String,
    /// Complaint sub-type.
    #[serde(default)]
    pub sub_type: String,
    /// Complained recipients.
    #[serde(default)]
    pub recipients: Vec<String>,
}

impl Complaint {
    pub fn builder() -> ComplaintBuilder {
        <ComplaintBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComplaintBuilder {
    inbox_id: Option<InboxesInboxId>,
    thread_id: Option<ThreadId>,
    message_id: Option<MessageId>,
    timestamp: Option<Timestamp>,
    r#type: Option<String>,
    sub_type: Option<String>,
    recipients: Option<Vec<String>>,
}

impl ComplaintBuilder {
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

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn sub_type(mut self, value: impl Into<String>) -> Self {
        self.sub_type = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: Vec<String>) -> Self {
        self.recipients = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Complaint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](ComplaintBuilder::inbox_id)
    /// - [`thread_id`](ComplaintBuilder::thread_id)
    /// - [`message_id`](ComplaintBuilder::message_id)
    /// - [`timestamp`](ComplaintBuilder::timestamp)
    /// - [`r#type`](ComplaintBuilder::r#type)
    /// - [`sub_type`](ComplaintBuilder::sub_type)
    /// - [`recipients`](ComplaintBuilder::recipients)
    pub fn build(self) -> Result<Complaint, BuildError> {
        Ok(Complaint {
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            sub_type: self.sub_type.ok_or_else(|| BuildError::missing_field("sub_type"))?,
            recipients: self.recipients.ok_or_else(|| BuildError::missing_field("recipients"))?,
        })
    }
}
