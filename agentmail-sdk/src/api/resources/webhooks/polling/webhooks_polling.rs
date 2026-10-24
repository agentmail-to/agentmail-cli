use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PollingClient3 {
    pub http_client: HttpClient,
}

impl PollingClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the polling webhooks created through this route. `page_token` must come from this
    /// endpoint; a token from another list endpoint is rejected.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail webhooks polling list
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
    ///         .webhooks
    ///         .polling
    ///         .list(
    ///             &WebhooksPollingListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &WebhooksPollingListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksListPollingWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/webhooks/polling",
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .serialize("ascending", request.ascending.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a subscription the agent polls for events, and returns the `token` that reads it. Each
    /// organization, pod, and inbox holds at most 10 polling webhooks; a repeat create with the same
    /// `client_id` re-issues the token of the existing one instead of counting against that limit.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail webhooks polling create --client-id my-agent --event-types message.received
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
    ///         .webhooks
    ///         .polling
    ///         .create(
    ///             &WebhooksCreatePollingWebhookRequest {
    ///                 client_id: WebhooksPollingClientID(WebhooksClientID("client_id".to_string())),
    ///                 event_types: WebhooksCreateWebhookEventTypes(EventTypes(vec![
    ///                     EventType::MessageReceived,
    ///                 ])),
    ///                 inbox_ids: None,
    ///                 pod_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &WebhooksCreatePollingWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksCreatePollingWebhookResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v0/webhooks/polling",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// The `token` is never returned here: create again with the same `client_id` to get a new one.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail webhooks polling get --webhook-id <webhook_id>
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
    ///         .webhooks
    ///         .polling
    ///         .get(&WebhooksWebhookID("webhook_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksPollingWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/webhooks/polling/{}", webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Stops delivery and invalidates the subscription's token.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail webhooks polling delete --webhook-id <webhook_id>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .webhooks
    ///         .polling
    ///         .delete(&WebhooksWebhookID("webhook_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/webhooks/polling/{}", webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
