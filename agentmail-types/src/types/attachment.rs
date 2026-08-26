pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Attachment {
    #[serde(default)]
    pub attachment_id: AttachmentId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<AttachmentFilename>,
    #[serde(default)]
    pub size: AttachmentSize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<AttachmentContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_disposition: Option<AttachmentContentDisposition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<AttachmentContentId>,
}

impl Attachment {
    pub fn builder() -> AttachmentBuilder {
        <AttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachmentBuilder {
    attachment_id: Option<AttachmentId>,
    filename: Option<AttachmentFilename>,
    size: Option<AttachmentSize>,
    content_type: Option<AttachmentContentType>,
    content_disposition: Option<AttachmentContentDisposition>,
    content_id: Option<AttachmentContentId>,
}

impl AttachmentBuilder {
    pub fn attachment_id(mut self, value: AttachmentId) -> Self {
        self.attachment_id = Some(value);
        self
    }

    pub fn filename(mut self, value: AttachmentFilename) -> Self {
        self.filename = Some(value);
        self
    }

    pub fn size(mut self, value: AttachmentSize) -> Self {
        self.size = Some(value);
        self
    }

    pub fn content_type(mut self, value: AttachmentContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    pub fn content_disposition(mut self, value: AttachmentContentDisposition) -> Self {
        self.content_disposition = Some(value);
        self
    }

    pub fn content_id(mut self, value: AttachmentContentId) -> Self {
        self.content_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Attachment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachment_id`](AttachmentBuilder::attachment_id)
    /// - [`size`](AttachmentBuilder::size)
    pub fn build(self) -> Result<Attachment, BuildError> {
        Ok(Attachment {
            attachment_id: self.attachment_id.ok_or_else(|| BuildError::missing_field("attachment_id"))?,
            filename: self.filename,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
            content_type: self.content_type,
            content_disposition: self.content_disposition,
            content_id: self.content_id,
        })
    }
}
