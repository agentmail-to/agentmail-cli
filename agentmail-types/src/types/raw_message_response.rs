pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// S3 presigned URL to download the raw .eml file.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RawMessageResponse {
    /// ID of the message.
    #[serde(default)]
    pub message_id: MessageId,
    /// Size of the raw message in bytes.
    #[serde(default)]
    pub size: MessageSize,
    /// S3 presigned URL to download the raw message. Expires at expires_at.
    #[serde(default)]
    pub download_url: String,
    /// Time at which the download URL expires.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl RawMessageResponse {
    pub fn builder() -> RawMessageResponseBuilder {
        <RawMessageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RawMessageResponseBuilder {
    message_id: Option<MessageId>,
    size: Option<MessageSize>,
    download_url: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl RawMessageResponseBuilder {
    pub fn message_id(mut self, value: MessageId) -> Self {
        self.message_id = Some(value);
        self
    }

    pub fn size(mut self, value: MessageSize) -> Self {
        self.size = Some(value);
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

    /// Consumes the builder and constructs a [`RawMessageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](RawMessageResponseBuilder::message_id)
    /// - [`size`](RawMessageResponseBuilder::size)
    /// - [`download_url`](RawMessageResponseBuilder::download_url)
    /// - [`expires_at`](RawMessageResponseBuilder::expires_at)
    pub fn build(self) -> Result<RawMessageResponse, BuildError> {
        Ok(RawMessageResponse {
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
            download_url: self.download_url.ok_or_else(|| BuildError::missing_field("download_url"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
