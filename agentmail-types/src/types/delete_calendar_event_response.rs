pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Deleting a one-off or recurring event (or a `mode=future` delete that covers a whole series)
/// returns `deletion_id`. Deleting dates of a series returns the cancelled date as `event` with an
/// `operation_id`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeleteCalendarEventResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_id: Option<DeletionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<CalendarEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<OperationId>,
}

impl DeleteCalendarEventResponse {
    pub fn builder() -> DeleteCalendarEventResponseBuilder {
        <DeleteCalendarEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteCalendarEventResponseBuilder {
    deletion_id: Option<DeletionId>,
    event: Option<CalendarEvent>,
    operation_id: Option<OperationId>,
}

impl DeleteCalendarEventResponseBuilder {
    pub fn deletion_id(mut self, value: DeletionId) -> Self {
        self.deletion_id = Some(value);
        self
    }

    pub fn event(mut self, value: CalendarEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn operation_id(mut self, value: OperationId) -> Self {
        self.operation_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteCalendarEventResponse`].
    pub fn build(self) -> Result<DeleteCalendarEventResponse, BuildError> {
        Ok(DeleteCalendarEventResponse {
            deletion_id: self.deletion_id,
            event: self.event,
            operation_id: self.operation_id,
        })
    }
}
