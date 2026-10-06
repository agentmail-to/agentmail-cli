pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An event, or one date of a recurring event, has started. Usually sent within a minute after
/// `scheduled_at`. If a date is first picked up after it has already ended and more than 5
/// minutes after its start (for example, an event created entirely in the past), neither its
/// `calendar.event.starting` nor its `calendar.event.ending` is sent. Retries carry the same
/// `event_id` and an identical body.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarEventStartingEvent {
    pub r#type: CalendarEventStartingEventType,
    pub event_type: CalendarEventStartingEventEventType,
    /// Stable for this date and boundary. Use it to deduplicate deliveries.
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    /// The event or date as it stood when the boundary was first processed.
    pub calendar_event: CalendarEvent,
    /// The start this webhook is for (the event's `start_at`).
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub scheduled_at: DateTime<FixedOffset>,
}

impl CalendarEventStartingEvent {
    pub fn builder() -> CalendarEventStartingEventBuilder {
        <CalendarEventStartingEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventStartingEventBuilder {
    r#type: Option<CalendarEventStartingEventType>,
    event_type: Option<CalendarEventStartingEventEventType>,
    event_id: Option<EventId>,
    inbox_id: Option<InboxesInboxId>,
    calendar_event: Option<CalendarEvent>,
    scheduled_at: Option<DateTime<FixedOffset>>,
}

impl CalendarEventStartingEventBuilder {
    pub fn r#type(mut self, value: CalendarEventStartingEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: CalendarEventStartingEventEventType) -> Self {
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

    pub fn scheduled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scheduled_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CalendarEventStartingEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CalendarEventStartingEventBuilder::r#type)
    /// - [`event_type`](CalendarEventStartingEventBuilder::event_type)
    /// - [`event_id`](CalendarEventStartingEventBuilder::event_id)
    /// - [`inbox_id`](CalendarEventStartingEventBuilder::inbox_id)
    /// - [`calendar_event`](CalendarEventStartingEventBuilder::calendar_event)
    /// - [`scheduled_at`](CalendarEventStartingEventBuilder::scheduled_at)
    pub fn build(self) -> Result<CalendarEventStartingEvent, BuildError> {
        Ok(CalendarEventStartingEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            calendar_event: self.calendar_event.ok_or_else(|| BuildError::missing_field("calendar_event"))?,
            scheduled_at: self.scheduled_at.ok_or_else(|| BuildError::missing_field("scheduled_at"))?,
        })
    }
}
