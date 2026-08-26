pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendMessageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<MessageLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<SendMessageReplyTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<SendMessageTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<SendMessageCc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<SendMessageBcc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<MessageSubject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<MessageText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<MessageHtml>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<SendMessageAttachments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<SendMessageHeaders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_opens: Option<TrackOpens>,
}

impl SendMessageRequest {
    pub fn builder() -> SendMessageRequestBuilder {
        <SendMessageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendMessageRequestBuilder {
    labels: Option<MessageLabels>,
    reply_to: Option<SendMessageReplyTo>,
    to: Option<SendMessageTo>,
    cc: Option<SendMessageCc>,
    bcc: Option<SendMessageBcc>,
    subject: Option<MessageSubject>,
    text: Option<MessageText>,
    html: Option<MessageHtml>,
    attachments: Option<SendMessageAttachments>,
    headers: Option<SendMessageHeaders>,
    track_opens: Option<TrackOpens>,
}

impl SendMessageRequestBuilder {
    pub fn labels(mut self, value: MessageLabels) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn reply_to(mut self, value: SendMessageReplyTo) -> Self {
        self.reply_to = Some(value);
        self
    }

    pub fn to(mut self, value: SendMessageTo) -> Self {
        self.to = Some(value);
        self
    }

    pub fn cc(mut self, value: SendMessageCc) -> Self {
        self.cc = Some(value);
        self
    }

    pub fn bcc(mut self, value: SendMessageBcc) -> Self {
        self.bcc = Some(value);
        self
    }

    pub fn subject(mut self, value: MessageSubject) -> Self {
        self.subject = Some(value);
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

    pub fn attachments(mut self, value: SendMessageAttachments) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn headers(mut self, value: SendMessageHeaders) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn track_opens(mut self, value: TrackOpens) -> Self {
        self.track_opens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendMessageRequest`].
    pub fn build(self) -> Result<SendMessageRequest, BuildError> {
        Ok(SendMessageRequest {
            labels: self.labels,
            reply_to: self.reply_to,
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            subject: self.subject,
            text: self.text,
            html: self.html,
            attachments: self.attachments,
            headers: self.headers,
            track_opens: self.track_opens,
        })
    }
}
