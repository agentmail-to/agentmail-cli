use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient2 {
    pub http_client: HttpClient,
}

impl WebhooksClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes webhooks list --inbox-id <inbox_id>
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
    ///         .webhooks
    ///         .list(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesWebhooksListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        inbox_id: &InboxesInboxId,
        request: &InboxesWebhooksListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksListWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/webhooks", inbox_id.0),
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

    /// Create a webhook scoped to this inbox.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes webhooks create --inbox-id <inbox_id> --url https://example.com/webhook --event-types message.received
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
    ///         .webhooks
    ///         .create(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &WebhooksCreateInboxWebhookRequest {
    ///                 url: WebhooksURL("url".to_string()),
    ///                 event_types: WebhooksCreateWebhookEventTypes(EventTypes(vec![
    ///                     EventType::MessageReceived,
    ///                 ])),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        inbox_id: &InboxesInboxId,
        request: &WebhooksCreateInboxWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/webhooks", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes webhooks get --inbox-id <inbox_id> --webhook-id <webhook_id>
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
    ///         .webhooks
    ///         .get(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        inbox_id: &InboxesInboxId,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/webhooks/{}", inbox_id.0, webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes webhooks delete --inbox-id <inbox_id> --webhook-id <webhook_id>
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
    ///         .inboxes
    ///         .webhooks
    ///         .delete(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        inbox_id: &InboxesInboxId,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/inboxes/{}/webhooks/{}", inbox_id.0, webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes webhooks update --inbox-id <inbox_id> --webhook-id <webhook_id> --event-types message.received
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
    ///         .webhooks
    ///         .update(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             &WebhooksUpdateInboxWebhookRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        inbox_id: &InboxesInboxId,
        webhook_id: &WebhooksWebhookId,
        request: &WebhooksUpdateInboxWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/inboxes/{}/webhooks/{}", inbox_id.0, webhook_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the names of custom HTTP headers included with deliveries to this inbox-scoped webhook.
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
    ///         .inboxes
    ///         .webhooks
    ///         .get_headers(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_headers(
        &self,
        inbox_id: &InboxesInboxId,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhookHeaderNamesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v0/inboxes/{}/webhooks/{}/headers",
                    inbox_id.0, webhook_id.0
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Atomically set, replace, or remove custom HTTP headers included with deliveries to this
    /// inbox-scoped webhook. Header values remain write-only.
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
    ///         .inboxes
    ///         .webhooks
    ///         .update_headers(
    ///             &InboxesInboxID("inbox_id".to_string()),
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
        inbox_id: &InboxesInboxId,
        webhook_id: &WebhooksWebhookId,
        request: &WebhooksUpdateWebhookHeadersRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v0/inboxes/{}/webhooks/{}/headers",
                    inbox_id.0, webhook_id.0
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
