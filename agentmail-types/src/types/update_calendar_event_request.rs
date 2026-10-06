pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateCalendarEventRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Set to `null` to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Set to `null` to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Replaces the metadata (any JSON value, usually an object). Set to `null` to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CalendarEventStatus>,
    /// Changing `all_day` requires `start`, `end` and `timezone` in the same request. Not accepted for dated event IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_day: Option<bool>,
    /// New start. For a one-off or series UUID, send it together with `end`. Not accepted with `mode=future`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<WallTime>,
    /// New end. For a one-off or series UUID, send it together with `start`. A dated ID accepts `end` alone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<WallTime>,
    /// New time zone. Not accepted for dated event IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<IanaTimezone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_mode: Option<DurationMode>,
    /// Replaces the recurrence. Set to `null` to turn a series into a one-off event (only when no
    /// date of it has been edited). Not accepted for dated event IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<RecurrenceInput>,
    /// Replaces the attendee list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,
    /// When `true`, emails the updated invitation to every attendee. Only the organizer (an `api`
    /// event) can send. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_invites: Option<bool>,
    #[serde(skip)]
    pub mode: Option<InstanceMutationMode>,
}

impl UpdateCalendarEventRequest {
    pub fn builder() -> UpdateCalendarEventRequestBuilder {
        <UpdateCalendarEventRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCalendarEventRequestBuilder {
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
    mode: Option<InstanceMutationMode>,
}

impl UpdateCalendarEventRequestBuilder {
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

    pub fn mode(mut self, value: InstanceMutationMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCalendarEventRequest`].
    pub fn build(self) -> Result<UpdateCalendarEventRequest, BuildError> {
        Ok(UpdateCalendarEventRequest {
            title: self.title,
            description: self.description,
            location: self.location,
            metadata: self.metadata,
            status: self.status,
            all_day: self.all_day,
            start: self.start,
            end: self.end,
            timezone: self.timezone,
            duration_mode: self.duration_mode,
            recurrence: self.recurrence,
            attendees: self.attendees,
            send_invites: self.send_invites,
            mode: self.mode,
        })
    }
}

