pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A calendar event. The same shape is used by every response, list item and webhook. `kind` says
/// whether it is a one-off event, the definition of a recurring series, or one date of a series.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarEvent {
    #[serde(default)]
    pub event_id: CalendarEventId,
    pub kind: CalendarEventKind,
    /// For `instance` events, the UUID of the series this date belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_id: Option<String>,
    /// For `instance` events, the date's start as the series rule generated it, before any edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_start: Option<WallTime>,
    /// For `instance` events, `original_start` as a UTC instant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_start_at: Option<DateTime<FixedOffset>>,
    /// For `instance` events, `true` when this date was edited or moved, individually or by a `mode=future` change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_exception: Option<bool>,
    /// Title of the event.
    #[serde(default)]
    pub title: String,
    /// Description of the event. Omitted from agenda and instance list items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Location of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Your own JSON value for the event, usually an object. Never sent to attendees. Omitted from agenda and instance list items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub status: CalendarEventStatus,
    /// Whether this is an all-day event.
    #[serde(default)]
    pub all_day: bool,
    /// Local start time in `timezone`. For all-day events, the first day.
    #[serde(default)]
    pub start: WallTime,
    /// Local end time in `timezone`. For all-day events, the last day (inclusive).
    #[serde(default)]
    pub end: WallTime,
    #[serde(default)]
    pub timezone: IanaTimezone,
    pub duration_mode: DurationMode,
    /// Length of the event, in milliseconds for timed events and in days for all-day events.
    #[serde(default)]
    pub duration_value: i64,
    /// Start as a UTC instant. This is when `calendar.event.starting` is due.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start_at: DateTime<FixedOffset>,
    /// End as a UTC instant. This is when `calendar.event.ending` is due.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end_at: DateTime<FixedOffset>,
    /// For `series` events, the recurrence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
    /// Attendees of the event. Omitted from agenda and instance list items; `attendee_count` is always present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,
    /// Number of attendees.
    #[serde(default)]
    pub attendee_count: i64,
    /// iCalendar UID of the event. Invitations about this event carry the same UID.
    #[serde(default)]
    pub uid: String,
    /// iCalendar SEQUENCE number. It increases when a change to a one-off or series event affects
    /// what attendees see (time, status, title, description, location, recurrence or attendees).
    /// A change to a dated event increases it only when the change is sent with `send_invites`.
    #[serde(default)]
    pub sequence: i64,
    pub source: CalendarEventSource,
    /// Email address of the organizer. For `api` events, the inbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer_email: Option<String>,
    /// For `email` events, the ID of the message whose invitation last created or updated the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_message_id: Option<String>,
    /// Present on `single` and `series` events, except agenda items and
    /// `calendar.event.starting` and `calendar.event.ending` payloads, which omit it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_revision: Option<ResourceRevision>,
    /// Time at which the event (or, for a date, the definition governing it) was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Time at which the event (or, for a date, the definition governing it) was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The event's or date's current version, also sent as the `ETag` response header. Send it in
    /// `If-Match` to make an update, delete or response conditional. Present on Get, Create,
    /// Update, Delete and Respond responses; list items and webhooks omit it. Treat it as opaque:
    /// a dated event's `etag` (`"occ-..."`) is not derived from `resource_revision`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}

