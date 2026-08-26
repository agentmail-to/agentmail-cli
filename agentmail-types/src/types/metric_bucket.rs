pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MetricBucket {
    /// Timestamp of the bucket.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Count of events in the bucket.
    #[serde(default)]
    pub count: i64,
}

impl MetricBucket {
    pub fn builder() -> MetricBucketBuilder {
        <MetricBucketBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricBucketBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    count: Option<i64>,
}

impl MetricBucketBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetricBucket`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp`](MetricBucketBuilder::timestamp)
    /// - [`count`](MetricBucketBuilder::count)
    pub fn build(self) -> Result<MetricBucket, BuildError> {
        Ok(MetricBucket {
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}
