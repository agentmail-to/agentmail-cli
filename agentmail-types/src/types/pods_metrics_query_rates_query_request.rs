pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for query-rates
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodsMetricsQueryRatesQueryRequest {
    #[serde(default)]
    pub rate_types: Vec<Option<RateType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Start>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<End>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<RatePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<Window>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<MetricLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descending: Option<Descending>,
}

impl PodsMetricsQueryRatesQueryRequest {
    pub fn builder() -> PodsMetricsQueryRatesQueryRequestBuilder {
        <PodsMetricsQueryRatesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodsMetricsQueryRatesQueryRequestBuilder {
    rate_types: Option<Vec<Option<RateType>>>,
    start: Option<Start>,
    end: Option<End>,
    period: Option<RatePeriod>,
    window: Option<Window>,
    limit: Option<MetricLimit>,
    descending: Option<Descending>,
}

impl PodsMetricsQueryRatesQueryRequestBuilder {
    pub fn rate_types(mut self, value: Vec<Option<RateType>>) -> Self {
        self.rate_types = Some(value);
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

    pub fn period(mut self, value: RatePeriod) -> Self {
        self.period = Some(value);
        self
    }

    pub fn window(mut self, value: Window) -> Self {
        self.window = Some(value);
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

    /// Consumes the builder and constructs a [`PodsMetricsQueryRatesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rate_types`](PodsMetricsQueryRatesQueryRequestBuilder::rate_types)
    pub fn build(self) -> Result<PodsMetricsQueryRatesQueryRequest, BuildError> {
        Ok(PodsMetricsQueryRatesQueryRequest {
            rate_types: self.rate_types.ok_or_else(|| BuildError::missing_field("rate_types"))?,
            start: self.start,
            end: self.end,
            period: self.period,
            window: self.window,
            limit: self.limit,
            descending: self.descending,
        })
    }
}

