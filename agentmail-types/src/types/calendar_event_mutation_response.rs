pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarEventMutationResponse {
    pub event: CalendarEvent,
    #[serde(default)]
    pub operation_id: OperationId,
}

impl CalendarEventMutationResponse {
    pub fn builder() -> CalendarEventMutationResponseBuilder {
        <CalendarEventMutationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventMutationResponseBuilder {
    event: Option<CalendarEvent>,
    operation_id: Option<OperationId>,
}

impl CalendarEventMutationResponseBuilder {
    pub fn event(mut self, value: CalendarEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn operation_id(mut self, value: OperationId) -> Self {
        self.operation_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CalendarEventMutationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](CalendarEventMutationResponseBuilder::event)
    /// - [`operation_id`](CalendarEventMutationResponseBuilder::operation_id)
    pub fn build(self) -> Result<CalendarEventMutationResponse, BuildError> {
        Ok(CalendarEventMutationResponse {
            event: self.event.ok_or_else(|| BuildError::missing_field("event"))?,
            operation_id: self.operation_id.ok_or_else(|| BuildError::missing_field("operation_id"))?,
        })
    }
}
