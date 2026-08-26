pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodListListEntriesResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by entry ascending.
    #[serde(default)]
    pub entries: Vec<PodListEntry>,
}

impl PodListListEntriesResponse {
    pub fn builder() -> PodListListEntriesResponseBuilder {
        <PodListListEntriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodListListEntriesResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    entries: Option<Vec<PodListEntry>>,
}

impl PodListListEntriesResponseBuilder {
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

    pub fn entries(mut self, value: Vec<PodListEntry>) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PodListListEntriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](PodListListEntriesResponseBuilder::count)
    /// - [`entries`](PodListListEntriesResponseBuilder::entries)
    pub fn build(self) -> Result<PodListListEntriesResponse, BuildError> {
        Ok(PodListListEntriesResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            entries: self.entries.ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
