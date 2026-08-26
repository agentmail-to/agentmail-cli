pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for query-events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MetricsQueryEventsQueryRequest {
    #[serde(default)]
    pub event_types: Vec<Option<MetricEventType>>,
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

impl MetricsQueryEventsQueryRequest {
    pub fn builder() -> MetricsQueryEventsQueryRequestBuilder {
        <MetricsQueryEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricsQueryEventsQueryRequestBuilder {
    event_types: Option<Vec<Option<MetricEventType>>>,
    start: Option<Start>,
    end: Option<End>,
    period: Option<Period>,
    limit: Option<MetricLimit>,
    descending: Option<Descending>,
}

impl MetricsQueryEventsQueryRequestBuilder {
    pub fn event_types(mut self, value: Vec<Option<MetricEventType>>) -> Self {
        self.event_types = Some(value);
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

    /// Consumes the builder and constructs a [`MetricsQueryEventsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_types`](MetricsQueryEventsQueryRequestBuilder::event_types)
    pub fn build(self) -> Result<MetricsQueryEventsQueryRequest, BuildError> {
        Ok(MetricsQueryEventsQueryRequest {
            event_types: self.event_types.ok_or_else(|| BuildError::missing_field("event_types"))?,
            start: self.start,
            end: self.end,
            period: self.period,
            limit: self.limit,
            descending: self.descending,
        })
    }
}

