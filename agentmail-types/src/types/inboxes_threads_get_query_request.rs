pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InboxesThreadsGetQueryRequest {
    /// Maximum number of messages to return. Cannot exceed 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    /// Token returned by the previous response for retrieving the next, older page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
}

impl InboxesThreadsGetQueryRequest {
    pub fn builder() -> InboxesThreadsGetQueryRequestBuilder {
        <InboxesThreadsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesThreadsGetQueryRequestBuilder {
    limit: Option<Limit>,
    page_token: Option<PageToken>,
}

impl InboxesThreadsGetQueryRequestBuilder {
    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxesThreadsGetQueryRequest`].
    pub fn build(self) -> Result<InboxesThreadsGetQueryRequest, BuildError> {
        Ok(InboxesThreadsGetQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
        })
    }
}

