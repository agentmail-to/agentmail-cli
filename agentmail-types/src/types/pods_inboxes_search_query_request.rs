pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodsInboxesSearchQueryRequest {
    /// Address or display name to search for. Matches word prefixes. Must be 2 to 256 characters.
    #[serde(default)]
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
}

impl PodsInboxesSearchQueryRequest {
    pub fn builder() -> PodsInboxesSearchQueryRequestBuilder {
        <PodsInboxesSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodsInboxesSearchQueryRequestBuilder {
    q: Option<String>,
    limit: Option<Limit>,
    page_token: Option<PageToken>,
}

impl PodsInboxesSearchQueryRequestBuilder {
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PodsInboxesSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`q`](PodsInboxesSearchQueryRequestBuilder::q)
    pub fn build(self) -> Result<PodsInboxesSearchQueryRequest, BuildError> {
        Ok(PodsInboxesSearchQueryRequest {
            q: self.q.ok_or_else(|| BuildError::missing_field("q"))?,
            limit: self.limit,
            page_token: self.page_token,
        })
    }
}

