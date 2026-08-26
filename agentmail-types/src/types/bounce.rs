pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Bounce {
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub thread_id: ThreadId,
    #[serde(default)]
    pub message_id: MessageId,
    #[serde(default)]
    pub timestamp: Timestamp,
    /// Bounce type.
    #[serde(default)]
    pub r#type: String,
    /// Bounce sub-type.
    #[serde(default)]
    pub sub_type: String,
    /// Bounced recipients.
    #[serde(default)]
    pub recipients: Vec<Recipient>,
}

impl Bounce {
    pub fn builder() -> BounceBuilder {
        <BounceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BounceBuilder {
    inbox_id: Option<InboxesInboxId>,
    thread_id: Option<ThreadId>,
    message_id: Option<MessageId>,
    timestamp: Option<Timestamp>,
    r#type: Option<String>,
    sub_type: Option<String>,
    recipients: Option<Vec<Recipient>>,
}

impl BounceBuilder {
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

    pub fn recipients(mut self, value: Vec<Recipient>) -> Self {
        self.recipients = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Bounce`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](BounceBuilder::inbox_id)
    /// - [`thread_id`](BounceBuilder::thread_id)
    /// - [`message_id`](BounceBuilder::message_id)
    /// - [`timestamp`](BounceBuilder::timestamp)
    /// - [`r#type`](BounceBuilder::r#type)
    /// - [`sub_type`](BounceBuilder::sub_type)
    /// - [`recipients`](BounceBuilder::recipients)
    pub fn build(self) -> Result<Bounce, BuildError> {
        Ok(Bounce {
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
