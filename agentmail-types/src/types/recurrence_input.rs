pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Repeats the event. `rule` is an RFC 5545 RRULE, evaluated in the event's `timezone` from the
/// event's `start`. Each date keeps its wall-clock time across daylight-saving changes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurrenceInput {
    /// RFC 5545 RRULE body, with or without the `RRULE:` prefix, for example
    /// `FREQ=WEEKLY;BYDAY=MO,WE,FR` or `FREQ=MONTHLY;BYDAY=-1FR;COUNT=12`. Supported parts: `FREQ`,
    /// `INTERVAL` (1 to 366), `COUNT` (1 to 10,000), `UNTIL` (`YYYYMMDD` or `YYYYMMDDTHHMMSSZ`),
    /// `WKST`, `BYMONTH`, `BYWEEKNO`, `BYYEARDAY`, `BYMONTHDAY`, `BYDAY`, `BYHOUR`, `BYMINUTE`,
    /// `BYSECOND` and `BYSETPOS`. `COUNT` and `UNTIL` are mutually exclusive. The rule is stored in
    /// a normalized form, so a response can differ textually from what you sent. At most 512
    /// characters after normalization.
    #[serde(default)]
    pub rule: String,
    /// Dates to skip, matching the original start of a date: `YYYY-MM-DDTHH:mm:ss` for timed events,
    /// `YYYY-MM-DD` for all-day events. At most 366.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exdates: Option<Vec<WallTime>>,
    /// Extra dates to add. Timed events take date-times or periods; all-day events take dates. At
    /// most 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdates: Option<Vec<RecurrenceDate>>,
}

impl RecurrenceInput {
    pub fn builder() -> RecurrenceInputBuilder {
        <RecurrenceInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurrenceInputBuilder {
    rule: Option<String>,
    exdates: Option<Vec<WallTime>>,
    rdates: Option<Vec<RecurrenceDate>>,
}

impl RecurrenceInputBuilder {
    pub fn rule(mut self, value: impl Into<String>) -> Self {
        self.rule = Some(value.into());
        self
    }

    pub fn exdates(mut self, value: Vec<WallTime>) -> Self {
        self.exdates = Some(value);
        self
    }

    pub fn rdates(mut self, value: Vec<RecurrenceDate>) -> Self {
        self.rdates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecurrenceInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule`](RecurrenceInputBuilder::rule)
    pub fn build(self) -> Result<RecurrenceInput, BuildError> {
        Ok(RecurrenceInput {
            rule: self.rule.ok_or_else(|| BuildError::missing_field("rule"))?,
            exdates: self.exdates,
            rdates: self.rdates,
        })
    }
}
