pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An extra date (RDATE) given as a period. Specify exactly one of `end` or `duration`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurrencePeriod {
    #[serde(default)]
    pub start: WallTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<WallTime>,
    /// ISO 8601 duration, for example `PT30M`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

impl RecurrencePeriod {
    pub fn builder() -> RecurrencePeriodBuilder {
        <RecurrencePeriodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurrencePeriodBuilder {
    start: Option<WallTime>,
    end: Option<WallTime>,
    duration: Option<String>,
}

impl RecurrencePeriodBuilder {
    pub fn start(mut self, value: WallTime) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: WallTime) -> Self {
        self.end = Some(value);
        self
    }

    pub fn duration(mut self, value: impl Into<String>) -> Self {
        self.duration = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecurrencePeriod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](RecurrencePeriodBuilder::start)
    pub fn build(self) -> Result<RecurrencePeriod, BuildError> {
        Ok(RecurrencePeriod {
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end,
            duration: self.duration,
        })
    }
}
