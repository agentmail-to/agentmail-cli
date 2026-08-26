use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MetricsClient3 {
    pub http_client: HttpClient,
}

impl MetricsClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Counts of email events (sent, delivered, bounced, etc.) over time for
    /// the pod. Defaults to the last 24 hours; `start` must be within the last
    /// 90 days, and a future `end` is clamped to now. Omit `period` for
    /// individual event counts, or set it to sum counts into buckets of that
    /// many seconds.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail pods metrics query-events --pod-id <pod_id>
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
    ///         .pods
    ///         .metrics
    ///         .query_events(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &PodsMetricsQueryEventsQueryRequest {
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
        pod_id: &PodsPodId,
        request: &PodsMetricsQueryEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryMetricsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/metrics/events", pod_id.0),
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

    /// Cumulative usage series for the pod. Each point is the running total of
    /// the usage type at that timestamp, not the change within the bucket.
    /// Pod-scoped queries carry every usage type except `pod_count`; requested
    /// types that don't apply to the scope are ignored. Defaults to the last
    /// 24 hours; `start` must be within the last 90 days, and a future `end`
    /// is clamped to now. The range divided by `period` must not exceed 1000
    /// buckets.
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
    ///         .pods
    ///         .metrics
    ///         .query_usage(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &PodsMetricsQueryUsageQueryRequest {
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
        pod_id: &PodsPodId,
        request: &PodsMetricsQueryUsageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryUsageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/metrics/usage", pod_id.0),
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
