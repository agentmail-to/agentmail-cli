pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<Ascending>,
}

impl DomainsListQueryRequest {
    pub fn builder() -> DomainsListQueryRequestBuilder {
        <DomainsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainsListQueryRequestBuilder {
    limit: Option<Limit>,
    page_token: Option<PageToken>,
    ascending: Option<Ascending>,
}

impl DomainsListQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`DomainsListQueryRequest`].
    pub fn build(self) -> Result<DomainsListQueryRequest, BuildError> {
        Ok(DomainsListQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
            ascending: self.ascending,
        })
    }
}

