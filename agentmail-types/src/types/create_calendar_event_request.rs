pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCalendarEventRequest {
    /// Your idempotency key for this create. Retrying with the same `client_id` and the same body
    /// returns the original event with `200` instead of creating a second one, for as long as the
    /// event exists (`404` once it has been deleted). Reusing it with a different body is a `409`
    /// `idempotency_conflict`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Title of the event. 1 to 1,000 characters.
    #[serde(default)]
    pub title: String,
    /// Description of the event. At most 65,536 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Location of the event. At most 1,024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Your own JSON value (usually an object), returned on reads and in webhooks (except agenda and instance list items, and the webhook for a date cancelled with `DELETE`). Never sent to attendees. At most 16,384 bytes serialized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Defaults to `confirmed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CalendarEventStatus>,
    /// Set to `true` for an all-day event. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_day: Option<bool>,
    /// Local start time in `timezone`: `YYYY-MM-DDTHH:mm:ss` for timed events, `YYYY-MM-DD` for
    /// all-day events. For a recurring event, this is the first date.
    #[serde(default)]
    pub start: WallTime,
    /// Local end time in `timezone`, in the same format as `start`. For timed events it must be
    /// after `start`; for all-day events it is the last day (inclusive) and can equal `start`.
    /// One-off events can last up to 366 days; each date of a recurring event up to 31 days.
    #[serde(default)]
    pub end: WallTime,
    /// Time zone of `start` and `end`. Defaults to the calendar's `timezone`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<IanaTimezone>,
    /// Timed events only. Defaults to `exact` for one-off events and `nominal` for recurring events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_mode: Option<DurationMode>,
    /// Makes this a recurring event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<RecurrenceInput>,
    /// Attendees of the event. At most 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,
    /// When `true`, the inbox emails an invitation (iCalendar `REQUEST`) to every attendee. Defaults
    /// to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_invites: Option<bool>,
}

impl CreateCalendarEventRequest {
    pub fn builder() -> CreateCalendarEventRequestBuilder {
        <CreateCalendarEventRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCalendarEventRequestBuilder {
    client_id: Option<String>,
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
    recurrence: Option<RecurrenceInput>,
    attendees: Option<Vec<Attendee>>,
    send_invites: Option<bool>,
}

impl CreateCalendarEventRequestBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
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

    pub fn recurrence(mut self, value: RecurrenceInput) -> Self {
        self.recurrence = Some(value);
        self
    }

    pub fn attendees(mut self, value: Vec<Attendee>) -> Self {
        self.attendees = Some(value);
        self
    }

    pub fn send_invites(mut self, value: bool) -> Self {
        self.send_invites = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCalendarEventRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](CreateCalendarEventRequestBuilder::title)
    /// - [`start`](CreateCalendarEventRequestBuilder::start)
    /// - [`end`](CreateCalendarEventRequestBuilder::end)
    pub fn build(self) -> Result<CreateCalendarEventRequest, BuildError> {
        Ok(CreateCalendarEventRequest {
            client_id: self.client_id,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description,
            location: self.location,
            metadata: self.metadata,
            status: self.status,
            all_day: self.all_day,
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            timezone: self.timezone,
            duration_mode: self.duration_mode,
            recurrence: self.recurrence,
            attendees: self.attendees,
            send_invites: self.send_invites,
        })
    }
}

