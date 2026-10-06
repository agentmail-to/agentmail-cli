pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<CalendarLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
}

impl ListEventsQueryRequest {
    pub fn builder() -> ListEventsQueryRequestBuilder {
        <ListEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsQueryRequestBuilder {
    limit: Option<CalendarLimit>,
    page_token: Option<PageToken>,
}

impl ListEventsQueryRequestBuilder {
    pub fn limit(mut self, value: CalendarLimit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEventsQueryRequest`].
    pub fn build(self) -> Result<ListEventsQueryRequest, BuildError> {
        Ok(ListEventsQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
        })
    }
}

