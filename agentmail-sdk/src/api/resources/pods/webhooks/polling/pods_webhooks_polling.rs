use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PollingClient2 {
    pub http_client: HttpClient,
}

impl PollingClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the polling webhooks created through this pod route.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail pods webhooks polling list --pod-id <pod_id>
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
    ///         .webhooks
    ///         .polling
    ///         .list(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &PodsWebhooksPollingListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        pod_id: &PodsPodId,
        request: &PodsWebhooksPollingListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksListPollingWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/webhooks/polling", pod_id.0),
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

    /// Create a polling webhook scoped to this pod and return the `token` that reads it.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail pods webhooks polling create --pod-id <pod_id> --client-id my-agent --event-types message.received
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
    ///         .webhooks
    ///         .polling
    ///         .create(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &WebhooksCreatePodPollingWebhookRequest {
    ///                 webhooks_create_inbox_polling_webhook_request_fields:
    ///                     WebhooksCreateInboxPollingWebhookRequest {
    ///                         client_id: WebhooksPollingClientID(WebhooksClientID(
    ///                             "client_id".to_string(),
    ///                         )),
    ///                         event_types: WebhooksCreateWebhookEventTypes(EventTypes(vec![
    ///                             EventType::MessageReceived,
    ///                         ])),
    ///                         ..Default::default()
    ///                     },
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        pod_id: &PodsPodId,
        request: &WebhooksCreatePodPollingWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksCreatePollingWebhookResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/pods/{}/webhooks/polling", pod_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods webhooks polling get --pod-id <pod_id> --webhook-id <webhook_id>
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
    ///         .webhooks
    ///         .polling
    ///         .get(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        pod_id: &PodsPodId,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksPollingWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/webhooks/polling/{}", pod_id.0, webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods webhooks polling delete --pod-id <pod_id> --webhook-id <webhook_id>
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
    ///         .pods
    ///         .webhooks
    ///         .polling
    ///         .delete(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        pod_id: &PodsPodId,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/pods/{}/webhooks/polling/{}", pod_id.0, webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
