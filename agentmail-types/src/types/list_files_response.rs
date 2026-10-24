pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFilesResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: Limit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// The inbox's files in path order. A list has no download URLs; get a file for one.
    #[serde(default)]
    pub files: Vec<FileItem>,
}

impl ListFilesResponse {
    pub fn builder() -> ListFilesResponseBuilder {
        <ListFilesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFilesResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    files: Option<Vec<FileItem>>,
}

impl ListFilesResponseBuilder {
    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: PageToken) -> Self {
        self.next_page_token = Some(value);
        self
    }

    pub fn files(mut self, value: Vec<FileItem>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFilesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListFilesResponseBuilder::count)
    /// - [`limit`](ListFilesResponseBuilder::limit)
    /// - [`files`](ListFilesResponseBuilder::files)
    pub fn build(self) -> Result<ListFilesResponse, BuildError> {
        Ok(ListFilesResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_page_token: self.next_page_token,
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
        })
    }
}
