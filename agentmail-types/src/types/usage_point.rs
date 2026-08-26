pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsagePoint {
    /// Timestamp of the point.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Cumulative value of the usage metric at the timestamp.
    #[serde(default)]
    pub value: i64,
}

impl UsagePoint {
    pub fn builder() -> UsagePointBuilder {
        <UsagePointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsagePointBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    value: Option<i64>,
}

impl UsagePointBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsagePoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp`](UsagePointBuilder::timestamp)
    /// - [`value`](UsagePointBuilder::value)
    pub fn build(self) -> Result<UsagePoint, BuildError> {
        Ok(UsagePoint {
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
