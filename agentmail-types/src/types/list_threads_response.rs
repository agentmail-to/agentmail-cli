pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListThreadsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by `timestamp` descending.
    #[serde(default)]
    pub threads: Vec<ThreadItem>,
}

impl ListThreadsResponse {
    pub fn builder() -> ListThreadsResponseBuilder {
        <ListThreadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListThreadsResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    threads: Option<Vec<ThreadItem>>,
}

impl ListThreadsResponseBuilder {
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

    pub fn threads(mut self, value: Vec<ThreadItem>) -> Self {
        self.threads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListThreadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListThreadsResponseBuilder::count)
    /// - [`threads`](ListThreadsResponseBuilder::threads)
    pub fn build(self) -> Result<ListThreadsResponse, BuildError> {
        Ok(ListThreadsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            threads: self.threads.ok_or_else(|| BuildError::missing_field("threads"))?,
        })
    }
}
