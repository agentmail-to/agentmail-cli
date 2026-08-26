pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDraftRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<DraftReplyTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DraftTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<DraftCc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<DraftBcc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<DraftSubject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<DraftText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<DraftHtml>,
    /// Attachments to add to the draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_attachments: Option<Vec<SendAttachment>>,
    /// IDs of attachments to remove from the draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_attachments: Option<Vec<AttachmentId>>,
    /// Label or labels to add to the draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_labels: Option<DraftLabels>,
    /// Label or labels to remove from the draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_labels: Option<DraftLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<DraftSendAt>,
}

impl UpdateDraftRequest {
    pub fn builder() -> UpdateDraftRequestBuilder {
        <UpdateDraftRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDraftRequestBuilder {
    reply_to: Option<DraftReplyTo>,
    to: Option<DraftTo>,
    cc: Option<DraftCc>,
    bcc: Option<DraftBcc>,
    subject: Option<DraftSubject>,
    text: Option<DraftText>,
    html: Option<DraftHtml>,
    add_attachments: Option<Vec<SendAttachment>>,
    remove_attachments: Option<Vec<AttachmentId>>,
    add_labels: Option<DraftLabels>,
    remove_labels: Option<DraftLabels>,
    send_at: Option<DraftSendAt>,
}

impl UpdateDraftRequestBuilder {
    pub fn reply_to(mut self, value: DraftReplyTo) -> Self {
        self.reply_to = Some(value);
        self
    }

    pub fn to(mut self, value: DraftTo) -> Self {
        self.to = Some(value);
        self
    }

    pub fn cc(mut self, value: DraftCc) -> Self {
        self.cc = Some(value);
        self
    }

    pub fn bcc(mut self, value: DraftBcc) -> Self {
        self.bcc = Some(value);
        self
    }

    pub fn subject(mut self, value: DraftSubject) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn text(mut self, value: DraftText) -> Self {
        self.text = Some(value);
        self
    }

    pub fn html(mut self, value: DraftHtml) -> Self {
        self.html = Some(value);
        self
    }

    pub fn add_attachments(mut self, value: Vec<SendAttachment>) -> Self {
        self.add_attachments = Some(value);
        self
    }

    pub fn remove_attachments(mut self, value: Vec<AttachmentId>) -> Self {
        self.remove_attachments = Some(value);
        self
    }

    pub fn add_labels(mut self, value: DraftLabels) -> Self {
        self.add_labels = Some(value);
        self
    }

    pub fn remove_labels(mut self, value: DraftLabels) -> Self {
        self.remove_labels = Some(value);
        self
    }

    pub fn send_at(mut self, value: DraftSendAt) -> Self {
        self.send_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDraftRequest`].
    pub fn build(self) -> Result<UpdateDraftRequest, BuildError> {
        Ok(UpdateDraftRequest {
            reply_to: self.reply_to,
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            subject: self.subject,
            text: self.text,
            html: self.html,
            add_attachments: self.add_attachments,
            remove_attachments: self.remove_attachments,
            add_labels: self.add_labels,
            remove_labels: self.remove_labels,
            send_at: self.send_at,
        })
    }
}

