pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The created file. A pending file carries `upload`; a create that returns an existing `ready` file does not.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFileResponse {
    #[serde(flatten)]
    pub file_fields: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<FileUpload>,
}

impl CreateFileResponse {
    pub fn builder() -> CreateFileResponseBuilder {
        <CreateFileResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFileResponseBuilder {
    file_fields: Option<File>,
    upload: Option<FileUpload>,
}

impl CreateFileResponseBuilder {
    pub fn file_fields(mut self, value: File) -> Self {
        self.file_fields = Some(value);
        self
    }

    pub fn upload(mut self, value: FileUpload) -> Self {
        self.upload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateFileResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_fields`](CreateFileResponseBuilder::file_fields)
    pub fn build(self) -> Result<CreateFileResponse, BuildError> {
        Ok(CreateFileResponse {
            file_fields: self.file_fields.ok_or_else(|| BuildError::missing_field("file_fields"))?,
            upload: self.upload,
        })
    }
}
