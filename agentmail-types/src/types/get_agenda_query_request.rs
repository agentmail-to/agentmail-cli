pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-agenda
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAgendaQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<CalendarConsistency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<WindowAfter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<WindowBefore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_overlapping: Option<IncludeOverlapping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<CalendarLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
}

impl GetAgendaQueryRequest {
    pub fn builder() -> GetAgendaQueryRequestBuilder {
        <GetAgendaQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgendaQueryRequestBuilder {
    consistency: Option<CalendarConsistency>,
    after: Option<WindowAfter>,
    before: Option<WindowBefore>,
    include_overlapping: Option<IncludeOverlapping>,
    limit: Option<CalendarLimit>,
    page_token: Option<PageToken>,
}

impl GetAgendaQueryRequestBuilder {
    pub fn consistency(mut self, value: CalendarConsistency) -> Self {
        self.consistency = Some(value);
        self
    }

    pub fn after(mut self, value: WindowAfter) -> Self {
        self.after = Some(value);
        self
    }

    pub fn before(mut self, value: WindowBefore) -> Self {
        self.before = Some(value);
        self
    }

    pub fn include_overlapping(mut self, value: IncludeOverlapping) -> Self {
        self.include_overlapping = Some(value);
        self
    }

    pub fn limit(mut self, value: CalendarLimit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAgendaQueryRequest`].
    pub fn build(self) -> Result<GetAgendaQueryRequest, BuildError> {
        Ok(GetAgendaQueryRequest {
            consistency: self.consistency,
            after: self.after,
            before: self.before,
            include_overlapping: self.include_overlapping,
            limit: self.limit,
            page_token: self.page_token,
        })
    }
}

