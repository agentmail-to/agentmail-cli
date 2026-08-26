pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListListEntriesResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by entry ascending.
    #[serde(default)]
    pub entries: Vec<ListEntry>,
}

impl ListListEntriesResponse {
    pub fn builder() -> ListListEntriesResponseBuilder {
        <ListListEntriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListListEntriesResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    entries: Option<Vec<ListEntry>>,
}

impl ListListEntriesResponseBuilder {
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

    pub fn entries(mut self, value: Vec<ListEntry>) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListListEntriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListListEntriesResponseBuilder::count)
    /// - [`entries`](ListListEntriesResponseBuilder::entries)
    pub fn build(self) -> Result<ListListEntriesResponse, BuildError> {
        Ok(ListListEntriesResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            entries: self.entries.ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
