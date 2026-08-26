pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDraftsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by `updated_at` descending.
    #[serde(default)]
    pub drafts: Vec<DraftItem>,
}

impl ListDraftsResponse {
    pub fn builder() -> ListDraftsResponseBuilder {
        <ListDraftsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDraftsResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    drafts: Option<Vec<DraftItem>>,
}

impl ListDraftsResponseBuilder {
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

    pub fn drafts(mut self, value: Vec<DraftItem>) -> Self {
        self.drafts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDraftsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListDraftsResponseBuilder::count)
    /// - [`drafts`](ListDraftsResponseBuilder::drafts)
    pub fn build(self) -> Result<ListDraftsResponse, BuildError> {
        Ok(ListDraftsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            drafts: self.drafts.ok_or_else(|| BuildError::missing_field("drafts"))?,
        })
    }
}
