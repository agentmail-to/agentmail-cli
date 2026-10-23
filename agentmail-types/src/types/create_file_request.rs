pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFileRequest {
    #[serde(default)]
    pub path: FilePath,
    #[serde(default)]
    pub size: FileSize,
    #[serde(default)]
    pub sha256: FileSha256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<FileContentType>,
    /// When the file expires, in the future. Rounded up to a whole second and returned in UTC. On a retry, send
    /// the `expires_at` the first response returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<FileExpiresAt>,
}

impl CreateFileRequest {
    pub fn builder() -> CreateFileRequestBuilder {
        <CreateFileRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFileRequestBuilder {
    path: Option<FilePath>,
    size: Option<FileSize>,
    sha256: Option<FileSha256>,
    content_type: Option<FileContentType>,
    expires_at: Option<FileExpiresAt>,
}

impl CreateFileRequestBuilder {
    pub fn path(mut self, value: FilePath) -> Self {
        self.path = Some(value);
        self
    }

    pub fn size(mut self, value: FileSize) -> Self {
        self.size = Some(value);
        self
    }

    pub fn sha256(mut self, value: FileSha256) -> Self {
        self.sha256 = Some(value);
        self
    }

    pub fn content_type(mut self, value: FileContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    pub fn expires_at(mut self, value: FileExpiresAt) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateFileRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](CreateFileRequestBuilder::path)
    /// - [`size`](CreateFileRequestBuilder::size)
    /// - [`sha256`](CreateFileRequestBuilder::sha256)
    pub fn build(self) -> Result<CreateFileRequest, BuildError> {
        Ok(CreateFileRequest {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
            sha256: self.sha256.ok_or_else(|| BuildError::missing_field("sha256"))?,
            content_type: self.content_type,
            expires_at: self.expires_at,
        })
    }
}

