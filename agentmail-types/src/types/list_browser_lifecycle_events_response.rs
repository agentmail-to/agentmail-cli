pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBrowserLifecycleEventsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: BrowserAuthorizationListLimit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[serde(default)]
    pub events: Vec<BrowserLifecycleEvent>,
}

impl ListBrowserLifecycleEventsResponse {
    pub fn builder() -> ListBrowserLifecycleEventsResponseBuilder {
        <ListBrowserLifecycleEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBrowserLifecycleEventsResponseBuilder {
    count: Option<Count>,
    limit: Option<BrowserAuthorizationListLimit>,
    next_page_token: Option<PageToken>,
    events: Option<Vec<BrowserLifecycleEvent>>,
}

impl ListBrowserLifecycleEventsResponseBuilder {
    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn limit(mut self, value: BrowserAuthorizationListLimit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: PageToken) -> Self {
        self.next_page_token = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<BrowserLifecycleEvent>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBrowserLifecycleEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListBrowserLifecycleEventsResponseBuilder::count)
    /// - [`limit`](ListBrowserLifecycleEventsResponseBuilder::limit)
    /// - [`events`](ListBrowserLifecycleEventsResponseBuilder::events)
    pub fn build(self) -> Result<ListBrowserLifecycleEventsResponse, BuildError> {
        Ok(ListBrowserLifecycleEventsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_page_token: self.next_page_token,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
        })
    }
}
