pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DraftItem {
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub draft_id: DraftId,
    #[serde(default)]
    pub labels: DraftLabels,
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
    pub attachments: Option<DraftAttachments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<DraftInReplyTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_of: Option<DraftForwardOf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_status: Option<DraftSendStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<DraftSendAt>,
    #[serde(default)]
    pub updated_at: DraftUpdatedAt,
}

impl DraftItem {
    pub fn builder() -> DraftItemBuilder {
        <DraftItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DraftItemBuilder {
    inbox_id: Option<InboxesInboxId>,
    draft_id: Option<DraftId>,
    labels: Option<DraftLabels>,
    to: Option<DraftTo>,
    cc: Option<DraftCc>,
    bcc: Option<DraftBcc>,
    subject: Option<DraftSubject>,
    preview: Option<DraftPreview>,
    attachments: Option<DraftAttachments>,
    in_reply_to: Option<DraftInReplyTo>,
    forward_of: Option<DraftForwardOf>,
    send_status: Option<DraftSendStatus>,
    send_at: Option<DraftSendAt>,
    updated_at: Option<DraftUpdatedAt>,
}

impl DraftItemBuilder {
    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn draft_id(mut self, value: DraftId) -> Self {
        self.draft_id = Some(value);
        self
    }

    pub fn labels(mut self, value: DraftLabels) -> Self {
        self.labels = Some(value);
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

    /// Consumes the builder and constructs a [`DraftItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](DraftItemBuilder::inbox_id)
    /// - [`draft_id`](DraftItemBuilder::draft_id)
    /// - [`labels`](DraftItemBuilder::labels)
    /// - [`updated_at`](DraftItemBuilder::updated_at)
    pub fn build(self) -> Result<DraftItem, BuildError> {
        Ok(DraftItem {
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            draft_id: self.draft_id.ok_or_else(|| BuildError::missing_field("draft_id"))?,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            subject: self.subject,
            preview: self.preview,
            attachments: self.attachments,
            in_reply_to: self.in_reply_to,
            forward_of: self.forward_of,
            send_status: self.send_status,
            send_at: self.send_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
