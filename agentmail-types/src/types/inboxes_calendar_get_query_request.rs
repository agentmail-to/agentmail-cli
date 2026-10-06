pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InboxesCalendarGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<CalendarConsistency>,
}

impl InboxesCalendarGetQueryRequest {
    pub fn builder() -> InboxesCalendarGetQueryRequestBuilder {
        <InboxesCalendarGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesCalendarGetQueryRequestBuilder {
    consistency: Option<CalendarConsistency>,
}

impl InboxesCalendarGetQueryRequestBuilder {
    pub fn consistency(mut self, value: CalendarConsistency) -> Self {
        self.consistency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxesCalendarGetQueryRequest`].
    pub fn build(self) -> Result<InboxesCalendarGetQueryRequest, BuildError> {
        Ok(InboxesCalendarGetQueryRequest {
            consistency: self.consistency,
        })
    }
}

