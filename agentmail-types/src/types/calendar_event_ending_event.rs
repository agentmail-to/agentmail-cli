pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An event, or one date of a recurring event, has ended. Usually sent within a minute after
/// `scheduled_at`. Retries carry the same `event_id` and an identical body.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarEventEndingEvent {
    pub r#type: CalendarEventEndingEventType,
    pub event_type: CalendarEventEndingEventEventType,
    /// Stable for this date and boundary. Use it to deduplicate deliveries.
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    /// The event or date as it stood when the boundary was first processed.
    pub calendar_event: CalendarEvent,
    /// The end this webhook is for (the event's `end_at`).
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub scheduled_at: DateTime<FixedOffset>,
}

impl CalendarEventEndingEvent {
    pub fn builder() -> CalendarEventEndingEventBuilder {
        <CalendarEventEndingEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventEndingEventBuilder {
    r#type: Option<CalendarEventEndingEventType>,
    event_type: Option<CalendarEventEndingEventEventType>,
    event_id: Option<EventId>,
    inbox_id: Option<InboxesInboxId>,
    calendar_event: Option<CalendarEvent>,
    scheduled_at: Option<DateTime<FixedOffset>>,
}

impl CalendarEventEndingEventBuilder {
    pub fn r#type(mut self, value: CalendarEventEndingEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: CalendarEventEndingEventEventType) -> Self {
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

    /// Consumes the builder and constructs a [`CalendarEventEndingEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CalendarEventEndingEventBuilder::r#type)
    /// - [`event_type`](CalendarEventEndingEventBuilder::event_type)
    /// - [`event_id`](CalendarEventEndingEventBuilder::event_id)
    /// - [`inbox_id`](CalendarEventEndingEventBuilder::inbox_id)
    /// - [`calendar_event`](CalendarEventEndingEventBuilder::calendar_event)
    /// - [`scheduled_at`](CalendarEventEndingEventBuilder::scheduled_at)
    pub fn build(self) -> Result<CalendarEventEndingEvent, BuildError> {
        Ok(CalendarEventEndingEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            calendar_event: self.calendar_event.ok_or_else(|| BuildError::missing_field("calendar_event"))?,
            scheduled_at: self.scheduled_at.ok_or_else(|| BuildError::missing_field("scheduled_at"))?,
        })
    }
}
