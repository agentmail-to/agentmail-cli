pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An attendee's response changed: the inbox responded with the respond endpoint, or an attendee
/// replied by email to an invitation the inbox sent. `calendar_event.attendees` holds the new
/// statuses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarEventRespondedEvent {
    pub r#type: CalendarEventRespondedEventType,
    pub event_type: CalendarEventRespondedEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    pub calendar_event: CalendarEvent,
}

impl CalendarEventRespondedEvent {
    pub fn builder() -> CalendarEventRespondedEventBuilder {
        <CalendarEventRespondedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventRespondedEventBuilder {
    r#type: Option<CalendarEventRespondedEventType>,
    event_type: Option<CalendarEventRespondedEventEventType>,
    event_id: Option<EventId>,
    inbox_id: Option<InboxesInboxId>,
    calendar_event: Option<CalendarEvent>,
}

impl CalendarEventRespondedEventBuilder {
    pub fn r#type(mut self, value: CalendarEventRespondedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: CalendarEventRespondedEventEventType) -> Self {
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

    /// Consumes the builder and constructs a [`CalendarEventRespondedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CalendarEventRespondedEventBuilder::r#type)
    /// - [`event_type`](CalendarEventRespondedEventBuilder::event_type)
    /// - [`event_id`](CalendarEventRespondedEventBuilder::event_id)
    /// - [`inbox_id`](CalendarEventRespondedEventBuilder::inbox_id)
    /// - [`calendar_event`](CalendarEventRespondedEventBuilder::calendar_event)
    pub fn build(self) -> Result<CalendarEventRespondedEvent, BuildError> {
        Ok(CalendarEventRespondedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            calendar_event: self.calendar_event.ok_or_else(|| BuildError::missing_field("calendar_event"))?,
        })
    }
}
