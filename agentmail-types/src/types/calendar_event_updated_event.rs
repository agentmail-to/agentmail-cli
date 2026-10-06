pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An event, or one or more dates of a recurring event, changed. `calendar_event` is the event
/// (or the date) after the change. Cancelling dates with a delete on a dated ID also sends this
/// event, with the cancelled date as `calendar_event`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarEventUpdatedEvent {
    pub r#type: CalendarEventUpdatedEventType,
    pub event_type: CalendarEventUpdatedEventEventType,
    /// Equals the `operation_id` returned by the request that made the change.
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    pub calendar_event: CalendarEvent,
    /// For changes to a one-off or series event, the prior values of the fields that changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<CalendarEventPrevious>,
}

impl CalendarEventUpdatedEvent {
    pub fn builder() -> CalendarEventUpdatedEventBuilder {
        <CalendarEventUpdatedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventUpdatedEventBuilder {
    r#type: Option<CalendarEventUpdatedEventType>,
    event_type: Option<CalendarEventUpdatedEventEventType>,
    event_id: Option<EventId>,
    inbox_id: Option<InboxesInboxId>,
    calendar_event: Option<CalendarEvent>,
    previous: Option<CalendarEventPrevious>,
}

impl CalendarEventUpdatedEventBuilder {
    pub fn r#type(mut self, value: CalendarEventUpdatedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: CalendarEventUpdatedEventEventType) -> Self {
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

    pub fn previous(mut self, value: CalendarEventPrevious) -> Self {
        self.previous = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CalendarEventUpdatedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CalendarEventUpdatedEventBuilder::r#type)
    /// - [`event_type`](CalendarEventUpdatedEventBuilder::event_type)
    /// - [`event_id`](CalendarEventUpdatedEventBuilder::event_id)
    /// - [`inbox_id`](CalendarEventUpdatedEventBuilder::inbox_id)
    /// - [`calendar_event`](CalendarEventUpdatedEventBuilder::calendar_event)
    pub fn build(self) -> Result<CalendarEventUpdatedEvent, BuildError> {
        Ok(CalendarEventUpdatedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            calendar_event: self.calendar_event.ok_or_else(|| BuildError::missing_field("calendar_event"))?,
            previous: self.previous,
        })
    }
}
