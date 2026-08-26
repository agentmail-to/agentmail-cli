pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AttachmentResponse {
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
    /// URL to download the attachment.
    #[serde(default)]
    pub download_url: String,
    /// Time at which the download URL expires.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl AttachmentResponse {
    pub fn builder() -> AttachmentResponseBuilder {
        <AttachmentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachmentResponseBuilder {
    attachment_id: Option<AttachmentId>,
    filename: Option<AttachmentFilename>,
    size: Option<AttachmentSize>,
    content_type: Option<AttachmentContentType>,
    content_disposition: Option<AttachmentContentDisposition>,
    content_id: Option<AttachmentContentId>,
    download_url: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl AttachmentResponseBuilder {
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

    pub fn download_url(mut self, value: impl Into<String>) -> Self {
        self.download_url = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AttachmentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachment_id`](AttachmentResponseBuilder::attachment_id)
    /// - [`size`](AttachmentResponseBuilder::size)
    /// - [`download_url`](AttachmentResponseBuilder::download_url)
    /// - [`expires_at`](AttachmentResponseBuilder::expires_at)
    pub fn build(self) -> Result<AttachmentResponse, BuildError> {
        Ok(AttachmentResponse {
            attachment_id: self.attachment_id.ok_or_else(|| BuildError::missing_field("attachment_id"))?,
            filename: self.filename,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
            content_type: self.content_type,
            content_disposition: self.content_disposition,
            content_id: self.content_id,
            download_url: self.download_url.ok_or_else(|| BuildError::missing_field("download_url"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
