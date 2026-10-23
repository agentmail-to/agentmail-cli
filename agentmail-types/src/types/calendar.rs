pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An inbox's calendar. Every inbox has exactly one calendar; it exists from the moment the inbox
/// does and is deleted with it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Calendar {
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    /// Default time zone for events created without a `timezone`. Defaults to `UTC`. Changing it
    /// does not move existing events.
    #[serde(default)]
    pub timezone: IanaTimezone,
    /// Number of one-off and recurring events on the calendar. Each recurring event counts once.
    #[serde(default)]
    pub event_count: i64,
    /// Time at which the calendar was created. The calendar exists from the moment the inbox does, so this is the inbox's creation time.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Time at which the calendar was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The calendar's current version, also sent as the `ETag` response header. Send it in
    /// `If-Match` to make an update conditional.
    #[serde(default)]
    pub etag: String,
}

impl Calendar {
    pub fn builder() -> CalendarBuilder {
        <CalendarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarBuilder {
    inbox_id: Option<InboxesInboxId>,
    timezone: Option<IanaTimezone>,
    event_count: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    etag: Option<String>,
}

impl CalendarBuilder {
    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn timezone(mut self, value: IanaTimezone) -> Self {
        self.timezone = Some(value);
        self
    }

    pub fn event_count(mut self, value: i64) -> Self {
        self.event_count = Some(value);
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

    /// Consumes the builder and constructs a [`Calendar`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](CalendarBuilder::inbox_id)
    /// - [`timezone`](CalendarBuilder::timezone)
    /// - [`event_count`](CalendarBuilder::event_count)
    /// - [`created_at`](CalendarBuilder::created_at)
    /// - [`updated_at`](CalendarBuilder::updated_at)
    /// - [`etag`](CalendarBuilder::etag)
    pub fn build(self) -> Result<Calendar, BuildError> {
        Ok(Calendar {
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            timezone: self.timezone.ok_or_else(|| BuildError::missing_field("timezone"))?,
            event_count: self.event_count.ok_or_else(|| BuildError::missing_field("event_count"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            etag: self.etag.ok_or_else(|| BuildError::missing_field("etag"))?,
        })
    }
}
