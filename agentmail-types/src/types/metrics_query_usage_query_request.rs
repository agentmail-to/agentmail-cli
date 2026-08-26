pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for query-usage
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MetricsQueryUsageQueryRequest {
    #[serde(default)]
    pub usage_types: Vec<Option<UsageType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Start>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<End>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<MetricLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descending: Option<Descending>,
}

impl MetricsQueryUsageQueryRequest {
    pub fn builder() -> MetricsQueryUsageQueryRequestBuilder {
        <MetricsQueryUsageQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricsQueryUsageQueryRequestBuilder {
    usage_types: Option<Vec<Option<UsageType>>>,
    start: Option<Start>,
    end: Option<End>,
    period: Option<Period>,
    limit: Option<MetricLimit>,
    descending: Option<Descending>,
}

impl MetricsQueryUsageQueryRequestBuilder {
    pub fn usage_types(mut self, value: Vec<Option<UsageType>>) -> Self {
        self.usage_types = Some(value);
        self
    }

    pub fn start(mut self, value: Start) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: End) -> Self {
        self.end = Some(value);
        self
    }

    pub fn period(mut self, value: Period) -> Self {
        self.period = Some(value);
        self
    }

    pub fn limit(mut self, value: MetricLimit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn descending(mut self, value: Descending) -> Self {
        self.descending = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetricsQueryUsageQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`usage_types`](MetricsQueryUsageQueryRequestBuilder::usage_types)
    pub fn build(self) -> Result<MetricsQueryUsageQueryRequest, BuildError> {
        Ok(MetricsQueryUsageQueryRequest {
            usage_types: self.usage_types.ok_or_else(|| BuildError::missing_field("usage_types"))?,
            start: self.start,
            end: self.end,
            period: self.period,
            limit: self.limit,
            descending: self.descending,
        })
    }
}

