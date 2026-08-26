use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail webhooks list
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
    ///         .list(
    ///             &WebhooksListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &WebhooksListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksListWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/webhooks",
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

    /// **CLI:**
    /// ```bash
    /// agentmail webhooks create --url https://example.com/webhook --event-types message.received
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
    ///         .create(
    ///             &WebhooksCreateWebhookRequest {
    ///                 url: WebhooksURL("url".to_string()),
    ///                 event_types: WebhooksCreateWebhookEventTypes(EventTypes(vec![
    ///                     EventType::MessageReceived,
    ///                 ])),
    ///                 inbox_ids: None,
    ///                 client_id: None,
    ///                 headers: None,
    ///                 pod_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &WebhooksCreateWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v0/webhooks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail webhooks get --webhook-id <webhook_id>
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
    ///         .get(&WebhooksWebhookID("webhook_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/webhooks/{}", webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail webhooks delete --webhook-id <webhook_id>
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
                &format!("v0/webhooks/{}", webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Update inbox or pod subscriptions, or replace the webhook's `event_types` in full when you pass a
    /// non-empty `event_types` array (see request field docs). Inbox and pod changes use add/remove lists.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail webhooks update --webhook-id <webhook_id> --add-inbox-ids <inbox_id>
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
    ///         .update(
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             &WebhooksUpdateWebhookRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        webhook_id: &WebhooksWebhookId,
        request: &WebhooksUpdateWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/webhooks/{}", webhook_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the names of custom HTTP headers included with deliveries to this webhook. Header values are
    /// write-only and are never returned.
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
    ///         .get_headers(&WebhooksWebhookID("webhook_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_headers(
        &self,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhookHeaderNamesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/webhooks/{}/headers", webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Atomically set, replace, or remove custom HTTP headers included with deliveries to this webhook.
    /// Header values remain write-only.
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
    ///         .update_headers(
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
        webhook_id: &WebhooksWebhookId,
        request: &WebhooksUpdateWebhookHeadersRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/webhooks/{}/headers", webhook_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
