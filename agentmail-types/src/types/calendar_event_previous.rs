pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The values that a `calendar.event.updated` change replaced, for the fields it changed. Only
/// changed fields appear; a field that had no value before the change is absent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CalendarEventPrevious {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CalendarEventStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_day: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<WallTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<WallTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<IanaTimezone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_mode: Option<DurationMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,
}

impl CalendarEventPrevious {
    pub fn builder() -> CalendarEventPreviousBuilder {
        <CalendarEventPreviousBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventPreviousBuilder {
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
    recurrence: Option<Recurrence>,
    attendees: Option<Vec<Attendee>>,
}

impl CalendarEventPreviousBuilder {
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

    pub fn recurrence(mut self, value: Recurrence) -> Self {
        self.recurrence = Some(value);
        self
    }

    pub fn attendees(mut self, value: Vec<Attendee>) -> Self {
        self.attendees = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CalendarEventPrevious`].
    pub fn build(self) -> Result<CalendarEventPrevious, BuildError> {
        Ok(CalendarEventPrevious {
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
        })
    }
}
