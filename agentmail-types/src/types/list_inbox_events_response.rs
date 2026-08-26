pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListInboxEventsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by `event_id` descending.
    #[serde(default)]
    pub events: Vec<InboxEvent>,
}

impl ListInboxEventsResponse {
    pub fn builder() -> ListInboxEventsResponseBuilder {
        <ListInboxEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListInboxEventsResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    events: Option<Vec<InboxEvent>>,
}

impl ListInboxEventsResponseBuilder {
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

    pub fn events(mut self, value: Vec<InboxEvent>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListInboxEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListInboxEventsResponseBuilder::count)
    /// - [`events`](ListInboxEventsResponseBuilder::events)
    pub fn build(self) -> Result<ListInboxEventsResponse, BuildError> {
        Ok(ListInboxEventsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
        })
    }
}
