pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
}

impl ListsListQueryRequest {
    pub fn builder() -> ListsListQueryRequestBuilder {
        <ListsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListsListQueryRequestBuilder {
    limit: Option<Limit>,
    page_token: Option<PageToken>,
}

impl ListsListQueryRequestBuilder {
    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListsListQueryRequest`].
    pub fn build(self) -> Result<ListsListQueryRequest, BuildError> {
        Ok(ListsListQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
        })
    }
}

