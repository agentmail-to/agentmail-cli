pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-event
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEventQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<CalendarConsistency>,
}

impl GetEventQueryRequest {
    pub fn builder() -> GetEventQueryRequestBuilder {
        <GetEventQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEventQueryRequestBuilder {
    consistency: Option<CalendarConsistency>,
}

impl GetEventQueryRequestBuilder {
    pub fn consistency(mut self, value: CalendarConsistency) -> Self {
        self.consistency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEventQueryRequest`].
    pub fn build(self) -> Result<GetEventQueryRequest, BuildError> {
        Ok(GetEventQueryRequest {
            consistency: self.consistency,
        })
    }
}

