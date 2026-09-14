pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InboxesSearchInboxesResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by relevance, best match first.
    #[serde(default)]
    pub inboxes: Vec<InboxesInbox>,
}

impl InboxesSearchInboxesResponse {
    pub fn builder() -> InboxesSearchInboxesResponseBuilder {
        <InboxesSearchInboxesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesSearchInboxesResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    inboxes: Option<Vec<InboxesInbox>>,
}

impl InboxesSearchInboxesResponseBuilder {
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

    pub fn inboxes(mut self, value: Vec<InboxesInbox>) -> Self {
        self.inboxes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxesSearchInboxesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](InboxesSearchInboxesResponseBuilder::count)
    /// - [`inboxes`](InboxesSearchInboxesResponseBuilder::inboxes)
    pub fn build(self) -> Result<InboxesSearchInboxesResponse, BuildError> {
        Ok(InboxesSearchInboxesResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            inboxes: self.inboxes.ok_or_else(|| BuildError::missing_field("inboxes"))?,
        })
    }
}
