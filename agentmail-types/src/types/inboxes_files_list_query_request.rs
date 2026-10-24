pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InboxesFilesListQueryRequest {
    /// Only files whose path starts with this. Held to the path rules, except it may stop partway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
    /// Page size, 100 by default and at most 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
    /// Path order, `true` by default. `false` lists in reverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
}

impl InboxesFilesListQueryRequest {
    pub fn builder() -> InboxesFilesListQueryRequestBuilder {
        <InboxesFilesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesFilesListQueryRequestBuilder {
    path_prefix: Option<String>,
    limit: Option<Limit>,
    page_token: Option<PageToken>,
    ascending: Option<bool>,
}

impl InboxesFilesListQueryRequestBuilder {
    pub fn path_prefix(mut self, value: impl Into<String>) -> Self {
        self.path_prefix = Some(value.into());
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    pub fn ascending(mut self, value: bool) -> Self {
        self.ascending = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxesFilesListQueryRequest`].
    pub fn build(self) -> Result<InboxesFilesListQueryRequest, BuildError> {
        Ok(InboxesFilesListQueryRequest {
            path_prefix: self.path_prefix,
            limit: self.limit,
            page_token: self.page_token,
            ascending: self.ascending,
        })
    }
}

