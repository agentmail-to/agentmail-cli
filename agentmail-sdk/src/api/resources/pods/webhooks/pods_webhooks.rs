use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient3 {
    pub http_client: HttpClient,
}

impl WebhooksClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods webhooks list --pod-id <pod_id>
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
    ///         .list(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &PodsWebhooksListQueryRequest {
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
        request: &PodsWebhooksListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksListWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/webhooks", pod_id.0),
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

    /// Create a webhook scoped to this pod.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail pods webhooks create --pod-id <pod_id> --url https://example.com/webhook --event-types message.received
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
    ///         .create(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &WebhooksCreatePodWebhookRequest {
    ///                 webhooks_create_inbox_webhook_request_fields: WebhooksCreateInboxWebhookRequest {
    ///                     url: WebhooksURL("url".to_string()),
    ///                     event_types: WebhooksCreateWebhookEventTypes(EventTypes(vec![
    ///                         EventType::MessageReceived,
    ///                     ])),
    ///                     ..Default::default()
    ///                 },
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
        request: &WebhooksCreatePodWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/pods/{}/webhooks", pod_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods webhooks get --pod-id <pod_id> --webhook-id <webhook_id>
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
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/webhooks/{}", pod_id.0, webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods webhooks delete --pod-id <pod_id> --webhook-id <webhook_id>
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
                &format!("v0/pods/{}/webhooks/{}", pod_id.0, webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods webhooks update --pod-id <pod_id> --webhook-id <webhook_id> --add-inbox-ids <inbox_id>
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
    ///         .update(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             &WebhooksUpdatePodWebhookRequest {
    ///                 webhooks_update_inbox_webhook_request_fields: WebhooksUpdateInboxWebhookRequest {
    ///                     ..Default::default()
    ///                 },
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        pod_id: &PodsPodId,
        webhook_id: &WebhooksWebhookId,
        request: &WebhooksUpdatePodWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/pods/{}/webhooks/{}", pod_id.0, webhook_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the names of custom HTTP headers included with deliveries to this pod-scoped webhook.
    /// Header values are write-only and are never returned.
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
    ///         .get_headers(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_headers(
        &self,
        pod_id: &PodsPodId,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhookHeaderNamesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/webhooks/{}/headers", pod_id.0, webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Atomically set, replace, or remove custom HTTP headers included with deliveries to this
    /// pod-scoped webhook. Header values remain write-only.
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
    ///         .update_headers(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             &WebhooksUpdateWebhookHeadersRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_headers(
        &self,
        pod_id: &PodsPodId,
        webhook_id: &WebhooksWebhookId,
        request: &WebhooksUpdateWebhookHeadersRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/pods/{}/webhooks/{}/headers", pod_id.0, webhook_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
