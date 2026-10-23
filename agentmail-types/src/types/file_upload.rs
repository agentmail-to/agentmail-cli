pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Where to upload a pending file's bytes: one HTTP `PUT` to `url` with exactly `headers` and the file's bytes as
/// the body. Every header is signed, so S3 refuses the upload if any is missing or different, and refuses bytes
/// whose size or SHA-256 doesn't match the file. Then call Complete File.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileUpload {
    /// Always `PUT`.
    pub method: FileUploadMethod,
    /// The presigned S3 URL. No API key is needed to use it.
    #[serde(default)]
    pub url: String,
    /// The headers to send with the `PUT`, exactly as given.
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// When `url` stops working, at most an hour after it was issued. Send the identical create request again for a fresh one.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl FileUpload {
    pub fn builder() -> FileUploadBuilder {
        <FileUploadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileUploadBuilder {
    method: Option<FileUploadMethod>,
    url: Option<String>,
    headers: Option<HashMap<String, String>>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl FileUploadBuilder {
    pub fn method(mut self, value: FileUploadMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileUpload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](FileUploadBuilder::method)
    /// - [`url`](FileUploadBuilder::url)
    /// - [`headers`](FileUploadBuilder::headers)
    /// - [`expires_at`](FileUploadBuilder::expires_at)
    pub fn build(self) -> Result<FileUpload, BuildError> {
        Ok(FileUpload {
            method: self.method.ok_or_else(|| BuildError::missing_field("method"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            headers: self.headers.ok_or_else(|| BuildError::missing_field("headers"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
