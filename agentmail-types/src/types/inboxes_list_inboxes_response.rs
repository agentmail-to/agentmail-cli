pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InboxesListInboxesResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by `created_at` descending.
    #[serde(default)]
    pub inboxes: Vec<InboxesInbox>,
}

impl InboxesListInboxesResponse {
    pub fn builder() -> InboxesListInboxesResponseBuilder {
        <InboxesListInboxesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesListInboxesResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    inboxes: Option<Vec<InboxesInbox>>,
}

impl InboxesListInboxesResponseBuilder {
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

    /// Consumes the builder and constructs a [`InboxesListInboxesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](InboxesListInboxesResponseBuilder::count)
    /// - [`inboxes`](InboxesListInboxesResponseBuilder::inboxes)
    pub fn build(self) -> Result<InboxesListInboxesResponse, BuildError> {
        Ok(InboxesListInboxesResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            inboxes: self.inboxes.ok_or_else(|| BuildError::missing_field("inboxes"))?,
        })
    }
}
