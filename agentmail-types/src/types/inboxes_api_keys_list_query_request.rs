pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InboxesApiKeysListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
}

impl InboxesApiKeysListQueryRequest {
    pub fn builder() -> InboxesApiKeysListQueryRequestBuilder {
        <InboxesApiKeysListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesApiKeysListQueryRequestBuilder {
    limit: Option<Limit>,
    page_token: Option<PageToken>,
}

impl InboxesApiKeysListQueryRequestBuilder {
    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxesApiKeysListQueryRequest`].
    pub fn build(self) -> Result<InboxesApiKeysListQueryRequest, BuildError> {
        Ok(InboxesApiKeysListQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
        })
    }
}

