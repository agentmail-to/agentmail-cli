pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InboxesThreadsSearchQueryRequest {
    #[serde(default)]
    pub q: Query,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<Before>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<After>,
}

impl InboxesThreadsSearchQueryRequest {
    pub fn builder() -> InboxesThreadsSearchQueryRequestBuilder {
        <InboxesThreadsSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesThreadsSearchQueryRequestBuilder {
    q: Option<Query>,
    limit: Option<Limit>,
    page_token: Option<PageToken>,
    before: Option<Before>,
    after: Option<After>,
}

impl InboxesThreadsSearchQueryRequestBuilder {
    pub fn q(mut self, value: Query) -> Self {
        self.q = Some(value);
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

    pub fn before(mut self, value: Before) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: After) -> Self {
        self.after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxesThreadsSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`q`](InboxesThreadsSearchQueryRequestBuilder::q)
    pub fn build(self) -> Result<InboxesThreadsSearchQueryRequest, BuildError> {
        Ok(InboxesThreadsSearchQueryRequest {
            q: self.q.ok_or_else(|| BuildError::missing_field("q"))?,
            limit: self.limit,
            page_token: self.page_token,
            before: self.before,
            after: self.after,
        })
    }
}

