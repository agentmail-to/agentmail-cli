pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCalendarEventsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: Limit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// On List Events, ordered by `updated_at` descending. On Get Agenda and List Event Instances,
    /// ordered by `start_at` ascending.
    #[serde(default)]
    pub events: Vec<CalendarEvent>,
}

impl ListCalendarEventsResponse {
    pub fn builder() -> ListCalendarEventsResponseBuilder {
        <ListCalendarEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCalendarEventsResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    events: Option<Vec<CalendarEvent>>,
}

impl ListCalendarEventsResponseBuilder {
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

    pub fn events(mut self, value: Vec<CalendarEvent>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCalendarEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListCalendarEventsResponseBuilder::count)
    /// - [`limit`](ListCalendarEventsResponseBuilder::limit)
    /// - [`events`](ListCalendarEventsResponseBuilder::events)
    pub fn build(self) -> Result<ListCalendarEventsResponse, BuildError> {
        Ok(ListCalendarEventsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_page_token: self.next_page_token,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
        })
    }
}
