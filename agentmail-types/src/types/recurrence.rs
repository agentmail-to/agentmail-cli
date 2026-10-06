pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The recurrence of a series, as stored.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Recurrence {
    /// Normalized RRULE body, without the `RRULE:` prefix.
    #[serde(default)]
    pub rule: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exdates: Option<Vec<WallTime>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdates: Option<Vec<RecurrenceDate>>,
    /// Set when the series was ended with a `mode=future` delete: no date starting at or after
    /// this wall-clock time occurs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate_before: Option<WallTime>,
}

impl Recurrence {
    pub fn builder() -> RecurrenceBuilder {
        <RecurrenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurrenceBuilder {
    rule: Option<String>,
    exdates: Option<Vec<WallTime>>,
    rdates: Option<Vec<RecurrenceDate>>,
    truncate_before: Option<WallTime>,
}

impl RecurrenceBuilder {
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

    pub fn truncate_before(mut self, value: WallTime) -> Self {
        self.truncate_before = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Recurrence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule`](RecurrenceBuilder::rule)
    pub fn build(self) -> Result<Recurrence, BuildError> {
        Ok(Recurrence {
            rule: self.rule.ok_or_else(|| BuildError::missing_field("rule"))?,
            exdates: self.exdates,
            rdates: self.rdates,
            truncate_before: self.truncate_before,
        })
    }
}
