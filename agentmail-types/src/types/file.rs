pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A file. A `ready` file returned to a caller with the `file_read` permission carries a signed `download_url`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct File {
    #[serde(flatten)]
    pub file_item_fields: FileItem,
    /// A signed URL that downloads the file's bytes, named by `name`. No API key is needed to use it, so treat it
    /// as a secret. It works until `download_expires_at`, at most an hour, even after the file is deleted. Get the
    /// file again for a fresh one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// When `download_url` stops working. Never after the file's own expiry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_expires_at: Option<DateTime<FixedOffset>>,
}

impl File {
    pub fn builder() -> FileBuilder {
        <FileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileBuilder {
    file_item_fields: Option<FileItem>,
    download_url: Option<String>,
    download_expires_at: Option<DateTime<FixedOffset>>,
}

impl FileBuilder {
    pub fn file_item_fields(mut self, value: FileItem) -> Self {
        self.file_item_fields = Some(value);
        self
    }

    pub fn download_url(mut self, value: impl Into<String>) -> Self {
        self.download_url = Some(value.into());
        self
    }

    pub fn download_expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.download_expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`File`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_item_fields`](FileBuilder::file_item_fields)
    pub fn build(self) -> Result<File, BuildError> {
        Ok(File {
            file_item_fields: self.file_item_fields.ok_or_else(|| BuildError::missing_field("file_item_fields"))?,
            download_url: self.download_url,
            download_expires_at: self.download_expires_at,
        })
    }
}