impl CalendarEvent {
    pub fn builder() -> CalendarEventBuilder {
        <CalendarEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventBuilder {
    event_id: Option<CalendarEventId>,
    kind: Option<CalendarEventKind>,
    series_id: Option<String>,
    original_start: Option<WallTime>,
    original_start_at: Option<DateTime<FixedOffset>>,
    is_exception: Option<bool>,
    title: Option<String>,
    description: Option<String>,
    location: Option<String>,
    metadata: Option<serde_json::Value>,
    status: Option<CalendarEventStatus>,
    all_day: Option<bool>,
    start: Option<WallTime>,
    end: Option<WallTime>,
    timezone: Option<IanaTimezone>,
    duration_mode: Option<DurationMode>,
    duration_value: Option<i64>,
    start_at: Option<DateTime<FixedOffset>>,
    end_at: Option<DateTime<FixedOffset>>,
    recurrence: Option<Recurrence>,
    attendees: Option<Vec<Attendee>>,
    attendee_count: Option<i64>,
    uid: Option<String>,
    sequence: Option<i64>,
    source: Option<CalendarEventSource>,
    organizer_email: Option<String>,
    origin_message_id: Option<String>,
    resource_revision: Option<ResourceRevision>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    etag: Option<String>,
}

impl CalendarEventBuilder {
    pub fn event_id(mut self, value: CalendarEventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn kind(mut self, value: CalendarEventKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn series_id(mut self, value: impl Into<String>) -> Self {
        self.series_id = Some(value.into());
        self
    }

    pub fn original_start(mut self, value: WallTime) -> Self {
        self.original_start = Some(value);
        self
    }

    pub fn original_start_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.original_start_at = Some(value);
        self
    }

    pub fn is_exception(mut self, value: bool) -> Self {
        self.is_exception = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: serde_json::Value) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn status(mut self, value: CalendarEventStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn all_day(mut self, value: bool) -> Self {
        self.all_day = Some(value);
        self
    }

    pub fn start(mut self, value: WallTime) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: WallTime) -> Self {
        self.end = Some(value);
        self
    }

    pub fn timezone(mut self, value: IanaTimezone) -> Self {
        self.timezone = Some(value);
        self
    }

    pub fn duration_mode(mut self, value: DurationMode) -> Self {
        self.duration_mode = Some(value);
        self
    }

    pub fn duration_value(mut self, value: i64) -> Self {
        self.duration_value = Some(value);
        self
    }

    pub fn start_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_at = Some(value);
        self
    }

    pub fn end_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_at = Some(value);
        self
    }

    pub fn recurrence(mut self, value: Recurrence) -> Self {
        self.recurrence = Some(value);
        self
    }

    pub fn attendees(mut self, value: Vec<Attendee>) -> Self {
        self.attendees = Some(value);
        self
    }

    pub fn attendee_count(mut self, value: i64) -> Self {
        self.attendee_count = Some(value);
        self
    }

    pub fn uid(mut self, value: impl Into<String>) -> Self {
        self.uid = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn source(mut self, value: CalendarEventSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn organizer_email(mut self, value: impl Into<String>) -> Self {
        self.organizer_email = Some(value.into());
        self
    }

    pub fn origin_message_id(mut self, value: impl Into<String>) -> Self {
        self.origin_message_id = Some(value.into());
        self
    }

    pub fn resource_revision(mut self, value: ResourceRevision) -> Self {
        self.resource_revision = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn etag(mut self, value: impl Into<String>) -> Self {
        self.etag = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CalendarEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](CalendarEventBuilder::event_id)
    /// - [`kind`](CalendarEventBuilder::kind)
    /// - [`title`](CalendarEventBuilder::title)
    /// - [`status`](CalendarEventBuilder::status)
    /// - [`all_day`](CalendarEventBuilder::all_day)
    /// - [`start`](CalendarEventBuilder::start)
    /// - [`end`](CalendarEventBuilder::end)
    /// - [`timezone`](CalendarEventBuilder::timezone)
    /// - [`duration_mode`](CalendarEventBuilder::duration_mode)
    /// - [`duration_value`](CalendarEventBuilder::duration_value)
    /// - [`start_at`](CalendarEventBuilder::start_at)
    /// - [`end_at`](CalendarEventBuilder::end_at)
    /// - [`attendee_count`](CalendarEventBuilder::attendee_count)
    /// - [`uid`](CalendarEventBuilder::uid)
    /// - [`sequence`](CalendarEventBuilder::sequence)
    /// - [`source`](CalendarEventBuilder::source)
    /// - [`created_at`](CalendarEventBuilder::created_at)
    /// - [`updated_at`](CalendarEventBuilder::updated_at)
    pub fn build(self) -> Result<CalendarEvent, BuildError> {
        Ok(CalendarEvent {
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            series_id: self.series_id,
            original_start: self.original_start,
            original_start_at: self.original_start_at,
            is_exception: self.is_exception,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description,
            location: self.location,
            metadata: self.metadata,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            all_day: self.all_day.ok_or_else(|| BuildError::missing_field("all_day"))?,
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            timezone: self.timezone.ok_or_else(|| BuildError::missing_field("timezone"))?,
            duration_mode: self.duration_mode.ok_or_else(|| BuildError::missing_field("duration_mode"))?,
            duration_value: self.duration_value.ok_or_else(|| BuildError::missing_field("duration_value"))?,
            start_at: self.start_at.ok_or_else(|| BuildError::missing_field("start_at"))?,
            end_at: self.end_at.ok_or_else(|| BuildError::missing_field("end_at"))?,
            recurrence: self.recurrence,
            attendees: self.attendees,
            attendee_count: self.attendee_count.ok_or_else(|| BuildError::missing_field("attendee_count"))?,
            uid: self.uid.ok_or_else(|| BuildError::missing_field("uid"))?,
            sequence: self.sequence.ok_or_else(|| BuildError::missing_field("sequence"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            organizer_email: self.organizer_email,
            origin_message_id: self.origin_message_id,
            resource_revision: self.resource_revision,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            etag: self.etag,
        })
    }
}
