pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchThreadsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by relevance, best match first.
    #[serde(default)]
    pub threads: Vec<SearchThreadItem>,
}

impl SearchThreadsResponse {
    pub fn builder() -> SearchThreadsResponseBuilder {
        <SearchThreadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchThreadsResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    threads: Option<Vec<SearchThreadItem>>,
}

impl SearchThreadsResponseBuilder {
    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: PageToken) -> Self {
        self.next_page_token = Some(value);
        self
    }

    pub fn threads(mut self, value: Vec<SearchThreadItem>) -> Self {
        self.threads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchThreadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](SearchThreadsResponseBuilder::count)
    /// - [`threads`](SearchThreadsResponseBuilder::threads)
    pub fn build(self) -> Result<SearchThreadsResponse, BuildError> {
        Ok(SearchThreadsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            threads: self.threads.ok_or_else(|| BuildError::missing_field("threads"))?,
        })
    }
}
