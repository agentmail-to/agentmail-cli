pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
    /// Only apps in this category. A filtered page can hold fewer than `limit` apps while more remain, so page until `next_page_token` is absent. A `page_token` works only with the `category` it was returned for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<AppCategory>,
}

impl AppsListQueryRequest {
    pub fn builder() -> AppsListQueryRequestBuilder {
        <AppsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppsListQueryRequestBuilder {
    limit: Option<Limit>,
    page_token: Option<PageToken>,
    category: Option<AppCategory>,
}

impl AppsListQueryRequestBuilder {
    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    pub fn category(mut self, value: AppCategory) -> Self {
        self.category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppsListQueryRequest`].
    pub fn build(self) -> Result<AppsListQueryRequest, BuildError> {
        Ok(AppsListQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
            category: self.category,
        })
    }
}

