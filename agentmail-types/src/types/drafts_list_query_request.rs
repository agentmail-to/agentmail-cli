pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DraftsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
    #[serde(default)]
    pub labels: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<Before>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<After>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<Ascending>,
}

impl DraftsListQueryRequest {
    pub fn builder() -> DraftsListQueryRequestBuilder {
        <DraftsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DraftsListQueryRequestBuilder {
    limit: Option<Limit>,
    page_token: Option<PageToken>,
    labels: Option<Vec<Option<String>>>,
    before: Option<Before>,
    after: Option<After>,
    ascending: Option<Ascending>,
}

impl DraftsListQueryRequestBuilder {
    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    pub fn labels(mut self, value: Vec<Option<String>>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn before(mut self, value: Before) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: After) -> Self {
        self.after = Some(value);
        self
    }

    pub fn ascending(mut self, value: Ascending) -> Self {
        self.ascending = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DraftsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`labels`](DraftsListQueryRequestBuilder::labels)
    pub fn build(self) -> Result<DraftsListQueryRequest, BuildError> {
        Ok(DraftsListQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
            before: self.before,
            after: self.after,
            ascending: self.ascending,
        })
    }
}

