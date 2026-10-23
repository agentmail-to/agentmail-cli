use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MetricsClient2 {
    pub http_client: HttpClient,
}

impl MetricsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Counts of email events (sent, delivered, bounced, etc.) over time for
    /// the inbox. Defaults to the last 24 hours; `start` must be within the
    /// last 90 days, and a future `end` is clamped to now. Omit `period` for
    /// individual event counts, or set it to sum counts into buckets of that
    /// many seconds.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes metrics query-events --inbox-id <inbox_id>
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
    ///         .inboxes
    ///         .metrics
    ///         .query_events(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesMetricsQueryEventsQueryRequest {
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
        inbox_id: &InboxesInboxId,
        request: &InboxesMetricsQueryEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryMetricsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/metrics/events", inbox_id.0),
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

    /// Cumulative usage series for the inbox. Each point is the running total
    /// of the usage type at that timestamp, not the change within the bucket.
    /// Inbox-scoped queries carry `storage_bytes`, `message_count`, and
    /// `thread_count`; requested types that don't apply to the scope are
    /// ignored. Defaults to the last 24 hours; `start` must be within the
    /// last 90 days, and a future `end` is clamped to now. The range divided
    /// by `period` must not exceed 1000 buckets.
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
    ///         .inboxes
    ///         .metrics
    ///         .query_usage(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesMetricsQueryUsageQueryRequest {
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
        inbox_id: &InboxesInboxId,
        request: &InboxesMetricsQueryUsageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryUsageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/metrics/usage", inbox_id.0),
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

    /// Rolling bounce and complaint rates for the inbox. At each `period`
    /// grid point, the bounced (or complained) messages over the preceding
    /// `window` divided by the messages sent over the same window, with the
    /// send count alongside. Account moderation evaluates the organization-wide
    /// rate, so use the organization endpoint to see the number it acts on;
    /// the inbox view shows which inboxes contribute. Defaults to the rolling
    /// 24-hour rate sampled hourly over the last day; `start` must be within
    /// the last 90 days, `window` must be a whole multiple of `period`, and
    /// the range plus window divided by `period` must not exceed 1000
    /// buckets.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes metrics query-rates --inbox-id <inbox_id>
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
    ///         .inboxes
    ///         .metrics
    ///         .query_rates(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesMetricsQueryRatesQueryRequest {
    ///                 rate_types: vec![],
    ///                 start: None,
    ///                 end: None,
    ///                 period: None,
    ///                 window: None,
    ///                 limit: None,
    ///                 descending: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn query_rates(
        &self,
        inbox_id: &InboxesInboxId,
        request: &InboxesMetricsQueryRatesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryRatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/metrics/rates", inbox_id.0),
                None,
                QueryBuilder::new()
                    .serialize_array("rate_types", request.rate_types.clone())
                    .serialize("start", request.start.clone())
                    .serialize("end", request.end.clone())
                    .serialize("period", request.period.clone())
                    .serialize("window", request.window.clone())
                    .serialize("limit", request.limit.clone())
                    .serialize("descending", request.descending.clone())
                    .build(),
                options,
            )
            .await
    }
}
