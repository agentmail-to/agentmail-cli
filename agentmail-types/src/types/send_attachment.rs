pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<AttachmentFilename>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<AttachmentContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_disposition: Option<AttachmentContentDisposition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<AttachmentContentId>,
    /// Base64 encoded content of attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// URL to the attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SendAttachment {
    pub fn builder() -> SendAttachmentBuilder {
        <SendAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendAttachmentBuilder {
    filename: Option<AttachmentFilename>,
    content_type: Option<AttachmentContentType>,
    content_disposition: Option<AttachmentContentDisposition>,
    content_id: Option<AttachmentContentId>,
    content: Option<String>,
    url: Option<String>,
}

impl SendAttachmentBuilder {
    pub fn filename(mut self, value: AttachmentFilename) -> Self {
        self.filename = Some(value);
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

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendAttachment`].
    pub fn build(self) -> Result<SendAttachment, BuildError> {
        Ok(SendAttachment {
            filename: self.filename,
            content_type: self.content_type,
            content_disposition: self.content_disposition,
            content_id: self.content_id,
            content: self.content,
            url: self.url,
        })
    }
}
