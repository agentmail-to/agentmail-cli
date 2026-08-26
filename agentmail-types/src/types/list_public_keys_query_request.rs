pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-public-keys
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPublicKeysQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<Ascending>,
}

impl ListPublicKeysQueryRequest {
    pub fn builder() -> ListPublicKeysQueryRequestBuilder {
        <ListPublicKeysQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPublicKeysQueryRequestBuilder {
    limit: Option<Limit>,
    page_token: Option<PageToken>,
    ascending: Option<Ascending>,
}

impl ListPublicKeysQueryRequestBuilder {
    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    pub fn ascending(mut self, value: Ascending) -> Self {
        self.ascending = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPublicKeysQueryRequest`].
    pub fn build(self) -> Result<ListPublicKeysQueryRequest, BuildError> {
        Ok(ListPublicKeysQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
            ascending: self.ascending,
        })
    }
}

