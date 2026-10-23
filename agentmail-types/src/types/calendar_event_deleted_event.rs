pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A one-off or recurring event was deleted. `calendar_event` is the event as it was. Cancelling
/// dates of a recurring event sends `calendar.event.updated` instead.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarEventDeletedEvent {
    pub r#type: CalendarEventDeletedEventType,
    pub event_type: CalendarEventDeletedEventEventType,
    /// Equals the `deletion_id` returned by the request that made the change.
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    pub calendar_event: CalendarEvent,
}

impl CalendarEventDeletedEvent {
    pub fn builder() -> CalendarEventDeletedEventBuilder {
        <CalendarEventDeletedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventDeletedEventBuilder {
    r#type: Option<CalendarEventDeletedEventType>,
    event_type: Option<CalendarEventDeletedEventEventType>,
    event_id: Option<EventId>,
    inbox_id: Option<InboxesInboxId>,
    calendar_event: Option<CalendarEvent>,
}

impl CalendarEventDeletedEventBuilder {
    pub fn r#type(mut self, value: CalendarEventDeletedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: CalendarEventDeletedEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn calendar_event(mut self, value: CalendarEvent) -> Self {
        self.calendar_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CalendarEventDeletedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CalendarEventDeletedEventBuilder::r#type)
    /// - [`event_type`](CalendarEventDeletedEventBuilder::event_type)
    /// - [`event_id`](CalendarEventDeletedEventBuilder::event_id)
    /// - [`inbox_id`](CalendarEventDeletedEventBuilder::inbox_id)
    /// - [`calendar_event`](CalendarEventDeletedEventBuilder::calendar_event)
    pub fn build(self) -> Result<CalendarEventDeletedEvent, BuildError> {
        Ok(CalendarEventDeletedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            calendar_event: self.calendar_event.ok_or_else(|| BuildError::missing_field("calendar_event"))?,
        })
    }
}
