pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ThreadsGetQueryRequest {
    /// Maximum number of messages to return. Cannot exceed 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    /// Token returned by the previous response for retrieving the next, older page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
}

impl ThreadsGetQueryRequest {
    pub fn builder() -> ThreadsGetQueryRequestBuilder {
        <ThreadsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadsGetQueryRequestBuilder {
    limit: Option<Limit>,
    page_token: Option<PageToken>,
}

impl ThreadsGetQueryRequestBuilder {
    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThreadsGetQueryRequest`].
    pub fn build(self) -> Result<ThreadsGetQueryRequest, BuildError> {
        Ok(ThreadsGetQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
        })
    }
}

