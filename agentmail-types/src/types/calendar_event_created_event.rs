pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An event was created, through the API or from an invitation the inbox received. Also sent when
/// one date of a recurring event is edited for the first time; `calendar_event` is then that date.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarEventCreatedEvent {
    pub r#type: CalendarEventCreatedEventType,
    pub event_type: CalendarEventCreatedEventEventType,
    /// Equals the `operation_id` returned by the request that made the change.
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    pub calendar_event: CalendarEvent,
}

impl CalendarEventCreatedEvent {
    pub fn builder() -> CalendarEventCreatedEventBuilder {
        <CalendarEventCreatedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventCreatedEventBuilder {
    r#type: Option<CalendarEventCreatedEventType>,
    event_type: Option<CalendarEventCreatedEventEventType>,
    event_id: Option<EventId>,
    inbox_id: Option<InboxesInboxId>,
    calendar_event: Option<CalendarEvent>,
}

impl CalendarEventCreatedEventBuilder {
    pub fn r#type(mut self, value: CalendarEventCreatedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: CalendarEventCreatedEventEventType) -> Self {
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

    /// Consumes the builder and constructs a [`CalendarEventCreatedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CalendarEventCreatedEventBuilder::r#type)
    /// - [`event_type`](CalendarEventCreatedEventBuilder::event_type)
    /// - [`event_id`](CalendarEventCreatedEventBuilder::event_id)
    /// - [`inbox_id`](CalendarEventCreatedEventBuilder::inbox_id)
    /// - [`calendar_event`](CalendarEventCreatedEventBuilder::calendar_event)
    pub fn build(self) -> Result<CalendarEventCreatedEvent, BuildError> {
        Ok(CalendarEventCreatedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            calendar_event: self.calendar_event.ok_or_else(|| BuildError::missing_field("calendar_event"))?,
        })
    }
}
