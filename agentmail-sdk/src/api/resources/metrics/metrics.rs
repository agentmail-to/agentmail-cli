use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MetricsClient {
    pub http_client: HttpClient,
}

impl MetricsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Counts of email events (sent, delivered, bounced, etc.) over time for
    /// the organization. Defaults to the last 24 hours; `start` must be within
    /// the last 90 days, and a future `end` is clamped to now. Omit `period`
    /// for individual event counts, or set it to sum counts into buckets of
    /// that many seconds.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail metrics query-events
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .metrics
    ///         .query_events(
    ///             &MetricsQueryEventsQueryRequest {
    ///                 event_types: vec![],
    ///                 start: None,
    ///                 end: None,
    ///                 period: None,
    ///                 limit: None,
    ///                 descending: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn query_events(
        &self,
        request: &MetricsQueryEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryMetricsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/metrics/events",
                None,
                QueryBuilder::new()
                    .serialize_array("event_types", request.event_types.clone())
                    .serialize("start", request.start.clone())
                    .serialize("end", request.end.clone())
                    .serialize("period", request.period.clone())
                    .serialize("limit", request.limit.clone())
                    .serialize("descending", request.descending.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Cumulative usage series for the organization. Each point is the running
    /// total of the usage type at that timestamp, not the change within the
    /// bucket. Defaults to the last 24 hours; `start` must be within the last
    /// 90 days, and a future `end` is clamped to now. The range divided by
    /// `period` must not exceed 1000 buckets.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .metrics
    ///         .query_usage(
    ///             &MetricsQueryUsageQueryRequest {
    ///                 usage_types: vec![],
    ///                 start: None,
    ///                 end: None,
    ///                 period: None,
    ///                 limit: None,
    ///                 descending: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn query_usage(
        &self,
        request: &MetricsQueryUsageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryUsageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/metrics/usage",
                None,
                QueryBuilder::new()
                    .serialize_array("usage_types", request.usage_types.clone())
                    .serialize("start", request.start.clone())
                    .serialize("end", request.end.clone())
                    .serialize("period", request.period.clone())
                    .serialize("limit", request.limit.clone())
                    .serialize("descending", request.descending.clone())
                    .build(),
                options,
            )
            .await
    }
}
