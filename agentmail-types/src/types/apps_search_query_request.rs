pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppsSearchQueryRequest {
    /// Name prefix to search for.
    #[serde(default)]
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
}

impl AppsSearchQueryRequest {
    pub fn builder() -> AppsSearchQueryRequestBuilder {
        <AppsSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppsSearchQueryRequestBuilder {
    q: Option<String>,
    limit: Option<Limit>,
}

impl AppsSearchQueryRequestBuilder {
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppsSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`q`](AppsSearchQueryRequestBuilder::q)
    pub fn build(self) -> Result<AppsSearchQueryRequest, BuildError> {
        Ok(AppsSearchQueryRequest {
            q: self.q.ok_or_else(|| BuildError::missing_field("q"))?,
            limit: self.limit,
        })
    }
}

