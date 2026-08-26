pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Message {
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub thread_id: ThreadId,
    #[serde(default)]
    pub message_id: MessageId,
    #[serde(default)]
    pub labels: MessageLabels,
    #[serde(default)]
    pub timestamp: MessageTimestamp,
    #[serde(default)]
    pub from: MessageFrom,
    /// Reply-to addresses. In format `username@domain.com` or `Display Name <username@domain.com>`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Vec<String>>,
    #[serde(default)]
    pub to: MessageTo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<MessageCc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<MessageBcc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<MessageSubject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<MessagePreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<MessageText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<MessageHtml>,
    /// Extracted new text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_text: Option<String>,
    /// Extracted new HTML content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<MessageAttachments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<MessageInReplyTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<MessageReferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
    #[serde(default)]
    pub size: MessageSize,
    #[serde(default)]
    pub updated_at: MessageUpdatedAt,
    #[serde(default)]
    pub created_at: MessageCreatedAt,
}

impl Message {
    pub fn builder() -> MessageBuilder {
        <MessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageBuilder {
    inbox_id: Option<InboxesInboxId>,
    thread_id: Option<ThreadId>,
    message_id: Option<MessageId>,
    labels: Option<MessageLabels>,
    timestamp: Option<MessageTimestamp>,
    from: Option<MessageFrom>,
    reply_to: Option<Vec<String>>,
    to: Option<MessageTo>,
    cc: Option<MessageCc>,
    bcc: Option<MessageBcc>,
    subject: Option<MessageSubject>,
    preview: Option<MessagePreview>,
    text: Option<MessageText>,
    html: Option<MessageHtml>,
    extracted_text: Option<String>,
    extracted_html: Option<String>,
    attachments: Option<MessageAttachments>,
    in_reply_to: Option<MessageInReplyTo>,
    references: Option<MessageReferences>,
    headers: Option<MessageHeaders>,
    size: Option<MessageSize>,
    updated_at: Option<MessageUpdatedAt>,
    created_at: Option<MessageCreatedAt>,
}

impl MessageBuilder {
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

    pub fn labels(mut self, value: MessageLabels) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn timestamp(mut self, value: MessageTimestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn from(mut self, value: MessageFrom) -> Self {
        self.from = Some(value);
        self
    }

    pub fn reply_to(mut self, value: Vec<String>) -> Self {
        self.reply_to = Some(value);
        self
    }

    pub fn to(mut self, value: MessageTo) -> Self {
        self.to = Some(value);
        self
    }

    pub fn cc(mut self, value: MessageCc) -> Self {
        self.cc = Some(value);
        self
    }

    pub fn bcc(mut self, value: MessageBcc) -> Self {
        self.bcc = Some(value);
        self
    }

    pub fn subject(mut self, value: MessageSubject) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn preview(mut self, value: MessagePreview) -> Self {
        self.preview = Some(value);
        self
    }

    pub fn text(mut self, value: MessageText) -> Self {
        self.text = Some(value);
        self
    }

    pub fn html(mut self, value: MessageHtml) -> Self {
        self.html = Some(value);
        self
    }

    pub fn extracted_text(mut self, value: impl Into<String>) -> Self {
        self.extracted_text = Some(value.into());
        self
    }

    pub fn extracted_html(mut self, value: impl Into<String>) -> Self {
        self.extracted_html = Some(value.into());
        self
    }

    pub fn attachments(mut self, value: MessageAttachments) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn in_reply_to(mut self, value: MessageInReplyTo) -> Self {
        self.in_reply_to = Some(value);
        self
    }

    pub fn references(mut self, value: MessageReferences) -> Self {
        self.references = Some(value);
        self
    }

    pub fn headers(mut self, value: MessageHeaders) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn size(mut self, value: MessageSize) -> Self {
        self.size = Some(value);
        self
    }

    pub fn updated_at(mut self, value: MessageUpdatedAt) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: MessageCreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Message`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](MessageBuilder::inbox_id)
    /// - [`thread_id`](MessageBuilder::thread_id)
    /// - [`message_id`](MessageBuilder::message_id)
    /// - [`labels`](MessageBuilder::labels)
    /// - [`timestamp`](MessageBuilder::timestamp)
    /// - [`from`](MessageBuilder::from)
    /// - [`to`](MessageBuilder::to)
    /// - [`size`](MessageBuilder::size)
    /// - [`updated_at`](MessageBuilder::updated_at)
    /// - [`created_at`](MessageBuilder::created_at)
    pub fn build(self) -> Result<Message, BuildError> {
        Ok(Message {
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            reply_to: self.reply_to,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            cc: self.cc,
            bcc: self.bcc,
            subject: self.subject,
            preview: self.preview,
            text: self.text,
            html: self.html,
            extracted_text: self.extracted_text,
            extracted_html: self.extracted_html,
            attachments: self.attachments,
            in_reply_to: self.in_reply_to,
            references: self.references,
            headers: self.headers,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
