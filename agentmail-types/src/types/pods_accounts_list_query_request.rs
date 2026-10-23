pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodsAccountsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<Ascending>,
}

impl PodsAccountsListQueryRequest {
    pub fn builder() -> PodsAccountsListQueryRequestBuilder {
        <PodsAccountsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodsAccountsListQueryRequestBuilder {
    limit: Option<Limit>,
    page_token: Option<PageToken>,
    ascending: Option<Ascending>,
}

impl PodsAccountsListQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`PodsAccountsListQueryRequest`].
    pub fn build(self) -> Result<PodsAccountsListQueryRequest, BuildError> {
        Ok(PodsAccountsListQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
            ascending: self.ascending,
        })
    }
}

