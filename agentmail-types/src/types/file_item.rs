pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A file, as a list returns it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FileItem {
    #[serde(default)]
    pub file_id: FileId,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub email: InboxesEmail,
    #[serde(default)]
    pub path: FilePath,
    #[serde(default)]
    pub name: FileName,
    #[serde(default)]
    pub size: FileSize,
    #[serde(default)]
    pub content_type: FileContentType,
    #[serde(default)]
    pub sha256: FileSha256,
    pub status: FileStatus,
    /// When the file was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When the file last changed, such as when its upload was completed.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<FileExpiresAt>,
}

impl FileItem {
    pub fn builder() -> FileItemBuilder {
        <FileItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileItemBuilder {
    file_id: Option<FileId>,
    inbox_id: Option<InboxesInboxId>,
    email: Option<InboxesEmail>,
    path: Option<FilePath>,
    name: Option<FileName>,
    size: Option<FileSize>,
    content_type: Option<FileContentType>,
    sha256: Option<FileSha256>,
    status: Option<FileStatus>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    expires_at: Option<FileExpiresAt>,
}

impl FileItemBuilder {
    pub fn file_id(mut self, value: FileId) -> Self {
        self.file_id = Some(value);
        self
    }

    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn email(mut self, value: InboxesEmail) -> Self {
        self.email = Some(value);
        self
    }

    pub fn path(mut self, value: FilePath) -> Self {
        self.path = Some(value);
        self
    }

    pub fn name(mut self, value: FileName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn size(mut self, value: FileSize) -> Self {
        self.size = Some(value);
        self
    }

    pub fn content_type(mut self, value: FileContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    pub fn sha256(mut self, value: FileSha256) -> Self {
        self.sha256 = Some(value);
        self
    }

    pub fn status(mut self, value: FileStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: FileExpiresAt) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](FileItemBuilder::file_id)
    /// - [`inbox_id`](FileItemBuilder::inbox_id)
    /// - [`email`](FileItemBuilder::email)
    /// - [`path`](FileItemBuilder::path)
    /// - [`name`](FileItemBuilder::name)
    /// - [`size`](FileItemBuilder::size)
    /// - [`content_type`](FileItemBuilder::content_type)
    /// - [`sha256`](FileItemBuilder::sha256)
    /// - [`status`](FileItemBuilder::status)
    /// - [`created_at`](FileItemBuilder::created_at)
    /// - [`updated_at`](FileItemBuilder::updated_at)
    pub fn build(self) -> Result<FileItem, BuildError> {
        Ok(FileItem {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
            content_type: self.content_type.ok_or_else(|| BuildError::missing_field("content_type"))?,
            sha256: self.sha256.ok_or_else(|| BuildError::missing_field("sha256"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            expires_at: self.expires_at,
        })
    }
}
