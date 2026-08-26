pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Draft {
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub draft_id: DraftId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<DraftClientId>,
    #[serde(default)]
    pub labels: DraftLabels,
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
    pub preview: Option<DraftPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<DraftText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<DraftHtml>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<DraftAttachments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<DraftInReplyTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_of: Option<DraftForwardOf>,
    /// IDs of previous messages in thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_status: Option<DraftSendStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<DraftSendAt>,
    #[serde(default)]
    pub updated_at: DraftUpdatedAt,
    /// Time at which draft was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl Draft {
    pub fn builder() -> DraftBuilder {
        <DraftBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DraftBuilder {
    inbox_id: Option<InboxesInboxId>,
    draft_id: Option<DraftId>,
    client_id: Option<DraftClientId>,
    labels: Option<DraftLabels>,
    reply_to: Option<DraftReplyTo>,
    to: Option<DraftTo>,
    cc: Option<DraftCc>,
    bcc: Option<DraftBcc>,
    subject: Option<DraftSubject>,
    preview: Option<DraftPreview>,
    text: Option<DraftText>,
    html: Option<DraftHtml>,
    attachments: Option<DraftAttachments>,
    in_reply_to: Option<DraftInReplyTo>,
    forward_of: Option<DraftForwardOf>,
    references: Option<Vec<String>>,
    send_status: Option<DraftSendStatus>,
    send_at: Option<DraftSendAt>,
    updated_at: Option<DraftUpdatedAt>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl DraftBuilder {
    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn draft_id(mut self, value: DraftId) -> Self {
        self.draft_id = Some(value);
        self
    }

    pub fn client_id(mut self, value: DraftClientId) -> Self {
        self.client_id = Some(value);
        self
    }

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

    pub fn preview(mut self, value: DraftPreview) -> Self {
        self.preview = Some(value);
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

    pub fn attachments(mut self, value: DraftAttachments) -> Self {
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

    pub fn references(mut self, value: Vec<String>) -> Self {
        self.references = Some(value);
        self
    }

    pub fn send_status(mut self, value: DraftSendStatus) -> Self {
        self.send_status = Some(value);
        self
    }

    pub fn send_at(mut self, value: DraftSendAt) -> Self {
        self.send_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DraftUpdatedAt) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Draft`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](DraftBuilder::inbox_id)
    /// - [`draft_id`](DraftBuilder::draft_id)
    /// - [`labels`](DraftBuilder::labels)
    /// - [`updated_at`](DraftBuilder::updated_at)
    /// - [`created_at`](DraftBuilder::created_at)
    pub fn build(self) -> Result<Draft, BuildError> {
        Ok(Draft {
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            draft_id: self.draft_id.ok_or_else(|| BuildError::missing_field("draft_id"))?,
            client_id: self.client_id,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
            reply_to: self.reply_to,
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            subject: self.subject,
            preview: self.preview,
            text: self.text,
            html: self.html,
            attachments: self.attachments,
            in_reply_to: self.in_reply_to,
            forward_of: self.forward_of,
            references: self.references,
            send_status: self.send_status,
            send_at: self.send_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
