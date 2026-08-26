pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDraftRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<DraftLabels>,
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
    /// Attachments to include in draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<SendAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<DraftInReplyTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_of: Option<DraftForwardOf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_all: Option<DraftReplyAll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<DraftSendAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<DraftClientId>,
}

impl CreateDraftRequest {
    pub fn builder() -> CreateDraftRequestBuilder {
        <CreateDraftRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDraftRequestBuilder {
    labels: Option<DraftLabels>,
    reply_to: Option<DraftReplyTo>,
    to: Option<DraftTo>,
    cc: Option<DraftCc>,
    bcc: Option<DraftBcc>,
    subject: Option<DraftSubject>,
    text: Option<DraftText>,
    html: Option<DraftHtml>,
    attachments: Option<Vec<SendAttachment>>,
    in_reply_to: Option<DraftInReplyTo>,
    forward_of: Option<DraftForwardOf>,
    reply_all: Option<DraftReplyAll>,
    send_at: Option<DraftSendAt>,
    client_id: Option<DraftClientId>,
}

impl CreateDraftRequestBuilder {
    pub fn labels(mut self, value: DraftLabels) -> Self {
        self.labels = Some(value);
        self
    }

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

    pub fn attachments(mut self, value: Vec<SendAttachment>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn in_reply_to(mut self, value: DraftInReplyTo) -> Self {
        self.in_reply_to = Some(value);
        self
    }

    pub fn forward_of(mut self, value: DraftForwardOf) -> Self {
        self.forward_of = Some(value);
        self
    }

    pub fn reply_all(mut self, value: DraftReplyAll) -> Self {
        self.reply_all = Some(value);
        self
    }

    pub fn send_at(mut self, value: DraftSendAt) -> Self {
        self.send_at = Some(value);
        self
    }

    pub fn client_id(mut self, value: DraftClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDraftRequest`].
    pub fn build(self) -> Result<CreateDraftRequest, BuildError> {
        Ok(CreateDraftRequest {
            labels: self.labels,
            reply_to: self.reply_to,
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            subject: self.subject,
            text: self.text,
            html: self.html,
            attachments: self.attachments,
            in_reply_to: self.in_reply_to,
            forward_of: self.forward_of,
            reply_all: self.reply_all,
            send_at: self.send_at,
            client_id: self.client_id,
        })
    }
}

