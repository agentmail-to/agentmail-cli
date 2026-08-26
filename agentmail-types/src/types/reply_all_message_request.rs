pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReplyAllMessageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<MessageLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<SendMessageReplyTo>,
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

impl ReplyAllMessageRequest {
    pub fn builder() -> ReplyAllMessageRequestBuilder {
        <ReplyAllMessageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplyAllMessageRequestBuilder {
    labels: Option<MessageLabels>,
    reply_to: Option<SendMessageReplyTo>,
    text: Option<MessageText>,
    html: Option<MessageHtml>,
    attachments: Option<SendMessageAttachments>,
    headers: Option<SendMessageHeaders>,
    track_opens: Option<TrackOpens>,
}

impl ReplyAllMessageRequestBuilder {
    pub fn labels(mut self, value: MessageLabels) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn reply_to(mut self, value: SendMessageReplyTo) -> Self {
        self.reply_to = Some(value);
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

    /// Consumes the builder and constructs a [`ReplyAllMessageRequest`].
    pub fn build(self) -> Result<ReplyAllMessageRequest, BuildError> {
        Ok(ReplyAllMessageRequest {
            labels: self.labels,
            reply_to: self.reply_to,
            text: self.text,
            html: self.html,
            attachments: self.attachments,
            headers: self.headers,
            track_opens: self.track_opens,
        })
    }
}

